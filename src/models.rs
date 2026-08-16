use std::io::Error;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
pub enum LogSource {
    Auth,
    Sys,
    Wtmp,
    Journal,
    Container,
}

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
];

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
pub enum LogEntry {
    Sys(SysRecord),
    Auth(AuthRecord),
    Wtmp(WtmpRecord),
    Journal(JournalRecord),
    Container(ContainerRecord),
}

#[derive(Debug)]
pub struct SysRecord;
// TODO: study structure and look for the main fields

#[derive(Debug)]
pub struct AuthRecord;
// TODO: study structure and look for the main fields

#[derive(Debug)]
pub struct WtmpRecord;
// TODO: study structure and look for the main fields

#[derive(Debug)]
pub struct JournalRecord;
// TODO: study structure and look for the main fields

#[derive(Debug)]
pub struct ContainerRecord;
// TODO: study structure and look for the main fields
