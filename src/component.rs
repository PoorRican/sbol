use url::Url;

use crate::ontologies::{ComponentRole, ComponentTypeOntology, TopologyOntology};
use crate::Sequence;

pub enum ComponentType {
    Type(ComponentTypeOntology),
    Topology(TopologyOntology),
}

/// Represents the structureal and/or functional entities of a biological design.
///
/// The primary usage is to represent entities with designed sequences, such as DNA, RNA, and
/// proteins, but it can also be used to represent any other entity that is part of a design, such
/// as simple chemicals, molecular complexes, strains, media, light, and abstract functional
/// groupings of other entities.
///
/// The component class uses the following properties: `type`, `role, `has_sequence`, `has_feature`,
/// `has_constraint`, `has_interaction`, `has_interface`, and `has_model`.
///
/// The `has_sequence`, `has_feature`, and `has_constraint` properties are used to represent
/// structural information, while the `has_interaction`, `has_interface`, and `has_model` are used
/// to represent functional information.
pub trait Component {
    /// Specifies the category of biochemical or physical entity.
    ///
    /// A `Component` must have one or more `type` properties that describe the entity for the
    /// purpose of engineering design. For nucleic acid entities, additional `type` properties may
    /// be used to describe nucleic acid topology (circular/linear) and strandedness (double- or
    /// single-stranded). `type` properties must be from the appropriate ontologies, such as the
    /// physical entity representation branch of the Systems Biology Ontology (SBO) or the ontology of
    /// Chemical Entities of Biological interest (ChEBI). In order to maximize the compatability of
    /// designs, the `type` property of a `Component` should contain a `URI` from the physical
    /// entity representation branch of the SBO. Any `Component` that can be well-described by one
    /// of the terms below must use that term as a `type`.
    ///
    /// Partial list:
    /// DNA     => https://identifiers.org/SBO:0000251
    /// RNA     => https://identifiers.org/SBO:0000250
    /// Protein => https://identifiers.org/SBO:0000252
    /// Simple Chemical      => https://identifiers.org/SBO:0000249
    /// Non-covalent Complex => https://identifiers.org/SBO:0000253
    /// Functional Entity    => https://identifiers.org/SBO:0000241
    ///
    /// If the `type` property contains multiple URIs, then they must identify non-conflicting
    /// terms (otherwise, it might not be clear how to interpret them, ie: RNA + DNA).
    ///
    /// ## Nucleic Acid Topolgy types
    /// Any `Component` classified as DNA is recommended to encode circular/linear topology
    /// information in an additional `type` field. This (topology) `type` field should specify a
    /// URI from the Topology Attribute branch of the Sequence Ontology (SO): this is currently
    /// just "linear" or "circular" as given below. Topology information should be specified for
    /// DNA `Component` records with a fully specified sequence, except in three scenarios: if the
    /// DNA record doesn't have sequence information, or if the DNA record has incomplete sequence
    /// information, or if topology is genuinely unknown. For any `Component` classified as RNA, a
    /// topology type field is optional. The default assumption in this case is linear topology.
    ///
    /// Any other type of `Component` record (protein, simple chemical, etc) should not have any
    /// type field pointing to SO terms from the topology or strand attribute branches of SO.
    ///
    /// Partial List of Topology Ontology:
    /// linear      => http://identifiers.org/SO:0000987
    /// circular    => http://identifiers.org/SO:0000988
    /// single-stranded     => http://identifiers.org/SO:0000984
    /// double-stranded     => http://identifiers.org/SO:0000985
    ///
    /// Note that a _circular_ topology instructs software to interpret the beginning / end
    /// position of a given sequence (be it DNA or RNA) as arbitrary, meanning that sequence
    /// features may be mapped or identified across this junction. _Double stranded_ instructs
    /// software to apply sequence searches to both strands (ie; sequence and reverse complement of
    /// sequence).
    fn r#type(&self) -> Vec<ComponentType>;

    /// Identify terms that are consistent with the `type` property.
    ///
    /// For example, the `role` property of a DNA or RNA `Component` could contain URIs identifying
    /// terms from the `Sequence Ontology` (SO). As a best practice, a `Component` should contain
    /// exactly one URI that refers to a term from the sequence feature branch of the SO.
    /// Similarly, the role properties of a protein and simple chemical `Component` should
    /// respectively contain URIs identifying terms from the "MolecularFunction" (GO:0003674)
    /// branch of the Gene Ontology (GO) and the `role` (CHEBI:50906) branch of the CHEBI ontology.
    ///
    /// These URIs might identify descriptive biological roles such as "metabolic pathway" and
    /// "signaling cascade", but they can also identify "logical" roles such as "inverter" or "AND
    /// gate", or other abstract roles for describing the function of design.
    ///
    /// Below is a partial list of role ontologies organized by type of `Component`:
    ///
    /// =================================================================================
    /// | Component Role       |   URI for Ontology Term            | Component Type    |
    /// |======================|====================================|===================|
    /// | Promoter             | http://identifiers.org/SO:0000167  | DNA               |
    /// | RBS                  | http://identifiers.org/SO:0000139  | DNA               |
    /// | CDS                  | http://identifiers.org/SO:0000316  | DNA               |
    /// | Terminator           | http://identifiers.org/SO:0000141  | DNA               |
    /// | Gene                 | http://identifiers.org/SO:0000704  | DNA               |
    /// | Operator             | http://identifiers.org/SO:0000057  | DNA               |
    /// | Engineered Region    | http://identifiers.org/SO:0000804  | DNA               |
    /// | mRNA                 | http://identifiers.org/SO:0000234  | DNA               |
    /// | Effector             | http://identifiers.org/CHEBI:35224 | Small Molecule    |
    /// | Transcription Factor | http://identifiers.org/GO:0003700  | Protein           |
    /// =================================================================================
    ///
    /// Any component that can be well described by one of the above must use the URI for that term
    /// as a `role`.
    fn role(&self) -> Vec<ComponentRole>;

    /// Return an arbitrary number of `Sequence` object URIs.
    ///
    /// These objects define the primary structure or structures of the `Component`. If a `Feature`
    /// of a `Component` refers to a `Location`, and this `Location` refers to a `Sequence`, then
    /// the `Component` must also include a `has_sequence` property that refers to this `Sequence`.
    ///
    /// Many `Component` objects will have exactly one `has_sequence` property that refers to a
    /// `Sequence` object. In this case, the `Sequence` must have appropriate IUPAC `encoding`.
    fn has_sequence(&self) -> Vec<Sequence>;

    /// Return an arbitrary number of `Feature` objects URIs.
    ///
    /// The set of relations between `Feature` and Component objects must be strictly acyclic.
    ///
    /// Taking the `Component` class as analagous to a blueprint or specification sheet for a
    /// biological part or a system of interacting biological elements, the `Feature` class
    /// represents the specific occurrence of a part, subsystem, or other notable aspect within
    /// that design. This mechanism also allows a biological design to incnlude multiple instances
    /// of a particular part (defined by reference to the same `Component`). For example, the
    /// `Component` of a polycistronic gene could contain two `SubComponent` objects that refer to
    /// the same `Component` of a CDS. As another example, consider the `Component` for a network
    /// of two-input repressor devices in which the particular repressors have not yet been chosen.
    /// This `Component` could contain multiple `SubComponent` objects that refer to the same
    /// `Component` of an abstract two-input repressor device.
    ///
    /// The `has_feature` properties of `Component` objects can be used to construct a hierarchy of
    /// `SubComponent` and `Component` objects. If a `Component` in such a hierarchy refers to a
    /// `Location` object that refers to the same `Sequence` with the same `encoding`, then the
    /// `elements` properties of these `Sequence` objects should be consistent with each other,
    /// such that well-defined mappings exist from the "lower level" `elements` to the "higher
    /// level" `elements` in accordance with their shared `encoding` properties. This mapping is
    /// also subject to any restructions on the positions of the `Feature` objects in the hierarchy
    /// that are imposed by the `SubComponent`, `SequenceFeature`, or `Constraint` objects
    /// contained by the `Component` objects in the hierarchy.
    ///
    /// For example, in a plasmid `Component` with a promoter `SubComponent`, the sequence at the
    /// promoter's `Location` within the plasmid should be the sequence for the promoter. More
    /// concretely, consider the DNA `Component` that refers to a `Sequence` with an IUPAC DNA
    /// `encoding` and an `elements` String of "gattaca". In turn, this `Component` could contain a
    /// `SubComponent` that refers to a "lower level" `Component` that also refers to a `Sequence`
    /// could be "gatta" or perhaps "tgta" if the `SubComponent` is positioned by a `Location` with
    /// an `orientation` of "reverse complement".
    fn has_feature(&self) -> Vec<Url>;

    fn has_constraint(&self) -> Vec<Url>;

    fn has_interaction(&self) -> Vec<Url>;

    fn has_interface(&self) -> Vec<Url>;

    fn has_model(&self) -> Vec<Url>;
}
