use std::io::BufRead;
use std::rc::Rc;
use super::{
    IsoBoxInfo,
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
    fn parse<T: BufRead>(
        reader: &mut BoxReader<T>,
        _content_size: Option<u64>,
        box_info: &Rc<IsoBoxInfo>
    ) -> Result<Self, BoxParsingError> {
        let version = reader.read_u8()?;
        if version != 0 {
            return Err(BoxParsingError::InvalidVersion {
                expected: vec![0],
                actual: version,
                box_info: Rc::clone(box_info),
            });
        }
        let flags = Flags::read(reader)?;
        let rate = reader.read_u32()?;
        let delay = reader.read_u32()?;
        Ok(Self { version, flags, rate, delay })
    }

    fn get_inner_values_ref(&self) -> Vec<(&'static str, BoxValue)> {
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

    fn get_inner_boxes(self) -> Option<Vec<super::IsoBoxData>> {
        None
    }

    fn get_inner_boxes_ref(&self) -> Option<Vec<(&IsoBoxInfo, Option<&dyn IsoBoxEntry>)>> {
        None
    }
}
