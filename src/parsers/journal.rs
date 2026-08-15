use crate::parsers::selector::LogParser;

pub struct JournalLog;

impl LogParser for JournalLog {
    fn parser(
        &self,
        _path: &std::path::Path,
    ) -> Result<crate::models::LogEntry, crate::models::ParseError> {
        todo!()
    }
}
