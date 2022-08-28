use crate::column_mapper::column_to_property;
use crate::entity_writer::EntityWriter;
use crate::transformer::{StructTable, Transformer};

use crate::query_ident::struct_ref;
use catalytic::capitalizing::table_name_to_struct_name;
use catalytic::env_property_reader::keyspace;
use catalytic::materialized_view::{
    query_materialized_view, query_materialized_views, MaterializedView,
};
use catalytic::query_metadata::query_columns;
use catalytic::runtime::query_collect_to_vec;
use catalytic::table_metadata::TableName;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

mod column_mapper;

mod entity_writer;
pub mod query_ident;
pub mod transformer;

pub const GENERATED: &str = "generated";

#[derive(Clone, PartialEq, Debug, Eq)]
pub struct Table {
    pub table_name: String,
    pub materialized_view: Option<MaterializedView>,
}

/// base_dir: an absolute path where to place the generated files.
///     CAREFUL: all the files in this map can be deleted
///     Some something like: Users/myself/project/src/generated/
///     On Windows, the path should be something like C:\\users\\myself\\project\\src\\generated\\
///
/// transformer: trait in which customization can take place.
pub fn generate(base_dir: &Path, transformer: impl Transformer) {
    let keyspace = keyspace();
    let mut current_dir = std::env::current_dir().unwrap();

    comp_pb(base_dir, &current_dir);

    current_dir.push("src");

    comp_pb(base_dir, &current_dir);

    // Query all the tables
    let query = format!(
        "select table_name from system_schema.tables where keyspace_name = '{}'",
        keyspace
    );

    let non_materialized_views: Vec<TableName> = query_collect_to_vec(query, &[]);
    let materialized_views = query_materialized_views();
    let mut tables = vec![];

    for t in non_materialized_views {
        tables.push(Table {
            table_name: t.table_name,
            materialized_view: None,
        });
    }

    for t in materialized_views {
        tables.push(Table {
            materialized_view: query_materialized_view(&t.table_name),
            table_name: t.table_name,
        });
    }

    // Ignore result, because it can fail if this is the first time generating the structs
    let _ = std::fs::remove_dir_all(base_dir);
    std::fs::create_dir_all(base_dir).unwrap();

    let mut path_gen_entities_file = base_dir.to_path_buf();

    path_gen_entities_file.push("mod.rs");

    let mut mod_file = File::create(path_gen_entities_file).unwrap();

    add_generated_header(&mut mod_file);

    for table in tables {
        println!("Processing table: {}", table.table_name);

        let struct_name = table_name_to_struct_name(&table.table_name);
        let struct_name_ref = struct_ref(&quote::format_ident!("{}", struct_name));

        writeln!(
            mod_file,
            "#[allow(dead_code, clippy::clone_on_copy)]\npub mod {m};\npub use {m}::{{{}, {}}};",
            struct_name,
            struct_name_ref,
            m = table.table_name
        )
        .unwrap();

        // Query all the columns for this table
        let columns = query_columns(&table.table_name);

        // Create the file to place the generated rust code in
        let path_to_struct_file = format!("{}.rs", table.table_name);
        let mut file = File::create(base_dir.join(&path_to_struct_file)).unwrap();

        add_generated_header(&mut file);

        // Maps columns to properties
        let struct_field_metadata = column_to_property(
            StructTable {
                table: table.clone(),
                struct_name: struct_name.clone(),
            },
            &columns,
            &transformer,
        );
        // Generate the tokens needed for the rust struct
        let entity_write = EntityWriter {
            table,
            struct_name: &struct_name,
            struct_field_metadata,
            transformer: &transformer,
            columns: &columns,
        };

        let tokens = entity_write.create_tokens();

        write!(file, "{}", tokens).unwrap();

        // Format the output, since everything is on 1 line
        // This should always work
        assert!(format(&path_to_struct_file, base_dir));
    }

    // Close the strem
    drop(mod_file);

    // Format the output
    // This may fail sometimes with weird NULL bytes, in cause of failure, recursion
    if !format("mod.rs", base_dir) {
        generate(base_dir, transformer);
    }
}

fn format(file: &str, dir: &Path) -> bool {
    Command::new("rustfmt")
        .arg(file)
        .current_dir(dir)
        .status()
        .unwrap()
        .success()
}

pub fn add_generated_header(file: &mut File) {
    assert_eq!(0, file.metadata().unwrap().len());

    writeln!(file, "// Generated file").unwrap();
}

fn comp_pb(left: &Path, right: &Path) {
    assert_ne!(
        left.to_str().unwrap(),
        right.to_str().unwrap(),
        "Please create a map inside src, e.g. src/GEN_ENTITIES",
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::transformer::DefaultTransformer;
    use std::env;

    #[should_panic]
    #[test]
    fn test_illegal_base_dir0() {
        test_dir(&env::current_dir().unwrap());
    }

    #[should_panic]
    #[test]
    fn test_illegal_base_dir1() {
        let mut current_dir = env::current_dir().unwrap();

        current_dir.push("src");

        test_dir(&current_dir);
    }

    fn test_dir(dir: &Path) {
        generate(dir, DefaultTransformer)
    }
}
