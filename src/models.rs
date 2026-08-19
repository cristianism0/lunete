use std::io::Error;
use std::path::PathBuf;

use tabled::Tabled;

#[derive(Debug, Clone, Copy)]
pub enum LogSource {
    Auth,
    Sys,
    Wtmp,
    Container,
}

#[derive(Debug)]
pub enum JournalScope {
    System,
    User,
}

// ---------- Paths ----------
#[derive(Debug)]
pub struct SourceCandidate {
    pub source: LogSource,
    pub path: &'static str,
}

pub const SOURCES: &[SourceCandidate] = &[
    SourceCandidate {
        source: LogSource::Auth,
        path: "/var/log/secure",
    },
    SourceCandidate {
        source: LogSource::Wtmp,
        path: "/var/log/wtmp",
    },
    SourceCandidate {
        source: LogSource::Sys,
        path: "/var/log/syslog",
    },
    SourceCandidate {
        source: LogSource::Sys,
        path: "/var/log/messages",
    },
    // legacy - fallback for journald
    SourceCandidate {
        source: LogSource::Sys,
        path: "/var/log/kern.log",
    },
    SourceCandidate {
        source: LogSource::Auth,
        path: "/var/log/auth.log",
    },
];
// ---------- File Variants ----------
#[derive(Debug)]
pub enum FsKind {
    Regular,
    Dir,
    Symlink,
    Socket,
    Fifo,
    CharDevice,
    BlockDevice,
    Unknown,
}

#[derive(Debug)]
pub enum ContentFormat {
    PlainText,
    Json,
    Binary,
    Unknown,
}

// ---------- File Data Structures ----------
#[derive(Debug)]
pub struct Finfo {
    pub path: PathBuf,
    pub source: LogSource,
    pub pstatus: PathStatus,
    pub data: Option<FiData>,
}

#[derive(Debug)]
pub struct FiData {
    pub kind: FsKind,
    pub mode: u32,
    pub readable: bool,
    pub format: ContentFormat,
}

// ---------- Error Models ----------
#[derive(Debug)]
pub enum PathStatus {
    Found,
    NotFound,
    Indeterminate(Error),
}

#[derive(Debug)]
pub enum ParseError {
    IoError(Error),
    MalformedLine(String),
    UnexpectedFormat(String),
}

#[derive(Debug)]
pub enum JournalError {
    IoError(Error),
    FieldMissing(String), // ENODATA
    NotPositioned,        // EADDRNOTAVAIL
    Unavailable(String),
}

// ---------- Records ----------
#[derive(Debug, Tabled)]
pub struct SysRecord {
    pub raw: String,
}

#[derive(Debug, Tabled)]
pub struct AuthRecord {
    pub raw: String,
}

#[derive(Debug, Tabled)]
pub struct WtmpRecord {
    pub ut_type: i16,
    pub ut_pid: i32,
    pub ut_dname: String,
    pub ut_id: String,
    pub ut_user: String,
    pub ut_host: String,
    pub e_termination: i16,
    pub e_exit: i16,
}

#[derive(Debug, Tabled)]
pub struct JournalRecord {
    pub raw: String,
}

#[derive(Debug, Tabled)]
pub struct ContainerRecord {
    pub raw: String,
}

#[derive(Debug, Tabled)]
pub enum LogEntry {
    Sys(#[tabled(inline)] SysRecord),
    Auth(#[tabled(inline)] AuthRecord),
    Wtmp(#[tabled(inline)] WtmpRecord),
    Journal(#[tabled(inline)] JournalRecord),
    Container(#[tabled(inline)] ContainerRecord),
}

impl LogEntry {
    pub fn label(&self) -> &'static str {
        match self {
            LogEntry::Sys(_) => "SYS",
            LogEntry::Auth(_) => "AUTH",
            LogEntry::Wtmp(_) => "WTMP",
            LogEntry::Journal(_) => "JOURNAL",
            LogEntry::Container(_) => "CONTAINER",
        }
    }
}

pub trait FromLogEntry {
    fn from_entry(entry: &LogEntry) -> Option<&Self>;
}

impl FromLogEntry for SysRecord {
    fn from_entry(entry: &LogEntry) -> Option<&Self> {
        match entry {
            LogEntry::Sys(r) => Some(r),
            _ => None,
        }
    }
}

impl FromLogEntry for AuthRecord {
    fn from_entry(entry: &LogEntry) -> Option<&Self> {
        match entry {
            LogEntry::Auth(r) => Some(r),
            _ => None,
        }
    }
}

impl FromLogEntry for WtmpRecord {
    fn from_entry(entry: &LogEntry) -> Option<&Self> {
        match entry {
            LogEntry::Wtmp(r) => Some(r),
            _ => None,
        }
    }
}

impl FromLogEntry for JournalRecord {
    fn from_entry(entry: &LogEntry) -> Option<&Self> {
        match entry {
            LogEntry::Journal(r) => Some(r),
            _ => None,
        }
    }
}

impl FromLogEntry for ContainerRecord {
    fn from_entry(entry: &LogEntry) -> Option<&Self> {
        match entry {
            LogEntry::Container(r) => Some(r),
            _ => None,
        }
    }
}
