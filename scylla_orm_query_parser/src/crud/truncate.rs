use crate::crud::operation::Operation;
use scylla_orm::query_metadata::{ColumnInQuery, QueryType};

pub struct Truncate;

impl Operation for Truncate {
    fn crud_query_start(&self) -> &'static str {
        "truncate"
    }

    fn table_name_after(&self) -> &'static str {
        " "
    }

    fn column_clauses(&self, query: &str) -> Vec<ColumnInQuery> {
        // Check if a table is followed after 'truncate' and nothing more
        // A check on a single whitespace is enough
        let table = &query[query.find(' ').unwrap() + 1..];
        // If there is another whitespace, it is an invalid query
        assert!(
            !table.contains(' '),
            "This truncate query contains multiple whitespaces, but only 1 is allowed: '{}'",
            query
        );

        // No columns are present in a truncate query
        vec![]
    }

    fn query_type(&self, _query: &str, full_pk: bool) -> QueryType {
        assert!(!full_pk);

        QueryType::Truncate
    }
}

#[test]
fn valid() {
    let select = Truncate;
    let _ = select.column_clauses("truncate my_table");
}

#[test]
#[should_panic]
fn invalid() {
    let select = Truncate;
    let _ = select.column_clauses("truncate my_table where a = 1");
}
