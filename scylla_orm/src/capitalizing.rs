use heck::{CamelCase, SnakeCase};

pub fn table_name_to_struct_name(table_name: &str) -> String {
    table_name.to_camel_case()
}

pub fn struct_name_to_table_name(struct_name: &str) -> String {
    struct_name.to_snake_case()
}
