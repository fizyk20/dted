use failure_derive::*;
use nom::*;
use std::convert::{AsRef, From};
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub struct DtedData {
    pub header: String,
    pub records: Vec<DtedRecord>,
}

pub struct DtedRecord {
    pub block_count: u32,
    pub lon_count: u16,
    pub lat_count: u16,
    pub elevations: Vec<i16>,
}

// convert signed magnitude int to i16
fn to_i16(x: u16) -> i16 {
    let sign_bit = 1u16 << 15;
    if x & sign_bit == sign_bit {
        -((x & !sign_bit) as i16)
    } else {
        x as i16
    }
}

named!(parse_dted_file <&[u8], DtedData>, do_parse!(
    header: take!(3428) >>
    records: count!(parse_record, 3600) >>
    (DtedData { header: String::from_utf8(header.to_vec()).unwrap(), records })
));

named!(parse_record <&[u8], DtedRecord>, do_parse!(
    tag!(&[0xaa][..]) >>
    block_byte0: take!(1) >>
    block_rest: u16!(Endianness::Big) >>
    lon_count: u16!(Endianness::Big) >>
    lat_count: u16!(Endianness::Big) >>
    elevations: count!(u16!(Endianness::Big), 3600) >>
    (DtedRecord {
        block_count: block_byte0[0] as u32 * 65536 + block_rest as u32,
        lon_count,
        lat_count,
        elevations: elevations.into_iter().map(to_i16).collect(),
    })
));

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "IO error: {}", _0)]
    Io(io::Error),
    #[fail(display = "Parse error: {}", _0)]
    ParseError(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl<I: Debug> From<nom::Err<I>> for Error {
    fn from(err: nom::Err<I>) -> Error {
        Error::ParseError(format!("{}", err))
    }
}

pub fn read_dted<P: AsRef<Path>>(path: P) -> Result<DtedData, Error> {
    let mut file = File::open(path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;

    let data = parse_dted_file(&content)?.1;
    Ok(data)
}
