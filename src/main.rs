pub mod display;
pub mod models;
pub mod parsers;
pub mod utils;

use crate::display::table_cli::render_all_tables;
use crate::models::*;
use crate::utils::runner::run_all;
use std::collections::HashMap;

fn main() {
    let mut pf = vec![];
    for c in SOURCES {
        let info = Finfo::gather_info(c);
        pf.push(info);
    }

    let mut j_c = HashMap::new();
    j_c.insert(
        TableKey::Journal,
        vec!["_pid".into(), "code_file".into(), "priority".into()],
    );

    j_c.insert(
        TableKey::Wtmp,
        vec!["ut_type".into(), "ut_id".into(), "ut_dname".into()],
    );

    let scope = JournalScope::User;
    let mode = TableMode::Summary { columns: j_c };

    let vec_entry = run_all(pf, scope.clone());

    for entry in &vec_entry {
        let records = vec![
            RecordType::Sys(entry),
            RecordType::Auth(entry),
            //RecordType::Wtmp(entry),
            //RecordType::Journal(entry, &scope),
        ];

        let tables = render_all_tables(records, &mode);
        for table in tables {
            println!("{table}");
        }
    }
}
