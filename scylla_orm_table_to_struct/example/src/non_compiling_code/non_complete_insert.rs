use scylla_orm_macro::query;

fn main() {
    let a = 1;

    query!("insert into test_table (b, c, d, a) values (2, 3, 4, 5)");
}