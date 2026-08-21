pub mod display;
pub mod models;
pub mod parsers;
pub mod utils;

use crate::display::table_cli::render_all_tables;
use crate::models::*;
use crate::utils::runner::run_all;

fn main() {
    let mut pf = vec![];
    for c in SOURCES {
        let info = Finfo::gather_info(c);
        pf.push(info);
    }

    let scope = JournalScope::User;
    let mode = TableMode::Summary {
        columns: (1..9).collect(),
    };

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
