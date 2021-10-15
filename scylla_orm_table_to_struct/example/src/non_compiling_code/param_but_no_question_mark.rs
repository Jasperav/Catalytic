use scylla_orm_macro::query;

fn main() {
    query!("select * from test_table where b = ? and c = ?", b)
}