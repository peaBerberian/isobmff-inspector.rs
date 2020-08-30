extern crate clap;

mod boxes;
mod rendering;

use std::fs::File;
use std::io::BufReader;

use clap::{Arg, App};
use boxes::parse_isobmff;
use rendering::{DisplayOptions, render_result};

fn main() {
    let matches = App::new("ISOBMFF-inspector")
        .version("0.1")
        .author("Paul Berberian <pea.berberian@gmail.com>")
        .about("Display metadata contained in an ISOBMFF file.")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("only-boxes")
            .short("b")
            .long("only-boxes")
            .value_name("BOX_NAME1,BOX_NAME2,...")
            .help(
                "Only display the content of the named box(es) (based on their \
                four letters short name, e.g. \"moof\").\n\
                You can specify multiple boxes by separating them with a comma \
                (without any spaces between the boxes' names.\n\
                For example, to show both the content from a \"tfdt\" box and \
                from a \"pssh\" box, you can set its value to \"tfdt,pssh\"")
            .takes_value(true))
        .arg(Arg::with_name("only-size")
            .short("s")
            .long("only-size")
            .help(
                "Only display the combined size (in bytes) of the selected boxes (all \
                top-level boxes by default, which should be equal to the length \
                of the file).\n\
                Can be combined with the \"-b\"/\"--only-boxes\" option to only \
                combine the size of given boxes.\n\
                If both a container box and an inner box pass the filter given through \
                the \"-b\" option, only the container box will be considered in \
                this calculation."
            ))
        .arg(Arg::with_name("show-all")
            .short("a")
            .long("show-all")
            .help(
                "Values corresponding to a collection of multiple entries are \
                hidden by default.\nThis option allows to display them."
            ))
        .get_matches();

    let file_name = matches.value_of("INPUT").unwrap();
    let only_boxes = if let Some(box_filter) = matches.value_of("only-boxes") {
        Some(box_filter
                .split(',')
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
    } else {
        None
    };
    let only_size = matches.is_present("only-size");
    let show_all = matches.is_present("show-all");

    let f = File::open(&file_name).unwrap_or_else(|err| {
        eprintln!("Error: Error while opening \"{}\": {}", &file_name, err);
        std::process::exit(1);
    });

    let rdr = BufReader::new(f);
    let result = parse_isobmff(rdr);
    match result {
        Err(e) => {
            eprintln!("Error: something went wrong when parsing the file: {}", e);
        },
        Ok(data) => {
            render_result(data, DisplayOptions {
                hide_collections: !show_all,
                filter_boxes: only_boxes,
                display_only_size: only_size,
            });
        }
    }
}
