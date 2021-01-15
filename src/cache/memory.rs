use std::{collections::HashMap, sync::Mutex};

use crate::exceptions::RustySoapError;
use async_trait::async_trait;
use chrono::Utc;
use log::debug;

use super::utils::{is_expired, Base};

lazy_static! {
    static ref IN_MEMORY_CACHE: Mutex<HashMap<String, (chrono::DateTime<Utc>, String)>> =
        Mutex::new(HashMap::new());
}

/// Simple in-memory caching using dict lookup with support for timeouts
pub struct InMemoryCache {
    timeout: Option<i64>,
}

impl InMemoryCache {
    pub fn new() -> Self {
        debug!("InMemoryCache");
        InMemoryCache {
            timeout: Some(36000),
        }
    }
}

#[async_trait]
impl Base for InMemoryCache {
    async fn add(&mut self, url: &str, content: &str) -> Result<(), RustySoapError> {
        debug!("Caching contents of {}", &url);
        IN_MEMORY_CACHE
            .lock()
            .unwrap()
            .insert(url.to_owned(), (Utc::now(), content.to_string()));
        Ok(())
    }

    async fn get(&mut self, url: &str) -> Result<Option<String>, RustySoapError> {
        let hm = IN_MEMORY_CACHE.lock().unwrap();
        let item = hm.get(url);
        return match item {
            Some((time, data)) if !is_expired(time, self.timeout) => {
                debug!("Cache HIT for {}", &url);
                Ok(Some(data.clone()))
            }
            _ => {
                debug!("Cache MISS for {}", &url);
                Ok(None)
            }
        };
    }
}

#[cfg(test)]
mod test_memory_cache {
    use super::*;
    use guerrilla;

    #[tokio::test]
    async fn memory_cache() {
        // timeout
        {
            IN_MEMORY_CACHE.lock().unwrap().clear();
            let mut c = InMemoryCache::new();
            let input = "content";
            c.add("http://tests.python-zeep.org/example.wsdl", input)
                .await
                .unwrap();
            let result = c.get("http://tests.python-zeep.org/example.wsdl").await;
            match result {
                Ok(Some(v)) => assert_eq!(v, input),
                _ => panic!("Does not contain a string"),
            };

            let _guard = guerrilla::patch2(is_expired, |_, _| true);
            let result = c
                .get("http://tests.python-zeep.org/example.wsdl")
                .await
                .unwrap();
            drop(_guard);
            assert_eq!(result, None);
        }

        // cache_share_data
        {
            IN_MEMORY_CACHE.lock().unwrap().clear();
            let mut c = InMemoryCache::new();
            let mut b = InMemoryCache::new();
            let input = "content";
            c.add("http://tests.python-zeep.org/example.wsdl", input)
                .await
                .unwrap();

            let _guard = guerrilla::patch2(is_expired, |_, _| false);
            let result = b.get("http://tests.python-zeep.org/example.wsdl").await;
            drop(_guard);

            match result {
                Ok(Some(v)) => {
                    assert_eq!(v, input);
                }
                _ => panic!("Does not contain a string"),
            };
        }
    }
}
