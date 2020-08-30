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

pub struct Mvhd {
    version: u8,
    flags: Flags,
    creation_time: u64,
    modification_time: u64,
    timescale: u32,
    duration: u64,
    rate: [u16; 2],
    volume: [u8; 2],
    reserved_1: u16,
    reserved_2: [u32; 2],
    matrix: [u32; 9],
    pre_defined: [u32; 6],
    next_track_id: u32,
}

impl IsoBoxParser for Mvhd {
    fn parse<T: BufRead>(
        reader: &mut BoxReader<T>,
        _content_size: Option<u64>,
        box_info: &Rc<IsoBoxInfo>
    ) -> Result<Self, BoxParsingError> {
        let version = reader.read_u8()?;
        let flags = Flags::read(reader)?;
        let (creation_time, modification_time, timescale, duration) = match version {
            0 => (
                reader.read_u32()? as u64,
                reader.read_u32()? as u64,
                reader.read_u32()?,
                reader.read_u32()? as u64),
            1 => (
                reader.read_u64()?,
                reader.read_u64()?,
                reader.read_u32()?,
                reader.read_u64()?),
            v => {
                return Err(BoxParsingError::InvalidVersion {
                    box_info: Rc::clone(box_info),
                    expected: vec![0, 1],
                    actual: v });
            }
        };

        let rate = [reader.read_u16()?, reader.read_u16()?];
        let volume = [reader.read_u8()?, reader.read_u8()?];
        let reserved_1 = reader.read_u16()?;
        let reserved_2 = [reader.read_u32()?, reader.read_u32()?];
        let matrix = [
            reader.read_u32()?, reader.read_u32()?, reader.read_u32()?,
            reader.read_u32()?, reader.read_u32()?, reader.read_u32()?,
            reader.read_u32()?, reader.read_u32()?, reader.read_u32()?];
        let pre_defined = [
            reader.read_u32()?, reader.read_u32()?, reader.read_u32()?,
            reader.read_u32()?, reader.read_u32()?, reader.read_u32()?];
        let next_track_id = reader.read_u32()?;
        Ok(Self {
            version,
            flags,
            creation_time,
            modification_time,
            timescale,
            duration,
            rate,
            volume,
            reserved_1,
            reserved_2,
            matrix,
            pre_defined,
            next_track_id,
        })
    }

    fn get_inner_values_ref(&self) -> Vec<(&'static str, BoxValue)> {
        vec![
            ("version", BoxValue::from(self.version)),
            ("flags", BoxValue::from(self.flags)),
            ("creation_time", BoxValue::from(self.creation_time)),
            ("modification_time", BoxValue::from(self.modification_time)),
            ("timescale", BoxValue::from(self.timescale)),
            ("duration", BoxValue::from(self.duration)),
            ("rate", BoxValue::FixedPoint16(self.rate)),
            ("volume", BoxValue::FixedPoint8(self.volume)),
            ("reserved_1", BoxValue::from(self.reserved_1)),
            ("reserved_2", BoxValue::from(self.reserved_2.as_ref())),
            ("matrix", BoxValue::Matrix3_3(&self.matrix)),
            ("pre_defined", BoxValue::from(self.pre_defined.as_ref())),
            ("next_track_id", BoxValue::from(self.next_track_id)),
        ]
    }

    fn get_short_name() -> &'static str {
        "mvhd"
    }

    fn get_long_name() -> &'static str {
        "Movie Header Box"
    }

    fn get_inner_boxes(self) -> Option<Vec<super::IsoBoxData>> {
        None
    }

    fn get_inner_boxes_ref(&self) -> Option<Vec<(&IsoBoxInfo, Option<&dyn IsoBoxEntry>)>> {
        None
    }
}
