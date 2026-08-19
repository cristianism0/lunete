use crate::{
    models::{JournalError, JournalRecord, LogEntry},
    parsers::selector::JournalParser,
};
use systemd::journal::Journal;

pub struct JournalLog;

// Fields User:
// CODE_FILE
// CODE_FUNC
// CODE_LINE
// MESSAGE
// PRIORITY
// SYSLOG_FACILITY
// SYSLOG_IDENTIFIER
// TID
// _AUDIT_LOGINUID
// _AUDIT_SESSION
// _BOOT_ID
// _GID
// _HOSTNAME
// _MACHINE_ID
// _PID
// _RUNTIME_SCOPE
// _SELINUX_CONTEXT (maychange)
// _SOURCE_REALTIME_TIMESTAMP
// _SYSTEMD_CGROUP
// _SYSTEMD_OWNER_UID
// _SYSTEMD_SLICE
// _SYSTEMD_UNIT
// _SYSTEMD_USER_SLICE
// _TRANSPORT
// _UID

// Fields System:
// MESSAGE
// PRIORITY
// SYSLOG_FACILITY
// SYSLOG_IDENTIFIER
// _BOOT_ID
// _HOSTNAME
// _MACHINE_ID
// _RUNTIME_SCOPE
// _SOURCE_BOOTTIME_TIMESTAMP
// _SOUCE_MONOTIC_TIMESTAMP
// _TRANSPORT

impl JournalParser for JournalLog {
    fn parse(&self, journal: &mut Journal) -> Result<Vec<LogEntry>, JournalError> {
        journal.seek_tail().map_err(|e| JournalError::IoError(e))?;
        journal
            .previous_skip(50)
            .map_err(|e| JournalError::IoError(e))?;

        let mut entries = Vec::new();
        while journal
            .next_entry()
            .map_err(|e| JournalError::IoError(e))?
            .is_some()
        {
            let record = extract_record(journal);
            entries.push(LogEntry::Journal(record));
        }
        Ok(entries)
    }
}

fn extract_record(_journal: &mut Journal) -> JournalRecord {
    todo!()
}
