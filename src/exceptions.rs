#[derive(Debug)]
pub enum RustySoapError {
    Error(Error),
    XMLSyntaxError(Error),
    XMLParseError(XMLParseError),
    UnexpectedElementError(Error),
    WsdlSyntaxError(Error),
    TransportError(TransportError),
    LookupError(LookupError),
    NamespaceError(Error),
    Fault(Fault),
    ValidationError(ValidationError),
    SignatureVerificationFailed(Error),
    IncompleteMessage(Error),
    IncompleteOperation(Error),
    Empty,
}

pub struct Error(&str);

pub struct XMLParseError {
    filename: &str,
    sourceline: &str,
}

pub struct TransportError {
    status_code: u32,
    content: &str,
}
pub struct LookupError {
    qname: &str,
    item_name: &str,
    location: &str,
}

// TODO These types may be incorrect. Hard to guess from the original code
pub struct Fault {
    message: &str,
    code: &str,
    actor: &str,
    detail: &str,
    subcodes: &str,
}

pub struct ValidationError {
    path: &str,
}
