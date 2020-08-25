use std::io::{BufRead, Seek};
use super::{
    BoxInfo,
    BoxParsingError,
    BoxReader,
    BoxValue,
    IsoBoxEntry,
    IsoBoxParser,
    utils::parse_children,
};

pub struct Moof {
    content: Vec<(BoxInfo, Option<Box<dyn IsoBoxEntry>>)>,
}

impl<'a> IsoBoxParser for Moof {
    fn parse<T: BufRead + Seek>(reader: &mut BoxReader<T>, size: u32) -> Result<Self, BoxParsingError> {
        let content = parse_children(reader, Some(size))?;
        Ok(Self { content })
    }

    fn get_contained_boxes(&self) -> Option<Vec<(&BoxInfo, Option<&dyn IsoBoxEntry>)>> {
        Some(self.content.iter().map(|c|
                (&c.0, c.1.as_ref().map(|boxed| { std::boxed::Box::as_ref(&boxed) }))
        ).collect())
    }

    fn get_inner_values(&self) -> Vec<(&'static str, BoxValue)> {
        vec![]
    }

    fn get_short_name() -> &'static str {
        "moof"
    }

    fn get_long_name() -> &'static str {
        "Movie Fragment Box"
    }
}
