pub mod models;
pub mod parsers;
pub mod utils;

use crate::utils::extract::build_table;

use crate::{models::*, parsers::selector::parser_selector};

fn main() {
    let mut pf = vec![];
    for c in SOURCES {
        let info = Finfo::gather_info(c);
        pf.push(info);
    }

    let mut entries = vec![];
    for info in pf {
        match parser_selector(info) {
            Ok(mut e) => entries.append(&mut e),
            Err(e) => eprintln!("error: {:#?}", e),
        }
    }
    let t = vec![
        build_table::<SysRecord>(&entries),
        build_table::<AuthRecord>(&entries),
        build_table::<WtmpRecord>(&entries),
        build_table::<JournalRecord>(&entries),
        build_table::<ContainerRecord>(&entries),
    ];

    for d in t {
        if let Some(table) = d {
            println!("{table}");
        }
    }
}
