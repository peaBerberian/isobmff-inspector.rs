use std::io::BufRead;
use super::{
    BoxInfo,
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
    fn parse<T: BufRead>(reader: &mut BoxReader<T>, size: u32) -> Result<Self, BoxParsingError> {
        let major_brand = reader.read_str(4)?;
        let minor_brand = reader.read_u32()?;
        let mut compatible_brands: Vec<String> = vec![];
        let mut remaining_size = size - 8;
        while !reader.is_empty()? && remaining_size > 0 {
            compatible_brands.push(reader.read_str(4)?);
            remaining_size = remaining_size - 4;
            // XXX TODO guard overflow?
            // check divisibility by 4?
        }
        Ok(Self { major_brand, minor_brand, compatible_brands })
    }

    fn get_inner_values(&self) -> Vec<(&'static str, BoxValue)> {
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

    fn get_contained_boxes(&self) -> Option<Vec<(&BoxInfo, Option<&Box<dyn IsoBoxEntry>>)>> {
        None
    }
}
