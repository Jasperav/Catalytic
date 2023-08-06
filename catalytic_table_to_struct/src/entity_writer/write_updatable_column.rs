use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::entity_writer::EntityWriter;
use crate::query_ident::{get_updatable_column_field, updatable_column, updatable_column_ref};
use crate::transformer::Transformer;

pub(crate) fn write<T: Transformer>(entity_writer: &'_ EntityWriter<T>) -> TokenStream {
    if entity_writer.table.materialized_view.is_some()
        || entity_writer
            .struct_field_metadata
            .non_primary_key_fields
            .is_empty()
    {
        return Default::default();
    }

    let struct_name_ident = entity_writer.struct_ident();
    let updatable_column = updatable_column();
    let updatable_column_ref = updatable_column_ref();

    let mut updatable_column_variants = vec![];
    let mut updatable_column_to_ref = vec![];
    let mut updatable_column_from_ref = vec![];
    let mut updatable_column_ref_variants = vec![];
    let mut get_updatable_column = vec![];

    for field in &entity_writer.struct_field_metadata.non_primary_key_fields {
        let variant_name = format_ident!("{}", field.field_name.to_string().to_upper_camel_case());
        let ty = &field.ty;
        let borrow_ty = &field.borrow_ty;
        let to_ref = &field.from_borrow_to_owned;
        let ident = &field.field_name;

        updatable_column_variants.push(quote! {
            #variant_name(#ty)
        });

        updatable_column_ref_variants.push(quote! {
            #variant_name(&'a #borrow_ty)
        });

        updatable_column_to_ref.push(quote! {
            #updatable_column::#variant_name(v) => #updatable_column_ref::#variant_name(v)
        });

        updatable_column_from_ref.push(quote! {
            #updatable_column_ref::#variant_name(v) => #updatable_column::#variant_name(v.#to_ref)
        });

        let fn_name = get_updatable_column_field(ident);
        let message = format!(
            "Creates the updatable column {} which can be used to update it in the database",
            ident
        );

        get_updatable_column.push(quote! {
            #[doc = #message]
            pub fn #fn_name(&self) -> #updatable_column_ref {
                #updatable_column_ref::#variant_name(&self.#ident)
            }
        });
    }

    let updatable_column_metadata = entity_writer
        .transformer
        .updatable_column_metadata(entity_writer.struct_table())
        .into_tokenstream();

    let updatable_column_metadata_ref = entity_writer
        .transformer
        .updatable_column_ref_metadata(entity_writer.struct_table())
        .into_tokenstream();

    quote! {
        /// This struct can be converted to a borrowed struct which can be used to update single rows
        #[allow(clippy::large_enum_variant)]
        #updatable_column_metadata
        pub enum #updatable_column {
            #(#updatable_column_variants),*
        }

        impl #updatable_column {
            /// Conversation method to go from an owned updatable column struct to a borrowed updatable column struct
            pub fn to_ref(&self) -> #updatable_column_ref<'_> {
                match &self {
                    #(#updatable_column_to_ref),*
                }
            }
        }

        /// This struct can be used to update columns
        /// If you have a borrowed primary key and you want to update a column, you can pass in
        /// one of the variants
        #updatable_column_metadata_ref
        pub enum #updatable_column_ref<'a> {
            #(#updatable_column_ref_variants),*
        }

        pub trait UpdatableColumnVec {
            fn to_ref(&self) -> Vec<#updatable_column_ref<'_>>;
        }

        impl UpdatableColumnVec for Vec<#updatable_column> {
            /// Conversation method to go from a vec of owned updatable column structs to a vec of borrowed updatable column structs
            fn to_ref(&self) -> Vec<#updatable_column_ref<'_>> {
                self.iter().map(|v| v.to_ref()).collect()
            }
        }

        impl From<#updatable_column_ref<'_>> for #updatable_column {
            /// Conversation method to go from a borrowed updatable column struct to an owned updatable column struct
            fn from(f: #updatable_column_ref<'_>) -> #updatable_column {
                match f {
                    #(#updatable_column_from_ref),*
                }
            }
        }

        impl #updatable_column_ref<'_> {
            /// Conversation method to go from a borrowed updatable column struct to an owned updatable column struct
            pub fn into_owned(self) -> #updatable_column {
                self.into()
            }
        }

        impl #struct_name_ident {
            #(#get_updatable_column)*
        }
    }
}
