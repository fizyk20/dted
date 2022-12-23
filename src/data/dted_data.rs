use super::Angle;

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

pub struct DtedData {
    pub header: DtedHeader,
    pub records: Vec<DtedRecord>,
}

impl DtedData {
    pub fn lat_interval(&self) -> f64 {
        (self.header.lat_interval as f64) / 36000.0
    }

    pub fn lon_interval(&self) -> f64 {
        (self.header.lon_interval as f64) / 36000.0
    }

    pub fn min_lat(&self) -> f64 {
        self.header.origin_lat.into()
    }

    pub fn min_lon(&self) -> f64 {
        self.header.origin_lon.into()
    }

    pub fn max_lat(&self) -> f64 {
        let origin_lat: f64 = self.header.origin_lat.into();
        origin_lat + self.lat_interval() * (self.header.num_lat_lines - 1) as f64
    }

    pub fn max_lon(&self) -> f64 {
        let origin_lon: f64 = self.header.origin_lon.into();
        origin_lon + self.lon_interval() * (self.header.num_lon_lines - 1) as f64
    }

    pub fn get_elev<T: Into<f64>, U: Into<f64>>(&self, lat: T, lon: U) -> Option<f64> {
        let lat = lat.into();
        let lon = lon.into();
        if lat < self.min_lat()
            || lat > self.max_lat()
            || lon < self.min_lon()
            || lon > self.max_lon()
        {
            return None;
        }
        let lat = (lat - self.min_lat()) / self.lat_interval();
        let lon = (lon - self.min_lon()) / self.lon_interval();

        let mut lat_int = lat as usize;
        let mut lon_int = lon as usize;

        let mut lat_frac = lat - lat_int as f64;
        let mut lon_frac = lon - lon_int as f64;

        // handle the edge case of max lat/lon
        if lat_int == self.header.num_lat_lines as usize - 1 {
            lat_int -= 1;
            lat_frac += 1.0;
        }
        if lon_int == self.header.num_lon_lines as usize - 1 {
            lon_int -= 1;
            lon_frac += 1.0;
        }

        // get values to interpolate
        let elev00 = self.records[lon_int].elevations[lat_int] as f64;
        let elev01 = self.records[lon_int].elevations[lat_int + 1] as f64;
        let elev10 = self.records[lon_int + 1].elevations[lat_int] as f64;
        let elev11 = self.records[lon_int + 1].elevations[lat_int + 1] as f64;

        let result = elev00 * (1.0 - lon_frac) * (1.0 - lat_frac)
            + elev01 * (1.0 - lon_frac) * lat_frac
            + elev10 * lon_frac * (1.0 - lat_frac)
            + elev11 * lon_frac * lat_frac;

        Some(result)
    }
}
impl IntoIterator for DtedData {
    type Item = (f64, f64, Option<f64>);
    type IntoIter = DtedDataIntoIterator;

    /// Returns an iterator over the (lat, long, elevation) of all points in DTED file.
    fn into_iter(self) -> Self::IntoIter {
        DtedDataIntoIterator {
            dted_data: self,
            lon_index: 0,
            lat_index: 0,
        }
    }
}
pub struct DtedDataIntoIterator {
    dted_data: DtedData,
    lon_index: usize,
    lat_index: usize,
}
impl Iterator for DtedDataIntoIterator {
    type Item = (f64, f64, Option<f64>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.lon_index >= self.dted_data.header.num_lon_lines as usize {
            return None;
        }
        let lon = self.dted_data.min_lon() + self.lon_index as f64 * self.dted_data.lon_interval();
        let lat = self.dted_data.min_lat() + self.lat_index as f64 * self.dted_data.lat_interval();
        let elev = Some(self.dted_data.records[self.lon_index].elevations[self.lat_index] as f64);
        self.lat_index += 1;
        if self.lat_index >= self.dted_data.header.num_lat_lines as usize {
            self.lat_index = 0;
            self.lon_index += 1;
        }
        Some((lat, lon, elev))
    }
}
