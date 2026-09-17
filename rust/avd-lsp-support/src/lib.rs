// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Curated pyavd-utils API for the AVD language server.
//!
//! This crate is the language server's dependency and feature boundary. Its
//! default feature graph contains the schema, validation, and YAML APIs used by
//! the browser WASM build without native compression or regex accelerators.
//! Native consumers can opt into `gzip` without changing the
//! public API imported by the language server.
//!
//! The modules below intentionally re-export individual API items instead of
//! their complete source crates. Changes to the language server contract are
//! therefore reviewed and checked here.

#![deny(unused_crate_dependencies)]

/// Schema types and navigation used by the language server.
#[allow(
    clippy::module_name_repetitions,
    reason = "The facade keeps the source type names while grouping them by API domain."
)]
pub mod schema {
    pub use avdschema::Dump;
    pub use avdschema::Load;
    pub use avdschema::Store;
    pub use avdschema::any::AnySchema;
    pub use avdschema::any::Shortcuts;
    pub use avdschema::dict::Dict;
    pub use avdschema::dict::DynamicKeyOverrides;
    pub use avdschema::get_schema_from_path;
    pub use avdschema::list::List;
    pub use avdschema::str::Format;
}

/// Validation configuration, results, and diagnostics used by the language
/// server.
pub mod validation {
    pub use validation::Configuration;
    pub use validation::StoreValidate;
    pub use validation::feedback::ErrorIssue;
    pub use validation::feedback::Feedback;
    pub use validation::feedback::InfoIssue;
    pub use validation::feedback::ParseDiagnostic;
    pub use validation::feedback::ParseDiagnosticSource;
    pub use validation::feedback::Path;
    pub use validation::feedback::SourceSpan;
    pub use validation::feedback::WarningIssue;
}

/// YAML parser AST and event APIs used by the language server.
pub mod yaml {
    pub use yaml_parser::CollectionStyle;
    pub use yaml_parser::Event;
    pub use yaml_parser::Integer;
    pub use yaml_parser::MappingPair;
    pub use yaml_parser::Node;
    pub use yaml_parser::Span;
    pub use yaml_parser::Value;
    pub use yaml_parser::emit_events;
    pub use yaml_parser::parse;
}

#[cfg(test)]
mod tests {
    use super::schema::AnySchema;
    use super::schema::DynamicKeyOverrides;
    use super::schema::Format;
    use super::schema::Load as _;
    use super::schema::Store;
    use super::schema::get_schema_from_path;
    use super::validation::Configuration;
    use super::validation::ErrorIssue;
    use super::validation::Feedback;
    use super::validation::InfoIssue;
    use super::validation::ParseDiagnostic;
    use super::validation::Path;
    use super::validation::SourceSpan;
    use super::validation::WarningIssue;
    use super::yaml::CollectionStyle;
    use super::yaml::Event;
    use super::yaml::Integer;
    use super::yaml::MappingPair;
    use super::yaml::Node;
    use super::yaml::Span;
    use super::yaml::Value;
    use super::yaml::emit_events;
    use super::yaml::parse;

    #[test]
    fn lsp_api_contract_is_available() {
        let store = Store::from_json("{}");
        let (documents, _) = parse("key: value\n");
        let _ = emit_events("key: value\n");

        if let (Ok(store), Some(document)) = (store, documents.first()) {
            let _ = get_schema_from_path("schema", &store, &[], &document.value, None);
        }

        // Keep the exact imported LSP surface type-checked, including traits
        // and types whose constructors are not part of the consumer contract.
        let _: Option<AnySchema> = None;
        let _: Option<DynamicKeyOverrides> = None;
        let _: Option<Format> = None;
        let _: Option<Configuration> = None;
        let _: Option<Feedback<ErrorIssue>> = None;
        let _: Option<Feedback<InfoIssue>> = None;
        let _: Option<Feedback<WarningIssue>> = None;
        let _: Option<ParseDiagnostic> = None;
        let _: Option<Path> = None;
        let _: Option<SourceSpan> = None;
        let _: Option<CollectionStyle> = None;
        let _: Option<Event<'_>> = None;
        let _: Option<Integer<'_>> = None;
        let _: Option<MappingPair<'_>> = None;
        let _: Option<Node<'_>> = None;
        let _: Option<Span> = None;
        let _: Option<Value<'_>> = None;
    }
}
