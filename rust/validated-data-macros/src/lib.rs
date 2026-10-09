// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Attribute declarations for borrowed, mode-parameterized validated-data views.
//!
//! Declared fields describe accessor return types; they are replaced with a single checked
//! backing token. Field order assigns local slots. Indexed lists name their primary-key fields
//! and resolve slots through the item declaration, without copying numeric identifiers.

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
/// carry `Mode`; a relaxed relationship instead carries `validated_data::RelaxedValidated`.
/// List tuple fields also use `Field<T>` or `RequiredValue<T, Mode>` to describe item nullability,
/// independently of the containing list's requiredness. Unit list declarations expose raw items.
/// List `primary_key(...)` names the item's Rust accessors (after any schema-key rename).
/// The generated contextual item wrapper guarantees those key values in either mode without
/// changing the dictionary model's requiredness when it is used elsewhere.
/// `list, primary_key(...)` permits duplicate keys and exposes positions only; `indexed_list`
/// additionally exposes lookup by a unique key.
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
            Some(quote!(::validated_data::ScalarType::Bool))
        }
        Type::Path(path) if path.path.is_ident("i64") => {
            Some(quote!(::validated_data::ScalarType::Int))
        }
        Type::Reference(reference) if matches!(&*reference.elem, Type::Path(path) if path.path.is_ident("str")) => {
            Some(quote!(::validated_data::ScalarType::Str))
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
            .push(quote!(<#item_type as ::validated_data::KeyField<'a, #hash>>::SLOT));
        key_methods.push(quote! {
            /// Guaranteed primary-key component of this list item, regardless of uniqueness.
            pub fn #key(&self) -> ::validated_data::Guaranteed<<#item_type as ::validated_data::KeyField<'a, #hash>>::Value> {
                self.0.key::<#hash>()
            }
        });
    }
    expansion.extra_types = quote! {
        /// Item reached through this list, with guaranteed primary-key accessors.
        #[derive(Clone, Copy, Debug)]
        #visibility struct #item_name<'a, __DataViewMode: ::validated_data::ValidationMode = ::validated_data::Validated>(
            ::validated_data::KeyedItem<#item_type>, ::core::marker::PhantomData<__DataViewMode>
        );
        impl<'a, __DataViewMode: ::validated_data::ValidationMode> #item_name<'a, __DataViewMode> {
            #(#key_methods)*
        }
        impl<'a, __DataViewMode: ::validated_data::ValidationMode> ::core::ops::Deref for #item_name<'a, __DataViewMode> {
            type Target = #item_type;
            fn deref(&self) -> &Self::Target { &self.0 }
        }
    };
    expansion.methods.push(quote! {
        /// Read an item by its position, retaining its primary-key guarantee.
        pub fn get(self, index: usize) -> Option<#item_name<'a, __DataViewMode>> {
            self.0.item::<#item_type>(index).map(|field| {
                let value = <::validated_data::Validated as ::validated_data::ValidationMode>::required(field).get();
                #item_name(::validated_data::KeyedItem::new(value), ::core::marker::PhantomData)
            })
        }
    });
    if options.indexed {
        expansion.methods.push(quote! {
            /// Look up the ordered primary-key components.
            pub fn get_by_primary_key(self, key: &[::validated_data::PrimaryKeyValue<'_>]) -> Option<#item_name<'a, __DataViewMode>> {
                self.0.indexed_item::<#item_type>(key).map(|value| #item_name(::validated_data::KeyedItem::new(value), ::core::marker::PhantomData))
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
        let item_scalar = scalar(item_type);
        let scalar_type = item_scalar
            .as_ref()
            .map_or_else(|| quote!(None), |kind| quote!(Some(#kind)));
        let target_model = if item_scalar.is_some() {
            quote!(None)
        } else {
            quote!(Some(&<#item_type as ::validated_data::ArchiveModel>::DESCRIPTOR))
        };
        expansion.descriptors.push(quote!(::validated_data::FieldDescriptor {
            id: 0, relation: ::validated_data::FieldRelation::Item,
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
            pub fn get(self, index: usize) -> Option<::validated_data::ValueView<'a>> { self.0.raw_item(index) }
        });
    }
    Ok(expansion)
}

fn expand_dict(item: &ItemStruct) -> syn::Result<Expansion> {
    let mut expansion = Expansion::default();
    let name = &item.ident;
    let syn::Fields::Named(fields) = &item.fields else {
        return Err(syn::Error::new_spanned(
            &item.fields,
            "dictionary views require named fields",
        ));
    };
    for (slot, field) in fields.named.iter().enumerate() {
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
            quote!(Some(&<#target as ::validated_data::ArchiveModel>::DESCRIPTOR))
        };
        let relation = if dynamic {
            quote!(::validated_data::FieldRelation::DynamicKey(#key))
        } else {
            quote!(::validated_data::FieldRelation::Key(#key))
        };
        expansion
            .descriptors
            .push(quote!(::validated_data::FieldDescriptor {
                id: #slot, relation: #relation, target_model: #target_model,
                scalar_type: #scalar_type, required: #required, relaxed: #relaxed,
            }));
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
                impl<'a, __DataViewMode: ::validated_data::ValidationMode> ::validated_data::KeyField<'a, #hash> for #name<'a, __DataViewMode> {
                    type Value = #target;
                    const SLOT: u32 = #slot;
                    fn key_field(&self) -> ::validated_data::Field<Self::Value> { #access }
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
        expand_dict(&item)?
    };
    let model_kind = if options.list {
        quote!(::validated_data::ModelKind::List)
    } else {
        quote!(::validated_data::ModelKind::Dict)
    };
    let methods = &expansion.methods;
    let descriptors = &expansion.descriptors;
    let primary_slots = &expansion.primary_slots;
    let field_traits = &expansion.field_traits;
    let extra_types = &expansion.extra_types;
    let indexed = options.indexed;
    Ok(quote! {
        #(#attributes)*
        #[derive(Clone, Copy, Debug)]
        #visibility struct #name<'a, __DataViewMode: ::validated_data::ValidationMode = ::validated_data::Validated>(::validated_data::CheckedModel<'a, Self>, ::core::marker::PhantomData<__DataViewMode>);
        impl<'a, __DataViewMode: ::validated_data::ValidationMode> #name<'a, __DataViewMode> { #(#methods)* }
        impl<'a, __DataViewMode: ::validated_data::ValidationMode> ::validated_data::ArchiveModel for #name<'a, __DataViewMode> {
            const DESCRIPTOR: ::validated_data::ModelDescriptor = ::validated_data::ModelDescriptor {
                identity: concat!(module_path!(), "::", stringify!(#name)),
                kind: #model_kind,
                fields: &[#(#descriptors),*], primary_key_fields: &[#(#primary_slots),*], indexed: #indexed,
            };
        }
        impl<'a, __DataViewMode: ::validated_data::ValidationMode> ::validated_data::DataView<'a> for #name<'a, __DataViewMode> {
            type Mode = __DataViewMode;
            fn from_checked(value: ::validated_data::CheckedModel<'a, Self>) -> Self { Self(value, ::core::marker::PhantomData) }
        }
        #(#field_traits)*
        #extra_types
    })
}
