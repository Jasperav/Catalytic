use scylla_orm_macro::query;

fn main() {
    let a = 1;

    query!("select * from test_table where b = ? and c = 2 and f = 2", a);
}