use super::super::boxes::{
    IsoBoxData,
    IsoBoxEntry,
    IsoBoxInfo,
};

/// Construct a new vector containing only the boxes in `results` which names
/// are contained in `display_only_boxes`.
pub fn filter_results<'results>(
    results: &'results [IsoBoxData],
    display_only_boxes: &[String]
) -> Vec<(&'results IsoBoxInfo, Option<&'results dyn IsoBoxEntry>)>  {
    let mut filtered_results: Vec<(
        &'results IsoBoxInfo,
        Option<&'results dyn IsoBoxEntry>
    )> = vec![];
    let results_iter = results.iter();
    for result in results_iter {
        if display_only_boxes.contains(&result.0.short_name) {
            let parsed_ref = result.1
                .as_ref()
                .map(|parsed| parsed.as_ref());
            filtered_results.push((&result.0, parsed_ref));
        } else if let Some(parsed) = &result.1 {
            add_filtered_inner_boxes(
                &mut filtered_results,
                parsed.as_ref(),
                display_only_boxes);
        }
    }
    filtered_results
}

/// If `container_box` contains inner boxes, add to the given
/// `filtered_results` vector only those whose named is contained in
/// `display_only_boxes`.
fn add_filtered_inner_boxes<'container>(
    filtered_results: &mut Vec<
        (&'container IsoBoxInfo,
         Option<&'container dyn IsoBoxEntry>)>,
    container: &'container dyn IsoBoxEntry,
    display_only_boxes: &[String]
) {
    if let Some(inner_boxes) = container.get_inner_boxes_ref() {
        let inner_boxes_iter = inner_boxes.iter();
        for inner_box in inner_boxes_iter {
            if display_only_boxes.contains(&inner_box.0.short_name) {
                filtered_results.push((&inner_box.0, inner_box.1));
            } else if let Some(parsed) = inner_box.1 {
                add_filtered_inner_boxes(
                    filtered_results,
                    parsed,
                    display_only_boxes);
            }
        }
    }
}
