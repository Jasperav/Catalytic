use catalytic::scylla;
use catalytic_macro::query;

mod generated {
    pub use example_project::generated::TestTable;
}

fn main() -> Result<(), scylla::frame::value::SerializeValuesError> {
    let a = &1;

    query!("select * from test_table where b = 1 and c in ?", a);

    Ok(())
}
