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

pub struct Saio {
    version: u8,
    flags: Flags,
    aux_info_type: Option<u32>,
    aux_info_type_parameter: Option<u32>,
    entry_count: u32,
    offset: Vec<u64>,
}

impl IsoBoxParser for Saio {
    fn parse<T: BufRead>(
        reader: &mut BoxReader<T>,
        _content_size: Option<u64>,
        _box_info: &std::rc::Rc<IsoBoxInfo>
    ) -> Result<Self, BoxParsingError> {
        let version = reader.read_u8()?;
        let flags = Flags::read(reader)?;
        let (aux_info_type, aux_info_type_parameter) =
            if flags.has_flag(0x01){
                (Some(reader.read_u32()?), Some(reader.read_u32()?))
            } else {
                (None, None)
            };

        let entry_count = reader.read_u32()?;

        let mut offset: Vec<u64> = Vec::with_capacity(entry_count as usize);
        if version == 0 {
            for _ in 0..entry_count { offset.push(reader.read_u32()? as u64); }
        } else {
            for _ in 0..entry_count { offset.push(reader.read_u64()?); }
        }

        Ok(Self {
            version,
            flags,
            aux_info_type,
            aux_info_type_parameter,
            entry_count,
            offset
        })
    }

    fn get_inner_values_ref(&self) -> Vec<(&'static str, BoxValue)> {
        let mut values = vec![
            ("version", BoxValue::from(self.version)),
            ("flags", BoxValue::from(self.flags)),
        ];
        if let Some(val) = self.aux_info_type {
            values.push(("aux_info_type", BoxValue::from(val)));
        }
        if let Some(val) = self.aux_info_type_parameter {
            values.push(("aux_info_type_parameter", BoxValue::from(val)));
        }
        values.push(("entry_count", BoxValue::from(self.entry_count)));
        values.push(
            ("entries", BoxValue::Collection(
                    self.offset.iter().map(|sample| {
                        vec![("offset", BoxValue::from(*sample))]
                    }).collect()
            )));
        values
    }

    fn get_short_name() -> &'static str {
        "saio"
    }

    fn get_long_name() -> &'static str {
        "Sample Auxiliary Information Offsets Box"
    }

    fn get_inner_boxes(self) -> Option<Vec<super::IsoBoxData>> {
        None
    }

    fn get_inner_boxes_ref(&self) -> Option<Vec<(&IsoBoxInfo, Option<&dyn IsoBoxEntry>)>> {
        None
    }
}
