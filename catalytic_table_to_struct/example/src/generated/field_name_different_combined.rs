// Generated file
#[allow(unused_imports)]
use catalytic::query_transform::{
    CountType, DeleteUnique, Insert, MultipleSelectQueryErrorTransform, QueryEntityVec,
    QueryEntityVecResult, QueryResultUniqueRow, QueryResultUniqueRowExpect, Qv, ScyllaQueryResult,
    SelectMultiple, SelectUnique, SelectUniqueExpect, SingleSelectQueryErrorTransform, Truncate,
    TtlType, Update,
};
use catalytic::scylla;
#[allow(unused_imports)]
use scylla::frame::value::SerializeValuesError;
use scylla::frame::value::SerializedValues;
use scylla::transport::errors::QueryError;
use scylla::transport::iterator::TypedRowIterator;
use scylla::CachingSession;
#[doc = r" The query to select all rows in the table"]
pub const SELECT_ALL_QUERY: &str = "select type, pub, struct from field_name_different_combined";
#[doc = r" The query to count all rows in the table"]
pub const SELECT_ALL_COUNT_QUERY: &str = "select count(*) from field_name_different_combined";
#[doc = r" The query to insert a unique row in the table"]
pub const INSERT_QUERY: &str =
    "insert into field_name_different_combined(type, pub, struct) values (?, ?, ?)";
#[doc = r" The query to insert a unique row in the table with a TTL"]
pub const INSERT_TTL_QUERY: &str =
    "insert into field_name_different_combined(type, pub, struct) values (?, ?, ?) using ttl ?";
#[doc = r" The query truncate the whole table"]
pub const TRUNCATE_QUERY: &str = "truncate field_name_different_combined";
#[doc = r" The query to retrieve a unique row in this table"]
pub const SELECT_UNIQUE_QUERY: &str =
    "select type, pub, struct from field_name_different_combined where type = ? and pub = ?";
#[doc = "The query to update column struct"]
pub const UPDATE_ROW_STRUCT_QUERY: &str =
    "update field_name_different_combined set struct = ? where type = ? and pub = ?";
#[doc = r" The query to delete a unique row in the table"]
pub const DELETE_QUERY: &str =
    "delete from field_name_different_combined where type = ? and pub = ?";
#[doc = r" This is the struct which is generated from the table"]
#[doc = r" If you want to perform CRUD operations, do the following:"]
#[doc = r"     Create -> convert this struct to a borrowed struct"]
#[doc = r"     Read, Update, Delete -> convert this struct to a borrowed primary key struct"]
#[doc = r" When you converted this struct to the specified type, you will have methods available"]
#[doc = r" for the things you want"]
#[derive(
    scylla :: FromRow, scylla :: ValueList, catalytic_macro :: Mirror, Debug, Clone, PartialEq,
)]
pub struct FieldNameDifferentCombined {
    #[partition_key]
    pub row_type: i32,
    #[clustering_key]
    pub row_pub: String,
    pub row_struct: String,
}
impl FieldNameDifferentCombined {
    #[doc = r" Create an borrowed primary key from the struct values"]
    #[doc = r" You can use this primary key struct to perform updates, deletions and selects on"]
    #[doc = r" a unique row"]
    pub fn primary_key(&self) -> PrimaryKeyRef {
        PrimaryKeyRef {
            row_type: &self.row_type,
            row_pub: &self.row_pub,
        }
    }
    #[doc = r" Create an owned primary key from the struct values, it will actually clone the values if needed"]
    pub fn primary_key_owned(&self) -> PrimaryKey {
        self.primary_key().into_owned()
    }
    #[doc = r" Create an owned primary key from the struct values without cloning"]
    pub fn into_primary_key_owned(self) -> PrimaryKey {
        PrimaryKey {
            row_type: self.row_type,
            row_pub: self.row_pub,
        }
    }
}
#[doc = r" Returns a struct that can perform a query which counts the rows in this table"]
pub fn select_all_count_qv(
) -> SelectUniqueExpect<catalytic::query_transform::Count, &'static str, &'static [u8; 0]> {
    SelectUniqueExpect::new(Qv {
        query: SELECT_ALL_COUNT_QUERY,
        values: &[],
    })
}
#[doc = r" Performs the count query"]
pub async fn select_all_count(
    session: &CachingSession,
) -> Result<QueryResultUniqueRowExpect<CountType>, SingleSelectQueryErrorTransform> {
    select_all_count_qv().select_count(session).await
}
#[doc = r" Returns a struct that can perform a selection of all rows in the database"]
pub fn select_all_qv() -> SelectMultiple<FieldNameDifferentCombined, &'static str, &'static [u8; 0]>
{
    SelectMultiple::new(Qv {
        query: SELECT_ALL_QUERY,
        values: &[],
    })
}
#[doc = r" Returns a struct that can perform a selection of all rows in the database"]
#[doc = r" with a specified page size"]
pub async fn select_all(
    session: &CachingSession,
    page_size: Option<i32>,
) -> Result<TypedRowIterator<FieldNameDifferentCombined>, QueryError> {
    select_all_qv().select(session, page_size).await
}
#[doc = r" Returns a struct that can perform a selection of all rows in the database"]
#[doc = r" It will accumulate all rows in memory by sending paged queries"]
pub async fn select_all_in_memory(
    session: &CachingSession,
    page_size: i32,
) -> Result<QueryEntityVec<FieldNameDifferentCombined>, MultipleSelectQueryErrorTransform> {
    select_all_qv()
        .select_all_in_memory(session, page_size)
        .await
}
#[doc = r" A struct that contains borrowed values"]
#[doc = r" This can be used to perform an insertion that is unique identified by the values of this struct"]
#[doc = r" If you want to perform an update, deletion or select or a unique row, convert this"]
#[doc = r" struct to the primary key struct"]
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct FieldNameDifferentCombinedRef<'a> {
    pub row_type: &'a i32,
    pub row_pub: &'a str,
    pub row_struct: &'a str,
}
impl From<FieldNameDifferentCombinedRef<'_>> for FieldNameDifferentCombined {
    #[doc = r" Conversation method to go from a borrowed struct to an owned struct"]
    fn from(f: FieldNameDifferentCombinedRef<'_>) -> FieldNameDifferentCombined {
        FieldNameDifferentCombined {
            row_type: f.row_type.clone(),
            row_pub: f.row_pub.to_string(),
            row_struct: f.row_struct.to_string(),
        }
    }
}
impl FieldNameDifferentCombined {
    #[doc = r" Conversation method to go from an owned struct to a borrowed struct"]
    pub fn to_ref(&self) -> FieldNameDifferentCombinedRef {
        FieldNameDifferentCombinedRef {
            row_type: &self.row_type,
            row_pub: &self.row_pub,
            row_struct: &self.row_struct,
        }
    }
}
impl<'a> FieldNameDifferentCombinedRef<'a> {
    #[doc = r" Conversation method to go from a borrowed struct to an owned struct"]
    pub fn primary_key(&self) -> PrimaryKeyRef {
        PrimaryKeyRef {
            row_type: self.row_type,
            row_pub: self.row_pub,
        }
    }
}
#[doc = r" Returns a struct that can perform a truncate operation"]
pub fn truncate_qv() -> Truncate<&'static str, &'static [u8; 0]> {
    Truncate::new(Qv {
        query: TRUNCATE_QUERY,
        values: &[],
    })
}
#[doc = r" Performs a truncate"]
#[doc = r" !This will delete all rows in the table!"]
pub async fn truncate(session: &CachingSession) -> ScyllaQueryResult {
    truncate_qv().truncate(session).await
}
impl<'a> FieldNameDifferentCombinedRef<'a> {
    #[doc = r" Returns a struct that can perform an insert operation"]
    pub fn insert_qv(&self) -> Result<Insert, SerializeValuesError> {
        let mut serialized = SerializedValues::new();
        serialized.add_value(&self.row_type)?;
        serialized.add_value(&self.row_pub)?;
        serialized.add_value(&self.row_struct)?;
        Ok(Insert::new(Qv {
            query: INSERT_QUERY,
            values: serialized,
        }))
    }
    #[doc = r" Performs an insert"]
    pub async fn insert(&self, session: &CachingSession) -> ScyllaQueryResult {
        tracing::debug!("Inserting: {:#?}", self);
        self.insert_qv()?.insert(session).await
    }
    #[doc = r" Returns a struct that can perform an insert operation with a TTL"]
    pub fn insert_ttl_qv(&self, ttl: TtlType) -> Result<Insert, SerializeValuesError> {
        let mut serialized = SerializedValues::new();
        serialized.add_value(&self.row_type)?;
        serialized.add_value(&self.row_pub)?;
        serialized.add_value(&self.row_struct)?;
        serialized.add_value(&ttl)?;
        Ok(Insert::new(Qv {
            query: INSERT_TTL_QUERY,
            values: serialized,
        }))
    }
    #[doc = r" Performs an insert with a TTL"]
    pub async fn insert_ttl(&self, session: &CachingSession, ttl: TtlType) -> ScyllaQueryResult {
        tracing::debug!("Insert with ttl {}, {:#?}", ttl, self);
        self.insert_ttl_qv(ttl)?.insert(session).await
    }
    #[doc = r" Performs either an insertion or deletion, depending on the insert parameter"]
    pub async fn insert_or_delete(
        &self,
        session: &CachingSession,
        insert: bool,
    ) -> ScyllaQueryResult {
        if insert {
            self.insert(session).await
        } else {
            self.primary_key().delete(session).await
        }
    }
}
impl FieldNameDifferentCombined {
    #[doc = r" Performs an update on the current struct based on the update parameter"]
    pub fn in_memory_update(&mut self, update: UpdatableColumn) {
        match update {
            UpdatableColumn::RowStruct(val) => {
                self.row_struct = val;
            }
        }
    }
    #[doc = r" Performs multiple updates on the current struct"]
    pub fn in_memory_updates(&mut self, updates: Vec<UpdatableColumn>) {
        for updatable_column in updates {
            self.in_memory_update(updatable_column)
        }
    }
}
#[doc = r" The owned primary key struct"]
#[doc = r" If you want to perform a read, delete or update, convert it to the borrowed type"]
#[derive(catalytic_macro :: PrimaryKey, Debug, Clone, PartialEq)]
pub struct PrimaryKey {
    #[partition_key]
    pub row_type: i32,
    #[clustering_key]
    pub row_pub: String,
}
#[doc = r" The borrowed primary key struct"]
#[doc = r" This struct can be used to perform reads, deletes and updates"]
#[derive(catalytic_macro :: PrimaryKey, Copy, Debug, Clone, PartialEq)]
pub struct PrimaryKeyRef<'a> {
    pub row_type: &'a i32,
    pub row_pub: &'a str,
}
#[doc = r" Conversation method to go from a borrowed primary key to an owned primary key"]
impl PrimaryKeyRef<'_> {
    pub fn into_owned(self) -> PrimaryKey {
        self.into()
    }
}
#[doc = r" Conversation method to go from an owned primary key to an borrowed primary key"]
impl PrimaryKey {
    pub fn to_ref(&self) -> PrimaryKeyRef<'_> {
        PrimaryKeyRef {
            row_type: &self.row_type,
            row_pub: &self.row_pub,
        }
    }
}
#[doc = r" Conversation method to go from a borrowed primary key to an owned primary key"]
impl From<PrimaryKeyRef<'_>> for PrimaryKey {
    fn from(f: PrimaryKeyRef<'_>) -> PrimaryKey {
        PrimaryKey {
            row_type: f.row_type.clone(),
            row_pub: f.row_pub.to_string(),
        }
    }
}
impl PrimaryKeyRef<'_> {
    #[doc = r" Returns a struct that can perform a unique row selection"]
    pub fn select_unique_qv(
        &self,
    ) -> Result<SelectUnique<FieldNameDifferentCombined>, SerializeValuesError> {
        let mut serialized_values = SerializedValues::new();
        serialized_values.add_value(&self.row_type)?;
        serialized_values.add_value(&self.row_pub)?;
        Ok(SelectUnique::new(Qv {
            query: SELECT_UNIQUE_QUERY,
            values: serialized_values,
        }))
    }
    #[doc = r" Performs the unique row selection"]
    pub async fn select_unique(
        &self,
        session: &CachingSession,
    ) -> Result<QueryResultUniqueRow<FieldNameDifferentCombined>, SingleSelectQueryErrorTransform>
    {
        tracing::debug!(
            "Selecting unique row for table {} with values: {:#?}",
            "field_name_different_combined",
            self
        );
        self.select_unique_qv()?.select(session).await
    }
}
impl PrimaryKeyRef<'_> {
    #[doc = r" Returns a struct that can perform a unique row selection"]
    pub fn select_unique_expect_qv(
        &self,
    ) -> Result<SelectUniqueExpect<FieldNameDifferentCombined>, SerializeValuesError> {
        let mut serialized_values = SerializedValues::new();
        serialized_values.add_value(&self.row_type)?;
        serialized_values.add_value(&self.row_pub)?;
        Ok(SelectUniqueExpect::new(Qv {
            query: SELECT_UNIQUE_QUERY,
            values: serialized_values,
        }))
    }
    #[doc = r" Performs the unique row selection"]
    pub async fn select_unique_expect(
        &self,
        session: &CachingSession,
    ) -> Result<
        QueryResultUniqueRowExpect<FieldNameDifferentCombined>,
        SingleSelectQueryErrorTransform,
    > {
        tracing::debug!(
            "Selecting unique row for table {} with values: {:#?}",
            "field_name_different_combined",
            self
        );
        self.select_unique_expect_qv()?.select(session).await
    }
}
impl PrimaryKeyRef<'_> {
    #[doc = "Returns a struct that can perform an update operation for column struct"]
    pub fn update_row_struct_qv(&self, val: &str) -> Result<Update, SerializeValuesError> {
        let mut serialized_values = SerializedValues::new();
        serialized_values.add_value(&val)?;
        serialized_values.add_value(&self.row_type)?;
        serialized_values.add_value(&self.row_pub)?;
        Ok(Update::new(Qv {
            query: UPDATE_ROW_STRUCT_QUERY,
            values: serialized_values,
        }))
    }
    #[doc = "Performs an update operation for column struct"]
    pub async fn update_row_struct(
        &self,
        session: &CachingSession,
        val: &str,
    ) -> ScyllaQueryResult {
        tracing::debug!(
            "Updating table {} with val {:#?} for row {:#?}",
            "field_name_different_combined",
            val,
            self
        );
        self.update_row_struct_qv(val)?.update(session).await
    }
}
impl PrimaryKeyRef<'_> {
    #[doc = r" Returns a struct that can perform an update on a dynamic updatable column"]
    pub fn update_dyn_qv(
        &self,
        val: UpdatableColumnRef<'_>,
    ) -> Result<Update, SerializeValuesError> {
        match val {
            UpdatableColumnRef::RowStruct(val) => self.update_row_struct_qv(val),
        }
    }
    #[doc = r" Performs the dynamic update"]
    pub async fn update_dyn(
        &self,
        session: &CachingSession,
        val: UpdatableColumnRef<'_>,
    ) -> ScyllaQueryResult {
        self.update_dyn_qv(val)?.update(session).await
    }
}
impl PrimaryKeyRef<'_> {
    #[doc = r" Returns a struct that can perform a dynamic amount of column updates"]
    pub fn update_dyn_multiple_qv(
        &self,
        val: &[UpdatableColumnRef<'_>],
    ) -> Result<Update<String, SerializedValues>, SerializeValuesError> {
        if val.is_empty() {
            panic!("Empty update array")
        }
        let mut query = vec![];
        let mut serialized_values = SerializedValues::new();
        for v in val {
            match v {
                UpdatableColumnRef::RowStruct(v) => {
                    query.push(concat!(stringify!(struct), " = ?"));
                    serialized_values.add_value(v)?;
                }
            }
        }
        let columns_to_update: String = query.join(", ");
        let update_statement = format!(
            "update {} set {} {}",
            "field_name_different_combined", columns_to_update, "where type = ? and pub = ?"
        );
        serialized_values.add_value(&self.row_type)?;
        serialized_values.add_value(&self.row_pub)?;
        Ok(Update::new(Qv {
            query: update_statement,
            values: serialized_values,
        }))
    }
    #[doc = r" Performs the dynamic column updates"]
    pub async fn update_dyn_multiple(
        &self,
        session: &CachingSession,
        val: &[UpdatableColumnRef<'_>],
    ) -> ScyllaQueryResult {
        tracing::debug!(
            "Updating table {} with vals {:#?} for row {:#?}",
            "field_name_different_combined",
            val,
            self
        );
        self.update_dyn_multiple_qv(val)?.update(session).await
    }
}
impl PrimaryKeyRef<'_> {
    #[doc = r" Returns a struct that can perform a single row deletion"]
    pub fn delete_qv(&self) -> Result<DeleteUnique, SerializeValuesError> {
        let mut serialized_values = SerializedValues::new();
        serialized_values.add_value(&self.row_type)?;
        serialized_values.add_value(&self.row_pub)?;
        Ok(DeleteUnique::new(Qv {
            query: DELETE_QUERY,
            values: serialized_values,
        }))
    }
    #[doc = r" Performs a single row deletion"]
    pub async fn delete(&self, session: &CachingSession) -> ScyllaQueryResult {
        tracing::debug!(
            "Deleting a row from table {} with values {:#?}",
            "field_name_different_combined",
            self
        );
        self.delete_qv()?.delete_unique(session).await
    }
}
#[doc = r" This struct can be converted to a borrowed struct which can be used to update single rows"]
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq)]
pub enum UpdatableColumn {
    RowStruct(String),
}
impl UpdatableColumn {
    #[doc = r" Conversation method to go from an owned updatable column struct to a borrowed updatable column struct"]
    pub fn to_ref(&self) -> UpdatableColumnRef<'_> {
        match &self {
            UpdatableColumn::RowStruct(v) => UpdatableColumnRef::RowStruct(v),
        }
    }
}
#[doc = r" This struct can be used to update columns"]
#[doc = r" If you have a borrowed primary key and you want to update a column, you can pass in"]
#[doc = r" one of the variants"]
#[derive(Copy, Debug, Clone, PartialEq)]
pub enum UpdatableColumnRef<'a> {
    RowStruct(&'a str),
}
pub trait UpdatableColumnVec {
    fn to_ref(&self) -> Vec<UpdatableColumnRef<'_>>;
}
impl UpdatableColumnVec for Vec<UpdatableColumn> {
    #[doc = r" Conversation method to go from a vec of owned updatable column structs to a vec of borrowed updatable column structs"]
    fn to_ref(&self) -> Vec<UpdatableColumnRef<'_>> {
        self.iter().map(|v| v.to_ref()).collect()
    }
}
impl From<UpdatableColumnRef<'_>> for UpdatableColumn {
    #[doc = r" Conversation method to go from a borrowed updatable column struct to an owned updatable column struct"]
    fn from(f: UpdatableColumnRef<'_>) -> UpdatableColumn {
        match f {
            UpdatableColumnRef::RowStruct(v) => UpdatableColumn::RowStruct(v.to_string()),
        }
    }
}
impl UpdatableColumnRef<'_> {
    #[doc = r" Conversation method to go from a borrowed updatable column struct to an owned updatable column struct"]
    pub fn into_owned(self) -> UpdatableColumn {
        self.into()
    }
}
impl FieldNameDifferentCombined {
    #[doc = "Creates the updatable column row_struct which can be used to update it in the database"]
    pub fn updatable_column_row_struct(&self) -> UpdatableColumnRef {
        UpdatableColumnRef::RowStruct(&self.row_struct)
    }
}
