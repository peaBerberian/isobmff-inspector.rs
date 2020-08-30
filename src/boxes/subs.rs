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

pub struct SubsEntry {
    sample_delta: u32,
    subsample_count: u16,
    subsample_size: Vec<u32>,
    subsample_priority: u8,
    discardable: u8,
    codec_specific_parameters: u32,
}

pub struct Subs {
    version: u8,
    flags: Flags,
    entry_count: u32,
    entries: Vec<SubsEntry>,
}

impl IsoBoxParser for Subs {
    fn parse<T: BufRead>(
        reader: &mut BoxReader<T>,
        _content_size: Option<u64>,
        _box_info: &std::rc::Rc<IsoBoxInfo>
    ) -> Result<Self, BoxParsingError> {
        let version = reader.read_u8()?;
        let flags = Flags::read(reader)?;
        let entry_count = reader.read_u32()?;
        let mut entries: Vec<SubsEntry> = Vec::with_capacity(entry_count as usize);
        for _ in 0..entry_count {
            let sample_delta = reader.read_u32()?;
            let subsample_count = reader.read_u16()?;
            let mut subsample_size = Vec::with_capacity(subsample_count as usize);
            for _ in 0..subsample_count {
                let size =
                    if version == 1 {
                        reader.read_u32()?
                    } else {
                        reader.read_u16()? as u32
                    };
                subsample_size.push(size);
            }
            // XXX TODO Re-check
            let subsample_priority = reader.read_u8()?;
            let discardable = reader.read_u8()?;
            let codec_specific_parameters = reader.read_u32()?;
            entries.push(SubsEntry {
                sample_delta,
                subsample_count,
                subsample_size,
                subsample_priority,
                discardable,
                codec_specific_parameters
            });
        }

        Ok(Self {
            version,
            flags,
            entry_count,
            entries,
        })
    }

    fn get_inner_values_ref(&self) -> Vec<(&'static str, BoxValue)> {
        vec![
            ("version", BoxValue::from(self.version)),
            ("flags", BoxValue::from(self.flags)),
            ("entry_count", BoxValue::from(self.entry_count)),
            ("entries", BoxValue::Collection(
                    self.entries.iter().map(|entry| {
                        vec![
                            ("sample_delta", BoxValue::from(entry.sample_delta)),
                            ("subsample_count", BoxValue::from(entry.subsample_count)),
                            ("subsample_size", BoxValue::from(entry.subsample_size.as_slice())),
                            ("subsample_priority", BoxValue::from(entry.subsample_priority)),
                            ("discardable", BoxValue::from(entry.discardable)),
                            ("codec_specific_parameters",
                             BoxValue::from(entry.codec_specific_parameters)),
                        ]
                    }).collect()
            ))
    ]
    }

    fn get_short_name() -> &'static str {
        "subs"
    }

    fn get_long_name() -> &'static str {
        "Sub-Sample Information Box"
    }

    fn get_inner_boxes(self) -> Option<Vec<super::IsoBoxData>> {
        None
    }

    fn get_inner_boxes_ref(&self) -> Option<Vec<(&IsoBoxInfo, Option<&dyn IsoBoxEntry>)>> {
        None
    }
}
