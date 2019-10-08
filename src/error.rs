use snafu::Snafu;

#[derive(Debug, Snafu)]
pub struct Error(pub(crate) InnerError);

impl Error {
    #[cfg(feature = "const_fn")]
    pub const fn kind(&self) -> ErrorKind {
        #[allow(clippy::unneeded_field_pattern)]
        match self.0 {
            InnerError::SampleError { source: _ } => ErrorKind::GeneralErrorCategory1,
        }
    }
    #[cfg(not(feature = "const_fn"))]
    #[allow(clippy::missing_const_for_fn)]
    pub fn kind(&self) -> ErrorKind {
        #[allow(clippy::unneeded_field_pattern)]
        match self.0 {
            InnerError::SampleError { source: _ } => ErrorKind::GeneralErrorCategory1,
        }
    }
}

#[derive(Debug, Snafu)]
pub(crate) enum InnerError {
    #[snafu(display("{}: {:?}", "msg::ERR_SAMPLE", "source"))]
    SampleError { source: std::num::ParseIntError },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[allow(clippy::pedantic)]
pub enum ErrorKind {
    GeneralErrorCategory1,
}
