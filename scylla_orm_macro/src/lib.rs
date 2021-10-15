mod json;

use proc_macro::TokenStream;
use scylla_orm::query_metadata::QueryType;
use scylla_orm_query_parser::Query;
use syn::parse_macro_input;
use syn::DeriveInput;

/// Transforms a query to to the corresponding type
/// let transformed_type = scylla_orm_query::query!("select * from my_table where some_property = ?", my_property);
#[proc_macro]
pub fn query(input: TokenStream) -> TokenStream {
    let query = parse_macro_input!(input as Query);

    check_predefined_queries(&query);

    let transformed = query.create_transformed();

    transformed.into()
}

/// When a materialized view has the same columns as the base table, an auto-conversion can take place between the structs
/// So when querying a materialized view, call this macro if you want the structs of the base table
#[proc_macro]
pub fn query_base_table(input: TokenStream) -> TokenStream {
    let query = parse_macro_input!(input as Query);

    check_predefined_queries(&query);

    let transformed = query.create_transformed_materialized_view();

    transformed.into()
}

fn check_predefined_queries(query: &Query) {
    let value =
        std::env::var("ALLOW_CUSTOM_PREDEFINED_QUERIES").unwrap_or_else(|_| "0".to_string());

    if value == "1" {
        return;
    }

    match query.qmd.query_type {
        QueryType::SelectUnique
        | QueryType::DeleteUnique
        | QueryType::InsertUnique
        | QueryType::Truncate => {
            panic!("Use predefined method")
        }
        QueryType::UpdateUnique => {
            if !query.qmd.query.contains(',') {
                panic!("Updating only 1 columns, do it with predefined method");
            }
        }
        QueryType::SelectMultiple
        | QueryType::SelectUniqueByLimit
        | QueryType::SelectCount
        | QueryType::DeleteMultiple => {
            // Nothing I guess
        }
    }
}

/// Annotating a struct with this macro will make it work with scylla's serialization/deserialization
#[proc_macro_derive(Json)]
pub fn json(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();

    json::json(derive_input).into()
}

/// Currently no implementation
/// This can be used by other users to create methods on the structs based on the attributes
#[proc_macro_derive(Mirror, attributes(partition_key, clustering_key, json))]
pub fn mirror(_input: TokenStream) -> TokenStream {
    Default::default()
}

/// See 'mirror'
#[proc_macro_derive(PrimaryKey, attributes(partition_key, clustering_key, json))]
pub fn primary_key(_input: TokenStream) -> TokenStream {
    Default::default()
}
