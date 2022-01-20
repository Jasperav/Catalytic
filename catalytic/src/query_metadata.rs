use crate::env_property_reader::keyspace;
use crate::runtime::query_collect_to_vec;
use crate::sort::sort_columns;
use crate::table_metadata::{ColumnInTable, ColumnType};

/// Meta data of a query
#[derive(Debug, PartialEq, Clone)]
pub struct QueryMetadata {
    /// The query that will be send to the server
    pub query: String,
    /// The columns that are used in this query
    pub extracted_columns: Vec<ColumnInQuery>,
    /// Parameterized columns
    pub parameterized_columns_types: Vec<ParameterizedColumnType>,
    pub query_type: QueryType,
    /// The corresponding Rust struct name of the query
    pub struct_name: String,
    pub table_name: String,
    /// Only true if the query is limited
    pub limited: bool,
    /// The TTL of the query if provided
    pub ttl: Option<Ttl>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Ttl {
    Parameterized,
    Fixed(i32),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParameterizedColumnType {
    pub column_type: ColumnType,
    pub value: ParameterizedValue,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParameterizedValue {
    ExtractedColumn(ColumnInQuery),
    UsingTtl,
    Limit,
}

/// The different types of a query
#[derive(PartialEq, Debug, Clone)]
pub enum QueryType {
    /// Select multiple rows
    SelectMultiple,
    /// Selects single row because it ends with limit 1
    SelectUniqueByLimit,
    /// Select a unique row
    SelectUnique,
    /// Selects a count
    SelectCount,
    /// Updates a row
    /// Note: this is always on full primary key
    UpdateUnique,
    /// Deletes multiple rows
    DeleteMultiple,
    /// Deletes a unique row
    DeleteUnique,
    /// Inserts a single row
    InsertUnique,
    /// Truncates a table
    Truncate,
}

/// Represents a column that is used in a query
#[derive(Debug, Clone, PartialEq)]
pub struct ColumnInQuery {
    pub column_name: String,
    /// If true, the column is assigned a question mark (... where a = ?)
    /// If false, the column has a fixed value (... where a = 1)
    pub parameterized: bool,
    /// Only true if the column uses an in value (... where a in ?)
    pub uses_in_value: bool,
    /// Only true if the column is used in the where clause
    /// False if e.g. the column is part of the select clause
    pub is_part_of_where_clause: bool,
}

/// Query columns for a given table
pub fn query_columns(table: &str) -> Vec<ColumnInTable> {
    // Not sure if this works with parameters ('?')
    let query = format!("select column_name, kind, position, type as data_type from system_schema.columns where keyspace_name = '{}' and table_name = '{}'", keyspace(), table.to_lowercase());

    let mut collected = query_collect_to_vec(query, &[]);

    sort_columns(&mut collected);

    collected
}
