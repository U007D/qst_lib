use snafu::Snafu;

#[derive(Debug, Snafu)]
pub struct Error(InnerError);

impl Error {
    pub fn kind(&self) -> ErrorKind {
        match self.0 {
            InnerError::SampleError { source: _ } => ErrorKind::GeneralErrorCategory1,
        }
    }
}

#[derive(Debug, Snafu)]
enum InnerError {
    #[snafu(display("{}: {:?}", "msg::ERR_SAMPLE", "source"))]
    SampleError { source: std::num::ParseIntError, },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ErrorKind {
    GeneralErrorCategory1,
}
