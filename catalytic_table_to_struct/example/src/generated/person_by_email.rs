// Generated file
use super::person::Person;
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
pub const SELECT_ALL_QUERY: &str = "select email, name, age, type from person_by_email";
#[doc = r" The query to count all rows in the table"]
pub const SELECT_ALL_COUNT_QUERY: &str = "select count(*) from person_by_email";
#[doc = r" The query to select all rows in the table, based on the base table"]
#[doc = r" The order of the columns in the query are the same as the order of the columns in the base table"]
#[doc = r" This means that a query can be done in this materialized view table, but a free conversation"]
#[doc = r" can be done to a struct of the base table"]
pub const SELECT_ALL_QUERY_BASE_TABLE: &str = "select name, age, email, type from person_by_email";
#[doc = r" The query to retrieve a unique row in this table"]
pub const SELECT_UNIQUE_QUERY: &str =
    "select email, name, age, type from person_by_email where email = ? and name = ? and age = ?";
pub const SELECT_UNIQUE_QUERY_BASE_TABLE: &str =
    "select name, age, email, type from person_by_email where email = ? and name = ? and age = ?";
#[doc = r" This is the struct which is generated from the table"]
#[doc = r" If you want to perform CRUD operations, do the following:"]
#[doc = r"     Create -> convert this struct to a borrowed struct"]
#[doc = r"     Read, Update, Delete -> convert this struct to a borrowed primary key struct"]
#[doc = r" When you converted this struct to the specified type, you will have methods available"]
#[doc = r" for the things you want"]
#[derive(
    scylla :: FromRow, scylla :: ValueList, catalytic_macro :: Mirror, Debug, Clone, PartialEq,
)]
pub struct PersonByEmail {
    #[partition_key]
    pub email: String,
    #[clustering_key]
    pub name: String,
    #[clustering_key]
    pub age: i32,
    pub row_type: String,
}
impl PersonByEmail {
    #[doc = r" Create an borrowed primary key from the struct values"]
    #[doc = r" You can use this primary key struct to perform updates, deletions and selects on"]
    #[doc = r" a unique row"]
    pub fn primary_key(&self) -> PrimaryKeyRef {
        PrimaryKeyRef {
            email: &self.email,
            name: &self.name,
            age: &self.age,
        }
    }
    #[doc = r" Create an owned primary key from the struct values, it will actually clone the values if needed"]
    pub fn primary_key_owned(&self) -> PrimaryKey {
        self.primary_key().into_owned()
    }
    #[doc = r" Create an owned primary key from the struct values without cloning"]
    pub fn into_primary_key_owned(self) -> PrimaryKey {
        PrimaryKey {
            email: self.email,
            name: self.name,
            age: self.age,
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
pub fn select_all_qv() -> SelectMultiple<PersonByEmail, &'static str, &'static [u8; 0]> {
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
) -> Result<TypedRowIterator<PersonByEmail>, QueryError> {
    select_all_qv().select(session, page_size).await
}
#[doc = r" Returns a struct that can perform a selection of all rows in the database"]
#[doc = r" It will accumulate all rows in memory by sending paged queries"]
pub async fn select_all_in_memory(
    session: &CachingSession,
    page_size: i32,
) -> Result<QueryEntityVec<PersonByEmail>, MultipleSelectQueryErrorTransform> {
    select_all_qv()
        .select_all_in_memory(session, page_size)
        .await
}
#[doc = r" A struct that contains borrowed values"]
#[doc = r" This can be used to perform an insertion that is unique identified by the values of this struct"]
#[doc = r" If you want to perform an update, deletion or select or a unique row, convert this"]
#[doc = r" struct to the primary key struct"]
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct PersonByEmailRef<'a> {
    pub email: &'a str,
    pub name: &'a str,
    pub age: &'a i32,
    pub row_type: &'a str,
}
impl From<PersonByEmailRef<'_>> for PersonByEmail {
    #[doc = r" Conversation method to go from a borrowed struct to an owned struct"]
    fn from(f: PersonByEmailRef<'_>) -> PersonByEmail {
        PersonByEmail {
            email: f.email.to_string(),
            name: f.name.to_string(),
            age: f.age.clone(),
            row_type: f.row_type.to_string(),
        }
    }
}
impl PersonByEmail {
    #[doc = r" Conversation method to go from an owned struct to a borrowed struct"]
    pub fn to_ref(&self) -> PersonByEmailRef {
        PersonByEmailRef {
            email: &self.email,
            name: &self.name,
            age: &self.age,
            row_type: &self.row_type,
        }
    }
}
impl<'a> PersonByEmailRef<'a> {
    #[doc = r" Conversation method to go from a borrowed struct to an owned struct"]
    pub fn primary_key(&self) -> PrimaryKeyRef {
        PrimaryKeyRef {
            email: self.email,
            name: self.name,
            age: self.age,
        }
    }
}
#[doc = r" Returns a struct that can perform a selection of all rows in the database"]
pub fn select_all_base_table_qv() -> SelectMultiple<Person, &'static str, &'static [u8; 0]> {
    SelectMultiple::new(Qv {
        query: SELECT_ALL_QUERY_BASE_TABLE,
        values: &[],
    })
}
#[doc = r" Returns a struct that can perform a selection of all rows in the database"]
#[doc = r" with a specified page size"]
pub async fn select_all_base_table(
    session: &CachingSession,
    page_size: Option<i32>,
) -> Result<TypedRowIterator<Person>, QueryError> {
    select_all_base_table_qv().select(session, page_size).await
}
#[doc = r" Returns a struct that can perform a selection of all rows in the database"]
#[doc = r" It will accumulate all rows in memory by sending paged queries"]
pub async fn select_all_base_table_in_memory(
    session: &CachingSession,
    page_size: i32,
) -> Result<QueryEntityVec<Person>, MultipleSelectQueryErrorTransform> {
    select_all_base_table_qv()
        .select_all_in_memory(session, page_size)
        .await
}
#[doc = r" The owned primary key struct"]
#[doc = r" If you want to perform a read, delete or update, convert it to the borrowed type"]
#[derive(catalytic_macro :: PrimaryKey, Debug, Clone, PartialEq)]
pub struct PrimaryKey {
    #[partition_key]
    pub email: String,
    #[clustering_key]
    pub name: String,
    #[clustering_key]
    pub age: i32,
}
#[doc = r" The borrowed primary key struct"]
#[doc = r" This struct can be used to perform reads, deletes and updates"]
#[derive(catalytic_macro :: PrimaryKey, Copy, Debug, Clone, PartialEq)]
pub struct PrimaryKeyRef<'a> {
    pub email: &'a str,
    pub name: &'a str,
    pub age: &'a i32,
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
            email: &self.email,
            name: &self.name,
            age: &self.age,
        }
    }
}
#[doc = r" Conversation method to go from a borrowed primary key to an owned primary key"]
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
    #[doc = r" Returns a struct that can perform a unique row selection"]
    pub fn select_unique_qv(&self) -> Result<SelectUnique<PersonByEmail>, SerializeValuesError> {
        let mut serialized_values = SerializedValues::new();
        serialized_values.add_value(&self.email)?;
        serialized_values.add_value(&self.name)?;
        serialized_values.add_value(&self.age)?;
        Ok(SelectUnique::new(Qv {
            query: SELECT_UNIQUE_QUERY,
            values: serialized_values,
        }))
    }
    #[doc = r" Performs the unique row selection"]
    pub async fn select_unique(
        &self,
        session: &CachingSession,
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
    #[doc = r" Returns a struct that can perform a unique row selection"]
    pub fn select_unique_expect_qv(
        &self,
    ) -> Result<SelectUniqueExpect<PersonByEmail>, SerializeValuesError> {
        let mut serialized_values = SerializedValues::new();
        serialized_values.add_value(&self.email)?;
        serialized_values.add_value(&self.name)?;
        serialized_values.add_value(&self.age)?;
        Ok(SelectUniqueExpect::new(Qv {
            query: SELECT_UNIQUE_QUERY,
            values: serialized_values,
        }))
    }
    #[doc = r" Performs the unique row selection"]
    pub async fn select_unique_expect(
        &self,
        session: &CachingSession,
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
    #[doc = r" Returns a struct that can perform a unique row selection"]
    pub fn select_unique_base_table_qv(
        &self,
    ) -> Result<SelectUnique<Person>, SerializeValuesError> {
        let mut serialized_values = SerializedValues::new();
        serialized_values.add_value(&self.email)?;
        serialized_values.add_value(&self.name)?;
        serialized_values.add_value(&self.age)?;
        Ok(SelectUnique::new(Qv {
            query: SELECT_UNIQUE_QUERY_BASE_TABLE,
            values: serialized_values,
        }))
    }
    #[doc = r" Performs the unique row selection"]
    pub async fn select_unique_base_table(
        &self,
        session: &CachingSession,
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
    #[doc = r" Returns a struct that can perform a unique row selection"]
    pub fn select_unique_expect_base_table_qv(
        &self,
    ) -> Result<SelectUniqueExpect<Person>, SerializeValuesError> {
        let mut serialized_values = SerializedValues::new();
        serialized_values.add_value(&self.email)?;
        serialized_values.add_value(&self.name)?;
        serialized_values.add_value(&self.age)?;
        Ok(SelectUniqueExpect::new(Qv {
            query: SELECT_UNIQUE_QUERY_BASE_TABLE,
            values: serialized_values,
        }))
    }
    #[doc = r" Performs the unique row selection"]
    pub async fn select_unique_expect_base_table(
        &self,
        session: &CachingSession,
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
