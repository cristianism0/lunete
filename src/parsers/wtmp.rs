use crate::parsers::selector::LogParser;

use crate::models::{LogEntry, ParseError};
pub struct WtmpLog;

impl LogParser for WtmpLog {
    fn parser(&self, _path: &std::path::Path) -> Result<LogEntry, ParseError> {
        Err(ParseError::MalformedLine(
            "WtmpLog::parser not yet implemented".into(),
        ))
    }
}
