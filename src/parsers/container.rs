use crate::parsers::selector::LogParser;

pub struct ContainerLog;

impl LogParser for ContainerLog {
    fn parser(
        &self,
        _path: &std::path::Path,
    ) -> Result<crate::models::LogEntry, crate::models::ParseError> {
        todo!()
    }
}
