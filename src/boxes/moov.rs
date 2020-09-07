use std::io::{BufRead, Seek};
use super::{
    BoxParsingError,
    BoxReader,
    BoxValue,
    BoxInfo,

    IsoBoxParser,
    IsoBoxEntry,

    utils::parse_children,
};

pub struct Moov {
    content: Vec<(BoxInfo, Option<Box<dyn IsoBoxEntry>>)>,
}

impl<'a> IsoBoxParser for Moov {
    fn parse<T: BufRead + Seek>(reader: &mut BoxReader<T>, size: u32) -> Result<Self, BoxParsingError> {
        let content = parse_children(reader, Some(size))?;
        Ok(Self { content })
    }

    fn get_contained_boxes(&self) -> Option<Vec<(&BoxInfo, Option<&Box<dyn IsoBoxEntry>>)>> {
        Some(self.content.iter().map(|c| (&c.0, c.1.as_ref())).collect())
    }

    fn get_inner_values(&self) -> Vec<(&'static str, BoxValue)> {
        vec![]
    }

    fn get_short_name() -> &'static str {
        "moov"
    }

    fn get_long_name() -> &'static str {
        "Movie Box"
    }
}
