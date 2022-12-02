use crate::capitalizing::table_name_to_struct_name;
use crate::env_property_reader::keyspace;
use crate::query_metadata::query_columns;
use crate::runtime::query_collect_to_vec;
use std::collections::HashSet;

/// Information about a materialized view, as queried from the database
#[derive(scylla::FromRow)]
pub struct MaterializedViewFromDb {
    /// The table name of the materialized view
    pub table_name: String,
    /// The table name where the materialized view is based on
    pub base_table_name: String,
}

/// Detailed information about a materialized view
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MaterializedView {
    /// The corresponding Rust struct name where this materialized view should belong to
    pub struct_name: String,
    /// The table name where the materialized view is based on
    pub base_table_name: String,
    pub base_struct_name: String,
    /// Only true if the materialized view has exactly the same columns as the base table
    pub same_columns: bool,
}

/// Creates query that can be used to query all the materialized views from the database
pub fn query_for_materialized_view() -> String {
    format!(
        "select view_name as table_name, base_table_name from system_schema.views where keyspace_name = '{}'",
        keyspace()
    )
}

/// Queries all the materialized views from the database
pub fn query_materialized_views() -> Vec<MaterializedViewFromDb> {
    query_collect_to_vec(query_for_materialized_view(), [])
}

/// Queries a specific materialized view, and gives back information about the materialized view
pub fn query_materialized_view(table_name: &str) -> Option<MaterializedView> {
    let query = format!(
        "{}  and view_name = '{}'",
        query_for_materialized_view(),
        table_name
    );
    let mut rows: Vec<MaterializedViewFromDb> = query_collect_to_vec(query, []);

    if rows.is_empty() {
        return None;
    }

    assert_eq!(1, rows.len());

    let mv = rows.remove(0);

    let query_column_names = |table_name| {
        query_columns(table_name)
            .into_iter()
            .map(|r| r.column_name)
            .collect::<HashSet<_>>()
    };

    // Check if the base table has the same columns
    let columns_base = query_column_names(&mv.base_table_name);
    let columns_own = query_column_names(table_name);
    let base_table_name = mv.base_table_name;
    let base_struct_name = table_name_to_struct_name(&base_table_name);

    Some(MaterializedView {
        base_table_name,
        struct_name: table_name_to_struct_name(&mv.table_name),
        same_columns: columns_base.eq(&columns_own),
        base_struct_name,
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::runtime::{query, TEST_TABLE};

    #[test]
    fn materialized_view() {
        dotenv::dotenv().unwrap();

        let test_table_mv = format!("{}_mv", TEST_TABLE);
        let test_table_mv_struct_name = table_name_to_struct_name(&test_table_mv);
        let drop_mv = || {
            query(
                format!("drop materialized view if exists {}", test_table_mv),
                [],
            );
        };

        drop_mv();

        let assert = |val| {
            assert_eq!(val, query_materialized_view(&test_table_mv));
            drop_mv();
        };

        assert(None);

        let add_materialized_view = |select| {
            query(format!("
            create materialized view {} as
                                select {}
                                from {}
                                where b is not null and c is not null and a is not null and d is not null
                                primary key ((b), c, d, a)", &test_table_mv, select, TEST_TABLE), []);
        };

        let mv_inf = |same_columns| MaterializedView {
            base_table_name: TEST_TABLE.to_string(),
            struct_name: test_table_mv_struct_name.clone(),
            same_columns,
            base_struct_name: "TestTable".to_string(),
        };

        add_materialized_view("*");

        assert(Some(mv_inf(true)));

        add_materialized_view("a, b, c, d");

        assert(Some(mv_inf(false)));

        add_materialized_view("a, b, c, d, e");

        assert(Some(mv_inf(true)));

        add_materialized_view("a, b, c, e, d");

        // Order is different, but for some reason it keeps working
        // The materialized view is created with the same order, no matter how it is specified
        assert(Some(mv_inf(true)));
    }
}
