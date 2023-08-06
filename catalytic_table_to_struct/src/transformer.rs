use crate::Table;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone, PartialEq, Debug, Eq)]
pub struct StructTable {
    pub table: Table,
    pub struct_name: String,
}

pub struct StructField {
    /// Fill with the path + struct which type this should have
    pub json: Option<String>,
    /// Determines if the column is nullable
    pub is_nullable: bool,
    /// The attributes that needs to be placed on the property
    pub attributes: TokenStream,
    /// Leave empty when you want to match the column name of the database
    pub field_name: String,
}

pub const DEFAULT_DERIVES: [&str; 3] = ["Debug", "Clone", "PartialEq"];

#[derive(Debug, Clone)]
pub struct TypeMetadata {
    /// Add custom derives to the type
    pub derives: Vec<String>,
    /// Add annotations on top of the type, like #[allow(dead_code)]
    pub attributes: Vec<String>,
}

impl TypeMetadata {
    pub fn with_default_values<T: ToString>(derives: &[T]) -> TypeMetadata {
        let mut derives = derives.iter().map(|t| t.to_string()).collect::<Vec<_>>();

        derives.extend(
            DEFAULT_DERIVES
                .iter()
                .map(|d| d.to_string())
                .collect::<Vec<_>>(),
        );

        TypeMetadata {
            derives,
            attributes: vec![],
        }
    }

    pub fn add_derives<T: ToString>(&mut self, d: &[T]) {
        for derive in d {
            self.derives.push(derive.to_string());
        }
    }

    pub fn into_tokenstream(self) -> TokenStream {
        let derives = self
            .derives
            .into_iter()
            .map(|a| a.parse().unwrap())
            .collect::<Vec<TokenStream>>();
        let attributes = self
            .attributes
            .into_iter()
            .map(|a| a.parse().unwrap())
            .collect::<Vec<TokenStream>>();

        quote! {
            #(#attributes)*
            #[derive(#(#derives),*)]
        }
    }
}

/// Custom transformations can be done by implementing this trait
pub trait Transformer {
    /// The logging library to use
    /// If empty, no library is used for logging
    fn log_library(&self) -> String {
        "tracing".to_string()
    }

    /// Return true to disallow auto generated queries without a partition, like the select * from table
    /// It will still be available for debugging though
    fn disallow_partitionless_static_queries(&self) -> bool {
        false
    }

    /// Adds a way to add the JSON mapping to a property
    fn struct_field(&self, _struct_table: StructTable, _column_name: &str) -> StructField {
        StructField {
            json: None,
            is_nullable: false,
            attributes: Default::default(),
            field_name: "".to_string(),
        }
    }

    /// Add custom derives to the struct
    fn struct_metadata(&self, _struct_table: StructTable) -> TypeMetadata {
        TypeMetadata::with_default_values(&[
            "scylla::FromRow",
            "scylla::ValueList",
            "catalytic_macro::Mirror",
        ])
    }

    /// Add custom derives to the ref struct
    fn struct_ref_metadata(&self, _struct_table: StructTable) -> TypeMetadata {
        TypeMetadata::with_default_values(&["Copy"])
    }

    /// Add custom derives to the primary key struct
    fn primary_struct_metadata(&self, _struct_table: StructTable) -> TypeMetadata {
        TypeMetadata::with_default_values(&["catalytic_macro::PrimaryKey"])
    }

    /// Add custom derives to the primary key ref struct
    fn primary_struct_ref_metadata(&self, _struct_table: StructTable) -> TypeMetadata {
        TypeMetadata::with_default_values(&["catalytic_macro::PrimaryKey", "Copy"])
    }

    /// Add custom derives to the updatable column enum
    fn updatable_column_metadata(&self, _struct_table: StructTable) -> TypeMetadata {
        TypeMetadata::with_default_values::<&str>(&[])
    }

    /// Add custom derives to the updatable column ref enum
    fn updatable_column_ref_metadata(&self, _struct_table: StructTable) -> TypeMetadata {
        TypeMetadata::with_default_values::<&str>(&["Copy"])
    }
}

pub struct DefaultTransformer;

impl Transformer for DefaultTransformer {}
