use sparrow_core::ErrorCode;

#[derive(derive_more::Display, Debug)]
pub enum Error {
    #[display(fmt = "compute request missing '{_0}'")]
    MissingField(&'static str),
    #[display(fmt = "invalid output path '{_0}'")]
    InvalidOutputPath(String),
    #[display(fmt = "unspecified per-entity behavior")]
    UnspecifiedPerEntityBehavior,
    #[display(fmt = "unspecified output format")]
    UnspecifiedOutputFormat,
    #[display(fmt = "invalid batch input bounds")]
    InvalidBounds,
    #[display(fmt = "internal compute error: {_0}")]
    Internal(&'static str),
    #[display(fmt = "failed to create tempfile")]
    CreateTempFile,
    #[display(fmt = "invalid operation: {_0}")]
    InvalidOperation(String),
    #[display(fmt = "failed to get next input for operation")]
    GetNextInput,
    #[display(fmt = "failed to pre-process next input for operation")]
    PreprocessNextInput,
}

macro_rules! invalid_operation {
    ($fmt:expr) => {
        crate::execute::Error::InvalidOperation($fmt.to_owned())
    };
    ($fmt:expr, $($arg:expr),+) => {
        crate::execute::Error::InvalidOperation(format!($fmt, $($arg),+))
    };
}

pub(crate) use invalid_operation;

impl Error {
    pub fn internal() -> Self {
        Error::Internal("no additional context")
    }

    pub fn internal_msg(msg: &'static str) -> Self {
        Error::Internal(msg)
    }

    pub fn invalid_operation(msg: String) -> Self {
        Error::InvalidOperation(msg)
    }
}

impl error_stack::Context for Error {}

impl ErrorCode for Error {
    fn error_code(&self) -> tonic::Code {
        match self {
            Error::MissingField(_) | Error::InvalidOutputPath(_) => tonic::Code::InvalidArgument,
            _ => tonic::Code::Internal,
        }
    }
}
