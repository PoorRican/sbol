use url::Url;

use crate::identified::Identified;

/// Abstract class that is extended by any `Identified`
///
/// Found at the top level of an SBOL Document or file. `TopLevel` objects are not nested inside
/// any other object, instead subordinate `TopLevel` objects are referred to.
pub trait TopLevel: Identified {
    /// URI that defines the namespace portion of URLs for this object and child objects.
    fn has_namespace(&self) -> Url;

    /// TopLevel can have 0 or more `Attachment` objects.
    fn has_attachment(&self) -> Vec<Url>;
}
