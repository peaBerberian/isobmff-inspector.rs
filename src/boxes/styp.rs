use std::io::{BufRead, Seek};
use super::{
    BoxInfo,
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
    fn parse<T: BufRead + Seek>(reader: &mut BoxReader<T>, size: u32) -> Result<Self, BoxParsingError> {
        let ftyp_equiv = Ftyp::parse(reader, size)?;
        Ok(Self {
            major_brand: ftyp_equiv.major_brand,
            minor_brand: ftyp_equiv.minor_brand,
            compatible_brands: ftyp_equiv.compatible_brands
        })
    }

    fn get_inner_values(&self) -> Vec<(&'static str, BoxValue)> {
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

    fn get_contained_boxes(&self) -> Option<Vec<(&BoxInfo, Option<&dyn IsoBoxEntry>)>> {
        None
    }
}
