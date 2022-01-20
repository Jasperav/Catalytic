use crate::crud::operation::Operation;
use catalytic::query_metadata::{ColumnInQuery, QueryType};

pub struct Delete;

impl Operation for Delete {
    fn crud_query_start(&self) -> &'static str {
        "delete"
    }

    fn table_name_after(&self) -> &'static str {
        "from "
    }

    fn column_clauses(&self, _query: &str) -> Vec<ColumnInQuery> {
        vec![]
    }

    fn query_type(&self, _query: &str, full_pk: bool) -> QueryType {
        if full_pk {
            QueryType::DeleteUnique
        } else {
            QueryType::DeleteMultiple
        }
    }
}
