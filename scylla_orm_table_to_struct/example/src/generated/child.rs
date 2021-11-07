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
pub const SELECT_ALL_QUERY: &str = "select birthday, enum_json, json, json_nullable from child";
pub const SELECT_ALL_COUNT_QUERY: &str = "select count(*) from child";
pub const INSERT_QUERY: &str =
    "insert into child(birthday, enum_json, json, json_nullable) values (?, ?, ?, ?)";
pub const INSERT_TTL_QUERY: &str =
    "insert into child(birthday, enum_json, json, json_nullable) values (?, ?, ?, ?) using ttl ?";
pub const TRUNCATE_QUERY: &str = "truncate child";
pub const SELECT_UNIQUE_QUERY: &str =
    "select birthday, enum_json, json, json_nullable from child where birthday = ?";
pub const UPDATE_ENUM_JSON_QUERY: &str = "update child set enum_json = ? where birthday = ?";
pub const UPDATE_JSON_QUERY: &str = "update child set json = ? where birthday = ?";
pub const UPDATE_JSON_NULLABLE_QUERY: &str =
    "update child set json_nullable = ? where birthday = ?";
pub const DELETE_QUERY: &str = "delete from child where birthday = ?";
#[derive(
    scylla :: FromRow, scylla :: ValueList, scylla_orm_macro :: Mirror, Debug, Clone, PartialEq,
)]
pub struct Child {
    #[partition_key]
    pub birthday: i32,
    #[json]
    pub enum_json: crate::MyJsonEnum,
    #[json]
    pub json: crate::MyJsonType,
    #[json]
    pub json_nullable: std::option::Option<crate::MyJsonType>,
}
impl Child {
    pub fn primary_key(&self) -> PrimaryKeyRef {
        PrimaryKeyRef {
            birthday: &self.birthday,
        }
    }
    pub fn primary_key_owned(self) -> PrimaryKey {
        PrimaryKey {
            birthday: self.birthday,
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
pub fn select_all_qv() -> SelectMultiple<Child, &'static str, &'static [u8; 0]> {
    SelectMultiple::new(Qv {
        query: SELECT_ALL_QUERY,
        values: &[],
    })
}
pub async fn select_all(
    session: &Session,
    page_size: Option<i32>,
) -> Result<TypedRowIterator<Child>, QueryError> {
    select_all_qv().select(session, page_size).await
}
pub async fn select_all_in_memory(
    session: &Session,
    page_size: i32,
) -> Result<QueryEntityVec<Child>, MultipleSelectQueryErrorTransform> {
    select_all_qv()
        .select_all_in_memory(session, page_size)
        .await
}
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct ChildRef<'a> {
    pub birthday: &'a i32,
    pub enum_json: &'a crate::MyJsonEnum,
    pub json: &'a crate::MyJsonType,
    pub json_nullable: &'a std::option::Option<crate::MyJsonType>,
}
impl From<ChildRef<'_>> for Child {
    fn from(f: ChildRef<'_>) -> Child {
        Child {
            birthday: f.birthday.clone(),
            enum_json: f.enum_json.clone(),
            json: f.json.clone(),
            json_nullable: f.json_nullable.clone(),
        }
    }
}
impl Child {
    pub fn to_ref(&self) -> ChildRef {
        ChildRef {
            birthday: &self.birthday,
            enum_json: &self.enum_json,
            json: &self.json,
            json_nullable: &self.json_nullable,
        }
    }
}
impl<'a> ChildRef<'a> {
    pub fn primary_key(&self) -> PrimaryKeyRef {
        PrimaryKeyRef {
            birthday: self.birthday,
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
impl<'a> ChildRef<'a> {
    pub fn insert_qv(&self) -> Result<Insert, SerializeValuesError> {
        let mut serialized = SerializedValues::with_capacity(4usize);
        serialized.add_value(&self.birthday)?;
        serialized.add_value(&self.enum_json)?;
        serialized.add_value(&self.json)?;
        serialized.add_value(&self.json_nullable)?;
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
        let mut serialized = SerializedValues::with_capacity(5usize);
        serialized.add_value(&self.birthday)?;
        serialized.add_value(&self.enum_json)?;
        serialized.add_value(&self.json)?;
        serialized.add_value(&self.json_nullable)?;
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
impl Child {
    pub fn in_memory_update(&mut self, update: UpdatableColumn) {
        match update {
            UpdatableColumn::EnumJson(val) => {
                self.enum_json = val;
            }
            UpdatableColumn::Json(val) => {
                self.json = val;
            }
            UpdatableColumn::JsonNullable(val) => {
                self.json_nullable = val;
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
    pub birthday: i32,
}
#[derive(scylla_orm_macro :: PrimaryKey, Copy, Debug, Clone, PartialEq)]
pub struct PrimaryKeyRef<'a> {
    pub birthday: &'a i32,
}
impl PrimaryKeyRef<'_> {
    pub fn into_owned(self) -> PrimaryKey {
        self.into()
    }
}
impl PrimaryKey {
    pub fn to_ref(&self) -> PrimaryKeyRef<'_> {
        PrimaryKeyRef {
            birthday: &self.birthday,
        }
    }
}
impl From<PrimaryKeyRef<'_>> for PrimaryKey {
    fn from(f: PrimaryKeyRef<'_>) -> PrimaryKey {
        PrimaryKey {
            birthday: f.birthday.clone(),
        }
    }
}
impl PrimaryKeyRef<'_> {
    pub fn select_unique_qv(&self) -> Result<SelectUnique<Child>, SerializeValuesError> {
        let mut serialized_values = SerializedValues::with_capacity(1usize);
        serialized_values.add_value(&self.birthday)?;
        Ok(SelectUnique::new(Qv {
            query: SELECT_UNIQUE_QUERY,
            values: serialized_values,
        }))
    }
    pub async fn select_unique(
        &self,
        session: &Session,
    ) -> Result<QueryResultUniqueRow<Child>, SingleSelectQueryErrorTransform> {
        tracing::debug!(
            "Selecting unique row for table {} with values: {:#?}",
            "child",
            self
        );
        self.select_unique_qv()?.select(session).await
    }
}
impl PrimaryKeyRef<'_> {
    pub fn select_unique_expect_qv(
        &self,
    ) -> Result<SelectUniqueExpect<Child>, SerializeValuesError> {
        let mut serialized_values = SerializedValues::with_capacity(1usize);
        serialized_values.add_value(&self.birthday)?;
        Ok(SelectUniqueExpect::new(Qv {
            query: SELECT_UNIQUE_QUERY,
            values: serialized_values,
        }))
    }
    pub async fn select_unique_expect(
        &self,
        session: &Session,
    ) -> Result<QueryResultUniqueRowExpect<Child>, SingleSelectQueryErrorTransform> {
        tracing::debug!(
            "Selecting unique row for table {} with values: {:#?}",
            "child",
            self
        );
        self.select_unique_expect_qv()?.select(session).await
    }
}
impl PrimaryKeyRef<'_> {
    pub fn update_enum_json_qv(
        &self,
        val: &crate::MyJsonEnum,
    ) -> Result<Update, SerializeValuesError> {
        let mut serialized_values = SerializedValues::with_capacity(2usize);
        serialized_values.add_value(&val)?;
        serialized_values.add_value(&self.birthday)?;
        Ok(Update::new(Qv {
            query: UPDATE_ENUM_JSON_QUERY,
            values: serialized_values,
        }))
    }
    pub async fn update_enum_json(
        &self,
        session: &Session,
        val: &crate::MyJsonEnum,
    ) -> ScyllaQueryResult {
        tracing::debug!(
            "Updating table {} with val {:#?} for row {:#?}",
            "child",
            val,
            self
        );
        self.update_enum_json_qv(val)?.update(session).await
    }
}
impl PrimaryKeyRef<'_> {
    pub fn update_json_qv(&self, val: &crate::MyJsonType) -> Result<Update, SerializeValuesError> {
        let mut serialized_values = SerializedValues::with_capacity(2usize);
        serialized_values.add_value(&val)?;
        serialized_values.add_value(&self.birthday)?;
        Ok(Update::new(Qv {
            query: UPDATE_JSON_QUERY,
            values: serialized_values,
        }))
    }
    pub async fn update_json(
        &self,
        session: &Session,
        val: &crate::MyJsonType,
    ) -> ScyllaQueryResult {
        tracing::debug!(
            "Updating table {} with val {:#?} for row {:#?}",
            "child",
            val,
            self
        );
        self.update_json_qv(val)?.update(session).await
    }
}
impl PrimaryKeyRef<'_> {
    pub fn update_json_nullable_qv(
        &self,
        val: &std::option::Option<crate::MyJsonType>,
    ) -> Result<Update, SerializeValuesError> {
        let mut serialized_values = SerializedValues::with_capacity(2usize);
        serialized_values.add_value(&val)?;
        serialized_values.add_value(&self.birthday)?;
        Ok(Update::new(Qv {
            query: UPDATE_JSON_NULLABLE_QUERY,
            values: serialized_values,
        }))
    }
    pub async fn update_json_nullable(
        &self,
        session: &Session,
        val: &std::option::Option<crate::MyJsonType>,
    ) -> ScyllaQueryResult {
        tracing::debug!(
            "Updating table {} with val {:#?} for row {:#?}",
            "child",
            val,
            self
        );
        self.update_json_nullable_qv(val)?.update(session).await
    }
}
impl PrimaryKeyRef<'_> {
    pub fn update_dyn_qv(
        &self,
        val: UpdatableColumnRef<'_>,
    ) -> Result<Update, SerializeValuesError> {
        match val {
            UpdatableColumnRef::EnumJson(val) => self.update_enum_json_qv(val),
            UpdatableColumnRef::Json(val) => self.update_json_qv(val),
            UpdatableColumnRef::JsonNullable(val) => self.update_json_nullable_qv(val),
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
        let mut serialized_values = SerializedValues::with_capacity(val.len() + 1usize);
        for v in val {
            match v {
                UpdatableColumnRef::EnumJson(v) => {
                    query.push(concat!(stringify!(enum_json), " = ?"));
                    serialized_values.add_value(v)?;
                }
                UpdatableColumnRef::Json(v) => {
                    query.push(concat!(stringify!(json), " = ?"));
                    serialized_values.add_value(v)?;
                }
                UpdatableColumnRef::JsonNullable(v) => {
                    query.push(concat!(stringify!(json_nullable), " = ?"));
                    serialized_values.add_value(v)?;
                }
            }
        }
        let columns_to_update: String = query.join(", ");
        let update_statement = format!(
            "update {} set {} {}",
            "child", columns_to_update, "where birthday = ?"
        );
        serialized_values.add_value(&self.birthday)?;
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
            "child",
            val,
            self
        );
        self.update_dyn_multiple_qv(val)?.update(session).await
    }
}
impl PrimaryKeyRef<'_> {
    pub fn delete_qv(&self) -> Result<DeleteUnique, SerializeValuesError> {
        let mut serialized_values = SerializedValues::with_capacity(1usize);
        serialized_values.add_value(&self.birthday)?;
        Ok(DeleteUnique::new(Qv {
            query: DELETE_QUERY,
            values: serialized_values,
        }))
    }
    pub async fn delete(&self, session: &Session) -> ScyllaQueryResult {
        tracing::debug!(
            "Deleting a row from table {} with values {:#?}",
            "child",
            self
        );
        self.delete_qv()?.delete_unique(session).await
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum UpdatableColumn {
    EnumJson(crate::MyJsonEnum),
    Json(crate::MyJsonType),
    JsonNullable(std::option::Option<crate::MyJsonType>),
}
impl UpdatableColumn {
    pub fn to_ref(&self) -> UpdatableColumnRef<'_> {
        match &self {
            UpdatableColumn::EnumJson(v) => UpdatableColumnRef::EnumJson(v),
            UpdatableColumn::Json(v) => UpdatableColumnRef::Json(v),
            UpdatableColumn::JsonNullable(v) => UpdatableColumnRef::JsonNullable(v),
        }
    }
}
#[derive(Copy, Debug, Clone, PartialEq)]
pub enum UpdatableColumnRef<'a> {
    EnumJson(&'a crate::MyJsonEnum),
    Json(&'a crate::MyJsonType),
    JsonNullable(&'a std::option::Option<crate::MyJsonType>),
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
            UpdatableColumnRef::EnumJson(v) => UpdatableColumn::EnumJson(v.clone()),
            UpdatableColumnRef::Json(v) => UpdatableColumn::Json(v.clone()),
            UpdatableColumnRef::JsonNullable(v) => UpdatableColumn::JsonNullable(v.clone()),
        }
    }
}
impl UpdatableColumnRef<'_> {
    pub fn into_owned(self) -> UpdatableColumn {
        self.into()
    }
}
impl Child {
    pub fn updatable_column_enum_json(&self) -> UpdatableColumnRef {
        UpdatableColumnRef::EnumJson(&self.enum_json)
    }
    pub fn updatable_column_json(&self) -> UpdatableColumnRef {
        UpdatableColumnRef::Json(&self.json)
    }
    pub fn updatable_column_json_nullable(&self) -> UpdatableColumnRef {
        UpdatableColumnRef::JsonNullable(&self.json_nullable)
    }
}
