use url::Url;

use crate::ontologies::{Ontology, EDAM_NS, INVALID_URI};

/// Indicates how `Sequence::elements` are formed and interpreted
/// Pulled from SBOL 3 spec, Chapter 6.3, Table 1
///
/// # Variants
/// - NucleicAcid => IUPAC DNA, RNA
/// - Protein => IUPAC DNA, RNA
/// - InChl =>
/// - SMILES => Atoms and chemical bonds of a small molecule
pub enum Encoding {
    NucleicAcid,
    Protein,
    InChl,
    SMILES,
    Other(&'static str),
}
impl Ontology for Encoding {
    fn uri(&self) -> url::Url {
        let uri = EDAM_NS.to_string()
            + match self {
                Self::NucleicAcid => "format_1207",
                Self::Protein => "format_1208",
                Self::InChl => "format_1197",
                Self::SMILES => "format_1196",
                Self::Other(uri) => return Url::parse(uri).expect(INVALID_URI),
            };
        Url::parse(uri.as_str()).expect(INVALID_URI)
    }
}

#[cfg(test)]
mod tests {
    use url::Url;

    use crate::ontologies::{Encoding, Ontology};

    #[test]
    fn test_topology_ontology() {
        let variants = [
            (
                Encoding::NucleicAcid,
                "https://identifiers.org/edam:format_1207",
            ),
            (
                Encoding::Protein,
                "https://identifiers.org/edam:format_1208",
            ),
            (Encoding::InChl, "https://identifiers.org/edam:format_1197"),
            (Encoding::SMILES, "https://identifiers.org/edam:format_1196"),
            (Encoding::Other("https://test.org"), "https://test.org"),
        ];
        for (variant, expected) in variants.iter() {
            assert_eq!(variant.uri(), Url::parse(expected).unwrap())
        }
    }
}
