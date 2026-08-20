use crate::parsers::selector::{journal_parsed, parser_selector};

use crate::models::{Finfo, JournalScope, LogEntry};

pub fn run_all(file_infos: Vec<Finfo>, journal_scope: JournalScope) -> Vec<Vec<LogEntry>> {
    let mut ret = vec![];
    for f in file_infos {
        match parser_selector(f) {
            Ok(e) => ret.push(e),
            Err(e) => eprintln!("error: {:#?}", e),
        }
    }
    let j = journal_parsed(journal_scope).unwrap();
    ret.push(j);
    ret
}
