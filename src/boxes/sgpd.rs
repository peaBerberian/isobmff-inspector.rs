use std::io::BufRead;
use super::{
    IsoBoxInfo,
    BoxParsingError,
    BoxReader,
    BoxValue,
    IsoBoxEntry,
    IsoBoxParser,
};

pub struct Sgpd { }

// XXX TODO
impl IsoBoxParser for Sgpd {
    fn parse<T: BufRead>(
        reader: &mut BoxReader<T>,
        _content_size: Option<u64>,
        _box_info: &std::rc::Rc<IsoBoxInfo>
    ) -> Result<Self, BoxParsingError> {
        Ok(Self { })
    }

    fn get_inner_values_ref(&self) -> Vec<(&'static str, BoxValue)> {
        vec![]
    }

    fn get_short_name() -> &'static str {
        "sgpd"
    }

    fn get_long_name() -> &'static str {
        "Sample Group Description Box"
    }

    fn get_inner_boxes(self) -> Option<Vec<super::IsoBoxData>> {
        None
    }

    fn get_inner_boxes_ref(&self) -> Option<Vec<(&IsoBoxInfo, Option<&dyn IsoBoxEntry>)>> {
        None
    }
}
