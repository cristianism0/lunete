use crate::models::{LogEntry, ParseError};
use crate::parsers::selector::LogParser;

pub struct AuthLog;

impl LogParser for AuthLog {
    fn parser(&self, _path: &std::path::Path) -> Result<Vec<LogEntry>, ParseError> {
        Err(ParseError::MalformedLine(
            "Auth::parser not yet implemented".into(),
        ))
    }
}
