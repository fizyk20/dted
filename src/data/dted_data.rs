use super::Angle;

pub struct DtedData {
    pub header: DtedHeader,
    pub records: Vec<DtedRecord>,
}

pub struct DtedHeader {
    pub origin_lon: Angle,
    pub origin_lat: Angle,
    pub lon_interval: u16,
    pub lat_interval: u16,
    pub accuracy: Option<u16>,
    pub num_lon_lines: u16,
    pub num_lat_lines: u16,
}

pub struct DtedRecord {
    pub block_count: u32,
    pub lon_count: u16,
    pub lat_count: u16,
    pub elevations: Vec<i16>,
}
