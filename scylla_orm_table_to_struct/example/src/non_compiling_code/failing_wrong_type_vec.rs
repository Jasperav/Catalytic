use scylla_orm_macro::query;

fn main() {
    let a = 1;

    query!("select * from test_table where b = 1 and c in ?", a);
}
