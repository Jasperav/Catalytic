use catalytic_macro::query;

fn main() -> Result<(), scylla::frame::value::SerializeValuesError> {
    let a = 1;

    query!("insert into test_table (b, c, d, a) values (2, 3, 4, 5)");

    Ok(())
}