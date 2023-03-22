use url::Url;

/// Used to compose `Component` objects into a structural or functional hierarchy.
pub trait Feature {
    /// Describes the purpose or potential function in the context of its parent `Component`.
    ///
    /// If the `role` for a `SubComponent` is left unspecified, then the `role` is determined by
    /// the `role` property of the `Component` that it is an `instance_of`.
    ///
    fn role(&self) -> &Vec<Url>;

    fn orientation(&self) -> &Vec<Url>;
}

/// Subclass of the `Feature` class that can be used to specify structural hierarchy.
///
/// For example, the `Component` of a gene might contain four `SubComponent` objects: a promoter,
/// RMS, CDS, and terminator, each linked to the `Component` that provides the complete definition.
/// In turn, the `Component` of the promoter `SubComponent` might itself contain `SubComponent`
/// objects defining various operator sites, etc.
pub struct SubComponent {
    role: Vec<Url>,
    orientation: Vec<Url>,
}

impl SubComponent {
    /// Specifies the relationship between a `SubComponent` instance's own set of `role` properties
    /// and the set of `role` properties on the included `Component`.
    pub fn role_integration(&self) -> &Vec<Url> {
        todo!()
    }
}

impl Feature for SubComponent {
    fn role(&self) -> &Vec<Url> {
        &self.role
    }

    fn orientation(&self) -> &Vec<Url> {
        &self.orientation
    }
}
