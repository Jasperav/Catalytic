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
pub const SELECT_ALL_QUERY: &str = "select a, b, c, d from another_test_table";
#[doc = r" The query to count all rows in the table"]
pub const SELECT_ALL_COUNT_QUERY: &str = "select count(*) from another_test_table";
#[doc = r" The query to insert a unique row in the table"]
pub const INSERT_QUERY: &str = "insert into another_test_table(a, b, c, d) values (?, ?, ?, ?)";
#[doc = r" The query to insert a unique row in the table with a TTL"]
pub const INSERT_TTL_QUERY: &str =
    "insert into another_test_table(a, b, c, d) values (?, ?, ?, ?) using ttl ?";
#[doc = r" The query truncate the whole table"]
pub const TRUNCATE_QUERY: &str = "truncate another_test_table";
#[doc = r" The query to retrieve a unique row in this table"]
pub const SELECT_UNIQUE_QUERY: &str =
    "select a, b, c, d from another_test_table where a = ? and b = ? and c = ?";
#[doc = "The query to update column d"]
pub const UPDATE_D_QUERY: &str =
    "update another_test_table set d = ? where a = ? and b = ? and c = ?";
#[doc = r" The query to delete a unique row in the table"]
pub const DELETE_QUERY: &str = "delete from another_test_table where a = ? and b = ? and c = ?";
#[doc = r" This is the struct which is generated from the table"]
#[doc = r" If you want to perform CRUD operations, do the following:"]
#[doc = r"     Create -> convert this struct to a borrowed struct"]
#[doc = r"     Read, Update, Delete -> convert this struct to a borrowed primary key struct"]
#[doc = r" When you converted this struct to the specified type, you will have methods available"]
#[doc = r" for the things you want"]
#[derive(
    scylla :: FromRow, scylla :: ValueList, catalytic_macro :: Mirror, Debug, Clone, PartialEq,
)]
pub struct AnotherTestTable {
    #[partition_key]
    pub a: i32,
    #[clustering_key]
    pub b: String,
    #[clustering_key]
    pub c: String,
    pub d: i32,
}
impl AnotherTestTable {
    #[doc = r" Create an borrowed primary key from the struct values"]
    #[doc = r" You can use this primary key struct to perform updates, deletions and selects on"]
    #[doc = r" a unique row"]
    pub fn primary_key(&self) -> PrimaryKeyRef {
        PrimaryKeyRef {
            a: &self.a,
            b: &self.b,
            c: &self.c,
        }
    }
    #[doc = r" Create an owned primary key from the struct values, it will actually clone the values if needed"]
    pub fn primary_key_owned(&self) -> PrimaryKey {
        self.primary_key().into_owned()
    }
    #[doc = r" Create an owned primary key from the struct values without cloning"]
    pub fn into_primary_key_owned(self) -> PrimaryKey {
        PrimaryKey {
            a: self.a,
            b: self.b,
            c: self.c,
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
pub fn select_all_qv() -> SelectMultiple<AnotherTestTable, &'static str, &'static [u8; 0]> {
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
) -> Result<TypedRowIterator<AnotherTestTable>, QueryError> {
    select_all_qv().select(session, page_size).await
}
#[doc = r" Returns a struct that can perform a selection of all rows in the database"]
#[doc = r" It will accumulate all rows in memory by sending paged queries"]
pub async fn select_all_in_memory(
    session: &CachingSession,
    page_size: i32,
) -> Result<QueryEntityVec<AnotherTestTable>, MultipleSelectQueryErrorTransform> {
    select_all_qv()
        .select_all_in_memory(session, page_size)
        .await
}
#[doc = r" A struct that contains borrowed values"]
#[doc = r" This can be used to perform an insertion that is unique identified by the values of this struct"]
#[doc = r" If you want to perform an update, deletion or select or a unique row, convert this"]
#[doc = r" struct to the primary key struct"]
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct AnotherTestTableRef<'a> {
    pub a: &'a i32,
    pub b: &'a str,
    pub c: &'a str,
    pub d: &'a i32,
}
impl From<AnotherTestTableRef<'_>> for AnotherTestTable {
    #[doc = r" Conversation method to go from a borrowed struct to an owned struct"]
    fn from(f: AnotherTestTableRef<'_>) -> AnotherTestTable {
        AnotherTestTable {
            a: f.a.clone(),
            b: f.b.to_string(),
            c: f.c.to_string(),
            d: f.d.clone(),
        }
    }
}
impl AnotherTestTable {
    #[doc = r" Conversation method to go from an owned struct to a borrowed struct"]
    pub fn to_ref(&self) -> AnotherTestTableRef {
        AnotherTestTableRef {
            a: &self.a,
            b: &self.b,
            c: &self.c,
            d: &self.d,
        }
    }
}
impl<'a> AnotherTestTableRef<'a> {
    #[doc = r" Conversation method to go from a borrowed struct to an owned struct"]
    pub fn primary_key(&self) -> PrimaryKeyRef {
        PrimaryKeyRef {
            a: self.a,
            b: self.b,
            c: self.c,
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
impl<'a> AnotherTestTableRef<'a> {
    #[doc = r" Returns a struct that can perform an insert operation"]
    pub fn insert_qv(&self) -> Result<Insert, SerializeValuesError> {
        let mut serialized = SerializedValues::new();
        serialized.add_value(&self.a)?;
        serialized.add_value(&self.b)?;
        serialized.add_value(&self.c)?;
        serialized.add_value(&self.d)?;
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
        serialized.add_value(&self.a)?;
        serialized.add_value(&self.b)?;
        serialized.add_value(&self.c)?;
        serialized.add_value(&self.d)?;
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
impl AnotherTestTable {
    #[doc = r" Performs an update on the current struct based on the update parameter"]
    pub fn in_memory_update(&mut self, update: UpdatableColumn) {
        match update {
            UpdatableColumn::D(val) => {
                self.d = val;
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
    pub a: i32,
    #[clustering_key]
    pub b: String,
    #[clustering_key]
    pub c: String,
}
#[doc = r" The borrowed primary key struct"]
#[doc = r" This struct can be used to perform reads, deletes and updates"]
#[derive(catalytic_macro :: PrimaryKey, Copy, Debug, Clone, PartialEq)]
pub struct PrimaryKeyRef<'a> {
    pub a: &'a i32,
    pub b: &'a str,
    pub c: &'a str,
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
            a: &self.a,
            b: &self.b,
            c: &self.c,
        }
    }
}
#[doc = r" Conversation method to go from a borrowed primary key to an owned primary key"]
impl From<PrimaryKeyRef<'_>> for PrimaryKey {
    fn from(f: PrimaryKeyRef<'_>) -> PrimaryKey {
        PrimaryKey {
            a: f.a.clone(),
            b: f.b.to_string(),
            c: f.c.to_string(),
        }
    }
}
impl PrimaryKeyRef<'_> {
    #[doc = r" Returns a struct that can perform a unique row selection"]
    pub fn select_unique_qv(&self) -> Result<SelectUnique<AnotherTestTable>, SerializeValuesError> {
        let mut serialized_values = SerializedValues::new();
        serialized_values.add_value(&self.a)?;
        serialized_values.add_value(&self.b)?;
        serialized_values.add_value(&self.c)?;
        Ok(SelectUnique::new(Qv {
            query: SELECT_UNIQUE_QUERY,
            values: serialized_values,
        }))
    }
    #[doc = r" Performs the unique row selection"]
    pub async fn select_unique(
        &self,
        session: &CachingSession,
    ) -> Result<QueryResultUniqueRow<AnotherTestTable>, SingleSelectQueryErrorTransform> {
        tracing::debug!(
            "Selecting unique row for table {} with values: {:#?}",
            "another_test_table",
            self
        );
        self.select_unique_qv()?.select(session).await
    }
}
impl PrimaryKeyRef<'_> {
    #[doc = r" Returns a struct that can perform a unique row selection"]
    pub fn select_unique_expect_qv(
        &self,
    ) -> Result<SelectUniqueExpect<AnotherTestTable>, SerializeValuesError> {
        let mut serialized_values = SerializedValues::new();
        serialized_values.add_value(&self.a)?;
        serialized_values.add_value(&self.b)?;
        serialized_values.add_value(&self.c)?;
        Ok(SelectUniqueExpect::new(Qv {
            query: SELECT_UNIQUE_QUERY,
            values: serialized_values,
        }))
    }
    #[doc = r" Performs the unique row selection"]
    pub async fn select_unique_expect(
        &self,
        session: &CachingSession,
    ) -> Result<QueryResultUniqueRowExpect<AnotherTestTable>, SingleSelectQueryErrorTransform> {
        tracing::debug!(
            "Selecting unique row for table {} with values: {:#?}",
            "another_test_table",
            self
        );
        self.select_unique_expect_qv()?.select(session).await
    }
}
impl PrimaryKeyRef<'_> {
    #[doc = "Returns a struct that can perform an update operation for column d"]
    pub fn update_d_qv(&self, val: &i32) -> Result<Update, SerializeValuesError> {
        let mut serialized_values = SerializedValues::new();
        serialized_values.add_value(&val)?;
        serialized_values.add_value(&self.a)?;
        serialized_values.add_value(&self.b)?;
        serialized_values.add_value(&self.c)?;
        Ok(Update::new(Qv {
            query: UPDATE_D_QUERY,
            values: serialized_values,
        }))
    }
    #[doc = "Performs an update operation for column d"]
    pub async fn update_d(&self, session: &CachingSession, val: &i32) -> ScyllaQueryResult {
        tracing::debug!(
            "Updating table {} with val {:#?} for row {:#?}",
            "another_test_table",
            val,
            self
        );
        self.update_d_qv(val)?.update(session).await
    }
}
impl PrimaryKeyRef<'_> {
    #[doc = r" Returns a struct that can perform an update on a dynamic updatable column"]
    pub fn update_dyn_qv(
        &self,
        val: UpdatableColumnRef<'_>,
    ) -> Result<Update, SerializeValuesError> {
        match val {
            UpdatableColumnRef::D(val) => self.update_d_qv(val),
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
                UpdatableColumnRef::D(v) => {
                    query.push(concat!(stringify!(d), " = ?"));
                    serialized_values.add_value(v)?;
                }
            }
        }
        let columns_to_update: String = query.join(", ");
        let update_statement = format!(
            "update {} set {} {}",
            "another_test_table", columns_to_update, "where a = ? and b = ? and c = ?"
        );
        serialized_values.add_value(&self.a)?;
        serialized_values.add_value(&self.b)?;
        serialized_values.add_value(&self.c)?;
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
            "another_test_table",
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
        serialized_values.add_value(&self.a)?;
        serialized_values.add_value(&self.b)?;
        serialized_values.add_value(&self.c)?;
        Ok(DeleteUnique::new(Qv {
            query: DELETE_QUERY,
            values: serialized_values,
        }))
    }
    #[doc = r" Performs a single row deletion"]
    pub async fn delete(&self, session: &CachingSession) -> ScyllaQueryResult {
        tracing::debug!(
            "Deleting a row from table {} with values {:#?}",
            "another_test_table",
            self
        );
        self.delete_qv()?.delete_unique(session).await
    }
}
#[doc = r" This struct can be converted to a borrowed struct which can be used to update single rows"]
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq)]
pub enum UpdatableColumn {
    D(i32),
}
impl UpdatableColumn {
    #[doc = r" Conversation method to go from an owned updatable column struct to a borrowed updatable column struct"]
    pub fn to_ref(&self) -> UpdatableColumnRef<'_> {
        match &self {
            UpdatableColumn::D(v) => UpdatableColumnRef::D(v),
        }
    }
}
#[doc = r" This struct can be used to update columns"]
#[doc = r" If you have a borrowed primary key and you want to update a column, you can pass in"]
#[doc = r" one of the variants"]
#[derive(Copy, Debug, Clone, PartialEq)]
pub enum UpdatableColumnRef<'a> {
    D(&'a i32),
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
            UpdatableColumnRef::D(v) => UpdatableColumn::D(v.clone()),
        }
    }
}
impl UpdatableColumnRef<'_> {
    #[doc = r" Conversation method to go from a borrowed updatable column struct to an owned updatable column struct"]
    pub fn into_owned(self) -> UpdatableColumn {
        self.into()
    }
}
impl AnotherTestTable {
    #[doc = "Creates the updatable column d which can be used to update it in the database"]
    pub fn updatable_column_d(&self) -> UpdatableColumnRef {
        UpdatableColumnRef::D(&self.d)
    }
}
