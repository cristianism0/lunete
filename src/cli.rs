use std::collections::HashMap;

use crate::models::{JournalScope, TableKey, TableMode};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct ArgsC {
    #[command(subcommand)]
    log: LogKey,
    #[arg(long, group = "display", value_delimiter = ',')]
    summary: Option<Vec<String>>,
    #[arg(long, group = "display")]
    compact: Option<usize>,
    #[arg(long, group = "display")]
    standard: bool, //help display only (default behavior)
    #[arg(short, long)]
    key: Option<String>,
}

#[derive(Subcommand, Debug, PartialEq, Eq, Hash)]
enum LogKey {
    Sys,
    Auth,
    Wtmp,
    Journal {
        #[arg(long, default_value = "user")]
        scope: JournalScope,
    },
}

impl ArgsC {
    pub fn table_mode(&self) -> TableMode {
        let args = Self::parse();

        if args.summary.is_some() {
            let k = get_key(args.log);
            let mut hash = HashMap::new();
            hash.insert(k, args.summary.unwrap()).unwrap();
            return TableMode::Summary { columns: hash };
        } else if args.compact.is_some() {
            return TableMode::Compact {
                max_col_width: args.compact.unwrap(),
            };
        } else if args.key.is_some() {
            return TableMode::KeyValue;
        } else {
            TableMode::Standard
        }
    }
    pub fn get_scope(&self) -> Option<JournalScope> {
        let args = Self::parse();
        match args.log {
            LogKey::Journal { scope } => Some(scope),
            _ => None,
        }
    }
}

fn get_key(l: LogKey) -> TableKey {
    match l {
        LogKey::Sys => TableKey::Sys,
        LogKey::Auth => TableKey::Auth,
        LogKey::Wtmp => TableKey::Wtmp,
        LogKey::Journal { scope: _ } => TableKey::Journal,
    }
}

pub fn table_cli_args() -> ArgsC {
    ArgsC::parse()
}
