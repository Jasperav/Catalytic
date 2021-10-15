use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::column_mapper::StructFieldMetadata;
use crate::transformer::{StructTable, Transformer};
use crate::Table;

use scylla_orm::query_metadata::query_columns;
use scylla_orm::table_metadata::ColumnInTable;

mod write_primary_key;
mod write_struct;
mod write_updatable_column;

pub struct EntityWriter<'a, T: Transformer> {
    pub table: Table,
    pub struct_name: &'a str,
    pub struct_field_metadata: StructFieldMetadata,
    pub transformer: &'a T,
    pub columns: &'a Vec<ColumnInTable>,
}

macro_rules! create_transformer {
    ($ident: ident, $content: expr) => {
        #[allow(dead_code)]
        pub(crate) fn $ident(&self) -> Ident {
            format_ident!($content)
        }
    };
}

impl<T: Transformer> EntityWriter<'_, T> {
    pub fn create_tokens(self) -> TokenStream {
        let mut tokens = quote! {
            use scylla::Session;
            use scylla::transport::iterator::TypedRowIterator;
            use scylla::transport::errors::QueryError;
            use scylla::frame::value::SerializedValues;
            #[allow(unused_imports)]
            use scylla::frame::value::SerializeValuesError;
            #[allow(unused_imports)]
            use scylla_orm::query_transform::{
                ScyllaQueryResult,
                SingleSelectQueryErrorTransform,
                QueryEntityVecResult,
                MultipleSelectQueryErrorTransform,
                QueryResultUniqueRow,
                QueryResultUniqueRowExpect,
                CountType,
                TtlType,
                Qv,
                SelectUnique,
                SelectMultiple,
                SelectUniqueExpect,
                Insert,
                Update,
                DeleteUnique,
                Truncate
            };
        };

        if let Some(mv) = &self.table.materialized_view {
            if mv.same_columns {
                let module: TokenStream = mv.base_table_name.as_str().parse().unwrap();
                let base_table_struct_name = format_ident!("{}", mv.base_struct_name);

                tokens.extend(quote! {
                    use super::#module::#base_table_struct_name;
                });
            }
        }

        let (tokens_constant_struct, tokens_type_struct) = write_struct::write(&self);
        let (tokens_constant_primary_key, tokens_type_primary_key) =
            write_primary_key::write(&self);
        let tokens_type_updatable_columns = write_updatable_column::write(&self);

        tokens.extend(tokens_constant_struct);
        tokens.extend(tokens_constant_primary_key);
        tokens.extend(tokens_type_struct);
        tokens.extend(tokens_type_primary_key);
        tokens.extend(tokens_type_updatable_columns);

        tokens
    }

    pub fn struct_table(&self) -> StructTable {
        StructTable {
            table: self.table.clone(),
            struct_name: self.struct_name.to_string(),
        }
    }

    pub(crate) fn log_library(&self) -> TokenStream {
        self.transformer.log_library().as_str().parse().unwrap()
    }

    pub(crate) fn ident_fields(&self) -> Vec<Ident> {
        self.struct_field_metadata
            .fields
            .iter()
            .map(|f| format_ident!("{}", f.ident))
            .collect()
    }

    create_transformer!(query_result_unique_row, "QueryResultUniqueRow");
    create_transformer!(query_result_unique_row_expect, "QueryResultUniqueRowExpect");
    create_transformer!(select_multiple, "SelectMultiple");
    create_transformer!(select_unique, "SelectUnique");
    create_transformer!(select_unique_expect, "SelectUniqueExpect");
    create_transformer!(update, "Update");
    create_transformer!(truncate, "Truncate");
    create_transformer!(insert, "Insert");
    create_transformer!(delete_multiple, "DeleteMultiple");
    create_transformer!(delete_unique, "DeleteUnique");

    pub(crate) fn comma_separated_question_marks(&self, amount: usize) -> String {
        (0..amount)
            .into_iter()
            .map(|_| "?".to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub(crate) fn comma_separated_column_names(&self) -> String {
        self.comma_separated_column_names_columns(self.columns)
    }

    pub(crate) fn create_select_clause_table_table(&self, base_table: &str) -> String {
        // Create the select queries for the base table
        // Make sure the order of the columns equals the order of the struct fields of the base table
        let base_table_columns = query_columns(base_table);
        let comma_separated = self.comma_separated_column_names_columns(&base_table_columns);

        format!("select {} from {}", comma_separated, self.table.table_name)
    }

    pub(crate) fn comma_separated_column_names_columns(&self, columns: &[ColumnInTable]) -> String {
        columns
            .iter()
            .map(|c| c.column_name.clone())
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub(crate) fn create_where_clause(&self) -> String {
        let pk_fields = self
            .struct_field_metadata
            .primary_key_fields
            .iter()
            .map(|f| format!("{} = ?", f.ident.to_string()))
            .collect::<Vec<_>>()
            .join(" and ");

        format!("where {}", pk_fields)
    }

    pub(crate) fn struct_ident(&self) -> Ident {
        format_ident!("{}", self.struct_name)
    }
}
