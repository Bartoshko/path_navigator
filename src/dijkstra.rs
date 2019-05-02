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
            actual_index_from_parent = self.parents[&actual_index_from_parent].unwrap(); // all indexes are Some(_) as they are walked trough
            current_start_point = vertex.vector[actual_index_from_parent]
                .coordinates
                .clone();
            result.push( SphereConnection::new(current_start_point.clone(), current_end_point.clone()));
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

pub fn find_shortest_path(start: &SpherePoint, finish: &SpherePoint, vertex: &VertexBuffer) 
-> Option<Vec<SphereConnection>> {
    if start == finish {
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
