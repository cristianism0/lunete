use crate::models::{LogEntry, ParseError};
use crate::parsers::selector::LogParser;
pub struct JournalLog;

impl LogParser for JournalLog {
    fn parser(&self, _path: &std::path::Path) -> Result<LogEntry, ParseError> {
        Err(ParseError::MalformedLine(
            "Journal::parser not yet implemented".into(),
        ))
    }
}
