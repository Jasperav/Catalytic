use scylla_orm_macro::query;

fn main() {
    query!("select * from idontexist");
}