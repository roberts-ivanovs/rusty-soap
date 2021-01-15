use self::private_module::Settings;



mod private_module {
    pub struct Settings {
        pub strict: bool,
        pub raw_response: bool,
        pub force_https: bool,
        pub extra_https_headers: bool,
        pub xml_huge_tree: bool,
        pub forbid_dtd: bool,
        pub forbid_entities: bool,
        pub forbid_external: bool,
        pub xsd_ignore_sequence_order: bool
    }

    impl Settings {
        pub fn new() -> Self {
            Self {
                strict: true,
                raw_response: true,

                // transport
                force_https: false,
                extra_https_headers: false,

                // lxml processing
                xml_huge_tree: false,
                forbid_dtd: true,
                forbid_entities: true,
                forbid_external: true,

                // xsd workarounds
                xsd_ignore_sequence_order: false,
            }
        }
    }
}

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new();
}


#[cfg(test)]
mod settings_test {
    use super::*;
    #[test]
    fn settings_test() {
        // TODO finish this test

        // Make this changeable
        // SETTINGS.strict = false;

        assert_eq!(SETTINGS.strict, true);


        // Check that the original value returned
        assert_eq!(SETTINGS.raw_response, true);
    }
}
