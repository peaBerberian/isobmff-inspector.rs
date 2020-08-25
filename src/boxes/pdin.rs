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

pub struct Pdin {
    version: u8,
    flags: Flags,
    rate: u32,
    delay: u32,
}

impl IsoBoxParser for Pdin {
    fn parse<T: BufRead>(reader: &mut BoxReader<T>, _size: u32) -> Result<Self, BoxParsingError> {
        let version = reader.read_u8()?;
        if version != 0 {
            return Err(BoxParsingError::InvalidVersion { expected: vec![0], actual: version });
        }
        let flags = Flags::read(reader)?;
        let rate = reader.read_u32()?;
        let delay = reader.read_u32()?;
        Ok(Self { version, flags, rate, delay })
    }

    fn get_inner_values(&self) -> Vec<(&'static str, BoxValue)> {
        vec![
            ("version", BoxValue::from(self.version)),
            ("flags", BoxValue::from(self.flags)),
            ("rate", BoxValue::from(self.rate)),
            ("delay", BoxValue::from(self.delay)),
        ]
    }

    fn get_short_name() -> &'static str {
        "pdin"
    }

    fn get_long_name() -> &'static str {
        "Progressive Download Information Box"
    }

    fn get_contained_boxes(&self) -> Option<Vec<(&BoxInfo, Option<&dyn IsoBoxEntry>)>> {
        None
    }
}
