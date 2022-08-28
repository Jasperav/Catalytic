use catalytic::scylla;
use catalytic_macro::query;

fn main() -> Result<(), scylla::frame::value::SerializeValuesError> {
    let a = 1;

    query!("select * from test_table where b = ? and c = 2 and f = 2", a);

    Ok(())
}