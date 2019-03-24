use crate::PartialEq;
use crate::errors::*;

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

#[derive(Debug, Clone)]
pub struct Connection {
    start: Point,
    finish: Point,
}

impl Connection {
    pub fn new(start: Point, finish: Point) -> Self {
        Self {start, finish}
    }

    fn distance_cost(&self) -> f64 {
        // TODO: timplement distance cost calculation based on lat lng degrees
        // IDEA: take to account earth radius, and implement for other planets of solar system
        // this can be achieved using enum SpaceObject::EARTH, SpaceObject::MOON,
        // SpaceObject::URANUS,
        // Add altitude factor
        0_f64
    }
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.finish == other.finish
    }
}

#[derive(Debug, Clone)]
struct GraphRelation {
    vertex_index: usize,
    cost: f64,
}

impl GraphRelation {
    pub fn new(vertex_index: usize, cost: f64) -> Self {
        Self {vertex_index, cost}
    }
}

#[derive(Debug, Clone)]
struct VertexPoint {
    coordinates: Point,
    graphs: Vec<GraphRelation>,
}

impl VertexPoint {
    fn new(coordinates: Point) -> Self {
        let graphs = Vec::new();
        Self {coordinates, graphs}
    }

    fn push_relation(&mut self, graph_relation: GraphRelation) {
        self.graphs.push(graph_relation);
    }

    fn has_point(&self, other: &Point) -> bool {
        self.coordinates == *other
    }
}

impl PartialEq for VertexPoint {
    fn eq(&self, other: &Self) -> bool {
       self.coordinates == other.coordinates
    }
}

#[derive(Debug, Clone)]
pub struct VertexBuffer {
    vector: Vec<VertexPoint>
}

impl VertexBuffer {
   pub fn new(connections: Vec<Connection>) -> Result<Self> {
        let vector = Vec::new();
        let mut vertex_buffer = Self {vector};
        if !vertex_buffer.is_connections_vec_correct(&connections) {
            return Err(Error::from_kind(ErrorKind::DataItemIncorrect));
        }
        connections.iter().for_each(|conn| vertex_buffer.append(conn.clone()));
        Ok(vertex_buffer)
    }

    fn is_connections_vec_correct(&self, connections: &Vec<Connection>) -> bool {
        if connections.len() == 0 {
            return false;
        }
        for connection in connections {
            if connection.start == connection.finish {
                return false;
            }
        }
        true
    }

    fn append(&mut self, connection: Connection) {
        let start_index_option: Option<usize> = self
            .vector
            .iter()
            .position(|r| r.has_point(&connection.start));
        let end_index_option: Option<usize> = self
            .vector
            .iter()
            .position(|r| r.has_point(&connection.finish));
        let start_vertex_index = match start_index_option {
            Some(v) => v,
            None => self.add(connection.start.clone()),
        };
        let end_vertex_index = match end_index_option {
            Some(v) => v,
            None => self.add(connection.finish.clone()),
        };
        let cost: f64 = connection.distance_cost();
        &mut self.update(&start_vertex_index, &end_vertex_index, cost.clone());
        &mut self.update(&end_vertex_index, &start_vertex_index, cost.clone());
    }

    fn add(&mut self, coordinates: Point) -> usize {
        self.vector.push(VertexPoint::new(coordinates));
        self.vector.len() - 1
    }

    fn update(&mut self, index_to_update: &usize, index_related: &usize, cost: f64) {
        if self.vector[*index_to_update].graphs.iter()
            .position(|rel| rel.vertex_index == *index_related)
            .is_none() {
                &mut self.vector[*index_to_update].graphs
                .push(GraphRelation::new(*index_related, cost));
        }
    }
}

#[cfg(test)]
mod test {
   use super::*;

   #[test]
   fn test_point() {
        let point_0 = Point::new(20.99, 10.12);
        let point_1 = Point::new(20_98_f64,10.12_f64);
        let point_2 = Point::new(20.99_f64, 10.12_f64);
        assert!(point_0 != point_1);
        assert!(point_0 == point_2);
   }
}

