use ns;
use xmltree::Element;

#[derive(Debug)]
struct StaticIdentity {
    value: String
}
impl StaticIdentity {
    fn new(val: String) -> Self {
        Self {
            value: val
        }
    }
}
fn xsi_ns(localname: String) {
    return Element::parse(ns::XSI, localname); 
    // return etree.QName(ns.XSI, localname) <--
    
}

fn xsd_ns(localname: String) {
    return Element::parse(ns::XSD, localname);
    // return etree.QName(ns.XSD, localname) <--

}

static NotSet: StaticIdentity = StaticIdentity::new("NotSet");
static SkipValue: StaticIdentity = StaticIdentity::new("SkipValue");
static Nil: StaticIdentity = StaticIdentity::new("Nil");
