// created from https://github.com/hyperledger/indy-sdk/tree/master/vcx/libvcx

use std::cell::RefCell;
use std::fmt;
use std::ffi::CString;
use std::ptr;

use failure::{Context, Backtrace, Fail};

pub mod prelude {
    pub use super::{err_msg, GolServerError, GolServerErrorKind, GolResult};
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum GolServerErrorKind {
    // Common
    #[fail(display = "Server error")]
    ServerError,

    #[fail(display = "Common error {}", 0)]
    Common(u32),
}

#[derive(Debug)]
pub struct GolServerError {
    inner: Context<GolServerErrorKind>
}

impl Fail for GolServerError {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for GolServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;

        for cause in Fail::iter_chain(&self.inner) {
            if first {
                first = false;
                writeln!(f, "Error: {}", cause)?;
            } else {
                writeln!(f, "  Caused by: {}", cause)?;
            }
        }

        Ok(())
    }
}

impl GolServerError {
    pub fn from_msg<D>(kind: GolServerErrorKind, msg: D) -> GolServerError
        where D: fmt::Display + fmt::Debug + Send + Sync + 'static {
        GolServerError { inner: Context::new(msg).context(kind) }
    }

    pub fn kind(&self) -> GolServerErrorKind {
        *self.inner.get_context()
    }

    pub fn extend<D>(self, msg: D) -> GolServerError
        where D: fmt::Display + fmt::Debug + Send + Sync + 'static {
        let kind = self.kind();
        GolServerError { inner: self.inner.map(|_| msg).context(kind) }
    }

    pub fn map<D>(self, kind: GolServerErrorKind, msg: D) -> GolServerError
        where D: fmt::Display + fmt::Debug + Send + Sync + 'static {
        GolServerError { inner: self.inner.map(|_| msg).context(kind) }
    }
}

pub fn err_msg<D>(kind: GolServerErrorKind, msg: D) -> GolServerError
    where D: fmt::Display + fmt::Debug + Send + Sync + 'static {
    GolServerError::from_msg(kind, msg)
}

impl From<GolServerErrorKind> for GolServerError {
    fn from(kind: GolServerErrorKind) -> GolServerError {
        GolServerError::from_msg(kind, "some error message")
    }
}

impl From<Context<GolServerErrorKind>> for GolServerError {
    fn from(inner: Context<GolServerErrorKind>) -> GolServerError {
        GolServerError { inner }
    }
}

impl From<GolServerError> for u32 {
    fn from(code: GolServerError) -> u32 {
        code.kind().into()
    }
}

impl From<GolServerErrorKind> for u32 {
    fn from(code: GolServerErrorKind) -> u32 {
        match code {
            GolServerErrorKind::ServerError => 1,
            GolServerErrorKind::Common(num) => 2,
        }
    }
}

pub type GolResult<T> = Result<T, GolServerError>;
//
// /// Extension methods for `Result`.
// pub trait GolResultExt<T, E> {
//     fn to_gol<D>(self, kind: GolServerErrorKind, msg: D) -> GolResult<T> where D: fmt::Display + Send + Sync + 'static;
// }
//
// impl<T, E> GolResultExt<T, E> for Result<T, E> where E: Fail
// {
//     fn to_gol<D>(self, kind: GolServerErrorKind, msg: D) -> GolResult<T> where D: fmt::Display + Send + Sync + 'static {
//         self.map_err(|err| err.context(msg).context(kind).into())
//     }
// }
//
// /// Extension methods for `Error`.
// pub trait GolErrorExt {
//     fn to_gol<D>(self, kind: GolServerErrorKind, msg: D) -> GolServerError where D: fmt::Display + Send + Sync + 'static;
// }
//
// impl<E> GolErrorExt for E where E: Fail
// {
//     fn to_gol<D>(self, kind: GolServerErrorKind, msg: D) -> GolServerError where D: fmt::Display + Send + Sync + 'static {
//         self.context(format!("\n{}: {}", std::any::type_name::<E>(), msg)).context(kind).into()
//     }
// }