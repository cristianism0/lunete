use std::path::Path;

use crate::models::{Finfo, LogEntry, LogSource, ParseError};
use crate::parsers::*;

pub fn parser_selector(file_info: Finfo) -> () {
    //dispatch based on the LogSource, the differentiation will be done inside the trait functions.
    let _ = match file_info.source {
        LogSource::Sys => syslog::SysLog.parser(&file_info.path),
        LogSource::Auth => auth::AuthLog.parser(&file_info.path),
        LogSource::Wtmp => wtmp::WtmpLog.parser(&file_info.path),
        LogSource::Journal => journal::JournalLog.parser(&file_info.path),
        LogSource::Container => container::ContainerLog.parser(&file_info.path),
    };
}

pub trait LogParser {
    fn parser(&self, path: &Path) -> Result<LogEntry, ParseError>;
}
