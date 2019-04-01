use crate::PartialEq;
use std::fmt;

/// # Point
/// Geographical Point (Point) contains latitude and longitiude coordinates.
/// * Latitude is an angle between position in north - south direction and Geographical Equator.
/// * Longitude is an angle between position in west - east direction and Prime Meridian Point.
///
/// ## Range
/// * Latitude can take values from -90 to 90 degrees.
/// * Longitude can take values from -180 to 180 degrees.
///
/// ## Constructing Point
/// let point = Point::new(12.11, 45_f64);
///
/// # Connection
/// Geographical Connection (Connection) between two given Geographical Points (Points);
/// Connection direction has no impact on cost calculation which is in this case haversian distance calculation.
/// ## Constructing Connection
/// let point_a = Point::new(1_f64, 16.0);
/// let point_b = Point::new(-12.0, -122.1);
/// let connection = Connection::new(point_a, point_b);
///
/// ## Calculating cost - haversian distance, Altitude changes on connection are not taken to
/// account. Cost calculation assumes that travel is on the same altitude which is geographical
/// radius of celestial body.
/// let earth_radus = get_radius_km(CelestialBody::Earth);
/// let cost_earth = connection.(earth_radius);
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
///

#[derive(Debug, Clone)]
pub struct Point {
    lat: f64,
    lng: f64,
}

impl Point {
    pub fn new(lat: f64, lng: f64) -> Self {
        Self {lat, lng}
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.lat == other.lat && self.lng == other.lng
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({}, {})", self.lat, self.lng)
    }
}

#[derive(Debug, Clone)]
pub struct Connection {
    pub start: Point,
    pub finish: Point,
}

impl Connection {
    pub fn new(start: Point, finish: Point) -> Self {
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

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.finish == other.finish
    }
}

impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Connection({}, {})", self.start, self.finish)
    }
}

#[cfg(test)]
mod test {
   use super::*;
   use crate::data::*;

   #[test]
   fn test_point() {
        let point_0 = Point::new(20.99, 10.12);
        let point_1 = Point::new(20_98_f64,10.12_f64);
        let point_2 = Point::new(20.99_f64, 10.12_f64);
        assert!(point_0 != point_1);
        assert!(point_0 == point_2);
   }

   #[test]
   fn test_connection() {
       let point_0 = Point::new(33.3386, 44.3939); // Bagdad
       let point_1 = Point::new(34.6937, 135.502); // Osaka
       let point_2 = Point::new(-36.8667, 174.767); // Warsaw
       let point_3 = Point::new(52.25, 21_f64); // Aucklan
       let point_4 = Point::new(13.75, 100.517); // Bangkok
       let point_5 = Point::new(55.7522, 37.6156); // Moscow
       let point_6 = Point::new(54.4167, 13.4333); // Bergen
       let point_7 = Point::new(54.35, 18.6667); // Gdansk
       let connection_0 = Connection::new(point_0, point_1);
       let connection_1 = Connection::new(point_2, point_3);
       let connection_2 = Connection::new(point_4, point_5);
       let connection_3 = Connection::new(point_6, point_7);
       // test Bagdad to Osaka
       let radius = get_radius_km(&CelestialObject::EARTH);
       let distance_b_o = connection_0.cost(radius);
       assert_eq!(8069, distance_b_o as u32);
       // test Warsaw to Auckland
       let distance_w_a = connection_1.cost(radius);
       assert_eq!(17349, distance_w_a as u32);
       // test Bangkok to Moscow
       let distance_b_m = connection_2.cost(radius);
       assert_eq!(7065, distance_b_m as u32);
       // test Gdansk to Bergen
       let distance_g_b = connection_3.cost(radius);
       assert_eq!(338, distance_g_b  as u32);
   }
}

