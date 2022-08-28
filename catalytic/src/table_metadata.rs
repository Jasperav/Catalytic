/// The type of the column
#[derive(PartialEq, Debug, Eq)]
pub enum ColumnKind {
    /// Column is part of the partition key
    PartitionKey,
    /// Column is part of the clustering key
    Clustering,
    /// Column is not part of the primary key
    Regular,
}

impl ColumnKind {
    pub fn is_part_of_pk(&self) -> bool {
        match self {
            ColumnKind::PartitionKey | ColumnKind::Clustering => true,
            ColumnKind::Regular => false,
        }
    }
}

impl ToString for ColumnKind {
    fn to_string(&self) -> String {
        let column_kind_to_string = match self {
            ColumnKind::PartitionKey => "partition_key",
            ColumnKind::Regular => "regular",
            ColumnKind::Clustering => "clustering",
        };

        column_kind_to_string.to_string()
    }
}

/// Meta data about a column for a table
#[derive(Debug, scylla::ValueList, scylla::FromRow)]
pub struct ColumnInTable {
    /// Name of the column
    pub column_name: String,
    /// Either partition_key, regular or clustering
    pub kind: String,
    /// The position of the column, only relevant for primary key fields.
    /// Regular columns have the value -1, primary key fields >= 0.
    pub position: i32,
    /// The data type of the column
    pub data_type: String,
}

impl ColumnInTable {
    pub fn kind(&self) -> ColumnKind {
        match self.kind.as_str() {
            "partition_key" => ColumnKind::PartitionKey,
            "regular" => ColumnKind::Regular,
            "clustering" => ColumnKind::Clustering,
            _ => panic!("Invalid column type: {}", self.kind.as_str()),
        }
    }
}

/// Well this holds the table name, but a struct is needed for the FromRow trait
#[derive(scylla::FromRow, Debug)]
pub struct TableName {
    pub table_name: String,
}

/// Supported type, not every type is supported due to https://github.com/scylladb/scylla-rust-driver/issues/104
#[derive(Debug, PartialEq, Clone, Eq)]
pub enum ColumnType {
    TinyInt,
    SmallInt,
    Int,
    BigInt,
    Text,
    Ascii,
    Varchar,
    Boolean,
    Time,
    Timestamp,
    Float,
    Double,
    Uuid,
    Counter,
    Custom(String),
}

impl ColumnType {
    pub fn new<T: ToString>(from: T) -> Self {
        let s = from.to_string();

        match s.as_str() {
            "tinyint" => ColumnType::TinyInt,
            "smallint" => ColumnType::SmallInt,
            "int" => ColumnType::Int,
            "bigint" => ColumnType::BigInt,
            "text" => ColumnType::Text,
            "ascii" => ColumnType::Ascii,
            "varchar" => ColumnType::Varchar,
            "boolean" => ColumnType::Boolean,
            "time" => ColumnType::Time,
            "timestamp" => ColumnType::Timestamp,
            "float" => ColumnType::Float,
            "double" => ColumnType::Double,
            "uuid" => ColumnType::Uuid,
            "counter" => ColumnType::Counter,
            _ => ColumnType::Custom(s),
        }
    }

    pub fn to_ty(&self) -> String {
        let result = match self {
            ColumnType::TinyInt => "i8",
            ColumnType::SmallInt => "i16",
            ColumnType::Int => "i32",
            ColumnType::BigInt => "i64",
            ColumnType::Time => "scylla::frame::value::Time",
            ColumnType::Counter => "scylla::frame::value::Counter",
            ColumnType::Timestamp => "scylla::frame::value::Timestamp",
            ColumnType::Text | ColumnType::Ascii | ColumnType::Varchar => "String",
            ColumnType::Boolean => "bool",
            ColumnType::Float => "f32",
            ColumnType::Double => "f64",
            ColumnType::Uuid => "uuid::Uuid",
            ColumnType::Custom(c) => c.as_str(),
        };

        result.to_string()
    }
}
