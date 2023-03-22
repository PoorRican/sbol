use url::Url;

use super::{Ontology, SO_NS, SBOL3_NS, INVALID_URI,};

pub enum Orientation {
    /// The region specified by this `Feature` or `Location` is on the `elements` of a `Sequence`
    Inline,
    /// The region specified by this `Feature` or `Location` is on the reverse-complement mapping
    /// of the `elements of a `Sequence`. The exact nature of this mapping depends on the
    /// `encoding` of the `Sequence`.
    ReverseComplement,
    InlineAlt,
    ReverseComplementAlt,
}
impl Ontology for Orientation {
    fn uri(&self) -> url::Url {
        let uri = match self {
            Self::Inline => SO_NS.to_string() + "0001030",
            Self::ReverseComplement => SO_NS.to_string() + "0001031",
            Self::InlineAlt => SBOL3_NS.to_string() + "inline",
            Self::ReverseComplementAlt => SBOL3_NS.to_string() + "reverseComplement",
        };
        Url::parse(uri.as_str()).expect(INVALID_URI)
    }
}


#[cfg(test)]
mod tests {
    use url::Url;

    use crate::ontologies::{Orientation, Ontology};

    #[test]
    fn test_inline() {
        let val = Orientation::Inline;
        let expected = Url::parse("https://identifiers.org/SO:0001030").unwrap();
        assert_eq!(val.uri(), expected);
    }
    #[test]
    fn test_reverse_complement() {
        let val = Orientation::ReverseComplement;
        let expected = Url::parse("https://identifiers.org/SO:0001031").unwrap();
        assert_eq!(val.uri(), expected);
    }
    #[test]
    fn test_inline_alt() {
        let val = Orientation::InlineAlt;
        let expected = Url::parse("https://sbols.org/v3#inline").unwrap();
        assert_eq!(val.uri(), expected);
    }
    #[test]
    fn test_reverse_complement_alt() {
        let val = Orientation::ReverseComplementAlt;
        let expected = Url::parse("https://sbols.org/v3#reverseComplement").unwrap();
        assert_eq!(val.uri(), expected);
    }
}
