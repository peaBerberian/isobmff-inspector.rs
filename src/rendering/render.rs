use super::super::boxes::{
    BoxValue,
    IsoBoxEntry,
    IsoBoxInfo,
};
use super::options::DisplayOptions;

pub fn render_box_data(
    box_info: &IsoBoxInfo,
    parsed_box: Option<&dyn IsoBoxEntry>,
    indentation_level: usize,
    opts: &DisplayOptions
) {
    let padding = "\t".repeat(indentation_level);
    display_box_title(box_info, &padding);
    match parsed_box {
        None => {
            println!("{}no data available yet on this box", padding);
        },
        Some(val) => {
            for value in val.get_inner_values_ref().iter() {
                display_inner_value(value, &padding, opts);
            }
            if let Some(contained) = val.get_inner_boxes_ref() {
                for parsed in contained.iter() {
                    println!();
                    render_box_data(
                        parsed.0,
                        parsed.1,
                        indentation_level + 1,
                        opts);
                }
            }
        }
    };
}

fn display_box_title(box_info: &IsoBoxInfo, padding: &str) {
    let IsoBoxInfo { short_name, size, offset, .. } = box_info;
    println!("{}\x1b[0;31m{}\x1b[0m (offset: {}, size: {})", padding, short_name, offset, size);
    println!("{}-------------------------------", padding);
}

fn display_inner_value(
    inner_value: &(&str, BoxValue),
    padding: &str,
    opts: &DisplayOptions
) {
    let value_to_string = stringify_box_value(&inner_value.1, &padding, opts);
    println!("{}\x1b[0;32m{}:\x1b[0m {}", padding, inner_value.0, value_to_string);
}

fn stringify_box_value(
    value: &BoxValue,
    multi_line_padding: &str,
    opts: &DisplayOptions
) -> String {
    match value {
        BoxValue::UInt8(x) => x.to_string(),
        BoxValue::UInt16(x) => x.to_string(),
        BoxValue::UInt32(x) => x.to_string(),
        BoxValue::UInt64(x) => x.to_string(),
        BoxValue::Int32(x) => x.to_string(),
        BoxValue::Int64(x) => x.to_string(),
        BoxValue::Flags(flags) => flags.to_hex_string(),
        BoxValue::Bool(val) => val.to_string(),

        BoxValue::UInt8Arr(arr) =>
            arr.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "),
        BoxValue::UInt32Arr(arr) =>
            arr.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "),
        BoxValue::UInt64Arr(arr) =>
            arr.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "),

        BoxValue::Utf8(st) => st.to_string(),

        BoxValue::Utf8Arr(stv) => stv.join(", "),

        BoxValue::FixedPoint8(arr) =>
            arr.iter().map(|val| val.to_string()).collect::<Vec<String>>().join("."),
        BoxValue::FixedPoint16(arr) =>
            arr.iter().map(|val| val.to_string()).collect::<Vec<String>>().join("."),

        BoxValue::Matrix3_3(m) => {
            use std::fmt::Write;
            let mut s = String::new();
            write!(&mut s, "\n{}\t{}\t{}\t{}\n{}\t{}\t{}\t{}\n{}\t{}\t{}\t{}",
                multi_line_padding,
                m[0], m[1], m[2],
                multi_line_padding,
                m[3], m[4], m[5],
                multi_line_padding,
                m[6], m[7], m[8])
                .expect("Unable to write hex string for flags.");
            s
        },

        BoxValue::Collection(col) => {
            use std::fmt::Write;
            if opts.hide_collections {
                return "Collapsed collection of values".to_string();
            }
            col.iter().map(|items| {
                items.iter().map(|item| {
                    // XXX TODO
                    let mut s = String::new();
                    let value_to_string = stringify_box_value(
                        &item.1,
                        &multi_line_padding,
                        opts);
                    write!(&mut s, "\n{}\t\x1b[0;32m{}:\x1b[0m {}", multi_line_padding, item.0, value_to_string).expect("Issue formatting Collection");
                    s
                }).collect::<Vec<String>>().join("")
            }).collect::<Vec<String>>().join("\n")
        },
    }
}
