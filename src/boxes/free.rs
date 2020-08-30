use std::io::{BufRead, Seek};
use super::{
    IsoBoxInfo,
    BoxParsingError,
    BoxReader,
    BoxValue,
    IsoBoxEntry,
    IsoBoxParser,
};

pub struct Free {}
impl IsoBoxParser for Free {
    fn parse<T: BufRead + Seek>(
        reader: &mut BoxReader<T>,
        content_size: Option<u64>,
        _box_info: &std::rc::Rc<IsoBoxInfo>
    ) -> Result<Self, BoxParsingError> {
        if let Some(size_to_skip) = content_size {
            reader.skip_bytes(size_to_skip)?;
        } else {
            reader.skip_to_end()?;
        }
        Ok(Self {})
    }

    fn get_inner_values_ref(&self) -> Vec<(&'static str, BoxValue)> {
        vec![]
    }

    fn get_short_name() -> &'static str {
        "free"
    }

    fn get_long_name() -> &'static str {
        "Free space box"
    }

    fn get_inner_boxes(self) -> Option<Vec<super::IsoBoxData>> {
        None
    }

    fn get_inner_boxes_ref(&self) -> Option<Vec<(&IsoBoxInfo, Option<&dyn IsoBoxEntry>)>> {
        None
    }
}
