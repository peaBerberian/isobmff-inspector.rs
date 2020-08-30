use std::io::{BufRead, Seek};
use std::rc::Rc;
use super::{
    IsoBoxInfo,
    BoxParsingError,
    BoxReader,
    BoxValue,
    IsoBoxEntry,
    IsoBoxParser,
    utils::parse_children,
};

pub struct Moof {
    content: Vec<(Rc<IsoBoxInfo>, Option<Box<dyn IsoBoxEntry>>)>,
}

impl<'a> IsoBoxParser for Moof {
    fn parse<T: BufRead + Seek>(
        reader: &mut BoxReader<T>,
        content_size: Option<u64>,
        box_info: &Rc<IsoBoxInfo>
    ) -> Result<Self, BoxParsingError> {
        let content = parse_children(reader, content_size, Some(box_info))?;
        Ok(Self { content })
    }

    fn get_inner_boxes(self) -> Option<Vec<super::IsoBoxData>> {
        Some(self.content)
    }

    fn get_inner_boxes_ref(&self) -> Option<Vec<(&IsoBoxInfo, Option<&dyn IsoBoxEntry>)>> {
        Some(self.content.iter().map(|c|
                (c.0.as_ref(), c.1.as_ref().map(|boxed| { std::boxed::Box::as_ref(&boxed) }))
        ).collect())
    }

    fn get_inner_values_ref(&self) -> Vec<(&'static str, BoxValue)> {
        vec![]
    }

    fn get_short_name() -> &'static str {
        "moof"
    }

    fn get_long_name() -> &'static str {
        "Movie Fragment Box"
    }
}
