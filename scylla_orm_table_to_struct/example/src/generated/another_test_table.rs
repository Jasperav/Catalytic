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
pub const SELECT_ALL_QUERY: &str = "select a, b, c, d from another_test_table";
pub const SELECT_ALL_COUNT_QUERY: &str = "select count(*) from another_test_table";
pub const INSERT_QUERY: &str = "insert into another_test_table(a, b, c, d) values (?, ?, ?, ?)";
pub const INSERT_TTL_QUERY: &str =
    "insert into another_test_table(a, b, c, d) values (?, ?, ?, ?) using ttl ?";
pub const TRUNCATE_QUERY: &str = "truncate another_test_table";
pub const SELECT_UNIQUE_QUERY: &str =
    "select a, b, c, d from another_test_table where a = ? and b = ? and c = ?";
pub const UPDATE_D_QUERY: &str =
    "update another_test_table set d = ? where a = ? and b = ? and c = ?";
pub const DELETE_QUERY: &str = "delete from another_test_table where a = ? and b = ? and c = ?";
#[derive(
    scylla :: FromRow, scylla :: ValueList, scylla_orm_macro :: Mirror, Debug, Clone, PartialEq,
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
    pub fn primary_key(&self) -> PrimaryKeyRef {
        PrimaryKeyRef {
            a: &self.a,
            b: &self.b,
            c: &self.c,
        }
    }
    pub fn primary_key_owned(self) -> PrimaryKey {
        PrimaryKey {
            a: self.a,
            b: self.b,
            c: self.c,
        }
    }
}
pub fn select_all_count_qv(
) -> SelectUniqueExpect<&'static str, scylla_orm::query_transform::Count, &'static [u8; 0]> {
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
pub fn select_all_qv() -> SelectMultiple<&'static str, AnotherTestTable, &'static [u8; 0]> {
    SelectMultiple::new(Qv {
        query: SELECT_ALL_QUERY,
        values: &[],
    })
}
pub async fn select_all(
    session: &Session,
    page_size: Option<i32>,
) -> Result<TypedRowIterator<AnotherTestTable>, QueryError> {
    select_all_qv().select(session, page_size).await
}
pub async fn select_all_in_memory(
    session: &Session,
    page_size: i32,
) -> Result<QueryEntityVec<AnotherTestTable>, MultipleSelectQueryErrorTransform> {
    select_all_qv()
        .select_all_in_memory(session, page_size)
        .await
}
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct AnotherTestTableRef<'a> {
    pub a: &'a i32,
    pub b: &'a str,
    pub c: &'a str,
    pub d: &'a i32,
}
impl From<AnotherTestTableRef<'_>> for AnotherTestTable {
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
    pub fn primary_key(&self) -> PrimaryKeyRef {
        PrimaryKeyRef {
            a: self.a,
            b: self.b,
            c: self.c,
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
impl<'a> AnotherTestTableRef<'a> {
    pub fn insert_qv(
        &self,
    ) -> Result<Insert<&'static str, SerializedValues>, SerializeValuesError> {
        let mut serialized = SerializedValues::with_capacity(4usize);
        serialized.add_value(&self.a)?;
        serialized.add_value(&self.b)?;
        serialized.add_value(&self.c)?;
        serialized.add_value(&self.d)?;
        Ok(Insert::new(Qv {
            query: INSERT_QUERY,
            values: serialized,
        }))
    }
    pub async fn insert(&self, session: &Session) -> ScyllaQueryResult {
        tracing::debug!("Inserting: {:#?}", self);
        self.insert_qv()?.insert(session).await
    }
    pub fn insert_ttl_qv(
        &self,
        ttl: TtlType,
    ) -> Result<Insert<&'static str, SerializedValues>, SerializeValuesError> {
        let mut serialized = SerializedValues::with_capacity(5usize);
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
#[derive(scylla_orm_macro :: PrimaryKey, Debug, Clone, PartialEq)]
pub struct PrimaryKey {
    #[partition_key]
    pub a: i32,
    #[clustering_key]
    pub b: String,
    #[clustering_key]
    pub c: String,
}
#[derive(scylla_orm_macro :: PrimaryKey, Copy, Debug, Clone, PartialEq)]
pub struct PrimaryKeyRef<'a> {
    pub a: &'a i32,
    pub b: &'a str,
    pub c: &'a str,
}
impl PrimaryKeyRef<'_> {
    pub fn into_owned(self) -> PrimaryKey {
        self.into()
    }
}
impl PrimaryKey {
    pub fn to_ref(&self) -> PrimaryKeyRef<'_> {
        PrimaryKeyRef {
            a: &self.a,
            b: &self.b,
            c: &self.c,
        }
    }
}
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
    pub fn select_unique_qv(
        &self,
    ) -> Result<SelectUnique<&'static str, AnotherTestTable, SerializedValues>, SerializeValuesError>
    {
        let mut serialized_values = SerializedValues::with_capacity(3usize);
        serialized_values.add_value(&self.a)?;
        serialized_values.add_value(&self.b)?;
        serialized_values.add_value(&self.c)?;
        Ok(SelectUnique::new(Qv {
            query: SELECT_UNIQUE_QUERY,
            values: serialized_values,
        }))
    }
    pub async fn select_unique(
        &self,
        session: &Session,
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
    pub fn select_unique_expect_qv(
        &self,
    ) -> Result<
        SelectUniqueExpect<&'static str, AnotherTestTable, SerializedValues>,
        SerializeValuesError,
    > {
        let mut serialized_values = SerializedValues::with_capacity(3usize);
        serialized_values.add_value(&self.a)?;
        serialized_values.add_value(&self.b)?;
        serialized_values.add_value(&self.c)?;
        Ok(SelectUniqueExpect::new(Qv {
            query: SELECT_UNIQUE_QUERY,
            values: serialized_values,
        }))
    }
    pub async fn select_unique_expect(
        &self,
        session: &Session,
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
    pub fn update_d_qv(
        &self,
        val: &i32,
    ) -> Result<Update<&'static str, SerializedValues>, SerializeValuesError> {
        let mut serialized_values = SerializedValues::with_capacity(4usize);
        serialized_values.add_value(&val)?;
        serialized_values.add_value(&self.a)?;
        serialized_values.add_value(&self.b)?;
        serialized_values.add_value(&self.c)?;
        Ok(Update::new(Qv {
            query: UPDATE_D_QUERY,
            values: serialized_values,
        }))
    }
    pub async fn update_d(&self, session: &Session, val: &i32) -> ScyllaQueryResult {
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
    pub fn update_dyn_qv(
        &self,
        val: UpdatableColumnRef<'_>,
    ) -> Result<Update<&'static str, SerializedValues>, SerializeValuesError> {
        match val {
            UpdatableColumnRef::D(val) => self.update_d_qv(val),
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
        let mut serialized_values = SerializedValues::with_capacity(val.len() + 3usize);
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
    pub async fn update_dyn_multiple(
        &self,
        session: &Session,
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
    pub fn delete_qv(
        &self,
    ) -> Result<DeleteUnique<&'static str, SerializedValues>, SerializeValuesError> {
        let mut serialized_values = SerializedValues::with_capacity(3usize);
        serialized_values.add_value(&self.a)?;
        serialized_values.add_value(&self.b)?;
        serialized_values.add_value(&self.c)?;
        Ok(DeleteUnique::new(Qv {
            query: DELETE_QUERY,
            values: serialized_values,
        }))
    }
    pub async fn delete(&self, session: &Session) -> ScyllaQueryResult {
        tracing::debug!(
            "Deleting a row from table {} with values {:#?}",
            "another_test_table",
            self
        );
        self.delete_qv()?.delete_unique(session).await
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum UpdatableColumn {
    D(i32),
}
impl UpdatableColumn {
    pub fn to_ref(&self) -> UpdatableColumnRef<'_> {
        match &self {
            UpdatableColumn::D(v) => UpdatableColumnRef::D(v),
        }
    }
}
#[derive(Copy, Debug, Clone, PartialEq)]
pub enum UpdatableColumnRef<'a> {
    D(&'a i32),
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
            UpdatableColumnRef::D(v) => UpdatableColumn::D(v.clone()),
        }
    }
}
impl UpdatableColumnRef<'_> {
    pub fn into_owned(self) -> UpdatableColumn {
        self.into()
    }
}
impl AnotherTestTable {
    pub fn updatable_column_d(&self) -> UpdatableColumnRef {
        UpdatableColumnRef::D(&self.d)
    }
}
