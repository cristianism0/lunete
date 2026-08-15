use crate::parsers::selector::LogParser;

pub struct WtmpLog;

impl LogParser for WtmpLog {
    fn parser(
        &self,
        _path: &std::path::Path,
    ) -> Result<crate::models::LogEntry, crate::models::ParseError> {
        todo!()
    }
}
