use base64;

use crate::exceptions::RustySoapError;

/// Base class for caching backends.
trait Base {
    fn add(&self, url: &str, content: &str);
    fn get(&self, url: &str);
}

/// Versioned base class for caching backends.
/// Note when subclassing a version class attribute must be provided.
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
            let res = base64::decode(a)
                .map_err(|source| RustySoapError::Base64Error(source));
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
