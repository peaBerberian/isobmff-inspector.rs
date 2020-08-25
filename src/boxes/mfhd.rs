use std::io::BufRead;
use super::{
    BoxInfo,
    BoxParsingError,
    BoxReader,
    BoxValue,
    Flags,
    IsoBoxEntry,
    IsoBoxParser,
};

pub struct Mfhd {
    version: u8,
    flags: Flags,
    sequence_number: u32,
}

impl IsoBoxParser for Mfhd {
    fn parse<T: BufRead>(reader: &mut BoxReader<T>, _size: u32) -> Result<Self, BoxParsingError>
    where Self: Sized {
        let version = reader.read_u8()?;
        let flags = Flags::read(reader)?;
        let sequence_number = reader.read_u32()?;
        Ok(Self { version, flags, sequence_number })
    }

    fn get_inner_values(&self) -> Vec<(&'static str, BoxValue)> {
        vec![
            ("version", BoxValue::from(self.version)),
            ("flags", BoxValue::from(self.flags)),
            ("sequence_number", BoxValue::from(self.sequence_number)),
        ]
    }

    fn get_short_name() -> &'static str {
        "mfhd"
    }

    fn get_long_name() -> &'static str {
        "Movie Fragment Header Box"
    }

    fn get_contained_boxes(&self) -> Option<Vec<(&BoxInfo, Option<&dyn IsoBoxEntry>)>> {
        None
    }
}
