use crate::extract_query_metadata::{replace_select_wildcard, test_query};
use proc_macro2::TokenStream;

use quote::quote;
use scylla_orm::capitalizing::struct_name_to_table_name;
use scylla_orm::materialized_view::query_materialized_view;
use scylla_orm::query_metadata::{query_columns, QueryMetadata, QueryType};
use syn::parse::{Parse, ParseStream};
use syn::parse_quote::ParseQuote;
use syn::parse_str;
use syn::punctuated::Punctuated;

pub mod crud;
pub mod extract_query_metadata;

#[derive(Clone)]
pub struct Query {
    pub query_pretty: String,
    pub idents: Vec<syn::Ident>,
    /// The query values for the idents
    pub serialized_values: proc_macro2::TokenStream,
    /// More metadata
    pub qmd: QueryMetadata,
}

impl Query {
    pub fn create_transformed(self) -> proc_macro2::TokenStream {
        let query_to_server = &self.qmd.query;
        let struct_prefix = std::env::var("GENERATED_DB_ENTITIES_PATH_PREFIX")
            .expect("Please provide the struct prefix, e.g.: 'crate::entities' for environment variable GENERATED_DB_ENTITIES_PATH_PREFIX")
            // It ends sometimes with a newline
            .replace("\n", "");
        let path_to_struct = format!("{}::{}", struct_prefix, self.qmd.struct_name);
        let struct_name: TokenStream = path_to_struct.parse().unwrap();
        let serialized_values = &self.serialized_values;

        macro_rules! t {
            ($ty: ident) => {
                quote! {
                    scylla_orm::query_transform::$ty::new(scylla_orm::query_transform::Qv {
                        query: #query_to_server,
                        values: #serialized_values,
                    })
                }
            };
        }

        let ts = match self.qmd.query_type {
            QueryType::SelectMultiple => {
                assert!(self.query_pretty.starts_with("select *"));

                quote! {
                    scylla_orm::query_transform::SelectMultiple::<#struct_name>::new(scylla_orm::query_transform::Qv {
                        query: #query_to_server,
                        values: #serialized_values,
                    })
                }
            }
            QueryType::SelectUniqueByLimit | QueryType::SelectUnique => {
                assert!(self.query_pretty.starts_with("select *"));

                quote! {
                    scylla_orm::query_transform::SelectUnique::<#struct_name>::new(scylla_orm::query_transform::Qv {
                        query: #query_to_server,
                        values: #serialized_values,
                    })
                }
            }
            QueryType::SelectCount => {
                quote! {
                    scylla_orm::query_transform::SelectUniqueExpect::<scylla_orm::query_transform::Count>::new(scylla_orm::query_transform::Qv {
                        query: #query_to_server,
                        values: #serialized_values,
                    })
                }
            }
            QueryType::UpdateUnique => {
                t!(Update)
            }
            QueryType::DeleteMultiple => {
                t!(DeleteMultiple)
            }
            QueryType::DeleteUnique => {
                t!(DeleteUnique)
            }
            QueryType::InsertUnique => {
                t!(Insert)
            }
            QueryType::Truncate => {
                t!(Truncate)
            }
        };

        let idents = &self.idents;

        quote! {{
            #(tracing::debug!("Used value {:#?} for {:#?} for upcoming query", #idents, stringify!(#idents));)*

            #ts
        }}
    }

    /// The base table can be different from the table that is being queried from, if it's a materialized view
    /// with exactly the same columns as the base table
    pub fn create_transformed_materialized_view(mut self) -> proc_macro2::TokenStream {
        // Make sure the current table is a materialized view with the same columns
        let table_name = struct_name_to_table_name(&self.qmd.struct_name);
        let mv = query_materialized_view(&table_name).expect("Table is not a materialized view");

        // If the columns are not the same, the mapping will fail
        assert!(mv.same_columns);

        // Make sure all rows are selected
        // This is because the query needs to be transformed a little:
        // If a materialized view is created, it has a different pk than the base table
        // This is also reflected back in the table
        // Since the driver sets values by index, make sure the select columns has the exact same
        // order as the base table
        self.qmd.struct_name = mv.base_struct_name;

        // An odd thing about Scylla is that the ordering of the column of the mv is different than the base table
        // This is a problem since the Scylla driver will map rows to structs with indexes
        // When a select query is performed, rearrange it in the correct order
        if self.query_pretty.starts_with("select * ") {
            let columns_mv = query_columns(&table_name);
            let columns_base_table = query_columns(&mv.base_table_name);
            let query_should_be = replace_select_wildcard(&self.query_pretty, &columns_mv);

            // Make sure the query equals what is expected
            assert_eq!(self.qmd.query, query_should_be);

            // Now make sure the order is correct when mapping to the base table
            self.qmd.query = replace_select_wildcard(&self.query_pretty, &columns_base_table);
        }

        self.create_transformed()
    }
}

impl Parse for Query {
    /// Parses a query like this: my_proc_macro!("select * from table where a = 1 and b = ?", b);
    /// So a literal query followed by a comma separated list of arguments that will replace the
    /// question marks
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let query: syn::Lit = syn::parse::Parse::parse(input)?;
        // TODO: This is a lot of work just to get the str out of a LitStr, isn't there a better way?
        let query_raw = match query {
            syn::Lit::Str(s) => s,
            _ => panic!("First argument is not a literal"),
        };
        let stringified = format!("{:?}", query_raw);
        let starting_quote = stringified
            .find('\"')
            .expect("Failed to find leading quote");
        let ending_quote = stringified
            .rfind('\"')
            .expect("Failed to find trailing quote");
        let query_pretty = stringified[starting_quote + 1..ending_quote].to_string();
        let idents = if input.is_empty() {
            vec![]
        } else {
            let _: syn::Token![,] = syn::parse::Parse::parse(input)?;
            let punc_idents: Punctuated<syn::Ident, syn::Token![,]> =
                <Punctuated<syn::Ident, syn::Token![,]>>::parse(input)?;

            punc_idents.iter().cloned().collect()
        };

        let qmd = test_query(&query_pretty);

        // Parametrized columns equals columns values that are filled with dynamic values from the user
        let parameterized_columns = qmd
            .extracted_columns
            .iter()
            .filter(|r| r.parameterized)
            .collect::<Vec<_>>();
        let ident_count = idents.len();

        let types_comparison = idents
            .iter()
            .enumerate()
            .map(|(index, _)| {
                let column_type = &qmd.parameterized_columns_types[index].column_type;
                let mut ty_comparison = column_type.to_ty();

                if index == parameterized_columns.len() {
                    // The limit column
                    assert!(qmd.limited);
                } else if parameterized_columns[index].uses_in_value {
                    // This is used to compare types, to see if the types are assignable
                    // Another turbofish is needed
                    ty_comparison = format!("std::vec::Vec::<{}>", ty_comparison);
                }

                parse_str(&ty_comparison).expect("Failed to parse to type")
            })
            .collect::<Vec<syn::Type>>();

        let serialized_values = quote! {{
            let mut serialized_values = scylla::frame::value::SerializedValues::with_capacity(#ident_count);

            #(
                // Check if the type is correct
                debug_assert!((#types_comparison::from(#idents.clone()), true).1);

                serialized_values.add_value(&#idents)?;
            )*

            serialized_values
        }};

        Ok(Query {
            query_pretty,
            idents,
            serialized_values,
            qmd,
        })
    }
}
