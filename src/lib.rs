#[macro_use]
extern crate lazy_static;

mod cache;
mod exceptions;
mod ns;
mod settings;
mod xsd;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
