pub struct XmlParserContext<T> {
    pub schemas: Vec<T>,
}

impl<T> XmlParserContext<T> {
    pub fn new() -> Self {
        XmlParserContext { schemas: vec![] }
    }
}
