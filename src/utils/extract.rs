use crate::models::{FromLogEntry, LogEntry};
use tabled::{Table, Tabled};

pub fn group<T: FromLogEntry>(entries: &[LogEntry]) -> Vec<&T> {
    entries.iter().filter_map(T::from_entry).collect()
}

pub fn build_table<T: Tabled + FromLogEntry>(entries: &[LogEntry]) -> Option<String> {
    let rows = group::<T>(entries);
    if rows.is_empty() {
        return None;
    }
    Some(Table::new(&rows).to_string())
}
