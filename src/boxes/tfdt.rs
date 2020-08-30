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

pub struct Tfdt {
    version: u8,
    flags: Flags,
    base_media_decode_time: u64,
}

impl IsoBoxParser for Tfdt {
    fn parse<T: BufRead>(
        reader: &mut BoxReader<T>,
        _content_size: Option<u64>,
        _box_info: &std::rc::Rc<IsoBoxInfo>
    ) -> Result<Self, BoxParsingError> {
        let version = reader.read_u8()?;
        let flags = Flags::read(reader)?;
        let base_media_decode_time = if version == 1 {
            reader.read_u64()?
        } else {
            reader.read_u32()? as u64
        };

        Ok(Self {
            version,
            flags,
            base_media_decode_time
        })
    }

    fn get_inner_values_ref(&self) -> Vec<(&'static str, BoxValue)> {
        vec![
            ("version", BoxValue::from(self.version)),
            ("flags", BoxValue::from(self.flags)),
            ("base_media_decode_time", BoxValue::from(self.base_media_decode_time))
        ]
    }

    fn get_short_name() -> &'static str {
        "tfdt"
    }

    fn get_long_name() -> &'static str {
        "Track fragment decode time"
    }

    fn get_inner_boxes(self) -> Option<Vec<super::IsoBoxData>> {
        None
    }

    fn get_inner_boxes_ref(&self) -> Option<Vec<(&IsoBoxInfo, Option<&dyn IsoBoxEntry>)>> {
        None
    }
}
