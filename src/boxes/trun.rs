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

pub struct TrunSample {
    duration: Option<u32>,
    size: Option<u32>,
    flags: Option<u32>,

    // To make place both for an u32 and a i32
    // Alternatives (e.g. enums) require us to also make place for both in
    // memory anyway.
    composition_time_offset: Option<i64>,
}

pub struct Trun {
    version: u8,
    flags: Flags,
    sample_count: u32,
    data_offset: Option<i32>,
    first_sample_flags: Option<u32>,
    samples: Vec<TrunSample>,
}

impl IsoBoxParser for Trun {
    fn parse<T: BufRead>(reader: &mut BoxReader<T>, _size: u32) -> Result<Self, BoxParsingError> {
        let version = reader.read_u8()?;
        let flags = Flags::read(reader)?;

        let flag_data_offset = flags.has_flag(0x000001);
        let flag_first_sample = flags.has_flag(0x000004);
        let flag_sample_duration = flags.has_flag(0x000100);
        let flag_sample_size = flags.has_flag(0x000200);
        let flag_sample = flags.has_flag(0x000400);
        let flag_sample_composition_time_offset = flags.has_flag(0x000800);

        let sample_count = reader.read_u32()?;
        let data_offset = if flag_data_offset { Some(reader.read_i32()?) }
        else { None };

        let first_sample_flags = if flag_first_sample { Some(reader.read_u32()?) }
        else { None };

        let mut samples: Vec<TrunSample> = Vec::with_capacity(sample_count as usize);
        for _ in 0.. sample_count {
            let duration = if flag_sample_duration {
                Some(reader.read_u32()?)
            } else { None };
            let size = if flag_sample_size {
                Some(reader.read_u32()?)
            } else { None };
            let flags = if flag_sample {
                Some(reader.read_u32()?)
            } else { None };
            let composition_time_offset = if flag_sample_composition_time_offset {
                if version == 0 {
                    Some(reader.read_u32()? as i64)
                } else {
                    Some(reader.read_i32()? as i64)
                }
            } else {
                None
            };
            samples.push(TrunSample { duration, size, flags, composition_time_offset });
        }
        Ok(Self {
            version,
            flags,
            sample_count,
            data_offset,
            first_sample_flags,
            samples,
        })
    }

    fn get_inner_values(&self) -> Vec<(&'static str, BoxValue)> {
        let mut values = vec![
            ("version", BoxValue::from(self.version)),
            ("flags", BoxValue::from(self.flags)),
            ("sample_count", BoxValue::from(self.sample_count)),
        ];
        if let Some(val) = self.data_offset {
            values.push(("data_offset", BoxValue::from(val)));
        }
        if let Some(val) = self.first_sample_flags {
            values.push(("first_sample_flags", BoxValue::from(val)));
        }
        values.push(
            ("samples",
             BoxValue::Collection(
                 self.samples.iter().map(|sample| {
                     let mut sample_values = vec![];
                     if let Some(duration) = sample.duration {
                         sample_values.push(("duration", BoxValue::from(duration)));
                     }
                     if let Some(size) = sample.size {
                         sample_values.push(("size", BoxValue::from(size)));
                     }
                     if let Some(flags) = sample.flags {
                         sample_values.push(("flags", BoxValue::from(flags)));
                     }
                     if let Some(composition_time_offset) = sample.composition_time_offset {
                         sample_values.push(
                             ("composition_time_offset",
                              BoxValue::from(composition_time_offset)));
                     }
                     sample_values
                 }).collect()
             ))
        );
        values
    }

    fn get_short_name() -> &'static str {
        "trun"
    }

    fn get_long_name() -> &'static str {
        "Track Fragment Run Box"
    }

    fn get_contained_boxes(&self) -> Option<Vec<(&BoxInfo, Option<&dyn IsoBoxEntry>)>> {
        None
    }
}
