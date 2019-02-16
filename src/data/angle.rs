use std::convert::From;

pub struct Angle {
    pub deg: i16,
    pub min: u8,
    pub sec: u8,
}

impl From<Angle> for f32 {
    fn from(ang: Angle) -> f32 {
        let result = ang.deg.abs() as f32 + (ang.min as f32) / 60.0 + (ang.sec as f32) / 3600.0;
        if ang.deg < 0 {
            -result
        } else {
            result
        }
    }
}

impl From<Angle> for f64 {
    fn from(ang: Angle) -> f64 {
        let result = ang.deg.abs() as f64 + (ang.min as f64) / 60.0 + (ang.sec as f64) / 3600.0;
        if ang.deg < 0 {
            -result
        } else {
            result
        }
    }
}
