use crate::vertex::*;
use crate::components::{SphereConnection, SpherePoint};
use crate::data::get_radius_km;
use std::f64::INFINITY;
use std::f64::MAX;
use std::collections::HashMap;

struct Dijkstra{
    costs: HashMap<usize, f64>,
    parents: HashMap<usize, Option<usize>>,
    start_index: usize,
    finish_index: usize,
    processed: Vec<usize>,
    cheapest_vertex_index: usize,
}

impl Dijkstra {
    pub fn new(start_index: usize, finish_index: usize) -> Self {
        let mut costs = HashMap::new();
        let mut processed = Vec::new();
        let mut parents = HashMap::new();
        costs.insert(start_index, 0.0_f64);
        costs.insert(finish_index, MAX);
        processed.push(start_index);
        parents.insert(finish_index, None);
        Self {
            costs: costs,
            parents: parents,
            start_index: start_index,
            finish_index: finish_index,
            processed: processed,
            cheapest_vertex_index: start_index,
        }
    }

    pub fn calculate_path(&mut self, vertex: &VertexBuffer) -> Vec<SphereConnection> {
        self.search_for_shortest_path_in_vertex(vertex);
        let mut result: Vec<SphereConnection> = Vec::new();
        let mut actual_index_from_parent: usize = self.finish_index;
        let mut current_start_point: SpherePoint;
        let mut current_end_point: SpherePoint = vertex.vector
            [self.finish_index]
            .coordinates
            .clone();
        while actual_index_from_parent != self.start_index {
            actual_index_from_parent = self.parents[&actual_index_from_parent].unwrap(); // all parent are Some(_) as they are walked trough
            current_start_point = vertex.vector[actual_index_from_parent]
                .coordinates
                .clone();
            result.push(SphereConnection::new(current_start_point.clone(), current_end_point.clone()));
            current_end_point = current_start_point.clone();
        }
        result.reverse();
        result
    }

    fn search_for_shortest_path_in_vertex(&mut self, vertex: &VertexBuffer) {
        while !self.processed.contains(&self.finish_index) {
            let mut vertex_index: usize;
            let iteration_max: usize = vertex.vector[self.cheapest_vertex_index]
                .graphs
                .len();
            for graph_index in 0..iteration_max {
                vertex_index = vertex.vector[self.cheapest_vertex_index].graphs[graph_index].vertex_index;
                if !self.processed.contains(&vertex_index) {
                    let _parent_cost: f64 = self.costs[&self.cheapest_vertex_index];
                    let _graph_cost: f64 = vertex.vector[self.cheapest_vertex_index].graphs[graph_index].cost;
                    let _child_cost: f64 = _parent_cost + _graph_cost;
                    if self.costs.contains_key(&vertex_index) {
                        if self.costs[&vertex_index] > _child_cost {
                            *self.costs.get_mut(&vertex_index).unwrap() = _child_cost;
                            *self.parents.get_mut(&vertex_index).unwrap() = Some(self.cheapest_vertex_index);
                        }
                    } else {
                        self.costs.insert(vertex_index, _child_cost);
                        self.parents.insert(vertex_index, Some(self.cheapest_vertex_index));
                    }
                }
            }
            let mut min_cost = std::f64::MAX;
            let mut min_value_index: Option<usize> = None;
            for (k, v) in &self.costs {
                if !self.processed.contains(k) {
                    if min_cost > *v {
                        min_cost = *v;
                        min_value_index = Some(*k);
                    }
                }
            }
            if let Some(x) = min_value_index {
                self.cheapest_vertex_index = x;
                self.processed.push(self.cheapest_vertex_index);
            }
        }
    }
}

/// Returns Vec<SphereConnection> which is the shortest path between two given points.SphereConnection
/// 
/// # Arguments:
/// * `start` which is &SpherePoint - start sphere point representation on given geomentry
/// * `finish` which is &SpherePoint - finish sphere point representation on given geomentry
/// * `vertex` which is &VertexBuffer - precalculated certex for avaliable paths on given geometry
/// 
/// # Remarks:
/// 
/// This function finds the closest point on precalculated &VertexBuffer to given start and finish points
/// and starts shortest path calcualtion from this points.
///

pub fn find_shortest_path(start: &SpherePoint, finish: &SpherePoint, vertex: &VertexBuffer) 
-> Option<Vec<SphereConnection>> {
    if start == finish || vertex.vector.len() == 0 {
        return None;
    }
    let start_index: usize = get_closest_point(&start, &vertex);
    let finish_index: usize = get_closest_point(&finish, &vertex);
    if start_index == finish_index {
        return None;
    }
    let mut dijkstra = Dijkstra::new(start_index, finish_index);
    Some(dijkstra.calculate_path(vertex))
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

#[cfg(test)]
mod djikstra_tests {
    use super::*;
    use crate::vertex::VertexBuffer;
    use crate::components::{SphereConnection, SpherePoint};
    use crate::data::CelestialObject;

    #[test]
    fn test_shortest_path_calculations() {
        // given:
        let mut paths: Vec<SphereConnection> = Vec::new();
        let mut known_shortest_path: Vec<SphereConnection> = Vec::new();
        // calculate left arm
        let mut point_a: SpherePoint;
        let mut point_b: SpherePoint = SpherePoint::new(0.0, 0.0);
        for i in 0..10 {
            if i > 0 {
                point_a = point_b.clone();
                point_b = SpherePoint::new(i as f64, 0.0);
                paths.push(SphereConnection::new(point_a.clone(), point_b.clone()))
            } else {
                point_b = SpherePoint::new(i as f64 * 0.0, i as f64 * 0.0);
            }
        }
        paths.push(SphereConnection::new(point_b, SpherePoint::new(10.0, 10.0)));

        // calculate right arm
        point_b = SpherePoint::new(0.0, 0.0);
        for i in 0..10 {
            if i > 0 {
                point_a = point_b.clone();
                point_b = SpherePoint::new(0.0, i as f64);
                paths.push(SphereConnection::new(point_a.clone(), point_b.clone()))
            } else {
                point_b = SpherePoint::new(i as f64 * 0.0, i as f64 * 0.0);
            }
        }
        paths.push(SphereConnection::new(point_b, SpherePoint::new(10.0, 10.0)));
        
        // calculate diagonal
        point_b = SpherePoint::new(0.0, 0.0);
        for i in 0..11 {
            if i > 0 {
                point_a = point_b.clone();
                point_b = SpherePoint::new(i as f64, i as f64);
                paths.push(SphereConnection::new(point_a.clone(), point_b.clone()));
                known_shortest_path.push(SphereConnection::new(point_a.clone(), point_b.clone()));
            } else {
                point_b = SpherePoint::new(i as f64 * 0.0, i as f64 * 0.0);
            }
        }
        
        // when:
        let vertex = VertexBuffer::new(paths, CelestialObject::MERCURY).unwrap();

        // then:
        let shortest_path = find_shortest_path(&SpherePoint::new(0.0, 0.0), &SpherePoint::new(10.0, 10.0), &vertex).unwrap();
        let mut calc_cost = 0.0_f64;
        let mut known_cost = 0.0_f64;
        let radius = get_radius_km(&CelestialObject::MERCURY);
        for i in 0..10 {
            let connection_calculated = shortest_path[i].clone();
            let conenction_known = known_shortest_path[i].clone();
            calc_cost += connection_calculated.cost(radius);
            known_cost += conenction_known.cost(radius);
        }
        relative_eq!(calc_cost, known_cost);
    }

    #[test]
    fn test_shortest_path_not_possible_to_find() {
        //  when:
        let mut path: Vec<SphereConnection> = Vec::new();
        path.push(SphereConnection::new(SpherePoint::new(0.0, 0.0), SpherePoint::new(10.0, 10.0)));
        let vertex: VertexBuffer = VertexBuffer::new(path, CelestialObject::URANUS).unwrap();
        // given:
        let point: SpherePoint = SpherePoint::new(123.123, 456.123);
        let point_very_close: SpherePoint = SpherePoint::new(124.1, 456.1);

        // then:
        let is_path_calculated = match find_shortest_path(&point, &point, &vertex) {
            Some(_) => true,
            None => false
        };
        assert_eq!(is_path_calculated, false);
        let is_path_calculated = match find_shortest_path(&point, &point_very_close, &vertex) {
            Some(_) => true,
            None => false
        };
        assert_eq!(is_path_calculated, false);
    }
}
