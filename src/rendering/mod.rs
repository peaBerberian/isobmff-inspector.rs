mod filter;
mod options;
mod render;

pub use options::DisplayOptions;

use super::boxes::IsoBoxData;
use filter::filter_results;
use render::render_box_data;

pub fn render_result(results: Vec<IsoBoxData>, opts: DisplayOptions) {
    let mut is_initial_line = true;
    if let Some(boxes_to_display) = &opts.filter_boxes {
        let filtered_results = filter_results(&results, boxes_to_display);
        if opts.display_only_size {
            let combined_size = filtered_results
                .iter()
                .fold(0, |acc, box_data| acc + box_data.0.size);
            println!("{}", combined_size);
            return;
        }
        for result in filtered_results.iter() {
            if is_initial_line {
                is_initial_line = false;
            } else {
                println!(); // line break for subsequent boxes
            }

            render_box_data(result.0, result.1, 0, &opts);
        }
    } else {
        // TODO factorize size display code?
        if opts.display_only_size {
            let combined_size = results
                .iter()
                .fold(0, |acc, box_data| acc + box_data.0.size);
            println!("{}", combined_size);
            return;
        }
        for box_data in results.iter() {
            if is_initial_line {
                is_initial_line = false;
            } else {
                println!(); // line break for subsequent boxes
            }

            let parsed_ref = box_data.1
                .as_ref()
                .map(|boxed| std::boxed::Box::as_ref(&boxed));
            render_box_data(&box_data.0, parsed_ref, 0, &opts);
        }
    };
}
