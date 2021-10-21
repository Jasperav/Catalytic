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
pub const SELECT_ALL_QUERY: &str = "select u from uuidtable";
pub const SELECT_ALL_COUNT_QUERY: &str = "select count(*) from uuidtable";
pub const INSERT_QUERY: &str = "insert into uuidtable(u) values (?)";
pub const INSERT_TTL_QUERY: &str = "insert into uuidtable(u) values (?) using ttl ?";
pub const TRUNCATE_QUERY: &str = "truncate uuidtable";
pub const SELECT_UNIQUE_QUERY: &str = "select u from uuidtable where u = ?";
pub const DELETE_QUERY: &str = "delete from uuidtable where u = ?";
#[derive(
    scylla :: FromRow, scylla :: ValueList, scylla_orm_macro :: Mirror, Debug, Clone, PartialEq,
)]
pub struct Uuidtable {
    #[partition_key]
    pub u: uuid::Uuid,
}
impl Uuidtable {
    pub fn primary_key(&self) -> PrimaryKeyRef {
        PrimaryKeyRef { u: &self.u }
    }
    pub fn primary_key_owned(self) -> PrimaryKey {
        PrimaryKey { u: self.u }
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
pub fn select_all_qv() -> SelectMultiple<&'static str, Uuidtable, &'static [u8; 0]> {
    SelectMultiple::new(Qv {
        query: SELECT_ALL_QUERY,
        values: &[],
    })
}
pub async fn select_all(
    session: &Session,
    page_size: Option<i32>,
) -> Result<TypedRowIterator<Uuidtable>, QueryError> {
    select_all_qv().select(session, page_size).await
}
pub async fn select_all_in_memory(
    session: &Session,
    page_size: i32,
) -> Result<QueryEntityVec<Uuidtable>, MultipleSelectQueryErrorTransform> {
    select_all_qv()
        .select_all_in_memory(session, page_size)
        .await
}
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct UuidtableRef<'a> {
    pub u: &'a uuid::Uuid,
}
impl From<UuidtableRef<'_>> for Uuidtable {
    fn from(f: UuidtableRef<'_>) -> Uuidtable {
        Uuidtable { u: f.u.clone() }
    }
}
impl Uuidtable {
    pub fn to_ref(&self) -> UuidtableRef {
        UuidtableRef { u: &self.u }
    }
}
impl<'a> UuidtableRef<'a> {
    pub fn primary_key(&self) -> PrimaryKeyRef {
        PrimaryKeyRef { u: self.u }
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
impl<'a> UuidtableRef<'a> {
    pub fn insert_qv(
        &self,
    ) -> Result<Insert<&'static str, SerializedValues>, SerializeValuesError> {
        let mut serialized = SerializedValues::with_capacity(1usize);
        serialized.add_value(&self.u)?;
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
        let mut serialized = SerializedValues::with_capacity(2usize);
        serialized.add_value(&self.u)?;
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
    pub u: uuid::Uuid,
}
#[derive(scylla_orm_macro :: PrimaryKey, Copy, Debug, Clone, PartialEq)]
pub struct PrimaryKeyRef<'a> {
    pub u: &'a uuid::Uuid,
}
impl PrimaryKeyRef<'_> {
    pub fn into_owned(self) -> PrimaryKey {
        self.into()
    }
}
impl PrimaryKey {
    pub fn to_ref(&self) -> PrimaryKeyRef<'_> {
        PrimaryKeyRef { u: &self.u }
    }
}
impl From<PrimaryKeyRef<'_>> for PrimaryKey {
    fn from(f: PrimaryKeyRef<'_>) -> PrimaryKey {
        PrimaryKey { u: f.u.clone() }
    }
}
impl PrimaryKeyRef<'_> {
    pub fn select_unique_qv(
        &self,
    ) -> Result<SelectUnique<&'static str, Uuidtable, SerializedValues>, SerializeValuesError> {
        let mut serialized_values = SerializedValues::with_capacity(1usize);
        serialized_values.add_value(&self.u)?;
        Ok(SelectUnique::new(Qv {
            query: SELECT_UNIQUE_QUERY,
            values: serialized_values,
        }))
    }
    pub async fn select_unique(
        &self,
        session: &Session,
    ) -> Result<QueryResultUniqueRow<Uuidtable>, SingleSelectQueryErrorTransform> {
        tracing::debug!(
            "Selecting unique row for table {} with values: {:#?}",
            "uuidtable",
            self
        );
        self.select_unique_qv()?.select(session).await
    }
}
impl PrimaryKeyRef<'_> {
    pub fn select_unique_expect_qv(
        &self,
    ) -> Result<SelectUniqueExpect<&'static str, Uuidtable, SerializedValues>, SerializeValuesError>
    {
        let mut serialized_values = SerializedValues::with_capacity(1usize);
        serialized_values.add_value(&self.u)?;
        Ok(SelectUniqueExpect::new(Qv {
            query: SELECT_UNIQUE_QUERY,
            values: serialized_values,
        }))
    }
    pub async fn select_unique_expect(
        &self,
        session: &Session,
    ) -> Result<QueryResultUniqueRowExpect<Uuidtable>, SingleSelectQueryErrorTransform> {
        tracing::debug!(
            "Selecting unique row for table {} with values: {:#?}",
            "uuidtable",
            self
        );
        self.select_unique_expect_qv()?.select(session).await
    }
}
impl PrimaryKeyRef<'_> {
    pub fn delete_qv(
        &self,
    ) -> Result<DeleteUnique<&'static str, SerializedValues>, SerializeValuesError> {
        let mut serialized_values = SerializedValues::with_capacity(1usize);
        serialized_values.add_value(&self.u)?;
        Ok(DeleteUnique::new(Qv {
            query: DELETE_QUERY,
            values: serialized_values,
        }))
    }
    pub async fn delete(&self, session: &Session) -> ScyllaQueryResult {
        tracing::debug!(
            "Deleting a row from table {} with values {:#?}",
            "uuidtable",
            self
        );
        self.delete_qv()?.delete_unique(session).await
    }
}
