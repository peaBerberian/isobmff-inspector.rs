use std::io::{BufRead, Seek};
use super::{
    BoxParsingError,
    BoxReader,
    BoxInfo,

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
    styp,
    subs,
    tfdt,
    tfhd,
    traf,
    trun,
};

pub fn parse_children<T: BufRead + Seek>(
    reader: &mut BoxReader<T>,
    size_limit: Option<u32>
) -> Result<Vec<(BoxInfo, Option<Box<dyn IsoBoxEntry>>)>, BoxParsingError> {
    let mut contents = vec![];
    let mut size_limit_remaining : Option<u32> = None;
    if let Some(limit) = size_limit {
        size_limit_remaining = Some(limit);
        if limit < 4 {
            panic!("NOPE {}", limit); // XXX TODO
        }
    }

    while !reader.is_empty()? &&
        size_limit_remaining.map_or(true, |x| x > 0)
    {
        let pos_before = reader.get_pos()?;
        // XXX TODO better size computing
        // XXX TODO UUID
        let size = reader.read_u32()?;
        if size_limit_remaining.map_or(false, |limit| size > limit) {
            panic!("ARF {:?} {}", size_limit_remaining, size); // XXX TODO
        }

        let box_name = reader.read_str(4)?;
        let box_info = BoxInfo { size, short_name: box_name, offset: pos_before };
        let box_content_size = size - 8;

        // TODO HashMap implementation?
        let data: Option<Box<dyn IsoBoxEntry>> = match box_info.short_name.as_ref() {
            "free" => Some(Box::new(free::Free::parse(reader, box_content_size)?)),
            "ftyp" => Some(Box::new(ftyp::Ftyp::parse(reader, box_content_size)?)),
            "hdlr" => Some(Box::new(hdlr::Hdlr::parse(reader, box_content_size)?)),
            "mdat" => Some(Box::new(mdat::Mdat::parse(reader, box_content_size)?)),
            "mfhd" => Some(Box::new(mfhd::Mfhd::parse(reader, box_content_size)?)),
            "moof" => Some(Box::new(moof::Moof::parse(reader, box_content_size)?)),
            "moov" => Some(Box::new(moov::Moov::parse(reader, box_content_size)?)),
            "mvhd" => Some(Box::new(mvhd::Mvhd::parse(reader, box_content_size)?)),
            "pdin" => Some(Box::new(pdin::Pdin::parse(reader, box_content_size)?)),
            "saio" => Some(Box::new(saio::Saio::parse(reader, box_content_size)?)),
            "saiz" => Some(Box::new(saiz::Saiz::parse(reader, box_content_size)?)),
            "styp" => Some(Box::new(styp::Styp::parse(reader, box_content_size)?)),
            "subs" => Some(Box::new(subs::Subs::parse(reader, box_content_size)?)),
            "tfdt" => Some(Box::new(tfdt::Tfdt::parse(reader, box_content_size)?)),
            "tfhd" => Some(Box::new(tfhd::Tfhd::parse(reader, box_content_size)?)),
            "traf" => Some(Box::new(traf::Traf::parse(reader, box_content_size)?)),
            "trun" => Some(Box::new(trun::Trun::parse(reader, box_content_size)?)),
            _ => {
                reader.skip_bytes(box_content_size as u64)?;
                None
            },
        };

        let pos_after = reader.get_pos()?;
        let expected_pos = pos_before + size as u64;
        if expected_pos != pos_after {
            if expected_pos < pos_after {
                panic!("read too much"); // XXX TODO
            } else {
                let diff = expected_pos - pos_after;
                eprintln!("read not enough {} {}", diff, box_info.short_name); // XXX TODO
                reader.skip_bytes(diff)?;
            }
        }
        contents.push((box_info, data));
        if let Some(limit) = size_limit_remaining {
            size_limit_remaining = Some(limit - size);
        }
    }
    if size_limit_remaining.map_or(false, |l| l > 0) {
        panic!(); // XXX TODO
    }
    Ok(contents)
}
