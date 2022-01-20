use heck::CamelCase;
use proc_macro2::{Ident, TokenStream};
use quote::format_ident;

pub fn primary_key_struct() -> Ident {
    format_ident!("PrimaryKey")
}

pub fn primary_key_owned() -> Ident {
    format_ident!("primary_key_owned")
}

pub fn primary_key_struct_ref() -> Ident {
    struct_ref(&primary_key_struct())
}

pub fn struct_ref(struct_ref: &Ident) -> Ident {
    format_ident!("{}Ref", struct_ref)
}

pub fn to_ref() -> TokenStream {
    "to_ref".parse().unwrap()
}

pub fn primary_key_struct_parameter() -> Ident {
    format_ident!("primary_key")
}

pub fn updatable_column() -> Ident {
    format_ident!("UpdatableColumn")
}

pub fn get_updatable_column_field(field: &Ident) -> Ident {
    format_ident!("updatable_column_{}", field)
}

pub fn updatable_column_ref() -> Ident {
    struct_ref(&updatable_column())
}

pub fn create_variant(ident: &Ident) -> Ident {
    format_ident!("{}", ident.to_string().to_camel_case())
}

pub fn update_dyn() -> Ident {
    format_ident!("update_dyn")
}

pub fn update_dyn_multiple() -> Ident {
    format_ident!("update_dyn_multiple")
}

pub fn in_memory_update() -> Ident {
    format_ident!("in_memory_update")
}

pub fn in_memory_updates() -> Ident {
    format_ident!("in_memory_updates")
}

macro_rules! write_query {
    ($fn_name: ident, $content: expr) => {
        pub fn $fn_name() -> Ident {
            format_ident!($content)
        }
    };
    ($fn_name: ident, $content: expr, $fn_name_constant: ident) => {
        write_query!($fn_name, $content);

        pub fn $fn_name_constant() -> Ident {
            format_ident!("{}_QUERY", $content.to_uppercase())
        }
    };
}

write_query!(insert_or_delete_fn_name, "insert_or_delete");
write_query!(insert_fn_name, "insert", insert_constant);
write_query!(insert_ttl_fn_name, "insert_ttl", insert_ttl_constant);
write_query!(truncate_fn_name, "truncate", truncate_constant);
write_query!(
    select_unique_fn_name,
    "select_unique",
    select_unique_constant
);
write_query!(select_unique_expect_fn_name, "select_unique_expect");
write_query!(select_all_fn_name, "select_all", select_all_constant);
write_query!(
    select_all_count_fn_name,
    "select_all_count",
    select_all_count_constant
);
write_query!(delete_fn_name, "delete", delete_constant);

pub fn base_table(ident: &Ident) -> Ident {
    format_ident!("{}_base_table", ident)
}
pub fn base_table_query(ident: &Ident) -> Ident {
    format_ident!("{}_BASE_TABLE", ident)
}

pub fn qv(ident: &Ident) -> Ident {
    format_ident!("{}_qv", ident)
}

pub fn all_in_memory(ident: &Ident) -> Ident {
    format_ident!("{}_in_memory", ident)
}

pub fn update_field(ident: &Ident) -> (Ident, Ident) {
    let update_string = format!("update_{}", ident);
    let constant = update_string.to_uppercase() + "_QUERY";

    (
        format_ident!("{}", update_string),
        format_ident!("{}", constant),
    )
}
