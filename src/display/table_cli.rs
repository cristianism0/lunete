use crate::models::{AuthRecord, FromLogEntry, JournalRecord, LogEntry, SysRecord, WtmpRecord};
use crate::models::{JournalScope, RecordType, TableMode};
use tabled::{
    Table, Tabled,
    settings::{Remove, Style, Width, object::Columns},
};

const JOURNAL_USER_ONLY_COLUMNS: [usize; 4] = [8, 9, 17, 20];

pub fn group<T: FromLogEntry>(entries: &[LogEntry]) -> Vec<&T> {
    entries.iter().filter_map(T::from_entry).collect()
}

fn remove_columns(table: &mut Table, mut indices: Vec<usize>) {
    indices.sort_unstable();
    indices.dedup();
    for col in indices.into_iter().rev() {
        table.with(Remove::column(Columns::new(col..=col)));
    }
}

fn keep_only_columns(table: &mut Table, total_cols: usize, keep: &[usize]) {
    let to_remove: Vec<usize> = (0..total_cols).filter(|c| !keep.contains(c)).collect();
    remove_columns(table, to_remove);
}

fn apply_mode_style<T: Tabled>(table: &mut Table, mode: &TableMode) {
    match mode {
        TableMode::Standard => {
            table.with(Style::rounded());
        }
        TableMode::Compact { max_col_width } => {
            table
                .with(Style::sharp())
                .with(Width::truncate(*max_col_width).suffix("..."));
        }
        TableMode::Summary { columns } => {
            table.with(Style::modern());
            keep_only_columns(table, T::headers().len(), columns);
        }
        TableMode::KeyValue => unreachable!("KeyValue is rendered via render_key_value"),
    }
}

fn render_key_value<T: Tabled>(rows: Vec<&T>, hide_columns: Option<&[usize]>) -> String {
    let mut output = String::new();
    for (i, row) in rows.into_iter().enumerate() {
        let mut table = Table::new(vec![row]);
        if let Some(cols) = hide_columns {
            remove_columns(&mut table, cols.to_vec());
        }
        table.with(Style::extended());
        output.push_str(&format!("[ Entry {} ]\n", i + 1));
        output.push_str(&table.to_string());
        output.push_str("\n\n");
    }
    output
}

pub fn build_table<T: Tabled + FromLogEntry>(
    entries: &[LogEntry],
    mode: &TableMode,
) -> Option<String> {
    let rows = group::<T>(entries);
    if rows.is_empty() {
        return None;
    }

    if let TableMode::KeyValue = mode {
        return Some(render_key_value(rows, None));
    }

    let mut table = Table::new(rows);
    apply_mode_style::<T>(&mut table, mode);
    Some(table.to_string())
}

pub fn build_journal_table<T: Tabled + FromLogEntry>(
    entries: &[LogEntry],
    scope: &JournalScope,
    mode: &TableMode,
) -> Option<String> {
    let rows = group::<T>(entries);
    if rows.is_empty() {
        return None;
    }

    let hide_columns =
        matches!(scope, JournalScope::System).then_some(&JOURNAL_USER_ONLY_COLUMNS[..]);

    if let TableMode::KeyValue = mode {
        return Some(render_key_value(rows, hide_columns));
    }

    let mut table = Table::new(rows);
    if let Some(cols) = hide_columns {
        remove_columns(&mut table, cols.to_vec());
    }
    apply_mode_style::<T>(&mut table, mode);
    Some(table.to_string())
}

pub fn render_all_tables(records: Vec<RecordType<'_>>, mode: &TableMode) -> Vec<String> {
    let mut rendered = Vec::with_capacity(records.len());

    for record in records {
        let table_opt = match record {
            RecordType::Sys(entries) => build_table::<SysRecord>(entries, &mode),
            RecordType::Auth(entries) => build_table::<AuthRecord>(entries, &mode),
            RecordType::Wtmp(entries) => build_table::<WtmpRecord>(entries, &mode),
            RecordType::Journal(entries, scope) => {
                build_journal_table::<JournalRecord>(entries, &scope, &mode)
            }
        };

        if let Some(table) = table_opt {
            rendered.push(table);
        }
    }

    rendered
}
