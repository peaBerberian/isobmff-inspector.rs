use std::io::{BufRead, Seek};
use super::{
    BoxInfo,
    BoxParsingError,
    BoxReader,
    BoxValue,
    IsoBoxEntry,
    IsoBoxParser,
};

pub struct Free {}
impl IsoBoxParser for Free {
    fn parse<T: BufRead + Seek>(reader: &mut BoxReader<T>, size: u32) -> Result<Self, BoxParsingError> {
        reader.skip_bytes(size as u64)?;
        Ok(Self {})
    }

    fn get_inner_values(&self) -> Vec<(&'static str, BoxValue)> {
        vec![]
    }

    fn get_short_name() -> &'static str {
        "free"
    }

    fn get_long_name() -> &'static str {
        "Free space box"
    }

    fn get_contained_boxes(&self) -> Option<Vec<(&BoxInfo, Option<&Box<dyn IsoBoxEntry>>)>> {
        None
    }
}
