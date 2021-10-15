use crate::crud::operation::Operation;
use scylla_orm::query_metadata::{ColumnInQuery, QueryType};

pub struct Insert;

impl Operation for Insert {
    fn crud_query_start(&self) -> &'static str {
        "insert"
    }

    fn table_name_after(&self) -> &'static str {
        "into "
    }

    fn column_clauses(&self, query: &str) -> Vec<ColumnInQuery> {
        let clause = self.in_between_parentheses(query);
        let new_starting_point =
            &query[query.find(')').expect("Can not find trailing ')' in query") + 1..];
        let values = self.in_between_parentheses(new_starting_point);
        let columns = self.split(clause);
        let values = self.split(values);

        columns
            .into_iter()
            .enumerate()
            .map(|(index, val)| ColumnInQuery {
                column_name: val,
                parameterized: values[index].as_str() == "?",
                uses_in_value: false,
                is_part_of_where_clause: false,
            })
            .collect()
    }

    fn query_type(&self, _query: &str, full_pk: bool) -> QueryType {
        assert!(full_pk);

        QueryType::InsertUnique
    }
}

impl Insert {
    /// Finds the content between '(' and ')'
    fn in_between_parentheses<'a>(&self, q: &'a str) -> &'a str {
        let error = |c: char| format!("Can not find '{}' in query {}", c, q);
        let find = |c: char| q.find(c).unwrap_or_else(|| panic!("{}", error(c)));
        let opening = find('(');
        let closing = find(')');

        &q[opening + 1..closing]
    }

    fn split(&self, val: &str) -> Vec<String> {
        val.split(", ").map(|c| c.to_string()).collect()
    }
}

#[test]
fn test_columns_select_clause() {
    let insert = Insert;
    let q = insert.column_clauses("insert into table (a) values (1)");

    assert_eq!(q.len(), 1);
    assert_eq!("a", &q[0].column_name);
    assert!(!q[0].parameterized);

    let q = insert.column_clauses("insert into table (a, b, c) values (1, ?, 3)");

    assert_eq!(q.len(), 3);
    assert_eq!("a", &q[0].column_name);
    assert_eq!("b", &q[1].column_name);
    assert_eq!("c", &q[2].column_name);
    assert!(!q[0].parameterized);
    assert!(q[1].parameterized);
    assert!(!q[2].parameterized);
}
