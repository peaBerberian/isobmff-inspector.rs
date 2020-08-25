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

pub struct Saiz {
    version: u8,
    flags: Flags,
    aux_info_type: Option<u32>,
    aux_info_type_parameter: Option<u32>,
    default_sample_info_size: u8,
    entry_count: u32,
    sample_info_size: Vec<u8>,
}

impl IsoBoxParser for Saiz {
    fn parse<T: BufRead>(reader: &mut BoxReader<T>, _size: u32) -> Result<Self, BoxParsingError> {
        let version = reader.read_u8()?;
        let flags = Flags::read(reader)?;
        let (aux_info_type, aux_info_type_parameter) =
            if flags.has_flag(0x01) {
                (Some(reader.read_u32()?), Some(reader.read_u32()?))
            } else {
                (None, None)
            };
        let default_sample_info_size = reader.read_u8()?;
        let entry_count = reader.read_u32()?;
        let sample_info_size: Vec<u8> =
            if default_sample_info_size != 0 { vec![] }
            else {
                let mut sizes = Vec::with_capacity(entry_count as usize);
                for _ in 0..entry_count {
                    // Hopeing that the compiler moves out the invariant!
                    sizes.push(reader.read_u8()?);
                }
                sizes
            };

        Ok(Self {
            version,
            flags,
            aux_info_type,
            aux_info_type_parameter,
            default_sample_info_size,
            entry_count,
            sample_info_size,
        })
    }

    fn get_inner_values(&self) -> Vec<(&'static str, BoxValue)> {
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
        values.push(
            ("default_sample_info_size", BoxValue::from(self.default_sample_info_size)));
        values.push(
            ("entry_count", BoxValue::from(self.entry_count)));
        if !self.sample_info_size.is_empty() {
            values.push(
                ("entries", BoxValue::Collection(
                        self.sample_info_size.iter().map(|sample|
                            vec![("sample_info_size", BoxValue::UInt8(*sample))]
                        ).collect()
                )));
        }
        values
    }

    fn get_short_name() -> &'static str {
        "saiz"
    }

    fn get_long_name() -> &'static str {
        "Sample Auxiliary Information Sizes Box"
    }

    fn get_contained_boxes(&self) -> Option<Vec<(&BoxInfo, Option<&dyn IsoBoxEntry>)>> {
        None
    }
}
