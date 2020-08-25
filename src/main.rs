mod boxes;
mod render;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: Missing file path in argument.");
        std::process::exit(1);
    }
    let f = std::fs::File::open(&args[1]).unwrap_or_else(|err| {
        eprintln!("Error: Error while opening {}: {}", &args[1], err);
        std::process::exit(1);
    });

    let rdr = std::io::BufReader::new(f);
    let result = boxes::parse_iosbmff(rdr);
    match result {
        Err(e) => {
            eprintln!("Error: something went wrong when parsing the file: {:?}", e);
        },
        Ok(data) => {
            render::render_result(data);
        }
    }
}
