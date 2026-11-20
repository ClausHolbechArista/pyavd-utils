// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Attribute declarations for borrowed, mode-parameterized data views.
//!
//! Declared fields describe accessor return types; they are replaced with a single checked
//! backing token. Field order assigns local slots. Indexed lists name their primary-key fields
//! and resolve slots through the item declaration, without copying numeric identifiers.
//! Opt-in Python metadata refers back to these same descriptors. The macro emits metadata only;
//! shared native implementations in `data-views-py` own Python objects and perform access.

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro2::TokenStream as Tokens;
use quote::format_ident;
use quote::quote;
use syn::GenericArgument;
use syn::ItemStruct;
use syn::LitStr;
use syn::PathArguments;
use syn::Type;
use syn::parse::Parser as _;
use syn::punctuated::Punctuated;
use syn::visit_mut::VisitMut as _;

#[cfg(test)]
mod tests;

/// Generate a mode-parameterized data view from a struct-shaped declaration.
///
/// Views implement `Clone`, `Copy`, and `Debug` automatically. Copying a view only copies its
/// borrowed backing token, never the underlying data; declarations need no derives for these traits.
///
/// Named fields use `Field<T>` or `RequiredValue<T, Mode>`. Tuple declarations represent lists.
/// `#[data_view(rename = "...")]` preserves schema keys differing from Rust field names;
/// `#[data_view(dynamic = "...")]` retains dynamic descriptors without named accessors.
/// `#[data_view(relaxed)]` marks a relationship whose descendants use relaxed validation.
///
/// Declarations use the lifetime `'a` and mode parameter `Mode`. Child collection types normally
/// carry `Mode`; a relaxed relationship instead carries `data_views::RelaxedValidated`.
/// List tuple fields also use `Field<T>` or `RequiredValue<T, Mode>` to describe item nullability,
/// independently of the containing list's requiredness. Unit list declarations expose raw items.
/// List `primary_key(...)` names the item's Rust accessors (after any schema-key rename).
/// The generated contextual item wrapper guarantees those key values in either mode without
/// changing the dictionary model's requiredness when it is used elsewhere.
/// `list, primary_key(...)` permits duplicate keys and exposes positions only; `indexed_list`
/// additionally exposes lookup by a unique key.
///
/// `python(name = "...")` opts a model into native Python registration. Dotted names place the
/// class inside its public hierarchy, such as `AVDDesign.DevicesItemBase`. Keyed lists additionally
/// declare `item_name = "..."` for their contextual item identity. The declaration emits a
/// `data_views_py::PythonModel` implementation whose fields reference the original Rust
/// descriptors. Non-Python declarations need no Python binding dependency. Dynamic fields use
/// `python_name = "..."` for a typed group under `_dynamic_keys`, not a static property.
/// `python(feature = "python", name = "...")` gates the binding implementation on a feature
/// of the declaring crate. The Rust view and its descriptors remain available without that
/// feature, allowing the same declarations to serve Rust-only and Python-enabled consumers.
/// `schema = Str(0)` on a field and `python(..., schema = Dict(0))` on a model identify nodes
/// in the matching compiled schema for default lookup. Relaxed relationships remain opaque.
#[proc_macro_attribute]
pub fn data_view(arguments: TokenStream, input: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(input as ItemStruct);
    match expand(arguments.into(), &item) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

#[derive(Default)]
struct Options {
    list: bool,
    indexed: bool,
    primary_keys: Vec<syn::Ident>,
    python: Option<PythonOptions>,
}

#[derive(Default)]
struct PythonOptions {
    name: Option<LitStr>,
    item_name: Option<LitStr>,
    schema: Option<Tokens>,
    feature: Option<LitStr>,
}

/// Parse a compiled schema identity without making Rust-only views depend on avdschema.
fn schema_identity(meta: &syn::meta::ParseNestedMeta<'_>) -> syn::Result<Tokens> {
    let input = meta.value()?;
    let kind: syn::Ident = input.parse()?;
    if !["Bool", "Int", "Str", "List", "Dict"]
        .iter()
        .any(|name| kind == name)
    {
        return Err(meta.error("unknown schema table"));
    }
    let content;
    syn::parenthesized!(content in input);
    let index: syn::LitInt = content.parse()?;
    let _: u32 = index.base10_parse()?;
    Ok(quote!(Some(::data_views_py::SchemaId::#kind(#index))))
}

fn options(arguments: Tokens) -> syn::Result<Options> {
    let mut options = Options::default();
    syn::meta::parser(|meta| {
        if meta.path.is_ident("list") {
            options.list = true;
        } else if meta.path.is_ident("indexed_list") {
            options.list = true;
            options.indexed = true;
        } else if meta.path.is_ident("primary_key") {
            let content;
            syn::parenthesized!(content in meta.input);
            options.primary_keys =
                Punctuated::<syn::Ident, syn::Token![,]>::parse_terminated(&content)?
                    .into_iter()
                    .collect();
        } else if meta.path.is_ident("python") {
            let mut python = PythonOptions::default();
            meta.parse_nested_meta(|nested| {
                if nested.path.is_ident("name") {
                    python.name = Some(nested.value()?.parse()?);
                } else if nested.path.is_ident("item_name") {
                    python.item_name = Some(nested.value()?.parse()?);
                } else if nested.path.is_ident("schema") {
                    python.schema = Some(schema_identity(&nested)?);
                } else if nested.path.is_ident("feature") {
                    python.feature = Some(nested.value()?.parse()?);
                } else {
                    return Err(nested.error("unknown Python binding option"));
                }
                Ok(())
            })?;
            if python.name.is_none() {
                return Err(meta.error("python requires name = \"...\""));
            }
            options.python = Some(python);
        } else {
            return Err(meta.error("unknown data_view option"));
        }
        Ok(())
    })
    .parse2(arguments)?;
    if options.indexed && options.primary_keys.is_empty() {
        return Err(syn::Error::new(
            Span::call_site(),
            "indexed_list requires primary_key(field, ...)",
        ));
    }
    if !options.list && !options.primary_keys.is_empty() {
        return Err(syn::Error::new(
            Span::call_site(),
            "primary_key is only valid on list or indexed_list",
        ));
    }
    if let Some(python) = &options.python
        && python.item_name.is_some() == options.primary_keys.is_empty()
    {
        return Err(syn::Error::new(
            Span::call_site(),
            "Python primary-key lists require item_name; other models must not declare it",
        ));
    }
    Ok(options)
}

/// Stable field-name identity used solely for compile-time key metadata lookup.
fn name_hash(name: &str) -> u64 {
    name.bytes().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

fn inner_type(ty: &Type) -> syn::Result<(&Type, bool)> {
    if let Type::Path(path) = ty
        && let Some(segment) = path.path.segments.last()
        && (segment.ident == "Field" || segment.ident == "RequiredValue")
        && let PathArguments::AngleBracketed(arguments) = &segment.arguments
        && let Some(GenericArgument::Type(inner)) = arguments.args.first()
    {
        return Ok((inner, segment.ident == "RequiredValue"));
    }
    Err(syn::Error::new_spanned(
        ty,
        "fields must use Field<T> or RequiredValue<T, Mode>",
    ))
}

fn scalar(ty: &Type) -> Option<Tokens> {
    match ty {
        Type::Path(path) if path.path.is_ident("bool") => {
            Some(quote!(::data_views::ScalarType::Bool))
        }
        Type::Path(path) if path.path.is_ident("i64") => {
            Some(quote!(::data_views::ScalarType::Int))
        }
        Type::Reference(reference) if matches!(&*reference.elem, Type::Path(path) if path.path.is_ident("str")) => {
            Some(quote!(::data_views::ScalarType::Str))
        }
        _ => None,
    }
}

#[derive(Default)]
struct Expansion {
    methods: Vec<Tokens>,
    descriptors: Vec<Tokens>,
    field_traits: Vec<Tokens>,
    extra_types: Tokens,
    primary_slots: Vec<Tokens>,
    python_fields: Vec<Tokens>,
    python_item: Option<Tokens>,
}

fn python_target(target: &Type, relaxed: bool) -> Tokens {
    if scalar(target).is_some() {
        quote!(::data_views_py::Target::Scalar)
    } else if relaxed {
        quote!(::data_views_py::Target::Opaque)
    } else {
        quote!(::data_views_py::Target::Model(<#target as ::data_views_py::PythonModel>::binding))
    }
}

/// Rename only the declared mode parameter, not nominal types such as `module::Mode<'a, Mode>`.
/// The expansion uses an internal name so a schema model named `Mode` remains a valid type.
struct InternalMode;

impl syn::visit_mut::VisitMut for InternalMode {
    fn visit_type_path_mut(&mut self, i: &mut syn::TypePath) {
        if i.qself.is_none() && i.path.is_ident("Mode") {
            i.path = syn::parse_quote!(__DataViewMode);
        } else {
            syn::visit_mut::visit_type_path_mut(self, i);
        }
    }
}

fn add_keyed_list_accessors(
    expansion: &mut Expansion,
    options: &Options,
    name: &syn::Ident,
    visibility: &syn::Visibility,
    item_type: &Type,
    item_scalar: Option<&Tokens>,
) -> syn::Result<()> {
    if item_scalar.is_some() {
        return Err(syn::Error::new_spanned(
            item_type,
            "primary-key list items must be dictionary models",
        ));
    }
    let item_name = format_ident!("{}Item", name);
    let mut key_methods = Vec::new();
    for key in &options.primary_keys {
        let hash = name_hash(&key.to_string());
        expansion
            .primary_slots
            .push(quote!(<#item_type as ::data_views::KeyField<'a, #hash>>::SLOT));
        key_methods.push(quote! {
            /// Guaranteed primary-key component of this list item, regardless of uniqueness.
            pub fn #key(&self) -> ::data_views::Guaranteed<<#item_type as ::data_views::KeyField<'a, #hash>>::Value> {
                self.0.key::<#hash>()
            }
        });
    }
    expansion.extra_types = quote! {
        /// Item reached through this list, with guaranteed primary-key accessors.
        #[derive(Clone, Copy, Debug)]
        #visibility struct #item_name<'a, __DataViewMode: ::data_views::ValidationMode = ::data_views::Validated>(
            ::data_views::KeyedItem<#item_type>, ::core::marker::PhantomData<__DataViewMode>
        );
        impl<'a, __DataViewMode: ::data_views::ValidationMode> #item_name<'a, __DataViewMode> {
            #(#key_methods)*
        }
        impl<'a, __DataViewMode: ::data_views::ValidationMode> ::core::ops::Deref for #item_name<'a, __DataViewMode> {
            type Target = #item_type;
            fn deref(&self) -> &Self::Target { &self.0 }
        }
    };
    expansion.methods.push(quote! {
        /// Read an item by its position, retaining its primary-key guarantee.
        pub fn get(self, index: usize) -> Option<#item_name<'a, __DataViewMode>> {
            self.0.item::<#item_type>(index).map(|field| {
                let value = <::data_views::Validated as ::data_views::ValidationMode>::required(field).get();
                #item_name(::data_views::KeyedItem::new(value), ::core::marker::PhantomData)
            })
        }
    });
    if options.indexed {
        expansion.methods.push(quote! {
            /// Look up the ordered primary-key components.
            pub fn get_by_primary_key(self, key: &[::data_views::PrimaryKeyValue<'_>]) -> Option<#item_name<'a, __DataViewMode>> {
                self.0.indexed_item::<#item_type>(key).map(|value| #item_name(::data_views::KeyedItem::new(value), ::core::marker::PhantomData))
            }
        });
    }
    Ok(())
}

fn expand_list(options: &Options, item: &ItemStruct) -> syn::Result<Expansion> {
    let mut expansion = Expansion::default();
    let name = &item.ident;
    let visibility = &item.vis;
    let list_items = match &item.fields {
        syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => fields.unnamed.first(),
        syn::Fields::Unit => None,
        _ => {
            return Err(syn::Error::new_spanned(
                &item.fields,
                "list declarations use one tuple field or a unit struct",
            ));
        }
    };
    expansion.methods.push(quote! {
        /// Number of items.
        pub fn len(self) -> usize { self.0.len() }
        /// Whether the list has no items.
        pub fn is_empty(self) -> bool { self.0.is_empty() }
    });
    if let Some(field) = list_items {
        let declared = &field.ty;
        let mut relaxed = false;
        for attribute in &field.attrs {
            if attribute.path().is_ident("data_view") {
                attribute.parse_nested_meta(|meta| {
                    if meta.path.is_ident("relaxed") {
                        relaxed = true;
                        Ok(())
                    } else {
                        Err(meta.error("list items only support the relaxed option"))
                    }
                })?;
            }
        }
        let (item_type, required) = inner_type(declared)?;
        if options.python.is_some() {
            if relaxed && !options.primary_keys.is_empty() {
                return Err(syn::Error::new_spanned(
                    field,
                    "Python primary-key items cannot be opaque relaxed payloads",
                ));
            }
            expansion.python_item = Some(python_target(item_type, relaxed));
        }
        let item_scalar = scalar(item_type);
        let scalar_type = item_scalar
            .as_ref()
            .map_or_else(|| quote!(None), |kind| quote!(Some(#kind)));
        let target_model = if item_scalar.is_some() {
            quote!(None)
        } else {
            quote!(Some(&<#item_type as ::data_views::ArchiveModel>::DESCRIPTOR))
        };
        expansion.descriptors.push(quote!(::data_views::FieldDescriptor {
            id: 0, relation: ::data_views::FieldRelation::Item,
            target_model: #target_model, scalar_type: #scalar_type, required: #required, relaxed: #relaxed,
        }));
        if options.primary_keys.is_empty() {
            let result = if item_scalar.is_some() {
                if required {
                    quote!(self.0.scalar_item(index).map(__DataViewMode::required))
                } else {
                    quote!(self.0.scalar_item(index))
                }
            } else {
                if required {
                    quote!(self.0.item(index).map(__DataViewMode::required))
                } else {
                    quote!(self.0.item(index))
                }
            };
            expansion.methods.push(quote! {
                /// Read an item, preserving its validation-mode guarantee; None means out of range.
                pub fn get(self, index: usize) -> Option<#declared> { #result }
            });
        } else {
            add_keyed_list_accessors(
                &mut expansion,
                options,
                name,
                visibility,
                item_type,
                item_scalar.as_ref(),
            )?;
        }
    } else {
        if !options.primary_keys.is_empty() {
            return Err(syn::Error::new_spanned(
                item,
                "primary-key lists require an item model",
            ));
        }
        expansion.methods.push(quote! {
            /// Read an item from a schema without an item declaration.
            pub fn get(self, index: usize) -> Option<::data_views::ValueView<'a>> { self.0.raw_item(index) }
        });
    }
    Ok(expansion)
}

#[allow(
    clippy::too_many_lines,
    reason = "Each field's Rust accessor and Python binding metadata are emitted together from one declaration."
)]
fn expand_dict(options: &Options, item: &ItemStruct) -> syn::Result<Expansion> {
    let mut expansion = Expansion::default();
    let name = &item.ident;
    let syn::Fields::Named(fields) = &item.fields else {
        return Err(syn::Error::new_spanned(
            &item.fields,
            "dictionary views require named fields",
        ));
    };
    for (slot, field) in fields.named.iter().enumerate() {
        let field_index = slot;
        let slot = u32::try_from(slot).map_err(|error| syn::Error::new_spanned(field, error))?;
        let identifier = field
            .ident
            .as_ref()
            .ok_or_else(|| syn::Error::new_spanned(field, "named field required"))?;
        let return_type = &field.ty;
        let (target, required) = inner_type(return_type)?;
        let mut key = LitStr::new(&identifier.to_string(), identifier.span());
        let mut dynamic = false;
        let mut relaxed = false;
        let mut retained_attributes = Vec::new();
        let mut schema = quote!(None);
        let mut python_name = None;
        for attribute in &field.attrs {
            if !attribute.path().is_ident("data_view") {
                retained_attributes.push(attribute);
                continue;
            }
            attribute.parse_nested_meta(|meta| {
                if meta.path.is_ident("rename") {
                    key = meta.value()?.parse()?;
                } else if meta.path.is_ident("dynamic") {
                    dynamic = true;
                    key = meta.value()?.parse()?;
                } else if meta.path.is_ident("relaxed") {
                    relaxed = true;
                } else if meta.path.is_ident("schema") {
                    schema = schema_identity(&meta)?;
                } else if meta.path.is_ident("python_name") {
                    python_name = Some(meta.value()?.parse::<LitStr>()?);
                } else {
                    return Err(meta.error("unknown field option"));
                }
                Ok(())
            })?;
        }
        let target_scalar = scalar(target);
        let scalar_type = target_scalar
            .as_ref()
            .map_or_else(|| quote!(None), |kind| quote!(Some(#kind)));
        let target_model = if target_scalar.is_some() {
            quote!(None)
        } else {
            quote!(Some(&<#target as ::data_views::ArchiveModel>::DESCRIPTOR))
        };
        let relation = if dynamic {
            quote!(::data_views::FieldRelation::DynamicKey(#key))
        } else {
            quote!(::data_views::FieldRelation::Key(#key))
        };
        expansion
            .descriptors
            .push(quote!(::data_views::FieldDescriptor {
                id: #slot, relation: #relation, target_model: #target_model,
                scalar_type: #scalar_type, required: #required, relaxed: #relaxed,
            }));
        if options.python.is_some() {
            let property_name = python_name
                .unwrap_or_else(|| LitStr::new(&identifier.to_string(), identifier.span()));
            let conversion = python_target(target, relaxed);
            expansion.python_fields.push(quote!(::data_views_py::FieldBinding {
                name: #property_name,
                descriptor: &<Self as ::data_views::ArchiveModel>::DESCRIPTOR.fields[#field_index],
                target: #conversion,
                schema_id: #schema,
            }));
        }
        if dynamic {
            continue;
        }
        let access = if target_scalar.is_some() {
            quote!(self.0.scalar::<#target>(#slot))
        } else {
            quote!(self.0.child::<#target>(#slot))
        };
        let result = if required {
            quote!(__DataViewMode::required(#access))
        } else {
            access.clone()
        };
        expansion.methods.push(quote! {
            #(#retained_attributes)*
            #[doc = concat!("Read schema field ", #key, " with this view's presence guarantee.")]
            pub fn #identifier(self) -> #return_type { #result }
        });
        if target_scalar.is_some() {
            let hash = name_hash(&identifier.to_string());
            expansion.field_traits.push(quote! {
                impl<'a, __DataViewMode: ::data_views::ValidationMode> ::data_views::KeyField<'a, #hash> for #name<'a, __DataViewMode> {
                    type Value = #target;
                    const SLOT: u32 = #slot;
                    fn key_field(&self) -> ::data_views::Field<Self::Value> { #access }
                }
            });
        }
    }
    Ok(expansion)
}

fn expand(arguments: Tokens, item: &ItemStruct) -> syn::Result<Tokens> {
    let mut item = item.clone();
    for field in &mut item.fields {
        InternalMode.visit_type_mut(&mut field.ty);
    }
    let options = options(arguments)?;
    let name = &item.ident;
    let visibility = &item.vis;
    let attributes = &item.attrs;
    let expansion = if options.list {
        expand_list(&options, &item)?
    } else {
        expand_dict(&options, &item)?
    };
    let model_kind = if options.list {
        quote!(::data_views::ModelKind::List)
    } else {
        quote!(::data_views::ModelKind::Dict)
    };
    let methods = &expansion.methods;
    let descriptors = &expansion.descriptors;
    let primary_slots = &expansion.primary_slots;
    let field_traits = &expansion.field_traits;
    let extra_types = &expansion.extra_types;
    let indexed = options.indexed;
    let python = if let Some(python) = options.python {
        let feature = python
            .feature
            .map(|feature| quote!(#[cfg(feature = #feature)]));
        let python_name = python.name;
        let schema = python.schema.unwrap_or_else(|| quote!(None));
        let item_name = python.item_name.map_or_else(
            || quote!(None),
            |python_item_name| quote!(Some(#python_item_name)),
        );
        let fields = &expansion.python_fields;
        let item_expression = expansion
            .python_item
            .as_ref()
            .map_or_else(|| quote!(None), |target| quote!(Some(#target)));
        quote! {
            #feature
            impl<'a, __DataViewMode: ::data_views::ValidationMode> ::data_views_py::PythonModel for #name<'a, __DataViewMode> {
                const BINDING: ::data_views_py::ModelBinding = ::data_views_py::ModelBinding {
                    name: #python_name,
                    schema_id: #schema,
                    descriptor: &<Self as ::data_views::ArchiveModel>::DESCRIPTOR,
                    fields: &[#(#fields),*],
                    item: #item_expression,
                    keyed_item_name: #item_name,
                };
            }
        }
    } else {
        quote!()
    };
    Ok(quote! {
        #(#attributes)*
        #[derive(Clone, Copy, Debug)]
        #visibility struct #name<'a, __DataViewMode: ::data_views::ValidationMode = ::data_views::Validated>(::data_views::CheckedModel<'a, Self>, ::core::marker::PhantomData<__DataViewMode>);
        impl<'a, __DataViewMode: ::data_views::ValidationMode> #name<'a, __DataViewMode> { #(#methods)* }
        impl<'a, __DataViewMode: ::data_views::ValidationMode> ::data_views::ArchiveModel for #name<'a, __DataViewMode> {
            const DESCRIPTOR: ::data_views::ModelDescriptor = ::data_views::ModelDescriptor {
                identity: concat!(module_path!(), "::", stringify!(#name)),
                kind: #model_kind,
                fields: &[#(#descriptors),*], primary_key_fields: &[#(#primary_slots),*], indexed: #indexed,
            };
        }
        impl<'a, __DataViewMode: ::data_views::ValidationMode> ::data_views::DataView<'a> for #name<'a, __DataViewMode> {
            type Mode = __DataViewMode;
            fn from_checked(value: ::data_views::CheckedModel<'a, Self>) -> Self { Self(value, ::core::marker::PhantomData) }
        }
        #(#field_traits)*
        #extra_types
        #python
    })
}
