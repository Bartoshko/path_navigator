use crate::vertex::*;
use crate::components::{SphereConnection, SpherePoint};
use crate::data::get_radius_km;
use std::f64::INFINITY;

pub fn find_shortest_path(start: &SpherePoint, finish: &SpherePoint, vertex: &VertexBuffer) 
-> Option<Vec<SphereConnection>> {
    let mut path: Vec<SphereConnection> = Vec::new();
    if start == finish {
        return None;
    }
    let start_index: usize = get_closest_point(&start, &vertex);
    let finish_index: usize = get_closest_point(&finish, &vertex);
    if start_index == finish_index {
        return None;
    }
    Some(path)
}

fn get_closest_point(point: &SpherePoint, vertex: &VertexBuffer) -> usize {
    let mut index: usize = 0;
    let mut distance: f64 = INFINITY;
    let radius = get_radius_km(&vertex.celestial_object);
    vertex.vector.iter().enumerate().for_each(|(i, sphere_point)| {
        let connection = SphereConnection::new(point.clone(), sphere_point.coordinates.clone());
        let local_distance = connection.cost(radius);
        if local_distance < distance {
            distance = local_distance;
            index = i;
        }
    });
    index
}