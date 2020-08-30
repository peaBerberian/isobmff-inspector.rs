use std::io::{BufRead, Seek};
use super::{
    IsoBoxInfo,
    BoxParsingError,
    BoxReader,
    BoxValue,
    ftyp::Ftyp,
    IsoBoxEntry,
    IsoBoxParser,
};

pub struct Styp {
    major_brand: String,
    minor_brand: u32,
    compatible_brands: Vec<String>,
}

impl IsoBoxParser for Styp {
    fn parse<T: BufRead + Seek>(
        reader: &mut BoxReader<T>,
        content_size: Option<u64>,
        box_info: &std::rc::Rc<IsoBoxInfo>
    ) -> Result<Self, BoxParsingError> {
        let ftyp_equiv = Ftyp::parse(reader, content_size, box_info)?;
        Ok(Self {
            major_brand: ftyp_equiv.major_brand,
            minor_brand: ftyp_equiv.minor_brand,
            compatible_brands: ftyp_equiv.compatible_brands
        })
    }

    fn get_inner_values_ref(&self) -> Vec<(&'static str, BoxValue)> {
        vec![
            ("major_brand", BoxValue::from(self.major_brand.as_str())),
            ("minor_brand", BoxValue::from(self.minor_brand)),
            ("compatible_brands", BoxValue::from(self.compatible_brands.as_slice()))
        ]
    }

    fn get_short_name() -> &'static str {
        "styp"
    }

    fn get_long_name() -> &'static str {
        "Segment Type Box"
    }

    fn get_inner_boxes(self) -> Option<Vec<super::IsoBoxData>> {
        None
    }

    fn get_inner_boxes_ref(&self) -> Option<Vec<(&IsoBoxInfo, Option<&dyn IsoBoxEntry>)>> {
        None
    }
}
