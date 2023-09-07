mod delete;
mod insert;
mod operation;
mod select;
mod truncate;
mod update;

use crate::crud::operation::Operation;
use catalytic::query_metadata::ColumnInQuery;

pub use crate::crud::operation::find_operation;

/// Extracts a table name from a query
pub fn extract_table_name<'a, S: AsRef<str>>(query: &'a S, crud: &dyn Operation) -> &'a str {
    let index = query
        .as_ref()
        .find(crud.table_name_after())
        .expect("No table name found in query")
        + crud.table_name_after().len()
        - 1;
    let suffix = &query.as_ref()[index + 1..];
    let end = suffix.find(' ').unwrap_or(suffix.len());

    &suffix[..end]
}

/// Extracts the used columns in a query
pub fn extract_columns(query: &str, crud: &dyn Operation) -> Vec<ColumnInQuery> {
    // remove the parameterized ttl and limit since it isn't a column
    let without_ttl_and_limit = query.replace(" using ttl ?", "").replace(" limit ?", "");
    let (query, query_without_where) = split_query(&without_ttl_and_limit);

    crud.column_clauses(query_without_where)
        .into_iter()
        .chain(columns_after_where(query))
        .collect()
}

/// Splits the query in a tuple
/// The first element is the query without the limit clause (if any)
/// The second element is the query without the limit and where clause
///
/// use catalytic::crud::split_query;
/// let query = "select * from my_table where a = 1 limit 5";
/// let (q, p) = split_query(query);
///
/// assert_eq!("select * from my_table where a = 1", q);
/// assert_eq!("select * from my_table", p);
///
pub fn split_query(q: &str) -> (&str, &str) {
    let mut query = q;

    // Remove the 'limit' if present
    if let Some(i) = query.rfind(" limit ") {
        query = &query[..i]
    }

    let where_clause = query.find(" where ").unwrap_or(query.len());

    (query, &query[..where_clause])
}

/// Extracts columns that are used in the where clause
///
/// use catalytic::crud::columns_after_where;
/// let query = "select * from my_table where a = ? and c in ?";
/// let extracted = columns_after_where(query);
/// assert_eq!(2, extracted.len());
/// assert_eq!("a", &extracted[0].column_name);
/// assert_eq!("c", &extracted[1].column_name);
/// assert!(!extracted[0].uses_in_value);
/// assert!(extracted[1].uses_in_value);
///
pub fn columns_after_where(query: &str) -> Vec<ColumnInQuery> {
    let w = " where ";
    let index = match query.find(w) {
        None => return vec![],
        Some(i) => i,
    };

    let mut suffix = query[index + w.len()..].to_string();
    let mut column_values = vec![];
    let operators_before_column_name = [" = ", " >= ", " > ", " <= ", " < ", " in "];

    loop {
        let mut operator_with_lowest_index = None;

        for operator in operators_before_column_name.iter() {
            if let Some(s) = suffix.find(operator) {
                match operator_with_lowest_index {
                    None => operator_with_lowest_index = Some((s, operator)),
                    Some((o, _)) => {
                        if s < o {
                            operator_with_lowest_index = Some((s, operator))
                        }
                    }
                }
            }
        }

        match operator_with_lowest_index {
            None => break,
            Some((index, operator)) => {
                let and = " and ";
                let val = &suffix[suffix.find(operator).unwrap() + operator.len()
                    ..suffix.find(and).unwrap_or(suffix.len())];

                let cv = ColumnInQuery {
                    column_name: suffix[..index].to_string(),
                    parameterized: val.contains('?'),
                    uses_in_value: operator == &" in ",
                    is_part_of_where_clause: true,
                };

                column_values.push(cv);

                if let Some(p) = suffix.find(and) {
                    suffix = suffix[p + and.len()..].to_string();
                } else {
                    break;
                }
            }
        }
    }

    column_values
}

#[cfg(test)]
mod tests {
    use crate::crud::{columns_after_where, extract_columns, split_query};

    #[test]
    fn test_query_before_process() {
        let query = "select * from some_table where a = 1 and b = ?";
        let before_where = "select * from some_table";
        let (query_extracted, before_where_extracted) = split_query(query);

        assert_eq!(query, query_extracted);
        assert_eq!(before_where, before_where_extracted);

        let (query_extracted, before_where_extracted) =
            split_query("select * from some_table where a = 1 and b = ? limit 100");

        assert_eq!(
            "select * from some_table where a = 1 and b = ?",
            query_extracted
        );
        assert_eq!(before_where, before_where_extracted);

        let query = "insert into my_table(a, b) values (1, ?)";
        let (query_extracted, before_where_extracted) = split_query(query);

        assert_eq!(query, query_extracted);
        assert_eq!(query, before_where_extracted);
    }

    #[test]
    fn test_columns_after_where() {
        let v = columns_after_where("");

        assert!(v.is_empty());

        let v = columns_after_where("select * from dummy");

        assert!(v.is_empty());

        let v = columns_after_where("select * from dummy where a = 1");

        assert_eq!("a", &v[0].column_name);
        assert!(!v[0].parameterized);
        assert!(!v[0].uses_in_value);

        let v = columns_after_where("select * from dummy where a = ?");

        assert_eq!("a", &v[0].column_name);
        assert!(v[0].parameterized);
        assert!(!v[0].uses_in_value);

        let v = columns_after_where("select * from dummy where a in ?");

        assert_eq!("a", &v[0].column_name);
        assert!(v[0].parameterized);
        assert!(v[0].uses_in_value);

        let v = columns_after_where("select * from dummy where a = 1 and b > 0 and c <= somethingrandom and d < ? and e in (hi) and f = 2");

        assert_eq!("a", &v[0].column_name);
        assert_eq!("b", &v[1].column_name);
        assert_eq!("c", &v[2].column_name);
        assert_eq!("d", &v[3].column_name);
        assert_eq!("e", &v[4].column_name);
        assert_eq!("f", &v[5].column_name);
        assert!(!v[0].parameterized);
        assert!(!v[1].parameterized);
        assert!(!v[2].parameterized);
        assert!(v[3].parameterized);
        assert!(!v[4].parameterized);
        assert!(!v[5].parameterized);

        let v = columns_after_where(
            "select * from test_table where b = ? and c = 5 and d in ? limit 1",
        );

        assert!(!v[0].uses_in_value);
        assert!(v[0].parameterized);
        assert!(!v[1].uses_in_value);
        assert!(!v[1].parameterized);
        assert!(v[2].uses_in_value);
        assert!(v[2].parameterized);
    }

    #[test]
    fn test_extract_columns() {
        let c = extract_columns(
            "select a, b as c, d from table where a = 1 and b > 2 and e in ? limit ?",
            &crate::crud::select::Select,
        );

        assert_eq!(6, c.len());
        assert_eq!("a", &c[0].column_name);
        assert_eq!("b", &c[1].column_name);
        assert_eq!("d", &c[2].column_name);
        assert_eq!("a", &c[3].column_name);
        assert_eq!("b", &c[4].column_name);
        assert_eq!("e", &c[5].column_name);
    }
}
