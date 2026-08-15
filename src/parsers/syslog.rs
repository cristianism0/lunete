use crate::parsers::selector::LogParser;

pub struct SysLog;

impl LogParser for SysLog {
    fn parser(
        &self,
        _path: &std::path::Path,
    ) -> Result<crate::models::LogEntry, crate::models::ParseError> {
        todo!()
    }
}
