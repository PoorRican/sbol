use url::Url;

/// Base interface implementations for Identified class.
///
/// This interface is used to derive (either directly or indirectly) all other SBOL objects. This
/// allows objects to be uniquely identified using URI's and referenced from within a SBOL document
/// or at locations on the web.
pub trait Identified {
    /// Intermediate between URI and name property
    ///
    /// This is equivalent to "displayId" as per the SBOL specification.
    ///
    /// Should be composed of only alphanumeric or underscore characters. Must not begin with a
    /// digit.
    ///
    /// # Note
    /// For objects whose URI is a URL, the requirements on URL structure imply that display_id
    /// _must_ be set.
    fn display_id(&self) -> Option<String>;

    /// Human readable name.
    ///
    /// To be used when visualizing object. However, if an object lacks a name, then `display_id`
    /// or `URI` should be rendered.
    fn name(&self) -> Option<String>;

    /// Thorough text description.
    fn description(&self) -> Option<String>;

    /// One or more non-SBOL resources or Identified objects from which this object was derived.
    ///
    /// An Identified object must not refer to itself via its own `derived_from` property, or form
    /// a cyclical chain of references (ie: A -> B, B -> A)
    ///
    /// The values in this vector are defined by the PROV-O ontology and is located in the
    /// https://www.w3.org/ns/prov# namespace.
    fn derived_from(&self) -> Vec<Url>;

    /// One or more Activity objects objects that describe how this object was generated.
    ///
    /// Must not form circular reference chains.
    fn generated_by(&self) -> Vec<Url>;

    /// Describes the measured parameters for this object.
    ///
    /// Each member of `Vec` should refer to a `om:Measure` object.
    ///
    /// `Measure` objects are defined by the OM ontology and is located in the
    /// "http://www.ontology-of-units-of-measure.org/resource/om-2" namespace.
    fn has_measure(&self) -> Vec<Url>;
}
