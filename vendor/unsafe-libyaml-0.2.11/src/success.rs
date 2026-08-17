use core::ops::Deref;

pub const OK: Success = Success { ok: true };
pub const FAIL: Success = Success { ok: false };

#[must_use]
pub struct Success {
    pub ok: bool,
}

pub struct Failure {
    pub fail: bool,
}

impl Deref for Success {
    type Target = Failure;

    fn deref(&self) -> &Self::Target {
        if self.ok {
            &Failure { fail: false }
        } else {
            &Failure { fail: true }
        }
    }
}
