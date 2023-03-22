use crate::ontologies::Encoding;

/// Represents the primary structure of a `Component` object and the manner in which it is encoded.
///
/// Representation is accomplished by means of the `elements` property and `encoding` property
pub struct Sequence {
    /// Optional string of characters that represents the constituents of a biological or chemical
    /// molecule.
    ///
    /// For example, these characters could represent nucleotide bases, amino acid residues, or the
    /// atoms and chemical bonds of a small molecule.
    ///
    /// If the `elements` property is not set, then it means the particulars of this Sequence have
    /// not yet been determined.
    pub elements: Option<String>,

    /// Indicates how `elements` property of a `Sequence` are formed and interpreted.
    ///
    /// Should be a URI indentifying from the textual format branch of the EDAM ontology.
    ///
    /// It is optional, unless `elements` is set, in that case is required. 
    ///
    /// A partial list of possible URI values:
    ///     *============================================================*
    ///     | Encoding        | URI                                      |
    ///     *============================================================*
    ///     | IUPAC DNA, RNA  | https://identifiers.org/edam:format_1207 |
    ///     | IUPAC Protein   | https://identifiers.org/edam:format_1208 |
    ///     | InChl           | https://identifiers.org/edam:format_1197 |
    ///     | SMILES          | https://identifiers.org/edam:format_1196 |
    ///     *============================================================*
    ///     When the `encoding` of a `Sequence` is well described by one of these, than it must
    ///     contain that URI.
    ///
    pub encoding: Option<Encoding>,
}
