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
