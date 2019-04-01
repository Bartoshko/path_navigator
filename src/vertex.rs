use crate::errors::*;
use crate::data::*;
use crate::connection::*;

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
    celestial_object: CelestialObject,
    vector: Vec<VertexPoint>,
}

impl VertexBuffer {
   pub fn new(connections: Vec<Connection>, celestial_object: CelestialObject) -> Result<Self> {
        let vector = Vec::new();
        let mut vertex_buffer = Self {celestial_object, vector};
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
        let radius = get_radius_km(&self.celestial_object);
        let cost: f64 = connection.cost(radius);
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


