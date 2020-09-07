use std::io::BufRead;
use super::{
    BoxInfo,
    BoxParsingError,
    BoxReader,
    BoxValue,
    IsoBoxEntry,
    IsoBoxParser,
};

pub struct Sgpd { }

// XXX TODO
impl IsoBoxParser for Sgpd {
    fn parse<T: BufRead>(reader: &mut BoxReader<T>, _size: u32) -> Result<Self, BoxParsingError> {
        Ok(Self { })
    }

    fn get_inner_values(&self) -> Vec<(&'static str, BoxValue)> {
        vec![]
    }

    fn get_short_name() -> &'static str {
        "sgpd"
    }

    fn get_long_name() -> &'static str {
        "Sample Group Description Box"
    }

    fn get_contained_boxes(&self) -> Option<Vec<(&BoxInfo, Option<&Box<dyn IsoBoxEntry>>)>> {
        None
    }
}
