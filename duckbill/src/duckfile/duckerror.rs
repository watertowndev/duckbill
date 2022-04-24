use std::num::ParseIntError;
use std::string::FromUtf8Error;

#[derive(Debug,PartialEq)]
pub enum DuckError {
    BadData,
    NotEnoughMarkers,
    DataTooShort,
    LengthMarkerMissing,
    LengthMarkerFormatBad,
    MarkCountMismatch,
    BadStringData,
    BadNumberData,
    NoAccountIDFound,
    BadAccountIdFormat,
    AccountIDTooShort,
    BadHeaderFormat,
    HeaderTooShort,
    BadFooterFormat,
    BillCountOutOfBounds,
    BadIdentifierData,
    BadBillNumberFormat,
    NegativeNumber,
    OpCancelled,
    IoError,
    FileTooSmall,
    FileTooBig,
}

impl From<std::string::FromUtf8Error> for DuckError {
    fn from(_: FromUtf8Error) -> Self {
        DuckError::BadStringData
    }
}

impl From<std::num::ParseIntError> for DuckError {
    fn from(_: ParseIntError) -> Self {
        DuckError::BadNumberData
    }
}

impl From<std::io::Error> for DuckError {
    fn from(_: std::io::Error) -> Self {
        DuckError::IoError
    }
}
