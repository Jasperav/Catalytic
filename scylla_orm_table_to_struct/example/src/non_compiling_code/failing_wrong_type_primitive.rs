use scylla_orm_macro::query;

fn main() {
    let b = "";

    query!("select * from test_table where b = ? and c = ?", b, b);
}