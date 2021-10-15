use crate::entity_writer::EntityWriter;
use crate::query_ident::{
    base_table, base_table_query, delete_constant, delete_fn_name, primary_key_struct,
    primary_key_struct_ref, qv, select_unique_constant, select_unique_expect_fn_name,
    select_unique_fn_name, to_ref, updatable_column_ref, update_dyn, update_dyn_multiple,
    update_field,
};
use crate::transformer::Transformer;
use heck::CamelCase;
use proc_macro2::{Ident, TokenStream};
use quote::format_ident;
use quote::quote;

pub(crate) fn write<T: Transformer>(
    entity_writer: &'_ EntityWriter<T>,
) -> (TokenStream, TokenStream) {
    let primary_key_struct = primary_key_struct();
    let primary_key_struct_ref = primary_key_struct_ref();
    let struct_ident = entity_writer.struct_ident();
    let table_name = &entity_writer.table.table_name;
    let primary_key_metadata = entity_writer
        .transformer
        .primary_struct_metadata(entity_writer.struct_table())
        .into_tokenstream();
    let primary_key_ref_metadata = entity_writer
        .transformer
        .primary_struct_ref_metadata(entity_writer.struct_table())
        .into_tokenstream();
    let mut primary_key_fields = vec![];
    let mut primary_key_ref_fields = vec![];
    let mut primary_key_to_ref = vec![];
    let mut primary_key_from_ref = vec![];
    let primary_key_len = entity_writer.struct_field_metadata.primary_key_fields.len();
    let add_to_serialized_values = entity_writer
        .struct_field_metadata
        .primary_key_fields
        .iter()
        .map(|f| {
            let ident = &f.ident;

            quote! {
                serialized_values.add_value(&self.#ident)?;
            }
        })
        .collect::<Vec<_>>();

    let serialize = quote! {
        let mut serialized_values = SerializedValues::with_capacity(#primary_key_len);

        #(#add_to_serialized_values)*
    };

    for field in &entity_writer.struct_field_metadata.primary_key_fields {
        let ident_ty = &field.ident_ty;

        primary_key_fields.push(quote! {
            #ident_ty
        });

        let ident = &field.ident;
        let ty = &field.borrow_ty;
        let attributes = &field.pk_attributes;
        let from_ref = &field.from_borrow_to_owned;

        primary_key_ref_fields.push(quote! {
            #attributes
            pub #ident: &'a #ty,
        });

        primary_key_to_ref.push(quote! {
            #ident: &self.#ident,
        });

        primary_key_from_ref.push(quote! {
            #ident: f.#ident.#from_ref,
        });
    }

    let select_unique_constant = select_unique_constant();
    let select_unique_fn_name = select_unique_fn_name();
    let to_ref = to_ref();
    let select_unique_expect_fn_name = select_unique_expect_fn_name();
    let column_names = entity_writer.comma_separated_column_names();
    let where_clause = entity_writer.create_where_clause();
    let select_unique_query = format!(
        "select {} from {} {}",
        column_names, table_name, where_clause
    );
    let log_library = entity_writer.log_library();

    let mut tokens_constants = quote! {
        pub const #select_unique_constant: &str = #select_unique_query;
    };

    let mut tokens_type = quote! {
        #primary_key_metadata
        pub struct #primary_key_struct {
            #(#primary_key_fields)*
        }

        #primary_key_ref_metadata
        pub struct #primary_key_struct_ref<'a> {
            #(#primary_key_ref_fields)*
        }

        impl #primary_key_struct_ref<'_> {
            pub fn into_owned(self) -> #primary_key_struct {
                self.into()
            }
        }

        impl #primary_key_struct {
            pub fn #to_ref(&self) -> #primary_key_struct_ref<'_> {
                #primary_key_struct_ref {
                    #(#primary_key_to_ref)*
                }
            }
        }

        impl From<#primary_key_struct_ref<'_>> for #primary_key_struct {
            fn from(f: #primary_key_struct_ref<'_>) -> #primary_key_struct {
                #primary_key_struct {
                    #(#primary_key_from_ref)*
                }
            }
        }
    };

    tokens_type.extend(create_select_unique(
        entity_writer,
        &log_library,
        &serialize,
        &primary_key_struct_ref,
        &select_unique_fn_name,
        &select_unique_expect_fn_name,
        &struct_ident,
        &select_unique_constant,
    ));

    match &entity_writer.table.materialized_view {
        Some(mv) => {
            if mv.same_columns {
                // Create an autoconvert method
                let base_struct = format_ident!("{}", mv.base_struct_name);
                let select_unique_constant = base_table_query(&select_unique_constant);
                let select_unique_query = format!(
                    "{} {}",
                    entity_writer.create_select_clause_table_table(&mv.base_table_name),
                    where_clause
                );

                tokens_constants.extend(quote! {
                    pub const #select_unique_constant: &str = #select_unique_query;
                });

                tokens_type.extend(create_select_unique(
                    entity_writer,
                    &log_library,
                    &serialize,
                    &primary_key_struct_ref,
                    &base_table(&select_unique_fn_name),
                    &base_table(&select_unique_expect_fn_name),
                    &base_struct,
                    &select_unique_constant,
                ));
            }
        }
        None => {
            if !entity_writer
                .struct_field_metadata
                .non_primary_key_fields
                .is_empty()
            {
                let update = entity_writer.update();

                // Write the update methods
                for field in &entity_writer.struct_field_metadata.non_primary_key_fields {
                    let (method_name, constant) = update_field(&field.ident);
                    let ty = &field.borrow_ty;
                    let update_query = format!(
                        "update {} set {} = ? {}",
                        table_name, field.ident, where_clause
                    );
                    let single_update_len = primary_key_len + 1;
                    let method_name_qv = qv(&method_name);

                    tokens_constants.extend(quote! {
                        pub const #constant: &str = #update_query;
                    });

                    tokens_type.extend(quote! {
                    impl #primary_key_struct_ref<'_> {
                        pub fn #method_name_qv(&self, val: &#ty) -> Result<Update<&'static str, SerializedValues>, SerializeValuesError> {
                            let mut serialized_values = SerializedValues::with_capacity(#single_update_len);

                            serialized_values.add_value(&val)?;

                            #(#add_to_serialized_values)*;

                            Ok(#update::new(Qv {
                                    query: #constant,
                                    values: serialized_values
                                }
                            ))
                        }

                        pub async fn #method_name(
                            &self,
                            session: &Session,
                            val: &#ty,
                        ) -> ScyllaQueryResult {
                            #log_library::debug!("Updating table {} with val {:#?} for row {:#?}", #table_name, val, self);

                            self.#method_name_qv(val)?.update(session).await
                        }
                    }
                });
                }

                let update_dyn = update_dyn();
                let update_dyn_qv = qv(&update_dyn);
                let updatable_column_ref = updatable_column_ref();
                let mut variants = vec![];
                let mut method_names_qv = vec![];

                for f in entity_writer
                    .struct_field_metadata
                    .non_primary_key_fields
                    .iter()
                {
                    let v = format_ident!("{}", f.ident.to_string().to_camel_case());
                    let (method_name, _) = update_field(&f.ident);

                    variants.push(quote! {
                        #v
                    });
                    method_names_qv.push(qv(&method_name));
                }

                tokens_type.extend(quote! {
                impl #primary_key_struct_ref<'_> {
                    pub fn #update_dyn_qv(&self, val: #updatable_column_ref<'_>) -> Result<Update<&'static str, SerializedValues>, SerializeValuesError> {
                        match val {
                            #(#updatable_column_ref::#variants(val) => self.#method_names_qv(val)),*
                        }
                    }

                    pub async fn #update_dyn(&self, session: &Session, val: #updatable_column_ref<'_>) -> ScyllaQueryResult {
                        self.#update_dyn_qv(val)?.update(session).await
                    }
                }
            });

                let update_dyn_multiple = update_dyn_multiple();
                let update_dyn_multiple_qv = qv(&update_dyn_multiple);
                let update_column = entity_writer
                    .struct_field_metadata
                    .non_primary_key_fields
                    .iter()
                    .map(|f| &f.ident)
                    .collect::<Vec<_>>();

                tokens_type.extend(quote! {
                impl #primary_key_struct_ref<'_> {
                    pub fn #update_dyn_multiple_qv(&self, val: &[#updatable_column_ref<'_>]) -> Result<Update<String, SerializedValues>, SerializeValuesError> {
                         if val.is_empty() {
                            panic!("Empty update array")
                        }

                        let mut query = vec![];
                        let mut serialized_values = SerializedValues::with_capacity(val.len() + #primary_key_len);

                        for v in val {
                            match v {
                                #(#updatable_column_ref::#variants(v) => {
                                    query.push(concat!(stringify!(#update_column), " = ?"));
                                    serialized_values.add_value(v)?;
                                }),*
                            }
                        }

                        let columns_to_update: String = query.join(", ");
                        let update_statement = format!("update {} set {} {}", #table_name, columns_to_update, #where_clause);

                        #(#add_to_serialized_values)*;

                        Ok(#update::new(
                                Qv {
                                    query: update_statement,
                                    values: serialized_values
                                }
                            ))
                    }

                    pub async fn #update_dyn_multiple(&self, session: &Session, val: &[#updatable_column_ref<'_>]) -> ScyllaQueryResult {
                        #log_library::debug!("Updating table {} with vals {:#?} for row {:#?}", #table_name, val, self);

                       self.#update_dyn_multiple_qv(val)?.update(session).await
                    }
                }
            });
            }

            let delete_fn_name = delete_fn_name();
            let delete_fn_name_qv = qv(&delete_fn_name);
            let delete_constant = delete_constant();
            let delete_query = format!("delete from {} {}", table_name, where_clause);

            tokens_constants.extend(quote! {
                pub const #delete_constant: &str = #delete_query;
            });

            let delete_unique = entity_writer.delete_unique();

            // Delete query
            tokens_type.extend(quote! {
                impl #primary_key_struct_ref<'_> {
                    pub fn #delete_fn_name_qv(&self) -> Result<DeleteUnique<&'static str, SerializedValues>, SerializeValuesError> {
                        #serialize

                            Ok(#delete_unique::new(
                                Qv {
                                    query: #delete_constant,
                                    values: serialized_values
                                }
                            ))
                    }

                    pub async fn #delete_fn_name(&self, session: &Session) -> ScyllaQueryResult {
                        #log_library::debug!("Deleting a row from table {} with values {:#?}", #table_name, self);

                        self.#delete_fn_name_qv()?.delete_unique(session).await
                    }
                }
            })
        }
    }

    (tokens_constants, tokens_type)
}

#[allow(clippy::too_many_arguments)]
fn create_select_unique<T: Transformer>(
    entity_writer: &'_ EntityWriter<T>,
    log_library: &TokenStream,
    serialize: &TokenStream,
    primary_key_struct: &Ident,
    fn_name_no_expect: &Ident,
    fn_name_expect: &Ident,
    struct_ident: &Ident,
    select_unique_constant: &Ident,
) -> TokenStream {
    let table = &entity_writer.table.table_name;

    let write_query = |fn_name, return_type, transformer| {
        let fn_name_qv = qv(fn_name);

        quote! {
            impl #primary_key_struct<'_> {
                pub fn #fn_name_qv(&self) -> Result<#transformer<&'static str, #struct_ident, SerializedValues>, SerializeValuesError> {
                    #serialize

                    Ok(#transformer::new(
                        Qv {
                        query: #select_unique_constant,
                        values: serialized_values,
                    }))
                }

                pub async fn #fn_name(&self, session: &Session) -> Result<#return_type<#struct_ident>, SingleSelectQueryErrorTransform> {
                    #log_library::debug!("Selecting unique row for table {} with values: {:#?}", #table, self);

                    self.#fn_name_qv()?.select(session).await
                }
            }
        }
    };

    let mut unique_row = write_query(
        fn_name_no_expect,
        entity_writer.query_result_unique_row(),
        entity_writer.select_unique(),
    );
    let expect = write_query(
        fn_name_expect,
        entity_writer.query_result_unique_row_expect(),
        entity_writer.select_unique_expect(),
    );

    unique_row.extend(expect);

    unique_row
}
