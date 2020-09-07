use std::io::{BufRead, Seek};

use super::error;
use super::box_reader::BoxReader;

#[derive(Debug)]
pub struct BoxInfo {
    pub offset: u64,
    pub size: u32,
    pub short_name: String,
}

#[derive(Copy, Clone, Debug)]
pub struct Flags(u32);
impl Flags {
    pub fn new(data: [u8; 3]) -> Self {
        Self(
            (data[0] as u32) << 16 |
            (data[1] as u32) << 8 |
            (data[2] as u32)
        )
    }

    pub fn read(reader: &mut BoxReader<impl BufRead>) -> Result<Self, std::io::Error> {
        Ok(Flags::new([ reader.read_u8()?, reader.read_u8()?, reader.read_u8()? ]))
    }

    pub fn has_flag(&self, flag: u32) -> bool {
        self.0 & flag > 0
    }
}

impl From<Flags> for u32 {
    fn from(val: Flags) -> u32 {
        val.0
    }
}

impl From<Flags> for [u8; 3] {
    fn from(val: Flags) -> [u8; 3] {
        [
            ((val.0 >> 16) & 0xFF) as u8,
            ((val.0 >> 8) & 0xFF) as u8,
            ((val.0) & 0xFF) as u8
        ]
    }
}

/// Enum defining in a generic way a box's value.
///
/// The ISOBMFF specification defines multiple type for properties.
///
/// `BoxValue` allows to classify them into a discrete number of types to be
/// able to have a coherent way of signaling/displaying similar types in
/// different boxes.
pub enum BoxValue<'a> {
    // Simple, less or equal to 64 bit, Copy, integer types
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Int32(i32),
    Int64(i64),
    Flags(Flags),

    // More complex types, linked to the struct lifetime

    // Slices-based
    UInt8Arr(&'a [u8]),
    // UInt16Arr(&'a [u16]),
    UInt32Arr(&'a [u32]),
    UInt64Arr(&'a [u64]),

    // Matrix
    Matrix3_3(&'a [u32; 9]),

    // Strings
    Utf8(&'a str),
    Utf8Arr(&'a [String]),

    // Fixed point floats (no IEEE754 in ISOBMFF)
    FixedPoint8(&'a [u8; 2]),
    FixedPoint16(&'a [u16; 2]),
    // FixedPoint32(&'a [u32; 2]),

    // Collection of multiple BoxValue elements put together, each named
    Collection(Vec<Vec<(&'a str, BoxValue<'a>)>>),
}

impl<'a> From<u8> for BoxValue<'a> {
    fn from(val: u8) -> Self {
        BoxValue::UInt8(val)
    }
}

impl<'a> From<u16> for BoxValue<'a> {
    fn from(val: u16) -> Self {
        BoxValue::UInt16(val)
    }
}

impl<'a> From<u32> for BoxValue<'a> {
    fn from(val: u32) -> Self {
        BoxValue::UInt32(val)
    }
}

impl<'a> From<u64> for BoxValue<'a> {
    fn from(val: u64) -> Self {
        BoxValue::UInt64(val)
    }
}

impl<'a> From<i32> for BoxValue<'a> {
    fn from(val: i32) -> Self {
        BoxValue::Int32(val)
    }
}

impl<'a> From<i64> for BoxValue<'a> {
    fn from(val: i64) -> Self {
        BoxValue::Int64(val)
    }
}

impl<'a> From<Flags> for BoxValue<'a> {
    fn from(val: Flags) -> Self {
        BoxValue::Flags(val)
    }
}

impl<'a> From<&'a str> for BoxValue<'a> {
    fn from(val: &'a str) -> Self {
        BoxValue::Utf8(val)
    }
}

impl<'a> From<&'a [String]> for BoxValue<'a> {
    fn from(val: &'a [String]) -> Self {
        BoxValue::Utf8Arr(val)
    }
}

impl<'a> From<&'a [u8]> for BoxValue<'a> {
    fn from(val: &'a [u8]) -> Self {
        BoxValue::UInt8Arr(val)
    }
}

impl<'a> From<&'a [u32]> for BoxValue<'a> {
    fn from(val: &'a [u32]) -> Self {
        BoxValue::UInt32Arr(val)
    }
}

impl<'a> From<&'a [u64]> for BoxValue<'a> {
    fn from(val: &'a [u64]) -> Self {
        BoxValue::UInt64Arr(val)
    }
}

impl<'a> From<Vec<Vec<(&'a str, BoxValue<'a>)>>> for BoxValue<'a> {
    fn from(val: Vec<Vec<(&'a str, BoxValue<'a>)>>) -> Self {
        BoxValue::Collection(val)
    }
}

/// Trait for implementing ISOBMFF box parsers.
///
/// This is the trait you should implement on any new struct defining the parsing
/// logic for a given box.
/// This trait will allow to parse the box through its `parse` function as well
/// as obtain a readable content from it.
///
/// It also allows to define static methods to declare the name of that box in a
/// short form (the 4-letter variant) and long form (the full box name).
///
/// Any `IsoBoxParser` automatically implement the  `IsoBoxEntry` trait, which
/// can be used on more "exotic" case, e.g. when defining an enum of possibly
/// contained boxes in an ISOBMFF box containing other boxes.
pub trait IsoBoxParser {
    fn parse<T: BufRead + Seek>(reader: &mut BoxReader<T>, box_size: u32)
        -> Result<Self, error::BoxParsingError> where Self: Sized;

    /// Returns the short 4-characters version of the box' name.
    fn get_short_name() -> &'static str where Self: Sized;

    /// Returns a long version of the box' name.
    fn get_long_name() -> &'static str where Self: Sized;

    fn get_inner_values(&self) -> Vec<(&'static str, BoxValue)>;

    fn get_contained_boxes(&self) -> Option<Vec<(&BoxInfo, Option<&Box<dyn IsoBoxEntry>>)>>;
}

/// Trait for defining an ISOBMFF box.
///
/// This trait is used to obtain general information about a given ISOBMFF box.
///
/// When defining a new ISOBMFF parsing logic, it is recommended to implement
/// the `IsoBoxParser` trait instead. `IsoBoxEntry` automatically implements
/// the `IsoBoxEntry` trait.
///
/// Because the `IsoBoxEntry` trait rely on `&self` to know the current box's
/// name, it can be useful when the type of the box depend on the current state.
/// For example, it can be implemented on an enum of multiple possible ISOBMFF
/// boxes or on usized trait objects.
pub trait IsoBoxEntry {
    fn get_inner_values(&self) -> Vec<(&'static str, BoxValue)>;

    /// Returns the short 4-characters version of the box' name.
    fn get_short_name(&self) -> &'static str;

    /// Returns a long version of the box' name.
    fn get_long_name(&self) -> &'static str;

    fn get_contained_boxes(&self) -> Option<Vec<(&BoxInfo, Option<&Box<dyn IsoBoxEntry>>)>>;
}

impl<T: IsoBoxParser> IsoBoxEntry for T {
    fn get_short_name(&self) -> &'static str {
        T::get_short_name()
    }
    fn get_long_name(&self) -> &'static str {
        T::get_long_name()
    }
    fn get_inner_values(&self) -> Vec<(&'static str, BoxValue)> { self.get_inner_values() }

    fn get_contained_boxes(&self) -> Option<Vec<(&BoxInfo, Option<&Box<dyn IsoBoxEntry>>)>> {
        self.get_contained_boxes()
    }
}

pub trait IsobmffBox {
    fn parse<T: BufRead + Seek>(reader: &mut BoxReader<T>)
        -> Result<Self, error::BoxParsingError> where Self: Sized;

    /// Returns the short 4-characters version of the box' name.
    fn get_short_name() -> &'static str where Self: Sized;

    /// Returns a long version of the box' name.
    fn get_long_name() -> &'static str where Self: Sized;

    fn get_inner_value(&self) -> Vec<(&'static str, BoxValue)> {
        vec![]
    }

    fn get_contained_boxes(&self) -> Option<Vec<(BoxInfo, Option<&Box<dyn IsoBoxEntry>>)>> {
        None
    }
}
