use std::{collections::HashMap, sync::Mutex};

use base64;
use chrono::{DateTime, Duration, Utc};
use log::{debug, trace};
use async_trait::async_trait;

use crate::exceptions::RustySoapError;

/// Base class for caching backends.
#[async_trait]
pub trait Base {
    async fn add(&mut self, url: &str, content: &str) -> Result<(), RustySoapError>;
    async fn get(&self, url: &str) -> Result<Option<String>, RustySoapError>;
}

/// Versioned base class for caching backends.
/// NOTE: when subclassing a version class attribute must be provided.
pub struct VersionCacheBase {
    version: String,
}

impl VersionCacheBase {
    pub fn new(version: String) -> Self {
        Self { version }
    }

    /// Helper function for encoding cacheable content as base64.
    pub fn encode_data(&self, data: &str) -> String {
        let data = base64::encode(data);
        self.version_string() + data.as_ref()
    }

    /// Helper function for decoding base64 cached content.
    pub fn decode_data(&self, data: &str) -> Option<Result<Vec<u8>, RustySoapError>> {
        let version_string = self.version_string();
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
    fn version_string(&self) -> String {
        let prefix = format!("$ZEEP:{}$", self.version);
        prefix
    }
}



/// Return boolean if the value is expired
pub fn is_expired(value: &DateTime<Utc>, timeout: Option<i64>) -> bool {
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

// TODO Unit Test VersionCacheBase
