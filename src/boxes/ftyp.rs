use std::io::BufRead;
use std::rc::Rc;
use super::{
    IsoBoxInfo,
    BoxParsingError,
    BoxReader,
    BoxValue,
    IsoBoxEntry,
    IsoBoxParser,
};

pub struct Ftyp {
   pub major_brand: String,
   pub minor_brand: u32,
   pub compatible_brands: Vec<String>,
}

impl IsoBoxParser for Ftyp {
    fn parse<T: BufRead>(
        reader: &mut BoxReader<T>,
        content_size: Option<u64>,
        box_info: &Rc<IsoBoxInfo>
    ) -> Result<Self, BoxParsingError> {
        let major_brand = reader.read_str(4)?;
        let minor_brand = reader.read_u32()?;

        let mut compatible_brands: Vec<String> = vec![];

        let mut remaining_size = if let Some(size) = content_size {
            if size < 8 {
                let parent_box_info = box_info.parent_box_info.as_ref().map(|info| {
                    Rc::clone(info)
                });
                return Err(BoxParsingError::BoxTooSmall {
                    parent_box_info,
                    short_name: Some(box_info.short_name.clone()),
                    offset: box_info.offset,
                    size: box_info.size,
                });
            }
            Some(size - 8)
        } else {
            None
        };

        while !reader.is_empty()? && remaining_size.map_or(true, |size| size > 0) {
            compatible_brands.push(reader.read_str(4)?);

            remaining_size = if let Some(size) = remaining_size {
                if size < 4 {
                    let parent_box_info = box_info.parent_box_info.as_ref().map(|info| {
                        Rc::clone(info)
                    });
                    return Err(BoxParsingError::BoxTooSmall {
                        parent_box_info,
                        short_name: Some(box_info.short_name.clone()),
                        offset: box_info.offset,
                        size: box_info.size,
                    });
                }
                Some(size - 4)
            } else {
                None
            };
        }
        Ok(Self { major_brand, minor_brand, compatible_brands })
    }

    fn get_inner_values_ref(&self) -> Vec<(&'static str, BoxValue)> {
        vec![
            ("major_brand", BoxValue::from(self.major_brand.as_str())),
            ("minor_brand", BoxValue::from(self.minor_brand)),
            ("compatible_brands", BoxValue::from(self.compatible_brands.as_slice()))
        ]
    }

    fn get_short_name() -> &'static str {
        "ftyp"
    }

    fn get_long_name() -> &'static str {
        "File Type Box"
    }

    fn get_inner_boxes(self) -> Option<Vec<super::IsoBoxData>> {
        None
    }

    fn get_inner_boxes_ref(&self) -> Option<Vec<(&IsoBoxInfo, Option<&dyn IsoBoxEntry>)>> {
        None
    }
}
