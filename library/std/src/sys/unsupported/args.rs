use crate::ffi::OsString;

pub unsafe fn init(_argc: isize, _argv: *const *const u8) {}
pub unsafe fn cleanup() {}

pub struct Args {}

pub fn args() -> Args {
    Args {}
}

impl fmt::Debug for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().finish()
    }
}

impl Iterator for Args {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> {
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(0))
    }
}

impl ExactSizeIterator for Args {
    fn len(&self) -> usize {
        0
    }
}

impl DoubleEndedIterator for Args {
    fn next_back(&mut self) -> Option<OsString> {
        None
    }
}
