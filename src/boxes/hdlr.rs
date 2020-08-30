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

pub struct Hdlr {
    version: u8,
    flags: Flags,
    pre_defined: u32,
    handler_type: u32,
    reserved: [u8; 3],
    name: String,
}

impl IsoBoxParser for Hdlr {
    fn parse<T: BufRead>(
        reader: &mut BoxReader<T>,
        content_size: Option<u64>,
        box_info: &Rc<IsoBoxInfo>
    ) -> Result<Self, BoxParsingError> {
        let version = reader.read_u8()?;
        if version != 0 {
            return Err(BoxParsingError::InvalidVersion {
                box_info: Rc::clone(box_info),
                expected: vec![0],
                actual: version,
            });
        }
        let flags = Flags::read(reader)?;
        let pre_defined = reader.read_u32()?;
        let handler_type = reader.read_u32()?;
        let reserved = [reader.read_u8()?, reader.read_u8()?, reader.read_u8()?];
        let name = if let Some(size) = content_size {
            reader.read_str(size as usize)?
        } else {
            String::from_utf8(reader.read_to_end()?)?
        };
        Ok(Self { version, flags, handler_type, pre_defined, reserved, name })
    }

    fn get_inner_values_ref(&self) -> Vec<(&'static str, BoxValue)> {
        vec![
            ("version", BoxValue::from(self.version)),
            ("flags", BoxValue::from(self.flags)),
            ("pre_defined", BoxValue::from(self.pre_defined)),
            ("handler_type", BoxValue::from(self.handler_type)),
            ("reserved", BoxValue::from(self.reserved.as_ref())),
            ("name", BoxValue::from(self.name.as_str()))
        ]
    }

    fn get_short_name() -> &'static str {
        "hdlr"
    }

    fn get_long_name() -> &'static str {
        "Handler Reference Box"
    }

    fn get_inner_boxes(self) -> Option<Vec<super::IsoBoxData>> {
        None
    }

    fn get_inner_boxes_ref(&self) -> Option<Vec<(&IsoBoxInfo, Option<&dyn IsoBoxEntry>)>> {
        None
    }
}
