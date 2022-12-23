# Rust-DTED

## What is it?

This crate implements utilities for reading [DTED (Digital Terrain Elevation Data)](https://www.dlr.de/eoc/Portaldata/60/Resources/dokumente/7_sat_miss/SRTM-XSAR-DEM-DTED-1.1.pdf) files.

## Example usage

Code reading a file and getting the terrain elevation at a given latitude and longitude would look like this:

```rust
use dted::read_dted;

let elev_data = read_dted("path/to/file.dt2").unwrap();
let elevation = elev_data.get_elev(latitude, longitude).unwrap(); // returns None if lat/lon are out of range
// Iterate over all data points
for (lat, lon, elev) in elev_data.into_iter() {
    println!("lat: {}, lon: {}, elev: {}", lat, lon, elev.unwrap());
}
```
