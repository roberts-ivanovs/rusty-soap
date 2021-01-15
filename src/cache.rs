use std::collections::HashMap;

use base64;
use chrono::{DateTime, Duration, NaiveDateTime, TimeZone, Utc};
use log::{debug, trace};

use crate::exceptions::RustySoapError;

/// Base class for caching backends.
trait Base {
    fn add(&mut self, url: &str, content: &str);
    fn get(&self, url: &str) -> Option<&String>;
}

/// Versioned base class for caching backends.
/// NOTE: when subclassing a version class attribute must be provided.
struct VersionCacheBase {
    version: String,
}

impl VersionCacheBase {
    /// Helper function for encoding cacheable content as base64.
    fn _encode_data(&self, data: &str) -> String {
        let data = base64::encode(data);
        self._version_string() + data.as_ref()
    }

    /// Helper function for decoding base64 cached content.
    fn _decode_data(&self, data: &str) -> Option<Result<Vec<u8>, RustySoapError>> {
        let version_string = self._version_string();
        let version = version_string.as_str();
        if data.starts_with(version) {
            // TODO CHECK IF THIS IS CORRECT
            let a = data.split_at(version.len()).0;
            let res = base64::decode(a).map_err(|source| RustySoapError::Base64Error(source));
            return Some(res);
        }
        None
    }

    /// Expose the version prefix to be used in content serialization.
    fn _version_string(&self) -> String {
        let prefix = format!("$ZEEP:{}$", self.version);
        prefix
    }
}

/// Simple in-memory caching using dict lookup with support for timeouts
struct InMemoryCache {
    cache: HashMap<String, (chrono::DateTime<Utc>, String)>,
    timeout: Option<i64>,
}

impl InMemoryCache {
    pub fn new() -> Self {
        InMemoryCache {
            cache: HashMap::new(),
            timeout: Some(36000),
        }
    }
}

impl Base for InMemoryCache {
    fn add(&mut self, url: &str, content: &str) {
        debug!("Caching contents of {}", &url);
        self.cache
            .insert(url.to_owned(), (Utc::now(), content.to_string()));
    }

    fn get(&self, url: &str) -> Option<&String> {
        let item = self.cache.get(url);
        return match item {
            Some((time, data)) if !is_expired(time, self.timeout) => {
                debug!("Cache HIT for {}", &url);
                Some(data)
            }
            _ => {
                debug!("Cache MISS for {}", &url);
                None
            }
        };
    }
}

/// Return boolean if the value is expired
fn is_expired(value: &DateTime<Utc>, timeout: Option<i64>) -> bool {
    match timeout {
        Some(timeout) => {
            let now = Utc::now();
            let max_age = *value + Duration::seconds(timeout);
            now > max_age
        }
        None => false,
    }
}

#[cfg(test)]
mod test_memory_cache {
    use super::*;
    use guerrilla;

    #[test]
    fn memory_cache_timeout() {
        let mut c = InMemoryCache::new();
        let input = "content";
        c.add("http://tests.python-zeep.org/example.wsdl", input);
        let result = c.get("http://tests.python-zeep.org/example.wsdl");

        match result {
            Some(v) => {
                assert_eq!(v, input);
            }
            None => panic!("Does not contain a string"),
        };

        let _guard = guerrilla::patch2(is_expired, |time, timeout| true);

        let result = c.get("http://tests.python-zeep.org/example.wsdl");
        drop(_guard);
        assert_eq!(result, None)
    }

    #[test]
    fn memory_cache_share_data() {
        let mut c = InMemoryCache::new();
        let input = "content";
        c.add("http://tests.python-zeep.org/example.wsdl", input);
        let result = c.get("http://tests.python-zeep.org/example.wsdl");

        match result {
            Some(v) => {
                assert_eq!(v, input);
            }
            _ => panic!("Does not contain a string"),
        };
    }
}

#[cfg(test)]
mod test_is_expired {
    use super::*;

    #[test]
    fn test_timeout_is_none() {
        let res = is_expired(&Utc::now(), None);
        assert!(!res)
    }

    #[test]
    fn test_timeout_has_expired() {
        let timeout = 7200;
        let utcnow = Utc::now();
        let value = utcnow - Duration::seconds(timeout);
        let res = is_expired(&value, Some(timeout));
        assert!(res)
    }

    #[test]
    fn test_timeout_has_not_expired() {
        let timeout = 7200;
        let utcnow = Utc::now();
        let value = utcnow + Duration::seconds(timeout);
        let res = is_expired(&value, Some(timeout));
        assert!(!res)
    }
}
