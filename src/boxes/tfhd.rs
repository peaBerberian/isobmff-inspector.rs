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

pub struct Tfhd {
    version: u8,
    flags: Flags,
    track_id: u32,
    base_data_offset: Option<u64>,
    sample_description_index: Option<u32>,
    default_sample_duration: Option<u32>,
    default_sample_size: Option<u32>,
    default_sample_flags: Option<u32>,
}

impl IsoBoxParser for Tfhd {
    fn parse<T: BufRead>(reader: &mut BoxReader<T>, _size: u32) -> Result<Self, BoxParsingError> {
        let version = reader.read_u8()?;
        let flags = Flags::read(reader)?;

        let flag_base_data_offset = flags.has_flag(0x000001);
        let flag_sample_description_index = flags.has_flag(0x000002);
        let flag_default_sample_duration = flags.has_flag(0x000008);
        let flag_default_sample_size = flags.has_flag(0x000010);
        let flag_default_sample_flags = flags.has_flag(0x000020);

        // TODO indicate flags values in get_inner_values
        // let flag_duration_is_empty = flags.has_flag(0x010000);
        // let flag_default_base_is_moof = flags.has_flag(0x020000);

        let track_id = reader.read_u32()?;
        let base_data_offset = if flag_base_data_offset {
           Some(reader.read_u64()?)
        } else { None };
        let sample_description_index = if flag_sample_description_index {
            Some(reader.read_u32()?)
        } else { None };
        let default_sample_duration = if flag_default_sample_duration {
            Some(reader.read_u32()?)
        } else { None };
        let default_sample_size = if flag_default_sample_size {
            Some(reader.read_u32()?)
        } else { None };
        let default_sample_flags = if flag_default_sample_flags {
            Some(reader.read_u32()?)
        } else { None };
        Ok(Self {
            version,
            flags,
            track_id,
            base_data_offset,
            sample_description_index,
            default_sample_duration,
            default_sample_size,
            default_sample_flags,
        })
    }

    fn get_inner_values(&self) -> Vec<(&'static str, BoxValue)> {
        let mut values = vec![
            ("version", BoxValue::from(self.version)),
            ("flags", BoxValue::from(self.flags)),
            ("track_id", BoxValue::from(self.track_id)),
        ];
        if let Some(val) = self.base_data_offset {
            values.push(("base_data_offset", BoxValue::from(val)));
        }
        if let Some(val) = self.sample_description_index {
            values.push(("sample_description_index", BoxValue::from(val)));
        }
        if let Some(val) = self.default_sample_duration {
            values.push(("default_sample_duration", BoxValue::from(val)));
        }
        if let Some(val) = self.default_sample_size {
            values.push(("default_sample_size", BoxValue::from(val)));
        }
        if let Some(val) = self.default_sample_flags {
            values.push(("default_sample_flags", BoxValue::from(val)));
        }
        values
    }

    fn get_short_name() -> &'static str {
        "tfhd"
    }

    fn get_long_name() -> &'static str {
        "Track Fragment Header Box"
    }

    fn get_contained_boxes(&self) -> Option<Vec<(&BoxInfo, Option<&dyn IsoBoxEntry>)>> {
        None
    }
}
