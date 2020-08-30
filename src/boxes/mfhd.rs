use std::io::BufRead;
use super::{
    IsoBoxInfo,
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
    fn parse<T: BufRead>(
        reader: &mut BoxReader<T>,
        _content_size: Option<u64>,
        _box_info: &std::rc::Rc<IsoBoxInfo>
    ) -> Result<Self, BoxParsingError> {
        let version = reader.read_u8()?;
        let flags = Flags::read(reader)?;
        let sequence_number = reader.read_u32()?;
        Ok(Self { version, flags, sequence_number })
    }

    fn get_inner_values_ref(&self) -> Vec<(&'static str, BoxValue)> {
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

    fn get_inner_boxes(self) -> Option<Vec<super::IsoBoxData>> {
        None
    }

    fn get_inner_boxes_ref(&self) -> Option<Vec<(&IsoBoxInfo, Option<&dyn IsoBoxEntry>)>> {
        None
    }
}
