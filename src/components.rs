use crate::PartialEq;
use std::fmt;

/// # SpherePoint
/// Geographical SpherePoint (SpherePoint) contains latitude and longitude coordinates.
/// * Latitude is an angle between position in north - south direction and Geographical Equator.
/// * Longitude is an angle between position in west - east direction and Prime Meridian SpherePoint.
///
/// ## Range
/// * Latitude can take values from -90 to 90 degrees.
/// * Longitude can take values from -180 to 180 degrees.
///
/// ## Constructing SpherePoint
/// ```
/// use path_navigator::components::SpherePoint;
/// let point = SpherePoint::new(12.11, 45_f64);
/// ```
/// # SphereConnection
/// Geographical SphereConnection (SphereConnection) between two given Geographical SpherePoints (SpherePoints);
/// SphereConnection direction has no impact on cost calculation which is in this case haversian distance calculation.
///
/// ## Constructing SphereConnection
/// ```
/// use path_navigator::components::{SpherePoint, SphereConnection};
/// let point_a = SpherePoint::new(1_f64, 16.0);
/// let point_b = SpherePoint::new(-12.0, -122.1);
/// let connection = SphereConnection::new(point_a, point_b);
/// ```
///
/// ## Calculating cost - haversian distance, Altitude differences on connection are not taken to
/// account. Cost calculation assumes that travel is on the same altitude which is geographical
/// radius of celestial body.
/// ```
/// use path_navigator::components::{SpherePoint, SphereConnection};
/// use path_navigator::data::*;
/// let point_a = SpherePoint::new(1_f64, 16.0);
/// let point_b = SpherePoint::new(-12.0, -122.1);
/// let connection = SphereConnection::new(point_a, point_b);
/// let earth_radius = get_radius_km(&CelestialObject::EARTH);
/// let cost_earth = connection.cost(earth_radius);
/// ```
///
/// ## Formula
/// hav_a = sin²(Δφ/2) + cos φ1 ⋅ cos φ2 ⋅ sin²(Δλ/2)
/// iverse_hav = 2 ⋅ atan2( √a, √(1−a) )
/// distance = R ⋅ iverse_hav
/// where:
/// φ is an angle from difference between point_0 and point_1 lat,
/// λ is an angel from difference between point_0 and point_1 lng,
/// φ1 is lat of point_0,
/// φ2 is lat of point_1,
/// hav_a is haversian of C,
/// inverse_hav is inversian haversian to central angle of C.
/// R is geographical radius of celestial body, for Earth it is 6371e3 in meters.
///
/// ## Links
/// * [Haversian Schema, Wikipedia](https://en.wikipedia.org/wiki/Haversine_formula#/media/File:Law-of-haversines.svg)
/// * [Haversian Formula, Wikipedia](https://en.wikipedia.org/wiki/Haversine_formula)

#[derive(Debug, Clone)]
pub struct SpherePoint {
    pub lat: f64,
    pub lng: f64,
}

impl SpherePoint {
    pub fn new(lat: f64, lng: f64) -> Self {
        Self {lat, lng}
    }
}

impl PartialEq for SpherePoint {
    fn eq(&self, other: &Self) -> bool {
        self.lat == other.lat && self.lng == other.lng
    }
}

impl fmt::Display for SpherePoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SpherePoint({}, {})", self.lat, self.lng)
    }
}

#[derive(Debug, Clone)]
pub struct SphereConnection {
    pub start: SpherePoint,
    pub finish: SpherePoint,
}

impl SphereConnection {
    pub fn new(start: SpherePoint, finish: SpherePoint) -> Self {
        Self {start, finish}
    }

    pub fn cost(&self, radius: f64) -> f64 {
        let fi = (self.finish.lat - self.start.lat).to_radians();
        let fi_1 = self.start.lat.to_radians();
        let fi_2 = self.finish.lat.to_radians();
        let lambda = (self.finish.lng - self.start.lng).to_radians();
        let a = (fi / 2_f64).sin().powi(2) + fi_1.cos() * fi_2.cos() * (lambda / 2_f64).sin().powi(2);
        let c = 2_f64 * a.sqrt().atan2((1_f64 - a).sqrt());
        radius * c
    }
}

impl PartialEq for SphereConnection {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.finish == other.finish
    }
}

impl fmt::Display for SphereConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SphereConnection({}, {})", self.start, self.finish)
    }
}

#[cfg(test)]
mod test {
   use super::*;
   use crate::data::*;

   #[test]
   fn test_point() {
        let point_0 = SpherePoint::new(20.99, 10.12);
        let point_1 = SpherePoint::new(20_98_f64,10.12_f64);
        let point_2 = SpherePoint::new(20.99_f64, 10.12_f64);
        assert!(point_0 != point_1);
        assert!(point_0 == point_2);
   }

   #[test]
   fn test_long_connection() {
        let point_0 = SpherePoint::new(33.3386, 44.3939); // Bagdad
        let point_1 = SpherePoint::new(34.6937, 135.502); // Osaka
        let point_2 = SpherePoint::new(-36.8667, 174.767); // Warsaw
        let point_3 = SpherePoint::new(52.25, 21_f64); // Aucklan
        let point_4 = SpherePoint::new(13.75, 100.517); // Bangkok
        let point_5 = SpherePoint::new(55.7522, 37.6156); // Moscow
        let point_6 = SpherePoint::new(54.4167, 13.4333); // Bergen
        let point_7 = SpherePoint::new(54.35, 18.6667); // Gdansk
        let point_8 = SpherePoint::new(43.000350, -75.499900); // New York
        let point_9 = SpherePoint::new(59.91273, 10.74609); // Oslo
        let connection_0 = SphereConnection::new(point_0, point_1);
        let connection_1 = SphereConnection::new(point_2, point_3);
        let connection_2 = SphereConnection::new(point_4, point_5);
        let connection_3 = SphereConnection::new(point_6, point_7);
        let connection_4 = SphereConnection::new(point_8, point_9);
        let radius = get_radius_km(&CelestialObject::EARTH);
        // test Bagdad to Osaka
        assert_eq!(8069, connection_0.cost(radius) as u32);
        // test Warsaw to Auckland;
        assert_eq!(17349, connection_1.cost(radius) as u32);
        // test Bangkok to Moscow
        assert_eq!(7065, connection_2.cost(radius) as u32);
        // test Gdansk to Bergen
        assert_eq!(338, connection_3.cost(radius)  as u32);
        // test New York to Oslo
        assert_eq!(5794, connection_4.cost(radius) as u32);
    }

   #[test]
   fn test_short_connection() {
       let point_0 = SpherePoint::new(54.424579, 18.595444);
       let point_1 = SpherePoint::new(54.426383, 18.592333);
       let short_connection = SphereConnection::new(point_0, point_1);
       let radius = get_radius_km(&CelestialObject::EARTH);
       assert_eq!(284, (short_connection.cost(radius) * 1000_f64) as u32);
   }
}

