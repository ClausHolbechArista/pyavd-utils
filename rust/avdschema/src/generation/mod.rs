//! Schema occurrence graph and artifact generators.

mod graph;
mod legacy_python;

pub use self::graph::Occurrence;
pub use self::graph::OccurrenceId;
pub use self::graph::OccurrenceKind;
pub use self::graph::SchemaGraph;
pub use self::legacy_python::GenerationError;
pub use self::legacy_python::generate_python_models;
pub use self::legacy_python::generate_python_models_projection;
