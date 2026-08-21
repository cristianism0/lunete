use crate::models::{LogEntry, ParseError};
use crate::parsers::selector::LogParser;
use regex::Regex;
pub struct SysLog;

impl LogParser for SysLog {
    fn parser(&self, _path: &std::path::Path) -> Result<Vec<LogEntry>, ParseError> {
        let _sys_pattern = Regex::new(r"^(?:<(?P<pri>[0-9]+)>)?(?P<month>[A-Za-z]{3})\s+(?P<day>[0-9]{1,2})\s+(?P<time>[0-9]{2}:[0-9]{2}:[0-9]{2})\s+(?P<host>\S+)\s+(?P<process>[^:]+):\s*(?P<msg>.*)$").unwrap();

        Err(ParseError::MalformedLine(
            "SysLog::parser not yet implemented".into(),
        ))
    }
}
