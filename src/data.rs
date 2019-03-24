#[derive(Debug, Clone)]
pub enum CelestialObject {
    MERCURY,
    VENUS,
    EARTH,
    MARS,
    JUPITER,
    SATURN,
    URANUS,
    NEPTUNE,
}

pub fn get_radius_km(celestial_object: CelestialObject) -> f64 {
    match celestial_object {
        CelestialObject::MERCURY => 2_439.7_f64,
        CelestialObject::VENUS => 6_051.8_f64,
        CelestialObject::EARTH => 6_371_f64,
        CelestialObject::MARS => 3_389.5_f64,
        CelestialObject::JUPITER => 69_911_f64,
        CelestialObject::SATURN => 58_232_f64,
        CelestialObject::URANUS => 25_362_f64,
        CelestialObject::NEPTUNE => 24_622_f64,
    }
}

