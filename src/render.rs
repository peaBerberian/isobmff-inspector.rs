use super::boxes::{ BoxInfo, BoxValue, IsoBoxEntry };

pub fn render_result(result: Vec<(BoxInfo, Option<Box<dyn IsoBoxEntry>>)>) {
    for box_data in result.iter() {
        render_box_data(&box_data.0, box_data.1.as_ref(), 0);
    }
}

fn render_box_data(
    box_info: &BoxInfo,
    parsed_box: Option<&Box<dyn IsoBoxEntry>>,
    indentation_level: u32
) {
    let padding = "\t".repeat(indentation_level as usize);
    let BoxInfo { short_name, size, offset, .. } = box_info;
    println!("");
    println!("{}\x1b[0;31m{}\x1b[0m (offset: {}, size: {})", padding, short_name, offset, size);
    println!("{}-------------------------------", padding);
    match parsed_box {
        None => {
            println!("{}no data available yet on this box", padding);
        },
        Some(val) => {
            for value in val.get_inner_values().iter() {
                println!("{}\x1b[0;32m{}:\x1b[0m {}", padding, value.0, stringify_box_value(&value.1));
            }
            if let Some(contained) = val.get_contained_boxes() {
                for parsed in contained.iter() {
                    println!("");
                    render_box_data(parsed.0, parsed.1, indentation_level + 1);
                }
            }
        }
    };
    println!("");
}

fn stringify_box_value(value : &BoxValue) -> String {
    match value {
        BoxValue::UInt8(x) => x.to_string(),
        BoxValue::UInt16(x) => x.to_string(),
        BoxValue::UInt32(x) => x.to_string(),
        BoxValue::UInt64(x) => x.to_string(),
        BoxValue::Int32(x) => x.to_string(),
        BoxValue::Int64(x) => x.to_string(),
        BoxValue::Utf8(st) => st.to_string(),

        BoxValue::Utf8Arr(stv) => stv.join(", ").to_string(),

        BoxValue::FixedPoint8(arr) =>
            arr.iter().map(|val| val.to_string()).collect::<Vec<String>>().join("."),
        BoxValue::FixedPoint16(arr) =>
            arr.iter().map(|val| val.to_string()).collect::<Vec<String>>().join("."),

        _ => "".to_owned()
        // BoxValue::Flags(flags) => u32::From(flags).to_string(),
    }
}
