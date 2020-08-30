use std::io::{BufRead, Seek};
use std::rc::Rc;
use super::{
    BoxParsingError,
    BoxReader,
    IsoBoxInfo,
    IsoBoxData,

    IsoBoxParser,
    IsoBoxEntry,

    free,
    ftyp,
    hdlr,
    mdat,
    mfhd,
    moof,
    moov,
    mvhd,
    pdin,
    saio,
    saiz,
    sidx,
    styp,
    subs,
    tfdt,
    tfhd,
    traf,
    trun,
};

/// Parse every box found from the current offset until `size_limit` is reached.
/// If `size_limit` is not set, it will parse every box until the end of the
/// file.
pub fn parse_children<T: BufRead + Seek>(
    reader: &mut BoxReader<T>,
    size_limit: Option<u64>,
    container_box_info: Option<&Rc<IsoBoxInfo>>
) -> Result<Vec<IsoBoxData>, BoxParsingError> {
    let mut contents = vec![];
    let mut size_limit_remaining : Option<u64> = None;

    if let Some(limit) = size_limit {
        size_limit_remaining = Some(limit);
        if limit < 4 {
            let pos_before = reader.get_pos()?;
            let parent_box_info = container_box_info.map(|i| Rc::clone(i));
            return Err(
                BoxParsingError::BoxTooSmall {
                    offset: pos_before,
                    short_name: None,
                    size: limit,
                    parent_box_info,
                });
        }
    }

    while !reader.is_empty()? &&
        size_limit_remaining.map_or(true, |x| x > 0)
    {
        let pos_before = reader.get_pos()?;
        let mut size = reader.read_u32()? as u64;

        let box_name = reader.read_str(4)?;

        let box_remaining_size = match size {
            0 => None,
            1 => {
                size = reader.read_u64()?;
                if size < 16 {
                    let short_name = Some(box_name);
                    let parent_box_info = container_box_info.map(|i| Rc::clone(i));
                    return Err(BoxParsingError::BoxTooSmall {
                        offset: pos_before,
                        short_name,
                        size,
                        parent_box_info,
                    });
                }
                Some(size - 16)
            },
            _ => {
                if size < 8 {
                    let short_name = Some(box_name);
                    let parent_box_info = container_box_info.map(|i| Rc::clone(i));
                    return Err(BoxParsingError::BoxTooSmall {
                        offset: pos_before,
                        short_name,
                        size,
                        parent_box_info,
                    });
                }
                Some(size - 8)
            }
        };

        let user_type: Option<[u8; 16]> = if box_name == "uuid" {
            let mut user_type_arr = [0u8; 16];
            for i in 0..user_type_arr.len() {
                user_type_arr[i] = reader.read_u8()?;
            }
            Some(user_type_arr)
        } else {
            None
        };

        let parent_box_info = container_box_info.as_ref().map(|info| {
            Rc::clone(info)
        });
        let box_info = Rc::new(IsoBoxInfo {
            size,
            short_name: box_name,
            user_type,
            offset: pos_before,
            parent_box_info,
        });

        if let Some(limit) = size_limit_remaining {
            if size > limit {
                return Err(BoxParsingError::BoxTooLarge {
                    expected_maximum: limit,
                    actual: size,
                    box_info: Rc::clone(&box_info),
                });
            }
        }

        // TODO HashMap implementation? This might need to define a Sized return
        // type for the `parse` functions instead of the `Self` they return today.
        let data: Option<Box<dyn IsoBoxEntry>> = match box_info.short_name.as_ref() {
            "free" =>
                Some(Box::new(free::Free::parse(reader, box_remaining_size, &box_info)?)),
            "ftyp" =>
                Some(Box::new(ftyp::Ftyp::parse(reader, box_remaining_size, &box_info)?)),
            "hdlr" =>
                Some(Box::new(hdlr::Hdlr::parse(reader, box_remaining_size, &box_info)?)),
            "mdat" =>
                Some(Box::new(mdat::Mdat::parse(reader, box_remaining_size, &box_info)?)),
            "mfhd" =>
                Some(Box::new(mfhd::Mfhd::parse(reader, box_remaining_size, &box_info)?)),
            "moof" =>
                Some(Box::new(moof::Moof::parse(reader, box_remaining_size, &box_info)?)),
            "moov" =>
                Some(Box::new(moov::Moov::parse(reader, box_remaining_size, &box_info)?)),
            "mvhd" =>
                Some(Box::new(mvhd::Mvhd::parse(reader, box_remaining_size, &box_info)?)),
            "pdin" =>
                Some(Box::new(pdin::Pdin::parse(reader, box_remaining_size, &box_info)?)),
            "saio" =>
                Some(Box::new(saio::Saio::parse(reader, box_remaining_size, &box_info)?)),
            "saiz" =>
                Some(Box::new(saiz::Saiz::parse(reader, box_remaining_size, &box_info)?)),
            "sidx" =>
                Some(Box::new(sidx::Sidx::parse(reader, box_remaining_size, &box_info)?)),
            "styp" =>
                Some(Box::new(styp::Styp::parse(reader, box_remaining_size, &box_info)?)),
            "subs" =>
                Some(Box::new(subs::Subs::parse(reader, box_remaining_size, &box_info)?)),
            "tfdt" =>
                Some(Box::new(tfdt::Tfdt::parse(reader, box_remaining_size, &box_info)?)),
            "tfhd" =>
                Some(Box::new(tfhd::Tfhd::parse(reader, box_remaining_size, &box_info)?)),
            "traf" =>
                Some(Box::new(traf::Traf::parse(reader, box_remaining_size, &box_info)?)),
            "trun" =>
                Some(Box::new(trun::Trun::parse(reader, box_remaining_size, &box_info)?)),
            _ => {
                if let Some(size_to_read) = box_remaining_size {
                    reader.skip_bytes(size_to_read as u64)?;
                } else {
                    reader.read_to_end()?;
                }
                None
            },
        };

        let pos_after = reader.get_pos()?;
        let expected_pos = pos_before + size as u64;
        if expected_pos != pos_after {
            if expected_pos < pos_after {
                return Err(BoxParsingError::ParserReadTooMuch {
                    actual: pos_after,
                    expected: expected_pos,
                    box_info: Rc::clone(&box_info),
                });
            } else {
                return Err(BoxParsingError::ParserReadNotEnough {
                    actual: pos_after,
                    expected: expected_pos,
                    box_info: Rc::clone(&box_info),
                });
            }
        }
        contents.push((box_info, data));
        if size == 0 {
            size_limit_remaining = Some(0);
        } else if let Some(limit) = size_limit_remaining {
            size_limit_remaining = Some(limit - size);
        }
    }
    Ok(contents)
}
