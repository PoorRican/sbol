use url::Url;

use super::{
    Ontology,
    SBO_NS, SO_NS, CHEBI_NS, GO_NS,
    INVALID_URI, JOIN_ERROR_MSG,
};

/// Component Type Ontologies
/// Pulled from SBOL 3 spec, Section 6.4, Table 2
pub enum ComponentTypeOntology {
    DNA,
    RNA,
    Protein,
    SimpleChemical,
    NonCovalentComplex,
    FunctionalEntity,
    Other(&'static str),
}
impl Ontology for ComponentTypeOntology {
    fn uri(&self) -> Url {
        let uri = match self {
            Self::DNA => SBO_NS.to_string() + "0000251",
            Self::RNA => SBO_NS.to_string() + "0000250",
            Self::Protein => SBO_NS.to_string() + "0000252",
            Self::SimpleChemical => SBO_NS.to_string() + "0000249",
            Self::NonCovalentComplex => SBO_NS.to_string() + "0000253",
            Self::FunctionalEntity => SBO_NS.to_string() + "0000241",
            // If `ComponentType::error` variant, return entire URI
            Self::Other(uri) => 
                return Url::parse(uri)
                    .expect(INVALID_URI) 
        };
        Url::parse(uri.as_str()).expect(INVALID_URI)
    }
}

/// Topology type field ontologies for `Component`
/// Pulled from SBOL 3 spec, Section 6.4, Table ___
pub enum TopologyOntology {
    Linear,
    Circular,
    SingleStranded,
    DoubleStranded,
    Other(&'static str),
}
impl Ontology for TopologyOntology {
    fn uri(&self) -> Url {
        let uri = match self {
            Self::Linear => SO_NS.to_string() + "0000987",
            Self::Circular => SO_NS.to_string() + "0000988",
            Self::SingleStranded => SO_NS.to_string() + "0000984",
            Self::DoubleStranded => SO_NS.to_string() + "0000985",
            Self::Other(uri) =>
                return Url::parse(uri)
                    .expect(INVALID_URI)
        };
        Url::parse(uri.as_str()).expect(JOIN_ERROR_MSG)
    }
}

/// Describe the role of a `Component` 
///
/// Might describe the role properties of a protein or simple chemical component, but can also
/// identify biological roles, such as "metabolic pathway" and "signaling cascade", or more
/// abstract roles describing the function of design such as "logical" roles (i.e: "inverter" or
/// "AND gate"). Interpretation of the meaning of such roles currently depends on the software
/// tools that read and write them.
///
/// Variants must align with `ComponentTypeOntology` and must not conflict.
#[allow(non_camel_case_types)]
pub enum ComponentRole {
    Promoter,
    RBS,
    CDS,
    Terminator,
    Gene,
    Operator,
    EngineeredRegion,
    mRNA,
    Effector,
    TranscriptionFactor,
    Other(&'static str),
}
impl Ontology for ComponentRole {
    fn uri(&self) -> Url {
        let uri = match self {
            Self::Promoter => SO_NS.to_string() + "0000167",
            Self::RBS => SO_NS.to_string() + "0000139",
            Self::CDS => SO_NS.to_string() + "0000316",
            Self::Terminator => SO_NS.to_string() + "0000141",
            Self::Gene => SO_NS.to_string() + "0000704",
            Self::Operator => SO_NS.to_string() + "0000057",
            Self::EngineeredRegion => SO_NS.to_string() + "0000804",
            Self::mRNA => SO_NS.to_string() + "0000234",
            Self::Effector => CHEBI_NS.to_string() + "35224",
            Self::TranscriptionFactor => GO_NS.to_string() + "0003700",
            Self::Other(uri) =>
                return Url::parse(uri)
                    .expect(INVALID_URI),
        };
        Url::parse(uri.as_str()).expect(JOIN_ERROR_MSG)
    }
}


#[cfg(test)]
mod tests {
    use url::Url;

    use crate::ontologies::{ComponentTypeOntology, ComponentRole, Ontology, TopologyOntology};


    #[test]
    fn test_topology_ontology() {
        let variants = [
            (TopologyOntology::Linear,     "https://identifiers.org/SO:0000987"),
            (TopologyOntology::Circular,     "https://identifiers.org/SO:0000988"),
            (TopologyOntology::SingleStranded,     "https://identifiers.org/SO:0000984"),
            (TopologyOntology::DoubleStranded,     "https://identifiers.org/SO:0000985"),
            (TopologyOntology::Other("https://test.org"),     "https://test.org"),
        ];
        for (variant, expected) in variants.iter() {
            assert_eq!(variant.uri(), Url::parse(expected).unwrap())
        }
    }

    #[test]
    fn test_component_type() {
        let variants = [
            (ComponentTypeOntology::DNA,     "https://identifiers.org/SBO:0000251"),
            (ComponentTypeOntology::RNA,     "https://identifiers.org/SBO:0000250"),
            (ComponentTypeOntology::Protein, "https://identifiers.org/SBO:0000252"),
            (ComponentTypeOntology::SimpleChemical,     "https://identifiers.org/SBO:0000249"),
            (ComponentTypeOntology::NonCovalentComplex, "https://identifiers.org/SBO:0000253"),
            (ComponentTypeOntology::FunctionalEntity,   "https://identifiers.org/SBO:0000241"),
        ];
        for (variant, expected) in variants.iter() {
            assert_eq!(variant.uri(), Url::parse(expected).unwrap())
        }
    }

    #[test]
    fn test_component_role() {
        let variants = [
            (ComponentRole::Promoter,     "https://identifiers.org/SO:0000167"),
            (ComponentRole::RBS,     "https://identifiers.org/SO:0000139"),
            (ComponentRole::CDS, "https://identifiers.org/SO:0000316"),
            (ComponentRole::Terminator,     "https://identifiers.org/SO:0000141"),
            (ComponentRole::Gene, "https://identifiers.org/SO:0000704"),
            (ComponentRole::Operator,   "https://identifiers.org/SO:0000057"),
            (ComponentRole::EngineeredRegion,   "https://identifiers.org/SO:0000804"),
            (ComponentRole::mRNA,   "https://identifiers.org/SO:0000234"),
            (ComponentRole::Effector,   "https://identifiers.org/CHEBI:35224"),
            (ComponentRole::TranscriptionFactor,   "https://identifiers.org/GO:0003700"),
            (ComponentRole::Other("https://test.com"),   "https://test.com"),
        ];
        for (variant, expected) in variants.iter() {
            assert_eq!(variant.uri(), Url::parse(expected).unwrap())
        }
    }
}
