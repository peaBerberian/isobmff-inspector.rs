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

pub struct SidxReference {
    reference_type: u8,
    referenced_size: u32,
    subsegment_duration: u32,
    starts_with_sap: bool,
    sap_type: u8,
    sap_delta_time: u32,
}

pub struct Sidx {
    version: u8,
    flags: Flags,
    reference_id: u32,
    timescale: u32,
    earliest_presentation_time: u64,
    first_offset: u64,
    reserved: u16,
    reference_count: u16,
    references: Vec<SidxReference>,
}

impl IsoBoxParser for Sidx {
    fn parse<T: BufRead>(
        reader: &mut BoxReader<T>,
        _content_size: Option<u64>,
        _box_info: &Rc<IsoBoxInfo>
    ) -> Result<Self, BoxParsingError> {
        let version = reader.read_u8()?;
        let flags = Flags::read(reader)?;
        let reference_id = reader.read_u32()?;
        let timescale = reader.read_u32()?;
        let (earliest_presentation_time, first_offset) =
            if version == 0 {
                (reader.read_u32()? as u64, reader.read_u32()? as u64)
            } else {
                (reader.read_u64()?, reader.read_u64()?)
            };
        let reserved = reader.read_u16()?;
        let reference_count = reader.read_u16()?;
        let mut references = Vec::with_capacity(reference_count as usize);
        // TODO manually check that reference_count * 12  == remaining content_size?
        for _ in 0..reference_count {
            let first_4_bytes = reader.read_u32()?;
            let subsegment_duration = reader.read_u32()?;
            let third_4_bytes = reader.read_u32()?;
            references.push(SidxReference {
                reference_type: (first_4_bytes >> 31) as u8 & 0x01,
                referenced_size: first_4_bytes &0x7FFFFFFF,
                subsegment_duration,
                starts_with_sap: ((third_4_bytes >> 31) & 0x01) != 0,
                sap_type: ((third_4_bytes >> 28) & 0x07) as u8,
                sap_delta_time: third_4_bytes & 0x0FFFFFFF,
            });
        }
        Ok(Self {
            version,
            flags,
            reference_id,
            timescale,
            earliest_presentation_time,
            first_offset,
            reserved,
            reference_count,
            references,
        })
    }

    fn get_inner_values_ref(&self) -> Vec<(&'static str, BoxValue)> {
        vec![
            ("version", BoxValue::from(self.version)),
            ("flags", BoxValue::from(self.flags)),
            ("reference_id", BoxValue::from(self.reference_id)),
            ("timescale", BoxValue::from(self.timescale)),
            ("earliest_presentation_time", BoxValue::from(self.earliest_presentation_time)),
            ("first_offset", BoxValue::from(self.first_offset)),
            ("reserved", BoxValue::from(self.reserved)),
            ("reference_count", BoxValue::from(self.reference_count)),
            ("references", BoxValue::Collection(
                        self.references
                         .iter()
                         .map(|r| {
                            vec![
                                ("reference_type", BoxValue::from(r.reference_type)),
                                ("referenced_size", BoxValue::from(r.referenced_size)),
                                ("subsegment_duration", BoxValue::from(r.subsegment_duration)),
                                ("starts_with_sap", BoxValue::from(r.starts_with_sap)),
                                ("sap_type", BoxValue::from(r.sap_type)),
                                ("sap_delta_time", BoxValue::from(r.sap_delta_time)),
                            ]
                         })
                         .collect::<Vec<Vec<(&str, BoxValue)>>>()
                    )
            )
        ]
    }

    fn get_short_name() -> &'static str {
        "sidx"
    }

    fn get_long_name() -> &'static str {
        "Segment index Box"
    }

    fn get_inner_boxes(self) -> Option<Vec<super::IsoBoxData>> {
        None
    }

    fn get_inner_boxes_ref(&self) -> Option<Vec<(&IsoBoxInfo, Option<&dyn IsoBoxEntry>)>> {
        None
    }
}
