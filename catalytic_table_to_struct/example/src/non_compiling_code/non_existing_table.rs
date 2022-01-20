use catalytic_macro::query;

fn main() -> Result<(), scylla::frame::value::SerializeValuesError> {
    query!("select * from idontexist");

    Ok(())
}