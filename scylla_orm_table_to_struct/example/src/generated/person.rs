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
pub const SELECT_ALL_QUERY: &str = "select name, age, email from person";
pub const SELECT_ALL_COUNT_QUERY: &str = "select count(*) from person";
pub const INSERT_QUERY: &str = "insert into person(name, age, email) values (?, ?, ?)";
pub const INSERT_TTL_QUERY: &str =
    "insert into person(name, age, email) values (?, ?, ?) using ttl ?";
pub const TRUNCATE_QUERY: &str = "truncate person";
pub const SELECT_UNIQUE_QUERY: &str =
    "select name, age, email from person where name = ? and age = ?";
pub const UPDATE_EMAIL_QUERY: &str = "update person set email = ? where name = ? and age = ?";
pub const DELETE_QUERY: &str = "delete from person where name = ? and age = ?";
#[derive(
    scylla :: FromRow, scylla :: ValueList, scylla_orm_macro :: Mirror, Debug, Clone, PartialEq,
)]
pub struct Person {
    #[partition_key]
    pub name: String,
    #[clustering_key]
    pub age: i32,
    pub email: String,
}
impl Person {
    pub fn primary_key(&self) -> PrimaryKeyRef {
        PrimaryKeyRef {
            name: &self.name,
            age: &self.age,
        }
    }
    pub fn primary_key_owned(self) -> PrimaryKey {
        PrimaryKey {
            name: self.name,
            age: self.age,
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
pub fn select_all_qv() -> SelectMultiple<Person, &'static str, &'static [u8; 0]> {
    SelectMultiple::new(Qv {
        query: SELECT_ALL_QUERY,
        values: &[],
    })
}
pub async fn select_all(
    session: &Session,
    page_size: Option<i32>,
) -> Result<TypedRowIterator<Person>, QueryError> {
    select_all_qv().select(session, page_size).await
}
pub async fn select_all_in_memory(
    session: &Session,
    page_size: i32,
) -> Result<QueryEntityVec<Person>, MultipleSelectQueryErrorTransform> {
    select_all_qv()
        .select_all_in_memory(session, page_size)
        .await
}
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct PersonRef<'a> {
    pub name: &'a str,
    pub age: &'a i32,
    pub email: &'a str,
}
impl From<PersonRef<'_>> for Person {
    fn from(f: PersonRef<'_>) -> Person {
        Person {
            name: f.name.to_string(),
            age: f.age.clone(),
            email: f.email.to_string(),
        }
    }
}
impl Person {
    pub fn to_ref(&self) -> PersonRef {
        PersonRef {
            name: &self.name,
            age: &self.age,
            email: &self.email,
        }
    }
}
impl<'a> PersonRef<'a> {
    pub fn primary_key(&self) -> PrimaryKeyRef {
        PrimaryKeyRef {
            name: self.name,
            age: self.age,
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
impl<'a> PersonRef<'a> {
    pub fn insert_qv(&self) -> Result<Insert, SerializeValuesError> {
        let mut serialized = SerializedValues::with_capacity(3usize);
        serialized.add_value(&self.name)?;
        serialized.add_value(&self.age)?;
        serialized.add_value(&self.email)?;
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
        let mut serialized = SerializedValues::with_capacity(4usize);
        serialized.add_value(&self.name)?;
        serialized.add_value(&self.age)?;
        serialized.add_value(&self.email)?;
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
    pub name: String,
    #[clustering_key]
    pub age: i32,
}
#[derive(scylla_orm_macro :: PrimaryKey, Copy, Debug, Clone, PartialEq)]
pub struct PrimaryKeyRef<'a> {
    pub name: &'a str,
    pub age: &'a i32,
}
impl PrimaryKeyRef<'_> {
    pub fn into_owned(self) -> PrimaryKey {
        self.into()
    }
}
impl PrimaryKey {
    pub fn to_ref(&self) -> PrimaryKeyRef<'_> {
        PrimaryKeyRef {
            name: &self.name,
            age: &self.age,
        }
    }
}
impl From<PrimaryKeyRef<'_>> for PrimaryKey {
    fn from(f: PrimaryKeyRef<'_>) -> PrimaryKey {
        PrimaryKey {
            name: f.name.to_string(),
            age: f.age.clone(),
        }
    }
}
impl PrimaryKeyRef<'_> {
    pub fn select_unique_qv(&self) -> Result<SelectUnique<Person>, SerializeValuesError> {
        let mut serialized_values = SerializedValues::with_capacity(2usize);
        serialized_values.add_value(&self.name)?;
        serialized_values.add_value(&self.age)?;
        Ok(SelectUnique::new(Qv {
            query: SELECT_UNIQUE_QUERY,
            values: serialized_values,
        }))
    }
    pub async fn select_unique(
        &self,
        session: &Session,
    ) -> Result<QueryResultUniqueRow<Person>, SingleSelectQueryErrorTransform> {
        tracing::debug!(
            "Selecting unique row for table {} with values: {:#?}",
            "person",
            self
        );
        self.select_unique_qv()?.select(session).await
    }
}
impl PrimaryKeyRef<'_> {
    pub fn select_unique_expect_qv(
        &self,
    ) -> Result<SelectUniqueExpect<Person>, SerializeValuesError> {
        let mut serialized_values = SerializedValues::with_capacity(2usize);
        serialized_values.add_value(&self.name)?;
        serialized_values.add_value(&self.age)?;
        Ok(SelectUniqueExpect::new(Qv {
            query: SELECT_UNIQUE_QUERY,
            values: serialized_values,
        }))
    }
    pub async fn select_unique_expect(
        &self,
        session: &Session,
    ) -> Result<QueryResultUniqueRowExpect<Person>, SingleSelectQueryErrorTransform> {
        tracing::debug!(
            "Selecting unique row for table {} with values: {:#?}",
            "person",
            self
        );
        self.select_unique_expect_qv()?.select(session).await
    }
}
impl PrimaryKeyRef<'_> {
    pub fn update_email_qv(&self, val: &str) -> Result<Update, SerializeValuesError> {
        let mut serialized_values = SerializedValues::with_capacity(3usize);
        serialized_values.add_value(&val)?;
        serialized_values.add_value(&self.name)?;
        serialized_values.add_value(&self.age)?;
        Ok(Update::new(Qv {
            query: UPDATE_EMAIL_QUERY,
            values: serialized_values,
        }))
    }
    pub async fn update_email(&self, session: &Session, val: &str) -> ScyllaQueryResult {
        tracing::debug!(
            "Updating table {} with val {:#?} for row {:#?}",
            "person",
            val,
            self
        );
        self.update_email_qv(val)?.update(session).await
    }
}
impl PrimaryKeyRef<'_> {
    pub fn update_dyn_qv(
        &self,
        val: UpdatableColumnRef<'_>,
    ) -> Result<Update, SerializeValuesError> {
        match val {
            UpdatableColumnRef::Email(val) => self.update_email_qv(val),
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
        let mut serialized_values = SerializedValues::with_capacity(val.len() + 2usize);
        for v in val {
            match v {
                UpdatableColumnRef::Email(v) => {
                    query.push(concat!(stringify!(email), " = ?"));
                    serialized_values.add_value(v)?;
                }
            }
        }
        let columns_to_update: String = query.join(", ");
        let update_statement = format!(
            "update {} set {} {}",
            "person", columns_to_update, "where name = ? and age = ?"
        );
        serialized_values.add_value(&self.name)?;
        serialized_values.add_value(&self.age)?;
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
            "person",
            val,
            self
        );
        self.update_dyn_multiple_qv(val)?.update(session).await
    }
}
impl PrimaryKeyRef<'_> {
    pub fn delete_qv(&self) -> Result<DeleteUnique, SerializeValuesError> {
        let mut serialized_values = SerializedValues::with_capacity(2usize);
        serialized_values.add_value(&self.name)?;
        serialized_values.add_value(&self.age)?;
        Ok(DeleteUnique::new(Qv {
            query: DELETE_QUERY,
            values: serialized_values,
        }))
    }
    pub async fn delete(&self, session: &Session) -> ScyllaQueryResult {
        tracing::debug!(
            "Deleting a row from table {} with values {:#?}",
            "person",
            self
        );
        self.delete_qv()?.delete_unique(session).await
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum UpdatableColumn {
    Email(String),
}
impl UpdatableColumn {
    pub fn to_ref(&self) -> UpdatableColumnRef<'_> {
        match &self {
            UpdatableColumn::Email(v) => UpdatableColumnRef::Email(v),
        }
    }
}
#[derive(Copy, Debug, Clone, PartialEq)]
pub enum UpdatableColumnRef<'a> {
    Email(&'a str),
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
            UpdatableColumnRef::Email(v) => UpdatableColumn::Email(v.to_string()),
        }
    }
}
impl UpdatableColumnRef<'_> {
    pub fn into_owned(self) -> UpdatableColumn {
        self.into()
    }
}
impl Person {
    pub fn updatable_column_email(&self) -> UpdatableColumnRef {
        UpdatableColumnRef::Email(&self.email)
    }
}
