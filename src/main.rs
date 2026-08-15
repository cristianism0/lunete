pub mod models;
pub mod parsers;
pub mod utils;

use crate::models::*;

fn main() {
    for c in SOURCES {
        let info = Finfo::gather_info(c);
        println!("{:?}", info);
    }
}
