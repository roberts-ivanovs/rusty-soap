use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use log::debug;
use sqlx::{self, Connection, Executor, SqliteConnection};

use async_trait::async_trait;

use crate::exceptions::RustySoapError;

use super::utils::{is_expired, Base, VersionCacheBase};

pub struct SQLiteCache {
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
            version_cache_base,
            timeout,
            path: path.to_owned(),
        })
    }

    async fn open_connection(path: &str) -> Result<SqliteConnection, RustySoapError> {
        SqliteConnection::connect(path)
            .await
            .map_err(|source| RustySoapError::SQLiteError(source))
    }
}

#[async_trait]
impl Base for SQLiteCache {
    async fn add(&mut self, url: &str, content: &str) -> Result<(), RustySoapError> {
        debug!("Caching contents of {}", url);
        let c = SQLiteCache::open_connection(&self.path);
        let data = self.version_cache_base.encode_data(content);

        let mut c = c.await?;
        sqlx::query("DELETE FROM request WHERE url = ?")
            .bind(url)
            .execute(&mut c)
            .await?;
        let now = Utc::now();
        sqlx::query!(
            "INSERT INTO request (created, url, content) VALUES (?1, ?2, ?3)",
            now,
            url,
            data
        )
        .execute(&mut c)
        .await?;
        Ok(())
    }

    async fn get(&self, url: &str) -> Result<Option<String>, RustySoapError> {
        let mut c = SQLiteCache::open_connection(&self.path).await?;
        let request: Request = sqlx::query_as!(
            Request,
            "SELECT created, content, url FROM request WHERE url=?",
            url
        )
        .fetch_one(&mut c)
        .await?;

        let date = DateTime::from_utc(request.created.unwrap(), Utc);
        if !is_expired(&date, self.timeout) {
            debug!("Cache HIT for {}", url);
            let data = request.content.unwrap();
            let data = self.version_cache_base.decode_data(&data).unwrap()?;
            let data = std::str::from_utf8(&data)?;
            return Ok(Some(data.to_owned()));
        }
        debug!("Cache MISS for {}", url);
        Ok(None)
    }
}
