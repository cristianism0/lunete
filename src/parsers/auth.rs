use crate::models::{LogEntry, ParseError};
use crate::parsers::selector::LogParser;
use regex::Regex;

pub struct AuthLog;

// Structure for Auth and Sys follow RFC 3164:
// PRI HEADER MSG
// but, for a better readability most linus system ommit the priority
// if you have priority available

impl LogParser for AuthLog {
    fn parser(&self, _path: &std::path::Path) -> Result<Vec<LogEntry>, ParseError> {
        let _pattern = "<191>TIMESTAMP HOSTNAME/IP";
        //Aug 20 19:14:57 hostname systemd[1]: Starting user@0.service - User Manager for UID 0...

        Err(ParseError::MalformedLine(
            "Auth::parser not yet implemented".into(),
        ))
    }
}
