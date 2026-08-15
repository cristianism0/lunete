use crate::models::{LogEntry, ParseError};
use crate::parsers::selector::LogParser;

pub struct ContainerLog;

impl LogParser for ContainerLog {
    fn parser(&self, _path: &std::path::Path) -> Result<LogEntry, ParseError> {
        Err(ParseError::MalformedLine(
            "ContainerLog::parser not yet implemented".into(),
        ))
    }
}
