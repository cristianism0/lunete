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
        path: "/var/log/auth.log",
    },
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
    SourceCandidate {
        source: LogSource::Sys,
        path: "/var/log/kern.log",
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
