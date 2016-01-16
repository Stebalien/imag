use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error as FmtError;
use std::clone::Clone;
use std::convert::From;

use std::io::Error as IOError;

#[derive(Clone)]
pub enum StoreErrorType {
    IdNotFound,
    OutOfMemory,
    // maybe more
}

fn store_error_type_as_str(e: &StoreErrorType) -> &'static str {
    match e {
        &StoreErrorType::IdNotFound  => "ID not found",
        &StoreErrorType::OutOfMemory => "Out of Memory",
    }
}

impl Debug for StoreErrorType {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "{:?}", store_error_type_as_str(self)));
        Ok(())
    }

}

impl Display for StoreErrorType {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "{}", store_error_type_as_str(self)));
        Ok(())
    }

}

pub struct StoreError {
    err_type: StoreErrorType,
    cause: Option<Box<Error>>,
}

impl StoreError {

    pub fn new(errtype: StoreErrorType, cause: Option<Box<Error>>)
        -> StoreError
    {
        StoreError {
            err_type: errtype,
            cause: cause,
        }
    }

    pub fn err_type(&self) -> StoreErrorType {
        self.err_type.clone()
    }

}

impl Debug for StoreError {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "[{:?}]: caused by: {:?}",
                    self.err_type, self.cause));
        Ok(())
    }

}

impl Display for StoreError {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "[{}]", store_error_type_as_str(&self.err_type.clone())));
        Ok(())
    }

}

impl Error for StoreError {

    fn description(&self) -> &str {
        store_error_type_as_str(&self.err_type.clone())
    }

    fn cause(&self) -> Option<&Error> {
        self.cause.as_ref().map(|e| &**e)
    }

}

