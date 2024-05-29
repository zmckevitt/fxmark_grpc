use abomonation::Abomonation;

////////////////////////////// FS RPC Hdrs  //////////////////////////////

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone, Copy)]
#[repr(u8)]
pub(crate) enum DRPC {
    /// Open a file
    Open = 1,
    /// Read from a file
    Read = 2,
    /// Read from a file from the given offset
    PRead = 3,
    /// Write to a file
    Write = 4,
    /// Write to a file
    PWrite = 5,
    /// Close an opened file.
    Close = 6,
    /// Remove the file
    Remove = 7,
    /// Create a directory.
    MkDir = 8,
    /// Remove a directory.
    RmDir = 9,
}

pub struct OpenReq {
    pub path: Vec<u8>,
    pub flags: i32,
    pub mode: u32,
}

unsafe_abomonate!(OpenReq : path, flags, mode);

pub struct ReadReq {
    pub fd: i32,
    pub size: usize,
    pub offset: i64,
}

unsafe_abomonate!(ReadReq : fd, size, offset);

pub struct WriteReq {
    pub fd: i32,
    pub page: Vec<u8>,
    pub size: usize,
    pub offset: i64,
}

unsafe_abomonate!(WriteReq : fd, page, size, offset);

pub struct CloseReq {
    pub fd: i32,
}

unsafe_abomonate!(CloseReq : fd);

pub struct RemoveReq {
    pub path: Vec<u8>,
}

unsafe_abomonate!(RemoveReq : path);

pub struct MkdirReq {
    pub path: Vec<u8>,
    pub mode: u32,
}

unsafe_abomonate!(MkdirReq : path, mode);

pub struct Response {
    pub result: i32,
    pub size: usize,
    pub page: Vec<u8>, // only for read responses
}

unsafe_abomonate!(Response : result, size, page);
