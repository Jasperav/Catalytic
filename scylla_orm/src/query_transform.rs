use crate::Cursor;
use futures_util::{StreamExt, TryStreamExt};
use scylla::cql_to_rust::FromRowError;
use scylla::frame::value::{SerializeValuesError, ValueList};
use scylla::query::Query;
use scylla::transport::errors::QueryError;
use scylla::transport::iterator::TypedRowIterator;
use scylla::{FromRow, QueryResult, Session};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub type ScyllaQueryResult = Result<QueryResult, QueryError>;
pub type CountType = i64;
pub type TtlType = i32;

#[derive(scylla::FromRow, Debug, Clone, Copy, PartialEq)]
pub struct Count {
    pub count: CountType,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum UniqueQueryRowTransformError {
    #[error("No rows in query result")]
    NoRows,
    #[error("More than one row in query result")]
    MoreThanOneRow,
    #[error("From row error`{0}`")]
    FromRowError(FromRowError),
}

#[derive(Debug, Clone)]
pub enum SingleSelectQueryErrorTransform {
    UniqueQueryRowTransformError(UniqueQueryRowTransformError),
    QueryError(QueryError),
}

#[derive(Debug, Clone)]
pub enum MultipleSelectQueryErrorTransform {
    FromRowError(FromRowError),
    QueryError(QueryError),
}

pub struct QueryEntityVecResult<T> {
    pub entities: Vec<T>,
    pub query_result: QueryResult,
}

/// Wrapper, maybe additional fields are added later
pub struct QueryEntityVec<T> {
    pub entities: Vec<T>,
}

impl<T> Deref for QueryEntityVecResult<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Vec<T> {
        &self.entities
    }
}

impl From<FromRowError> for MultipleSelectQueryErrorTransform {
    fn from(u: FromRowError) -> Self {
        MultipleSelectQueryErrorTransform::FromRowError(u)
    }
}

impl From<QueryError> for MultipleSelectQueryErrorTransform {
    fn from(u: QueryError) -> Self {
        MultipleSelectQueryErrorTransform::QueryError(u)
    }
}

impl From<UniqueQueryRowTransformError> for SingleSelectQueryErrorTransform {
    fn from(u: UniqueQueryRowTransformError) -> Self {
        SingleSelectQueryErrorTransform::UniqueQueryRowTransformError(u)
    }
}

impl From<QueryError> for SingleSelectQueryErrorTransform {
    fn from(u: QueryError) -> Self {
        SingleSelectQueryErrorTransform::QueryError(u)
    }
}

impl From<SerializeValuesError> for SingleSelectQueryErrorTransform {
    fn from(u: SerializeValuesError) -> Self {
        SingleSelectQueryErrorTransform::QueryError(u.into())
    }
}

pub struct QueryResultUniqueRow<T> {
    pub query_result: QueryResult,
    pub entity: Option<T>,
}

impl<T> Deref for QueryResultUniqueRow<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}

impl<T> DerefMut for QueryResultUniqueRow<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}

impl<T: FromRow> QueryResultUniqueRow<T> {
    fn from_query_result(
        mut query_result: QueryResult,
    ) -> Result<QueryResultUniqueRow<T>, UniqueQueryRowTransformError> {
        let mut rows = None;

        std::mem::swap(&mut query_result.rows, &mut rows);

        let mut r = rows.unwrap_or_default();

        if r.len() <= 1 {
            let entity = if r.len() == 1 {
                let entity = r.remove(0);
                match entity.into_typed() {
                    Ok(e) => Some(e),
                    Err(parse_error) => {
                        return Err(UniqueQueryRowTransformError::FromRowError(parse_error));
                    }
                }
            } else {
                None
            };
            Ok(QueryResultUniqueRow {
                query_result,
                entity,
            })
        } else {
            Err(UniqueQueryRowTransformError::MoreThanOneRow)
        }
    }
}

pub struct QueryResultUniqueRowExpect<T> {
    pub query_result: QueryResult,
    pub entity: T,
}

impl<T> Deref for QueryResultUniqueRowExpect<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}

impl<T> DerefMut for QueryResultUniqueRowExpect<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}

impl<T: FromRow> QueryResultUniqueRowExpect<T> {
    fn from_query_result(
        query_result: QueryResult,
    ) -> Result<QueryResultUniqueRowExpect<T>, UniqueQueryRowTransformError> {
        QueryResultUniqueRowExpect::from_unique_row(QueryResultUniqueRow::from_query_result(
            query_result,
        )?)
    }
    fn from_unique_row(
        q: QueryResultUniqueRow<T>,
    ) -> Result<QueryResultUniqueRowExpect<T>, UniqueQueryRowTransformError> {
        match q.entity {
            Some(e) => Ok(QueryResultUniqueRowExpect {
                query_result: q.query_result,
                entity: e,
            }),
            None => Err(UniqueQueryRowTransformError::NoRows),
        }
    }
}

pub struct Qv<R: AsRef<str>, V: ValueList> {
    pub query: R,
    pub values: V,
}

impl<R: AsRef<str> + Clone, V: ValueList + Clone> Clone for Qv<R, V> {
    fn clone(&self) -> Self {
        Qv {
            query: self.query.clone(),
            values: self.values.clone(),
        }
    }
}

impl<R: AsRef<str>, V: ValueList> Debug for Qv<R, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Qv")
            .field("query", &self.query.as_ref())
            .finish()
    }
}

impl<R: AsRef<str>, V: ValueList> Qv<R, V> {
    async fn execute_cached(&self, session: &Session) -> ScyllaQueryResult {
        let as_ref = self.query.as_ref();

        tracing::debug!("Executing: {}", as_ref);

        session.execute_cached(as_ref, &self.values).await
    }

    async fn execute_all_in_memory_cached<T: FromRow, N>(
        &self,
        session: &Session,
        page_size: i32,
        transform: impl Fn(T) -> N + Copy,
    ) -> Result<QueryEntityVec<N>, MultipleSelectQueryErrorTransform> {
        let as_ref = self.query.as_ref();

        tracing::debug!("Executing with page size: {}: {}", page_size, as_ref);

        let mut query: Query = as_ref.into();

        query.set_page_size(page_size);

        let rows = session
            .execute_iter_cached(query, &self.values)
            .await?
            .map(|c| {
                let row = c.map(T::from_row);
                let transformed = row.map(|r| r.map(|c| transform(c)));

                match transformed {
                    Ok(ok) => match ok {
                        Ok(row) => Ok(row),
                        Err(err) => Err(MultipleSelectQueryErrorTransform::FromRowError(err)),
                    },
                    Err(err) => Err(MultipleSelectQueryErrorTransform::QueryError(err)),
                }
            })
            .try_collect::<Vec<_>>()
            .await?;

        Ok(QueryEntityVec { entities: rows })
    }

    async fn execute_iter_cached<T: FromRow>(
        &self,
        session: &Session,
        page_size: Option<i32>,
    ) -> Result<TypedRowIterator<T>, QueryError> {
        let as_ref = self.query.as_ref();

        tracing::debug!("Executing with page size: {:#?}: {}", page_size, as_ref);

        let mut query: Query = as_ref.into();

        if let Some(p) = page_size {
            query.set_page_size(p);
        }

        let result = session.execute_iter_cached(query, &self.values).await?;

        Ok(result.into_typed())
    }

    async fn execute_iter_paged_cached<T: FromRow, N>(
        &self,
        session: &Session,
        page_size: Option<i32>,
        paging_state: Cursor,
        transform: impl Fn(T) -> N + Copy,
    ) -> Result<QueryEntityVecResult<N>, MultipleSelectQueryErrorTransform> {
        let as_ref = self.query.as_ref();

        tracing::debug!(
            "Executing with page size: {:#?}, paging state: {}: {}",
            page_size,
            paging_state.is_some(),
            as_ref,
        );

        let mut query: Query = as_ref.into();

        if let Some(p) = page_size {
            query.set_page_size(p);
        }

        let mut result = session
            .execute_paged_cached(query, &self.values, paging_state)
            .await?;
        let rows = self.transform(&mut result, transform)?;

        Ok(QueryEntityVecResult {
            entities: rows,
            query_result: result,
        })
    }

    fn transform<T: FromRow, N>(
        &self,
        query_result: &mut QueryResult,
        transform: impl Fn(T) -> N + Copy,
    ) -> Result<Vec<N>, FromRowError> {
        let mut rows = None;

        std::mem::swap(&mut query_result.rows, &mut rows);

        let rows = rows.unwrap_or_default();

        // This should never fail when using exclusively the ORM (and no columns are dropped while running a server)
        rows.into_iter()
            .map(T::from_row)
            .map(|t| t.map(transform))
            .collect()
    }
}
macro_rules! simple_qv_holder {
    ($ ident : ident , $ method : ident) => {
        #[derive(Debug)]
        pub struct $ident<R: AsRef<str>, V: ValueList> {
            pub qv: Qv<R, V>,
        }
        impl<R: AsRef<str>, V: ValueList> $ident<R, V> {
            pub fn new(qv: Qv<R, V>) -> Self {
                Self { qv }
            }

            pub async fn $method(&self, session: &Session) -> ScyllaQueryResult {
                self.qv.execute_cached(session).await
            }
        }

        impl<R: AsRef<str>, V: ValueList> Deref for $ident<R, V> {
            type Target = Qv<R, V>;

            fn deref(&self) -> &Self::Target {
                &self.qv
            }
        }

        impl<R: AsRef<str> + Clone, V: ValueList + Clone> Clone for $ident<R, V> {
            fn clone(&self) -> Self {
                $ident::new(self.qv.clone())
            }
        }
    };
}
simple_qv_holder!(DeleteMultiple, delete_multiple);
simple_qv_holder!(DeleteUnique, delete_unique);
simple_qv_holder!(Insert, insert);
simple_qv_holder!(Update, update);
simple_qv_holder!(Truncate, truncate);

macro_rules! read_transform {
    ($ ident : ident) => {
        #[derive(Debug)]
        pub struct $ident<R: AsRef<str>, T: FromRow, V: ValueList> {
            pub qv: Qv<R, V>,
            p: PhantomData<T>,
        }
        impl<R: AsRef<str>, T: FromRow, V: ValueList> $ident<R, T, V> {
            pub fn new(qv: Qv<R, V>) -> $ident<R, T, V> {
                $ident { qv, p: PhantomData }
            }
        }

        impl<R: AsRef<str>, T: FromRow, V: ValueList> Deref for $ident<R, T, V> {
            type Target = Qv<R, V>;

            fn deref(&self) -> &Self::Target {
                &self.qv
            }
        }

        impl<R: AsRef<str> + Clone, T: FromRow, V: ValueList + Clone> Clone for $ident<R, T, V> {
            fn clone(&self) -> Self {
                $ident::new(self.qv.clone())
            }
        }
    };
}
read_transform!(SelectMultiple);
read_transform!(SelectUnique);
read_transform!(SelectUniqueExpect);

impl<R: AsRef<str>, T: FromRow, V: ValueList> SelectUnique<R, T, V> {
    pub fn expect(self) -> SelectUniqueExpect<R, T, V> {
        SelectUniqueExpect::new(self.qv)
    }

    pub async fn select(
        &self,
        session: &Session,
    ) -> Result<QueryResultUniqueRow<T>, SingleSelectQueryErrorTransform> {
        let result = self.qv.execute_cached(session).await?;
        let result = QueryResultUniqueRow::from_query_result(result)?;

        Ok(result)
    }
}

impl<R: AsRef<str>, T: FromRow, V: ValueList> SelectUniqueExpect<R, T, V> {
    pub async fn select(
        &self,
        session: &Session,
    ) -> Result<QueryResultUniqueRowExpect<T>, SingleSelectQueryErrorTransform> {
        let result = self.qv.execute_cached(session).await?;
        let result = QueryResultUniqueRowExpect::from_query_result(result)?;

        Ok(result)
    }
}

impl<R: AsRef<str>, V: ValueList> SelectUniqueExpect<R, Count, V> {
    pub async fn select_count(
        &self,
        session: &Session,
    ) -> Result<QueryResultUniqueRowExpect<CountType>, SingleSelectQueryErrorTransform> {
        let count: QueryResultUniqueRowExpect<Count> = self.select(session).await?;
        Ok(QueryResultUniqueRowExpect {
            entity: count.entity.count,
            query_result: count.query_result,
        })
    }
}

impl<R: AsRef<str>, T: FromRow, V: ValueList> SelectMultiple<R, T, V> {
    pub async fn select(
        &self,
        session: &Session,
        page_size: Option<i32>,
    ) -> Result<TypedRowIterator<T>, QueryError> {
        self.qv.execute_iter_cached(session, page_size).await
    }

    pub async fn select_paged(
        &self,
        session: &Session,
        page_size: Option<i32>,
        paging_state: Cursor,
    ) -> Result<QueryEntityVecResult<T>, MultipleSelectQueryErrorTransform> {
        self.select_paged_transform(session, page_size, paging_state, |v| v)
            .await
    }

    pub async fn select_paged_transform<N>(
        &self,
        session: &Session,
        page_size: Option<i32>,
        paging_state: Cursor,
        transform: impl Fn(T) -> N + Copy,
    ) -> Result<QueryEntityVecResult<N>, MultipleSelectQueryErrorTransform> {
        self.qv
            .execute_iter_paged_cached(session, page_size, paging_state, transform)
            .await
    }

    pub async fn select_all_in_memory(
        &self,
        session: &Session,
        page_size: i32,
    ) -> Result<QueryEntityVec<T>, MultipleSelectQueryErrorTransform> {
        self.select_all_in_memory_transform(session, page_size, |v| v)
            .await
    }

    pub async fn select_all_in_memory_transform<N>(
        &self,
        session: &Session,
        page_size: i32,
        transform: impl Fn(T) -> N + Copy,
    ) -> Result<QueryEntityVec<N>, MultipleSelectQueryErrorTransform> {
        self.qv
            .execute_all_in_memory_cached(session, page_size, transform)
            .await
    }
}
