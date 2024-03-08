use std::os::unix::io::AsFd;
use std::os::unix::io::BorrowedFd;

/// Slice of the file
/// This works only on raw fd
#[derive(Debug, Clone)]
pub struct AsyncFileSlice<'fd> {
    fd: BorrowedFd<'fd>,
    position: u64,
    len: u64,
}

impl<'fd> AsyncFileSlice<'fd> {
    pub fn new(fd: BorrowedFd<'fd>, position: u64, len: u64) -> AsyncFileSlice<'fd> {
        Self { fd, position, len }
    }

    pub fn position(&self) -> u64 {
        self.position
    }

    pub fn len(&self) -> u64 {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn fd(&self) -> BorrowedFd<'_> {
        self.fd
    }
}

impl AsFd for AsyncFileSlice<'_> {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.fd
    }
}
