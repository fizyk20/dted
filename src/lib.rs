mod data;
mod parser;

pub use data::*;
pub use parser::{read_dted, read_dted_header};

#[test]
fn test_input_data() {
    let data = read_dted("test_data/test_data.dt2").unwrap();
    assert_eq!(data.header.origin_lat.deg, 42);
    assert_eq!(data.header.origin_lat.min, 0);
    assert_eq!(data.header.origin_lat.sec, 0);
    assert_eq!(data.header.origin_lon.deg, 15);
    assert_eq!(data.header.origin_lon.min, 0);
    assert_eq!(data.header.origin_lon.sec, 0);
    assert_eq!(data.header.lat_interval, 10);
    assert_eq!(data.header.lon_interval, 10);
    assert_eq!(data.header.num_lat_lines, 3601);
    assert_eq!(data.header.num_lon_lines, 3601);
}

#[test]
fn test_read_header_only() {
    let header = read_dted_header("test_data/test_data.dt2").unwrap();
    assert_eq!(header.origin_lat.deg, 42);
    assert_eq!(header.origin_lat.min, 0);
    assert_eq!(header.origin_lat.sec, 0);
    assert_eq!(header.origin_lon.deg, 15);
    assert_eq!(header.origin_lon.min, 0);
    assert_eq!(header.origin_lon.sec, 0);
    assert_eq!(header.lat_interval, 10);
    assert_eq!(header.lon_interval, 10);
    assert_eq!(header.num_lat_lines, 3601);
    assert_eq!(header.num_lon_lines, 3601);
}

#[test]
fn test_iterator() {
    let data_1 = read_dted("test_data/test_data.dt2").unwrap();
    let data_2 = read_dted("test_data/test_data.dt2").unwrap();
    for (lat, lon, elev) in data_1.into_iter() {
        // floating point errors in get_level so round
        let elev = elev.unwrap().round();
        let elev_get = data_2.get_elev(lat, lon).unwrap().round();
        assert_eq!(elev, elev_get);
    }
}
