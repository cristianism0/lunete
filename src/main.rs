pub mod models;
pub mod parsers;
pub mod utils;

use crate::{models::*, parsers::selector::parser_selector};

fn main() {
    let mut pf = vec![];

    for c in SOURCES {
        let info = Finfo::gather_info(c);
        pf.push(info);
    }

    for info in pf {
        match parser_selector(info) {
            Ok(entry) => println!("entry: {:?}", entry),
            Err(e) => eprintln!("error: {:?}", e),
        }
    }
}
