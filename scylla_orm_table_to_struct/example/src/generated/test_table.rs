// Generated file
#[allow(unused_imports)]
use scylla::frame::value::SerializeValuesError;
use scylla::frame::value::SerializedValues;
use scylla::transport::errors::QueryError;
use scylla::transport::iterator::TypedRowIterator;
use scylla::Session;
#[allow(unused_imports)]
use scylla_orm::query_transform::{
    CountType, DeleteUnique, Insert, MultipleSelectQueryErrorTransform, QueryEntityVec,
    QueryEntityVecResult, QueryResultUniqueRow, QueryResultUniqueRowExpect, Qv, ScyllaQueryResult,
    SelectMultiple, SelectUnique, SelectUniqueExpect, SingleSelectQueryErrorTransform, Truncate,
    TtlType, Update,
};
pub const SELECT_ALL_QUERY: &str = "select b, c, d, a, e from test_table";
pub const SELECT_ALL_COUNT_QUERY: &str = "select count(*) from test_table";
pub const INSERT_QUERY: &str = "insert into test_table(b, c, d, a, e) values (?, ?, ?, ?, ?)";
pub const INSERT_TTL_QUERY: &str =
    "insert into test_table(b, c, d, a, e) values (?, ?, ?, ?, ?) using ttl ?";
pub const TRUNCATE_QUERY: &str = "truncate test_table";
pub const SELECT_UNIQUE_QUERY: &str =
    "select b, c, d, a, e from test_table where b = ? and c = ? and d = ? and a = ?";
pub const UPDATE_E_QUERY: &str =
    "update test_table set e = ? where b = ? and c = ? and d = ? and a = ?";
pub const DELETE_QUERY: &str = "delete from test_table where b = ? and c = ? and d = ? and a = ?";
#[derive(
    scylla :: FromRow, scylla :: ValueList, scylla_orm_macro :: Mirror, Debug, Clone, PartialEq,
)]
pub struct TestTable {
    #[partition_key]
    pub b: i32,
    #[partition_key]
    pub c: i32,
    #[clustering_key]
    pub d: i32,
    #[clustering_key]
    pub a: i32,
    pub e: i32,
}
impl TestTable {
    pub fn primary_key(&self) -> PrimaryKeyRef {
        PrimaryKeyRef {
            b: &self.b,
            c: &self.c,
            d: &self.d,
            a: &self.a,
        }
    }
    pub fn primary_key_owned(self) -> PrimaryKey {
        PrimaryKey {
            b: self.b,
            c: self.c,
            d: self.d,
            a: self.a,
        }
    }
}
pub fn select_all_count_qv(
) -> SelectUniqueExpect<scylla_orm::query_transform::Count, &'static str, &'static [u8; 0]> {
    SelectUniqueExpect::new(Qv {
        query: SELECT_ALL_COUNT_QUERY,
        values: &[],
    })
}
pub async fn select_all_count(
    session: &Session,
) -> Result<QueryResultUniqueRowExpect<CountType>, SingleSelectQueryErrorTransform> {
    select_all_count_qv().select_count(session).await
}
pub fn select_all_qv() -> SelectMultiple<TestTable, &'static str, &'static [u8; 0]> {
    SelectMultiple::new(Qv {
        query: SELECT_ALL_QUERY,
        values: &[],
    })
}
pub async fn select_all(
    session: &Session,
    page_size: Option<i32>,
) -> Result<TypedRowIterator<TestTable>, QueryError> {
    select_all_qv().select(session, page_size).await
}
pub async fn select_all_in_memory(
    session: &Session,
    page_size: i32,
) -> Result<QueryEntityVec<TestTable>, MultipleSelectQueryErrorTransform> {
    select_all_qv()
        .select_all_in_memory(session, page_size)
        .await
}
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct TestTableRef<'a> {
    pub b: &'a i32,
    pub c: &'a i32,
    pub d: &'a i32,
    pub a: &'a i32,
    pub e: &'a i32,
}
impl From<TestTableRef<'_>> for TestTable {
    fn from(f: TestTableRef<'_>) -> TestTable {
        TestTable {
            b: f.b.clone(),
            c: f.c.clone(),
            d: f.d.clone(),
            a: f.a.clone(),
            e: f.e.clone(),
        }
    }
}
impl TestTable {
    pub fn to_ref(&self) -> TestTableRef {
        TestTableRef {
            b: &self.b,
            c: &self.c,
            d: &self.d,
            a: &self.a,
            e: &self.e,
        }
    }
}
impl<'a> TestTableRef<'a> {
    pub fn primary_key(&self) -> PrimaryKeyRef {
        PrimaryKeyRef {
            b: self.b,
            c: self.c,
            d: self.d,
            a: self.a,
        }
    }
}
pub fn truncate_qv() -> Truncate<&'static str, &'static [u8; 0]> {
    Truncate::new(Qv {
        query: TRUNCATE_QUERY,
        values: &[],
    })
}
pub async fn truncate(session: &Session) -> ScyllaQueryResult {
    truncate_qv().truncate(session).await
}
impl<'a> TestTableRef<'a> {
    pub fn insert_qv(&self) -> Result<Insert, SerializeValuesError> {
        let mut serialized = SerializedValues::with_capacity(5usize);
        serialized.add_value(&self.b)?;
        serialized.add_value(&self.c)?;
        serialized.add_value(&self.d)?;
        serialized.add_value(&self.a)?;
        serialized.add_value(&self.e)?;
        Ok(Insert::new(Qv {
            query: INSERT_QUERY,
            values: serialized,
        }))
    }
    pub async fn insert(&self, session: &Session) -> ScyllaQueryResult {
        tracing::debug!("Inserting: {:#?}", self);
        self.insert_qv()?.insert(session).await
    }
    pub fn insert_ttl_qv(&self, ttl: TtlType) -> Result<Insert, SerializeValuesError> {
        let mut serialized = SerializedValues::with_capacity(6usize);
        serialized.add_value(&self.b)?;
        serialized.add_value(&self.c)?;
        serialized.add_value(&self.d)?;
        serialized.add_value(&self.a)?;
        serialized.add_value(&self.e)?;
        serialized.add_value(&ttl)?;
        Ok(Insert::new(Qv {
            query: INSERT_TTL_QUERY,
            values: serialized,
        }))
    }
    pub async fn insert_ttl(&self, session: &Session, ttl: TtlType) -> ScyllaQueryResult {
        tracing::debug!("Insert with ttl {}, {:#?}", ttl, self);
        self.insert_ttl_qv(ttl)?.insert(session).await
    }
    pub async fn insert_or_delete(&self, session: &Session, insert: bool) -> ScyllaQueryResult {
        if insert {
            self.insert(session).await
        } else {
            self.primary_key().delete(session).await
        }
    }
}
impl TestTable {
    pub fn in_memory_update(&mut self, update: UpdatableColumn) {
        match update {
            UpdatableColumn::E(val) => {
                self.e = val;
            }
        }
    }
    pub fn in_memory_updates(&mut self, updates: Vec<UpdatableColumn>) {
        for updatable_column in updates {
            self.in_memory_update(updatable_column)
        }
    }
}
#[derive(scylla_orm_macro :: PrimaryKey, Debug, Clone, PartialEq)]
pub struct PrimaryKey {
    #[partition_key]
    pub b: i32,
    #[partition_key]
    pub c: i32,
    #[clustering_key]
    pub d: i32,
    #[clustering_key]
    pub a: i32,
}
#[derive(scylla_orm_macro :: PrimaryKey, Copy, Debug, Clone, PartialEq)]
pub struct PrimaryKeyRef<'a> {
    pub b: &'a i32,
    pub c: &'a i32,
    pub d: &'a i32,
    pub a: &'a i32,
}
impl PrimaryKeyRef<'_> {
    pub fn into_owned(self) -> PrimaryKey {
        self.into()
    }
}
impl PrimaryKey {
    pub fn to_ref(&self) -> PrimaryKeyRef<'_> {
        PrimaryKeyRef {
            b: &self.b,
            c: &self.c,
            d: &self.d,
            a: &self.a,
        }
    }
}
impl From<PrimaryKeyRef<'_>> for PrimaryKey {
    fn from(f: PrimaryKeyRef<'_>) -> PrimaryKey {
        PrimaryKey {
            b: f.b.clone(),
            c: f.c.clone(),
            d: f.d.clone(),
            a: f.a.clone(),
        }
    }
}
impl PrimaryKeyRef<'_> {
    pub fn select_unique_qv(&self) -> Result<SelectUnique<TestTable>, SerializeValuesError> {
        let mut serialized_values = SerializedValues::with_capacity(4usize);
        serialized_values.add_value(&self.b)?;
        serialized_values.add_value(&self.c)?;
        serialized_values.add_value(&self.d)?;
        serialized_values.add_value(&self.a)?;
        Ok(SelectUnique::new(Qv {
            query: SELECT_UNIQUE_QUERY,
            values: serialized_values,
        }))
    }
    pub async fn select_unique(
        &self,
        session: &Session,
    ) -> Result<QueryResultUniqueRow<TestTable>, SingleSelectQueryErrorTransform> {
        tracing::debug!(
            "Selecting unique row for table {} with values: {:#?}",
            "test_table",
            self
        );
        self.select_unique_qv()?.select(session).await
    }
}
impl PrimaryKeyRef<'_> {
    pub fn select_unique_expect_qv(
        &self,
    ) -> Result<SelectUniqueExpect<TestTable>, SerializeValuesError> {
        let mut serialized_values = SerializedValues::with_capacity(4usize);
        serialized_values.add_value(&self.b)?;
        serialized_values.add_value(&self.c)?;
        serialized_values.add_value(&self.d)?;
        serialized_values.add_value(&self.a)?;
        Ok(SelectUniqueExpect::new(Qv {
            query: SELECT_UNIQUE_QUERY,
            values: serialized_values,
        }))
    }
    pub async fn select_unique_expect(
        &self,
        session: &Session,
    ) -> Result<QueryResultUniqueRowExpect<TestTable>, SingleSelectQueryErrorTransform> {
        tracing::debug!(
            "Selecting unique row for table {} with values: {:#?}",
            "test_table",
            self
        );
        self.select_unique_expect_qv()?.select(session).await
    }
}
impl PrimaryKeyRef<'_> {
    pub fn update_e_qv(&self, val: &i32) -> Result<Update, SerializeValuesError> {
        let mut serialized_values = SerializedValues::with_capacity(5usize);
        serialized_values.add_value(&val)?;
        serialized_values.add_value(&self.b)?;
        serialized_values.add_value(&self.c)?;
        serialized_values.add_value(&self.d)?;
        serialized_values.add_value(&self.a)?;
        Ok(Update::new(Qv {
            query: UPDATE_E_QUERY,
            values: serialized_values,
        }))
    }
    pub async fn update_e(&self, session: &Session, val: &i32) -> ScyllaQueryResult {
        tracing::debug!(
            "Updating table {} with val {:#?} for row {:#?}",
            "test_table",
            val,
            self
        );
        self.update_e_qv(val)?.update(session).await
    }
}
impl PrimaryKeyRef<'_> {
    pub fn update_dyn_qv(
        &self,
        val: UpdatableColumnRef<'_>,
    ) -> Result<Update, SerializeValuesError> {
        match val {
            UpdatableColumnRef::E(val) => self.update_e_qv(val),
        }
    }
    pub async fn update_dyn(
        &self,
        session: &Session,
        val: UpdatableColumnRef<'_>,
    ) -> ScyllaQueryResult {
        self.update_dyn_qv(val)?.update(session).await
    }
}
impl PrimaryKeyRef<'_> {
    pub fn update_dyn_multiple_qv(
        &self,
        val: &[UpdatableColumnRef<'_>],
    ) -> Result<Update<String, SerializedValues>, SerializeValuesError> {
        if val.is_empty() {
            panic!("Empty update array")
        }
        let mut query = vec![];
        let mut serialized_values = SerializedValues::with_capacity(val.len() + 4usize);
        for v in val {
            match v {
                UpdatableColumnRef::E(v) => {
                    query.push(concat!(stringify!(e), " = ?"));
                    serialized_values.add_value(v)?;
                }
            }
        }
        let columns_to_update: String = query.join(", ");
        let update_statement = format!(
            "update {} set {} {}",
            "test_table", columns_to_update, "where b = ? and c = ? and d = ? and a = ?"
        );
        serialized_values.add_value(&self.b)?;
        serialized_values.add_value(&self.c)?;
        serialized_values.add_value(&self.d)?;
        serialized_values.add_value(&self.a)?;
        Ok(Update::new(Qv {
            query: update_statement,
            values: serialized_values,
        }))
    }
    pub async fn update_dyn_multiple(
        &self,
        session: &Session,
        val: &[UpdatableColumnRef<'_>],
    ) -> ScyllaQueryResult {
        tracing::debug!(
            "Updating table {} with vals {:#?} for row {:#?}",
            "test_table",
            val,
            self
        );
        self.update_dyn_multiple_qv(val)?.update(session).await
    }
}
impl PrimaryKeyRef<'_> {
    pub fn delete_qv(&self) -> Result<DeleteUnique, SerializeValuesError> {
        let mut serialized_values = SerializedValues::with_capacity(4usize);
        serialized_values.add_value(&self.b)?;
        serialized_values.add_value(&self.c)?;
        serialized_values.add_value(&self.d)?;
        serialized_values.add_value(&self.a)?;
        Ok(DeleteUnique::new(Qv {
            query: DELETE_QUERY,
            values: serialized_values,
        }))
    }
    pub async fn delete(&self, session: &Session) -> ScyllaQueryResult {
        tracing::debug!(
            "Deleting a row from table {} with values {:#?}",
            "test_table",
            self
        );
        self.delete_qv()?.delete_unique(session).await
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum UpdatableColumn {
    E(i32),
}
impl UpdatableColumn {
    pub fn to_ref(&self) -> UpdatableColumnRef<'_> {
        match &self {
            UpdatableColumn::E(v) => UpdatableColumnRef::E(v),
        }
    }
}
#[derive(Copy, Debug, Clone, PartialEq)]
pub enum UpdatableColumnRef<'a> {
    E(&'a i32),
}
pub trait UpdatableColumnVec {
    fn to_ref(&self) -> Vec<UpdatableColumnRef<'_>>;
}
impl UpdatableColumnVec for Vec<UpdatableColumn> {
    fn to_ref(&self) -> Vec<UpdatableColumnRef<'_>> {
        self.iter().map(|v| v.to_ref()).collect()
    }
}
impl From<UpdatableColumnRef<'_>> for UpdatableColumn {
    fn from(f: UpdatableColumnRef<'_>) -> UpdatableColumn {
        match f {
            UpdatableColumnRef::E(v) => UpdatableColumn::E(v.clone()),
        }
    }
}
impl UpdatableColumnRef<'_> {
    pub fn into_owned(self) -> UpdatableColumn {
        self.into()
    }
}
impl TestTable {
    pub fn updatable_column_e(&self) -> UpdatableColumnRef {
        UpdatableColumnRef::E(&self.e)
    }
}
