use crate::crud::delete::Delete;
use crate::crud::insert::Insert;
use crate::crud::select::Select;
use crate::crud::truncate::Truncate;
use crate::crud::update::Update;
use scylla_orm::query_metadata::{ColumnInQuery, QueryType};

/// Trait that is implemented for every CRUD operation
pub trait Operation {
    /// Is either insert, select, update or delete
    /// Based on this value, the correct CRUD operation can be determined from a query
    fn crud_query_start(&self) -> &'static str;

    /// Is either from, into or update
    /// Based on this value, the table name can be determined to execute the CRUD operation for
    fn table_name_after(&self) -> &'static str;

    /// Determines all the columns that are used in the query
    fn column_clauses(&self, query: &str) -> Vec<ColumnInQuery>;

    /// Determines the query type for the query
    /// parameter full_pk means if the query parameter contains the full primary key
    fn query_type(&self, query: &str, full_pk: bool) -> QueryType;
}

/// Dynamically choose from a given query the correct CRUD type
pub fn find_operation(query: &str) -> Box<dyn Operation> {
    let cruds: Vec<Box<dyn Operation>> = vec![
        Box::new(Select),
        Box::new(Update),
        Box::new(Insert),
        Box::new(Delete),
        Box::new(Truncate),
    ];

    cruds
        .into_iter()
        .find(|c| query.starts_with(c.crud_query_start()))
        .expect("Queries should start with select, update, delete, insert or truncate")
}
