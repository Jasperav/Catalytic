use crate::entity_writer::EntityWriter;
use crate::query_ident::{
    all_in_memory, base_table, base_table_query, create_variant, delete_fn_name, in_memory_update,
    in_memory_updates, insert_constant, insert_fn_name, insert_or_delete_fn_name,
    insert_ttl_constant, insert_ttl_fn_name, into_primary_key_owned, primary_key_owned,
    primary_key_struct, primary_key_struct_parameter, primary_key_struct_ref, qv,
    select_all_constant, select_all_count_constant, select_all_count_fn_name, select_all_fn_name,
    struct_ref, to_ref, truncate_constant, truncate_fn_name, updatable_column,
};
use crate::transformer::Transformer;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

pub(crate) fn write<T: Transformer>(
    entity_writer: &'_ EntityWriter<T>,
) -> (TokenStream, TokenStream) {
    let table_name = &entity_writer.table.table_name;
    let meta_data = entity_writer
        .transformer
        .struct_metadata(entity_writer.struct_table())
        .into_tokenstream();
    let struct_name_ident = entity_writer.struct_ident();
    let field_ts = &entity_writer.struct_field_metadata.field_ts;

    let primary_key_owned = primary_key_owned();
    let into_primary_key_owned = into_primary_key_owned();
    let pk_struct = primary_key_struct();

    let pk_struct_ref = primary_key_struct_ref();
    let pk_struct_parameter = primary_key_struct_parameter();

    let pk_fields = entity_writer.struct_field_metadata.pk_fields_ident();
    let select_all_fn_name = select_all_fn_name();
    let select_all_count_fn_name = select_all_count_fn_name();
    let select_all_count_fn_name_qv = qv(&select_all_count_fn_name);
    let log_library = entity_writer.log_library();
    let column_names = entity_writer.comma_separated_column_names();
    let disallow_partitionless_static_queries =
        entity_writer.disallow_partitionless_static_queries();
    let select_all_constant = select_all_constant();
    let select_all_query = format!(
        "select {} from {}",
        column_names, entity_writer.table.table_name
    );
    let select_all_ts = create_select_all_query(
        &entity_writer.select_multiple(),
        &select_all_fn_name,
        &struct_name_ident,
        &select_all_constant,
        &disallow_partitionless_static_queries,
    );
    let select_all_count_constant = select_all_count_constant();
    let select_all_count_query = format!("select count(*) from {}", entity_writer.table.table_name);

    let mut tokens_constants = quote! {
        /// The query to select all rows in the table
        #disallow_partitionless_static_queries
        pub const #select_all_constant: &str = #select_all_query;
        /// The query to count all rows in the table
        #disallow_partitionless_static_queries
        pub const #select_all_count_constant: &str = #select_all_count_query;
    };

    let select_unique_expect = entity_writer.select_unique_expect();

    let mut tokens_type = quote! {
        /// This is the struct which is generated from the table
        /// If you want to perform CRUD operations, do the following:
        ///     Create -> convert this struct to a borrowed struct
        ///     Read, Update, Delete -> convert this struct to a borrowed primary key struct
        /// When you converted this struct to the specified type, you will have methods available
        /// for the things you want
        #meta_data
        pub struct #struct_name_ident {
            #field_ts
        }

        impl #struct_name_ident {
            /// Create an borrowed primary key from the struct values
            /// You can use this primary key struct to perform updates, deletions and selects on
            /// a unique row
            pub fn #pk_struct_parameter(&self) -> #pk_struct_ref {
                #pk_struct_ref {
                #(#pk_fields: &self.#pk_fields),*
                }
            }

            /// Create an owned primary key from the struct values, it will actually clone the values if needed
            pub fn #primary_key_owned(&self) -> #pk_struct {
                self.primary_key().into_owned()
            }

            /// Create an owned primary key from the struct values without cloning
            pub fn #into_primary_key_owned(self) -> #pk_struct {
                #pk_struct {
                    #(#pk_fields: self.#pk_fields),*
                }
            }
        }

        /// Returns a struct that can perform a query which counts the rows in this table
        #disallow_partitionless_static_queries
        pub fn #select_all_count_fn_name_qv() -> #select_unique_expect<catalytic::query_transform::Count, &'static str, &'static [u8; 0]> {
            #select_unique_expect::new(Qv {
                query: #select_all_count_constant,
                values: &[]
            })
        }

        /// Performs the count query
        #disallow_partitionless_static_queries
        pub async fn #select_all_count_fn_name(session: &CachingSession) -> Result<QueryResultUniqueRowExpect<CountType>, SingleSelectQueryErrorTransform> {
            #select_all_count_fn_name_qv().select_count(session).await
        }

        #select_all_ts
    };

    let struct_name_ref_ident = struct_ref(&struct_name_ident);
    let to_ref_fn = to_ref();
    let ref_metadata = entity_writer
        .transformer
        .struct_ref_metadata(entity_writer.struct_table())
        .into_tokenstream();
    let mut struct_ref_fields = vec![];
    let mut from_ref = vec![];
    let mut to_ref = vec![];

    for field in &entity_writer.struct_field_metadata.fields {
        let ty = &field.borrow_ty;
        let ident = &field.field_name;
        let borrow_to_owned = &field.from_borrow_to_owned;

        struct_ref_fields.push(quote! {
           pub #ident: &'a #ty,
        });

        from_ref.push(quote! {
           #ident: f.#ident.#borrow_to_owned,
        });

        to_ref.push(quote! {
           #ident: &self.#ident,
        });
    }

    tokens_type.extend(quote! {
        /// A struct that contains borrowed values
        /// This can be used to perform an insertion that is unique identified by the values of this struct
        /// If you want to perform an update, deletion or select or a unique row, convert this
        /// struct to the primary key struct
        #ref_metadata
        pub struct #struct_name_ref_ident<'a> {
            #(#struct_ref_fields)*
        }

        impl From<#struct_name_ref_ident<'_>> for #struct_name_ident {
            /// Conversation method to go from a borrowed struct to an owned struct
            fn from(f: #struct_name_ref_ident<'_>) -> #struct_name_ident {
                #struct_name_ident {
                    #(#from_ref)*
                }
            }
        }

        impl #struct_name_ident {
            /// Conversation method to go from an owned struct to a borrowed struct
            pub fn #to_ref_fn(&self) -> #struct_name_ref_ident {
                #struct_name_ref_ident {
                    #(#to_ref)*
                }
            }
        }

        impl<'a> #struct_name_ref_ident<'a> {
            /// Conversation method to go from a borrowed struct to an owned struct
            pub fn #pk_struct_parameter(&self) -> #pk_struct_ref {
                #pk_struct_ref {
                    #(#pk_fields: self.#pk_fields),*
                }
            }
        }
    });

    match &entity_writer.table.materialized_view {
        None => {
            let question_marks =
                entity_writer.comma_separated_question_marks(entity_writer.columns.len());
            let insert_query_const_name = insert_constant();
            let insert_query = format!(
                "insert into {}({}) values ({})",
                table_name, column_names, question_marks
            );

            tokens_constants.extend(quote! {
                /// The query to insert a unique row in the table
                pub const #insert_query_const_name: &str = #insert_query;
            });

            let insert_ttl_query_const_name = insert_ttl_constant();
            let insert_ttl_query = format!("{} using ttl ?", insert_query);

            tokens_constants.extend(quote! {
                /// The query to insert a unique row in the table with a TTL
                pub const #insert_ttl_query_const_name: &str = #insert_ttl_query;
            });

            let truncate_query_const_name = truncate_constant();
            let truncate_query = format!("truncate {}", table_name);

            tokens_constants.extend(quote! {
                /// The query truncate the whole table
                pub const #truncate_query_const_name: &str = #truncate_query;
            });

            let insert_fn_name = insert_fn_name();
            let insert_constant = insert_constant();
            let insert_ttl_fn_name = insert_ttl_fn_name();
            let insert_ttl_constant = insert_ttl_constant();
            let truncate_fn_name = truncate_fn_name();
            let truncate_constant = truncate_constant();
            let field_count = entity_writer.struct_field_metadata.fields.len();
            let insert_with_ttl_values_len = field_count + 1;
            let idents = entity_writer.ident_fields();
            let truncate = entity_writer.truncate();
            let insert = entity_writer.insert();
            let insert_or_delete = insert_or_delete_fn_name();
            let delete_unique = delete_fn_name();
            let truncate_qv = qv(&truncate_fn_name);
            let insert_qv = qv(&insert_fn_name);
            let insert_ttl_qv = qv(&insert_ttl_fn_name);

            tokens_type.extend(quote! {
                /// Returns a struct that can perform a truncate operation
                pub fn #truncate_qv() -> #truncate<&'static str, &'static [u8; 0]> {
                    #truncate::new(Qv {
                        query: #truncate_constant,
                        values: &[]
                    })
                }

                /// Performs a truncate
                /// !This will delete all rows in the table!
                pub async fn #truncate_fn_name(session: &CachingSession) -> ScyllaQueryResult {
                    #truncate_qv().truncate(session).await
                }

                impl <'a> #struct_name_ref_ident<'a> {
                    /// Returns a struct that can perform an insert operation
                    pub fn #insert_qv(&self) -> Result<#insert, SerializeValuesError> {
                        let mut serialized = SerializedValues::with_capacity(#field_count);

                        #(serialized.add_value(&self.#idents)?);*;

                        Ok(#insert::new(Qv {
                            query: #insert_constant,
                            values: serialized,
                        }
                        ))
                    }

                    /// Performs an insert
                    pub async fn #insert_fn_name(&self, session: &CachingSession) -> ScyllaQueryResult {
                        #log_library::debug!("Inserting: {:#?}", self);

                        self.#insert_qv()?.insert(session).await
                    }

                    /// Returns a struct that can perform an insert operation with a TTL
                    pub fn #insert_ttl_qv(&self, ttl: TtlType) -> Result<#insert, SerializeValuesError> {
                        let mut serialized = SerializedValues::with_capacity(#insert_with_ttl_values_len);

                        #(serialized.add_value(&self.#idents)?);*;

                        serialized.add_value(&ttl)?;

                        Ok(#insert::new(Qv {
                            query: #insert_ttl_constant,
                            values: serialized,
                        }))
                    }

                    /// Performs an insert with a TTL
                    pub async fn #insert_ttl_fn_name(&self, session: &CachingSession, ttl: TtlType) -> ScyllaQueryResult {
                        #log_library::debug!("Insert with ttl {}, {:#?}", ttl, self);

                        self.#insert_ttl_qv(ttl)?.insert(session).await
                    }

                    /// Performs either an insertion or deletion, depending on the insert parameter
                    pub async fn #insert_or_delete(&self, session: &CachingSession, insert: bool) -> ScyllaQueryResult {
                        if insert {
                            self.#insert_fn_name(session).await
                        } else {
                            self.#pk_struct_parameter().#delete_unique(session).await
                        }
                    }
                }
            });

            let in_memory_updates = in_memory_updates();
            let in_memory_update = in_memory_update();
            let updatable_column = updatable_column();
            let mut variants = vec![];
            let fields = entity_writer
                .struct_field_metadata
                .non_primary_key_fields
                .iter()
                .map(|f| &f.field_name)
                .collect::<Vec<_>>();

            for field in &fields {
                variants.push(create_variant(field));
            }

            if !fields.is_empty() {
                tokens_type.extend(quote! {
                    impl #struct_name_ident {
                        /// Performs an update on the current struct based on the update parameter
                        pub fn #in_memory_update(&mut self, update: #updatable_column) {
                            match update {
                                #(#updatable_column::#variants(val) => {
                                self.#fields = val;
                                }),*
                            }
                        }

                        /// Performs multiple updates on the current struct
                        pub fn #in_memory_updates(&mut self, updates: Vec<#updatable_column>) {
                            for updatable_column in updates {
                                self.#in_memory_update(updatable_column)
                            }
                        }
                    }
                });
            }
        }
        Some(mv) => {
            if mv.same_columns {
                // Create the select queries for the base table
                // Make sure the order of the columns equals the order of the struct fields of the base table
                let select_all_constant = base_table_query(&select_all_constant);
                let query = entity_writer.create_select_clause_table_table(&mv.base_table_name);

                tokens_constants.extend(quote! {
                    /// The query to select all rows in the table, based on the base table
                    /// The order of the columns in the query are the same as the order of the columns in the base table
                    /// This means that a query can be done in this materialized view table, but a free conversation
                    /// can be done to a struct of the base table
                    #disallow_partitionless_static_queries
                    pub const #select_all_constant: &str = #query;
                });

                let select_all_base_table = create_select_all_query(
                    &entity_writer.select_multiple(),
                    &base_table(&select_all_fn_name),
                    &format_ident!("{}", mv.base_struct_name),
                    &select_all_constant,
                    &disallow_partitionless_static_queries,
                );

                tokens_type.extend(quote! {
                    #disallow_partitionless_static_queries
                    #select_all_base_table
                });
            }
        }
    }

    (tokens_constants, tokens_type)
}

fn create_select_all_query(
    select_multiple: &Ident,
    fn_name: &Ident,
    row_type: &Ident,
    select_all_query: &Ident,
    disallow_partitionless_static_queries: &TokenStream,
) -> TokenStream {
    let select_multiple_qv = qv(fn_name);
    let select_multiple_all_in_memory = all_in_memory(fn_name);

    quote! {
        /// Returns a struct that can perform a selection of all rows in the database
        #disallow_partitionless_static_queries
        pub fn #select_multiple_qv() -> #select_multiple<#row_type, &'static str, &'static [u8; 0]> {
            #select_multiple::new(Qv {
                query: #select_all_query,
                values: &[]
            })
        }

        /// Returns a struct that can perform a selection of all rows in the database
        /// with a specified page size
        #disallow_partitionless_static_queries
        pub async fn #fn_name(session: &CachingSession, page_size: Option<i32>) -> Result<TypedRowIterator<#row_type>, QueryError> {
            #select_multiple_qv().select(session, page_size).await
        }

        /// Returns a struct that can perform a selection of all rows in the database
        /// It will accumulate all rows in memory by sending paged queries
        #disallow_partitionless_static_queries
        pub async fn #select_multiple_all_in_memory(session: &CachingSession, page_size: i32) -> Result<QueryEntityVec<#row_type>, MultipleSelectQueryErrorTransform> {
            #select_multiple_qv().select_all_in_memory(session, page_size).await
        }
    }
}
