use std::{io, str::Utf8Error};

use sqlx;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum RustySoapError {
    // Error(Error),
    // XMLSyntaxError(Error),
    // XMLParseError(XMLParseError),
    // UnexpectedElementError(Error),
    // WsdlSyntaxError(Error),
    // TransportError(TransportError),
    // LookupError(LookupError),
    // NamespaceError(Error),
    // Fault(Fault),
    // ValidationError(ValidationError),
    // SignatureVerificationFailed(Error),
    // IncompleteMessage(Error),
    // IncompleteOperation(Error),
    #[error(transparent)]
    Base64Error(#[from] base64::DecodeError),
    #[error(transparent)]
    SQLiteError(#[from] sqlx::Error),
    #[error(transparent)]
    UTF8ConversionError(#[from] Utf8Error),
    #[error(transparent)]
    FileError(#[from] io::Error),

    #[error("Error could not be determined")]
    Empty,
}

#[derive(Debug)]
pub struct Error(String);

#[derive(Debug)]
pub struct XMLParseError {
    filename: String,
    sourceline: String,
}

#[derive(Debug)]
pub struct TransportError {
    status_code: u32,
    content: String,
}
#[derive(Debug)]
pub struct LookupError {
    qname: String,
    item_name: String,
    location: String,
}

// TODO These types may be incorrect. Hard to guess from the original code
#[derive(Debug)]
pub struct Fault {
    message: String,
    code: String,
    actor: String,
    detail: String,
    subcodes: String,
}

#[derive(Debug)]
pub struct ValidationError {
    path: String,
}
