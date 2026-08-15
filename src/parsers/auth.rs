use crate::parsers::selector::LogParser;

pub struct AuthLog;

impl LogParser for AuthLog {
    fn parser(
        &self,
        _path: &std::path::Path,
    ) -> Result<crate::models::LogEntry, crate::models::ParseError> {
        todo!()
    }
}
