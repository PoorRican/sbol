mod component;
mod feature;
mod namespaces;
mod sequence;

pub use component::*;
pub use feature::*;
pub use namespaces::*;
pub use sequence::*;

use url::Url;

/// Generic error message when joining URI/URL
const JOIN_ERROR_MSG: &str = "Error joining URI";

/// Generic error message when parsing URI/URL
const INVALID_URI: &str = "Error parsing URI";

/// Interface for strictly type-checked ontologies
trait Ontology {
    fn uri(&self) -> Url;
}
