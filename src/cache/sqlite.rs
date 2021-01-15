use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use log::debug;
use sqlx::{self, Connection, Executor, SqliteConnection};

use crate::exceptions::RustySoapError;
use async_trait::async_trait;
use std::path::Path;
use tokio::fs::File;

use super::utils::{is_expired, Base, VersionCacheBase};

pub struct SQLiteCache {
    connection: SqliteConnection,
    version_cache_base: VersionCacheBase,
    path: String,
    timeout: Option<i64>,
}
struct Request {
    created: Option<NaiveDateTime>,
    url: Option<String>,
    content: Option<String>,
}

impl SQLiteCache {
    pub async fn new(path: &str, timeout: Option<i64>) -> Result<Self, RustySoapError> {
        let mut conn = SQLiteCache::open_connection(path).await?;
        sqlx::query!(
            "CREATE TABLE IF NOT EXISTS request (created timestamp, url text, content text)",
        )
        .execute(&mut conn)
        .await?;

        let version_cache_base = VersionCacheBase::new("1".to_owned());
        Ok(Self {
            connection: conn,
            version_cache_base,
            timeout,
            path: path.to_owned(),
        })
    }

    async fn open_connection(path: &str) -> Result<SqliteConnection, RustySoapError> {
        if path != "sqlite::memory:" && !Path::new(path).exists() {
            File::create(path).await?;
        }
        SqliteConnection::connect(path)
            .await
            .map_err(|source| RustySoapError::SQLiteError(source))
    }
}

#[async_trait]
impl Base for SQLiteCache {
    async fn add(&mut self, url: &str, content: &str) -> Result<(), RustySoapError> {
        debug!("Caching contents of {}", url);
        // let c = SQLiteCache::open_connection(&self.path);
        // let c = self.connection;
        let data = self.version_cache_base.encode_data(content);

        // let mut c = c.await?;
        sqlx::query("DELETE FROM request WHERE url = ?")
            .bind(url)
            .execute(&mut self.connection)
            .await?;
        let now = Utc::now();
        sqlx::query!(
            "INSERT INTO request (created, url, content) VALUES (?1, ?2, ?3)",
            now,
            url,
            data
        )
        .execute(&mut self.connection)
        .await?;
        Ok(())
    }

    async fn get(&mut self, url: &str) -> Result<Option<String>, RustySoapError> {
        let request = sqlx::query_as!(
            Request,
            "SELECT created, content, url FROM request WHERE url=?",
            url
        )
        .fetch_one(&mut self.connection)
        .await;

        if request.is_err() {
            debug!("Cache MISS for {}", url);
            return Ok(None);
        }
        let request = request.unwrap();
        let date = DateTime::from_utc(request.created.unwrap(), Utc);
        if !is_expired(&date, self.timeout) {
            debug!("Cache HIT for {}", url);
            let data = request.content.unwrap();
            let data = self.version_cache_base.decode_data(&data).unwrap()?;
            let data = std::str::from_utf8(&data)?;
            return Ok(Some(data.to_owned()));
        } else {
            debug!("Cache MISS for {}", url);
            return Ok(None);
        }
    }
}

#[cfg(test)]
mod test_sqlite_cache {
    use super::*;
    use guerrilla;
    use tokio::fs;

    static DB_LOCATION: &str = ".test.db";

    async fn file_cleanup(file: &str) {
        fs::remove_file(file).await.ok();
    }

    #[tokio::test]
    async fn cache() {
        let path = &format!("{:}{:}", DB_LOCATION, "1");
        file_cleanup(path).await;
        let mut c = SQLiteCache::new(path, None).await.unwrap();
        c.add("http://tests.python-zeep.org/example.wsdl", "content")
            .await
            .unwrap();

        let res = c
            .get("http://tests.python-zeep.org/example.wsdl")
            .await
            .unwrap()
            .unwrap();

        file_cleanup(path).await;
        assert_eq!(res, "content");
    }

    #[tokio::test]
    async fn cache_memory() {
        // NOTE: THIS IS NOT THREAD SAFE. REMOVE FUNCTIONALITY?
        let path = "sqlite::memory:";
        let mut c = SQLiteCache::new(path, None).await.unwrap();
        c.add("http://tests.python-zeep.org/example.wsdl", "content")
            .await
            .unwrap();

        let res = c
            .get("http://tests.python-zeep.org/example.wsdl")
            .await
            .unwrap()
            .unwrap();

        assert_eq!(res, "content");
    }

    #[tokio::test]
    async fn no_records() {
        let path = &format!("{:}{:}", DB_LOCATION, "2");
        file_cleanup(path).await;
        let mut c = SQLiteCache::new(path, None).await.unwrap();

        let res = c
            .get("http://tests.python-zeep.org/example.wsdl")
            .await
            .unwrap();
        file_cleanup(path).await;
        assert_eq!(res, None);
    }

    #[tokio::test]
    async fn has_expired() {
        let path = &format!("{:}{:}", DB_LOCATION, "3");
        file_cleanup(path).await;
        let mut c = SQLiteCache::new(path, None).await.unwrap();
        c.add("http://tests.python-zeep.org/example.wsdl", "content")
            .await
            .unwrap();

        let _guard = guerrilla::patch2(is_expired, |_, _| true);
        let res = c
            .get("http://tests.python-zeep.org/example.wsdl")
            .await
            .unwrap();
        drop(_guard);

        file_cleanup(path).await;
        assert_eq!(res, None);
    }

    #[tokio::test]
    async fn has_not_expired() {
        let path = &format!("{:}{:}", DB_LOCATION, "4");
        file_cleanup(path).await;
        let mut c = SQLiteCache::new(path, None).await.unwrap();
        c.add("http://tests.python-zeep.org/example.wsdl", "content")
            .await
            .unwrap();

        let _guard = guerrilla::patch2(is_expired, |_, _| false);
        let res = c
            .get("http://tests.python-zeep.org/example.wsdl")
            .await
            .unwrap()
            .unwrap();
        drop(_guard);

        file_cleanup(path).await;
        assert_eq!(res, "content");
    }
}
