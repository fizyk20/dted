use crate::{Angle, DtedData, DtedHeader, DtedRecord};
use failure_derive::*;
use nom::*;
use std::convert::{AsRef, From};
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

// convert signed magnitude int to i16
fn to_i16(x: u16) -> i16 {
    let sign_bit = 1u16 << 15;
    if x & sign_bit == sign_bit {
        -((x & !sign_bit) as i16)
    } else {
        x as i16
    }
}

fn bytes_to_num(bytes: &[u8]) -> u32 {
    let mut result = 0;
    for b in bytes {
        assert!(*b >= 0x30 && *b <= 0x39); // is a digit
        result *= 10;
        result += (*b - 0x30) as u32;
    }
    result
}

named!(parse_dted_file <&[u8], DtedData>, do_parse!(
    header: parse_dted_header >>
    take!(648 + 2700) >> // DSI + ACC
    records: count!(
        |input| parse_record(input, header.num_lat_lines as usize),
        header.num_lon_lines as usize) >>
    (DtedData { header, records })
));

named!(parse_dted_header <&[u8], DtedHeader>, do_parse!(
    tag!(b"UHL1") >>
    origin_lon: parse_angle >>
    origin_lat: parse_angle >>
    lon_interval: parse_u16_4char >>
    lat_interval: parse_u16_4char >>
    accuracy: alt!(
        is_na => { |_| None } |
        parse_u16_4char => { |num| Some(num) }) >>
    take!(15) >>
    num_lon_lines: parse_u16_4char >>
    num_lat_lines: parse_u16_4char >>
    take!(25) >>    // rest of UHL
    (DtedHeader {
        origin_lon,
        origin_lat,
        lon_interval,
        lat_interval,
        accuracy,
        num_lon_lines,
        num_lat_lines
    })
));

named!(parse_angle <&[u8], Angle>, do_parse!(
    deg: map!(take!(3), |chars| bytes_to_num(chars)) >>
    min: map!(take!(2), |chars| bytes_to_num(chars)) >>
    sec: map!(take!(2), |chars| bytes_to_num(chars)) >>
    sign: alt!(
        tag!(b"N") => { |_| 1i16 } |
        tag!(b"S") => { |_| -1i16 } |
        tag!(b"E") => { |_| 1i16 } |
        tag!(b"W") => { |_| -1i16 }
    ) >>
    (Angle {
        deg: deg as i16 * sign,
        min: min as u8,
        sec: sec as u8,
    })
));

named!(parse_u16_4char <&[u8], u16>,
    map!(take!(4), |chars| bytes_to_num(chars) as u16)
);

named!(is_na <&[u8], bool>,
    map!(take!(4), |chars| {
        chars[0] == b'N' && chars[1] == b'A'
    })
);

named_args!(parse_record(line_len: usize) <&[u8], DtedRecord>, do_parse!(
    tag!(&[0xaa][..]) >>
    block_byte0: take!(1) >>
    block_rest: u16!(Endianness::Big) >>
    lon_count: u16!(Endianness::Big) >>
    lat_count: u16!(Endianness::Big) >>
    elevations: count!(u16!(Endianness::Big), line_len) >>
    take!(4) >> // checksum
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

pub fn read_dted_header<P: AsRef<Path>>(path: P) -> Result<DtedHeader, Error> {
    let file = File::open(path)?;
    let mut content = Vec::new();
    file.take(80).read_to_end(&mut content)?;

    let data = parse_dted_header(&content)?.1;
    Ok(data)
}
