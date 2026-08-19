use std::fs::File;
use std::io::ErrorKind;
use std::path::Path;

use crate::models::{Finfo, LogEntry, LogSource, ParseError};
use crate::parsers::*;

pub fn parser_selector(file_info: Finfo) -> Result<Vec<LogEntry>, ParseError> {
    match file_info.source {
        LogSource::Sys => {
            let p = syslog::SysLog;
            p.check_access(&file_info.path)?;
            p.parser(&file_info.path)
        }
        LogSource::Auth => {
            let p = auth::AuthLog;
            p.check_access(&file_info.path)?;
            p.parser(&file_info.path)
        }
        LogSource::Wtmp => {
            let p = wtmp::WtmpLog;
            p.check_access(&file_info.path)?;
            p.parser(&file_info.path)
        }
        LogSource::Journal => {
            let p = journal::JournalLog;
            p.check_access(&file_info.path)?;
            p.parser(&file_info.path)
        }
        LogSource::Container => {
            let p = container::ContainerLog;
            p.check_access(&file_info.path)?;
            p.parser(&file_info.path)
        }
    }
}

pub trait LogParser {
    fn parser(&self, path: &Path) -> Result<Vec<LogEntry>, ParseError>;

    fn check_access(&self, path: &Path) -> Result<(), ParseError> {
        match File::open(path) {
            Ok(_) => Ok(()),
            Err(e) if e.kind() == ErrorKind::NotFound => Err(ParseError::MalformedLine(format!(
                "Path doesn't exists or was moved: {:#?}",
                path
            ))),
            Err(e) => Err(ParseError::IoError(e)),
        }
    }
}
