#[derive(Debug)]
pub enum BoxParsingError {
    /// Error related to standard IO (e.g. file opening)
    IOError(std::io::Error),
    UTF8Error(std::string::FromUtf8Error),
    InvalidVersion {
        expected : Vec<u8>,
        actual: u8
    }
}

impl From<std::io::Error> for BoxParsingError {
    fn from(err : std::io::Error) -> BoxParsingError {
        BoxParsingError::IOError(err)
    }
}

impl From<std::string::FromUtf8Error> for BoxParsingError {
    fn from(err : std::string::FromUtf8Error) -> BoxParsingError {
        BoxParsingError::UTF8Error(err)
    }
}

impl From<ReadStrError> for BoxParsingError {
    fn from(err : ReadStrError) -> BoxParsingError {
        match err {
            ReadStrError::IOError(e) => BoxParsingError::IOError(e),
            ReadStrError::FromUTF8Error(e) => BoxParsingError::UTF8Error(e)
        }
    }
}

#[derive(Debug)]
pub enum ReadStrError {
    /// Error related to standard IO (e.g. file opening)
    IOError(std::io::Error),

    /// Error related to byte to String (ASCII or UTF-8) conversion
    FromUTF8Error(std::string::FromUtf8Error),
}

impl From<std::io::Error> for ReadStrError {
    fn from(err : std::io::Error) -> ReadStrError {
        ReadStrError::IOError(err)
    }
}

impl From<std::string::FromUtf8Error> for ReadStrError {
    fn from(err : std::string::FromUtf8Error) -> ReadStrError {
        ReadStrError::FromUTF8Error(err)
    }
}
