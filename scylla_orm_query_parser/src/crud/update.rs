use crate::crud::operation::Operation;
use scylla_orm::query_metadata::{ColumnInQuery, QueryType};

pub struct Update;

impl Operation for Update {
    fn crud_query_start(&self) -> &'static str {
        "update"
    }

    fn table_name_after(&self) -> &'static str {
        "update "
    }

    fn column_clauses(&self, query: &str) -> Vec<ColumnInQuery> {
        let s = " set ";
        let set = query.find(s).expect("Could not find 'set' in query");
        let update = &query[set + s.len()..];

        update
            .split(", ")
            .map(|u| {
                let split = u.split(" = ").collect::<Vec<_>>();

                ColumnInQuery {
                    column_name: split[0].to_string(),
                    parameterized: split[1] == "?",
                    uses_in_value: false,
                    is_part_of_where_clause: false,
                }
            })
            .collect()
    }

    fn query_type(&self, _query: &str, full_pk: bool) -> QueryType {
        // Updates are always on full primary key
        assert!(full_pk);

        QueryType::UpdateUnique
    }
}

#[test]
fn test_columns_update_clause() {
    let update = Update;
    let q = update.column_clauses("update table set a = 1");

    assert_eq!(1, q.len());
    assert_eq!("a", &q[0].column_name);

    let q = update.column_clauses("update table set a = 1, b = ?, c = 3");

    assert_eq!(3, q.len());
    assert_eq!("a", &q[0].column_name);
    assert_eq!("b", &q[1].column_name);
    assert_eq!("c", &q[2].column_name);
    assert!(!q[0].parameterized);
    assert!(q[1].parameterized);
    assert!(!q[2].parameterized);

    let q = update.column_clauses("update table set a = ?, b = ?");

    assert!(q[0].parameterized);
    assert!(q[1].parameterized);
}
