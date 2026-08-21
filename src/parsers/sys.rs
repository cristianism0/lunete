use crate::parsers::selector::LogParser;

use crate::models::{LogEntry, ParseError};
pub struct SysLog;

impl LogParser for SysLog {
    fn parser(&self, _path: &std::path::Path) -> Result<Vec<LogEntry>, ParseError> {
        Err(ParseError::MalformedLine(
            "SysLog::parser not yet implemented".into(),
        ))
    }
}
