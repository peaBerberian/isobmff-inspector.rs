use thiserror::Error;

use std::rc::Rc;
use super::IsoBoxInfo;

fn display_expected_version_string(expected: &[u8]) -> String {
    match expected.len() {
        0 => "no version".to_string(),
        1 => "version ".to_string() + &expected[0].to_string(),
        _ => ["one of ".to_string(), expected
            .iter()
            .map(|u| u.to_string())
            .collect::<Vec<String>>()
            .join(", ")
        ].join("")
    }
}

#[derive(Error, Debug)]
pub enum BoxParsingError {
    /// Error related to standard IO (e.g. file opening)
    #[error("{0}")]
    IOError(#[from] std::io::Error),

    /// Error related to string conversion
    #[error("error when parsing a string: {0}")]
    UTF8Error(#[from] std::string::FromUtf8Error),

    /// The version of the box is not handled.
    #[error(
            "invalid version: expected {}, found {}",
            display_expected_version_string(&.expected),
            .actual)]
    InvalidVersion {
        /// The box in which the error happened
        box_info: Rc<IsoBoxInfo>,
        /// All valid version that would have been accepted.
        expected : Vec<u8>,
        /// Actual version number.
        actual: u8,
    },

    /// The size for the current box is too small to be properly
    /// parsed.
    #[error("data store discaonnected")]
    BoxTooSmall {
        /// The "short" name of the box (the name on 4 ASCII characters)
        /// `None` if the name of the box cannot even be parsed.
        short_name: Option<String>,

        /// Offset the box starts at in the ISOBMFF file.
        offset: u64,

        /// The complete size of the box, as anounced in the ISOBMFF.
        /// `0` is a special value which means that the box goes until the
        /// end of the file.
        size: u64,

        /// Parent box in which the error happened.
        /// `None` if this was the top-level box.
        parent_box_info: Option<Rc<IsoBoxInfo>>,
    },

    /// The box size for the current box is too large when compared to its
    /// container.
    #[error("data store dbisconnected")]
    BoxTooLarge {
        /// The box in which the error happened
        box_info: Rc<IsoBoxInfo>,

        /// The expected length the current box should have most likely readhed.
        expected_maximum: u64,

        /// The actual size of the box.
        actual: u64,
    },

    /// Error for when an internal box parser read too much data when compared
    /// to the size of the box it had to parse.
    /// This usually means that the box given was too small.
    /// TODO Merge with BoxTooSmall?
    #[error("data store dcisconnected")]
    ParserReadTooMuch {
        /// The box in which the error happened
        box_info: Rc<IsoBoxInfo>,
        /// The number of bytes that should have been parsed by that parser
        expected: u64,
        /// The actual number of bytes parsed by that parser
        actual: u64
    },

    /// Error for when an internal box parser read not enough data when compared
    /// to the size of the box it had to parse.
    /// This usually means that the box given was too big.
    /// TODO Merge with BoxTooLarge?
    #[error("data store ddisconnected")]
    ParserReadNotEnough {
        /// The box in which the error happened
        box_info: Rc<IsoBoxInfo>,
        /// The number of bytes that should have been parsed by that parser
        expected: u64,
        /// The actual number of bytes parsed by that parser
        actual: u64
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
