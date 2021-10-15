// Generated file
use super::person::Person;
#[allow(unused_imports)]
use scylla::frame::value::SerializeValuesError;
use scylla::frame::value::SerializedValues;
use scylla::transport::errors::QueryError;
use scylla::transport::iterator::TypedRowIterator;
use scylla::Session;
#[allow(unused_imports)]
use scylla_orm::query_transform::{
    CountType, DeleteUnique, Insert, MultipleSelectQueryErrorTransform, QueryEntityVecResult,
    QueryResultUniqueRow, QueryResultUniqueRowExpect, Qv, ScyllaQueryResult, SelectMultiple,
    SelectUnique, SelectUniqueExpect, SingleSelectQueryErrorTransform, Truncate, TtlType, Update,
};
pub const SELECT_ALL_QUERY: &str = "select email, name, age from person_by_email";
pub const SELECT_ALL_COUNT_QUERY: &str = "select count(*) from person_by_email";
pub const SELECT_ALL_QUERY_BASE_TABLE: &str = "select name, age, email from person_by_email";
pub const SELECT_UNIQUE_QUERY: &str =
    "select email, name, age from person_by_email where email = ? and name = ? and age = ?";
pub const SELECT_UNIQUE_QUERY_BASE_TABLE: &str =
    "select name, age, email from person_by_email where email = ? and name = ? and age = ?";
#[derive(
    scylla :: FromRow, scylla :: ValueList, scylla_orm_macro :: Mirror, Debug, Clone, PartialEq,
)]
pub struct PersonByEmail {
    #[partition_key]
    pub email: String,
    #[clustering_key]
    pub name: String,
    #[clustering_key]
    pub age: i32,
}
impl PersonByEmail {
    pub fn primary_key(&self) -> PrimaryKeyRef {
        PrimaryKeyRef {
            email: &self.email,
            name: &self.name,
            age: &self.age,
        }
    }
    pub fn primary_key_owned(self) -> PrimaryKey {
        PrimaryKey {
            email: self.email,
            name: self.name,
            age: self.age,
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
pub fn select_all_qv() -> SelectMultiple<&'static str, PersonByEmail, &'static [u8; 0]> {
    SelectMultiple::new(Qv {
        query: SELECT_ALL_QUERY,
        values: &[],
    })
}
pub async fn select_all(
    session: &Session,
    page_size: Option<i32>,
) -> Result<TypedRowIterator<PersonByEmail>, QueryError> {
    select_all_qv().select(session, page_size).await
}
pub async fn select_all_in_memory(
    session: &Session,
    page_size: i32,
) -> Result<QueryEntityVecResult<PersonByEmail>, MultipleSelectQueryErrorTransform> {
    select_all_qv()
        .select_all_in_memory(session, page_size)
        .await
}
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct PersonByEmailRef<'a> {
    pub email: &'a str,
    pub name: &'a str,
    pub age: &'a i32,
}
impl From<PersonByEmailRef<'_>> for PersonByEmail {
    fn from(f: PersonByEmailRef<'_>) -> PersonByEmail {
        PersonByEmail {
            email: f.email.to_string(),
            name: f.name.to_string(),
            age: f.age.clone(),
        }
    }
}
impl PersonByEmail {
    pub fn to_ref(&self) -> PersonByEmailRef {
        PersonByEmailRef {
            email: &self.email,
            name: &self.name,
            age: &self.age,
        }
    }
}
impl<'a> PersonByEmailRef<'a> {
    pub fn primary_key(&self) -> PrimaryKeyRef {
        PrimaryKeyRef {
            email: self.email,
            name: self.name,
            age: self.age,
        }
    }
}
pub fn select_all_base_table_qv() -> SelectMultiple<&'static str, Person, &'static [u8; 0]> {
    SelectMultiple::new(Qv {
        query: SELECT_ALL_QUERY_BASE_TABLE,
        values: &[],
    })
}
pub async fn select_all_base_table(
    session: &Session,
    page_size: Option<i32>,
) -> Result<TypedRowIterator<Person>, QueryError> {
    select_all_base_table_qv().select(session, page_size).await
}
pub async fn select_all_base_table_in_memory(
    session: &Session,
    page_size: i32,
) -> Result<QueryEntityVecResult<Person>, MultipleSelectQueryErrorTransform> {
    select_all_base_table_qv()
        .select_all_in_memory(session, page_size)
        .await
}
#[derive(scylla_orm_macro :: PrimaryKey, Debug, Clone, PartialEq)]
pub struct PrimaryKey {
    #[partition_key]
    pub email: String,
    #[clustering_key]
    pub name: String,
    #[clustering_key]
    pub age: i32,
}
#[derive(scylla_orm_macro :: PrimaryKey, Copy, Debug, Clone, PartialEq)]
pub struct PrimaryKeyRef<'a> {
    pub email: &'a str,
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
            email: &self.email,
            name: &self.name,
            age: &self.age,
        }
    }
}
impl From<PrimaryKeyRef<'_>> for PrimaryKey {
    fn from(f: PrimaryKeyRef<'_>) -> PrimaryKey {
        PrimaryKey {
            email: f.email.to_string(),
            name: f.name.to_string(),
            age: f.age.clone(),
        }
    }
}
impl PrimaryKeyRef<'_> {
    pub fn select_unique_qv(
        &self,
    ) -> Result<SelectUnique<&'static str, PersonByEmail, SerializedValues>, SerializeValuesError>
    {
        let mut serialized_values = SerializedValues::with_capacity(3usize);
        serialized_values.add_value(&self.email)?;
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
    ) -> Result<QueryResultUniqueRow<PersonByEmail>, SingleSelectQueryErrorTransform> {
        tracing::debug!(
            "Selecting unique row for table {} with values: {:#?}",
            "person_by_email",
            self
        );
        self.select_unique_qv()?.select(session).await
    }
}
impl PrimaryKeyRef<'_> {
    pub fn select_unique_expect_qv(
        &self,
    ) -> Result<
        SelectUniqueExpect<&'static str, PersonByEmail, SerializedValues>,
        SerializeValuesError,
    > {
        let mut serialized_values = SerializedValues::with_capacity(3usize);
        serialized_values.add_value(&self.email)?;
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
    ) -> Result<QueryResultUniqueRowExpect<PersonByEmail>, SingleSelectQueryErrorTransform> {
        tracing::debug!(
            "Selecting unique row for table {} with values: {:#?}",
            "person_by_email",
            self
        );
        self.select_unique_expect_qv()?.select(session).await
    }
}
impl PrimaryKeyRef<'_> {
    pub fn select_unique_base_table_qv(
        &self,
    ) -> Result<SelectUnique<&'static str, Person, SerializedValues>, SerializeValuesError> {
        let mut serialized_values = SerializedValues::with_capacity(3usize);
        serialized_values.add_value(&self.email)?;
        serialized_values.add_value(&self.name)?;
        serialized_values.add_value(&self.age)?;
        Ok(SelectUnique::new(Qv {
            query: SELECT_UNIQUE_QUERY_BASE_TABLE,
            values: serialized_values,
        }))
    }
    pub async fn select_unique_base_table(
        &self,
        session: &Session,
    ) -> Result<QueryResultUniqueRow<Person>, SingleSelectQueryErrorTransform> {
        tracing::debug!(
            "Selecting unique row for table {} with values: {:#?}",
            "person_by_email",
            self
        );
        self.select_unique_base_table_qv()?.select(session).await
    }
}
impl PrimaryKeyRef<'_> {
    pub fn select_unique_expect_base_table_qv(
        &self,
    ) -> Result<SelectUniqueExpect<&'static str, Person, SerializedValues>, SerializeValuesError>
    {
        let mut serialized_values = SerializedValues::with_capacity(3usize);
        serialized_values.add_value(&self.email)?;
        serialized_values.add_value(&self.name)?;
        serialized_values.add_value(&self.age)?;
        Ok(SelectUniqueExpect::new(Qv {
            query: SELECT_UNIQUE_QUERY_BASE_TABLE,
            values: serialized_values,
        }))
    }
    pub async fn select_unique_expect_base_table(
        &self,
        session: &Session,
    ) -> Result<QueryResultUniqueRowExpect<Person>, SingleSelectQueryErrorTransform> {
        tracing::debug!(
            "Selecting unique row for table {} with values: {:#?}",
            "person_by_email",
            self
        );
        self.select_unique_expect_base_table_qv()?
            .select(session)
            .await
    }
}
