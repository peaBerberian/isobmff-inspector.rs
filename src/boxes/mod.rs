mod box_types;
mod box_reader;
mod error;
mod utils;

// individual boxes
mod free;
mod ftyp;
mod hdlr;
mod mdat;
mod mfhd;
mod moof;
mod moov;
mod mvhd;
mod pdin;
mod saio;
mod saiz;
mod styp;
mod subs;
mod tfdt;
mod tfhd;
mod traf;
mod trun;

pub use box_types::{BoxInfo, BoxValue, Flags, IsoBoxEntry, IsoBoxParser};
pub use box_reader::BoxReader;
pub use error::BoxParsingError;
pub use utils::IsoBoxData;

use std::io::{BufRead, Seek};
pub fn parse_iosbmff(
    reader: impl BufRead + Seek
) -> Result<Vec<utils::IsoBoxData>, BoxParsingError> {
    let mut box_reader = BoxReader::create(reader);
    utils::parse_children(&mut box_reader, None)
}
