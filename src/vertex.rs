use crate::errors::*;
use crate::data::*;
use crate::components::*;

/// Vertex Buffer (VB).
/// Vertex Buffer stores nodes of each connection alongside with relation to other nodes and travel
/// cost.
/// Each node has minimum one connection with other node, and cost of this connection can be
/// calculated using haversine formula.
/// All nodes have their own vertex index in VertexBuffer and individual graph that represents indexes of nodes to which given
/// node is connected alongside with cost to reach this nodes.
/// In this way VertexBuffer stors vector of costs between all nodes and coordiante of each node.
/// VertexBuffer is used by DjikstraAlgorithm to calculate travel cost between any two nodes whithout need to go trough all nodes.
/// For more information go to DijkstraAlgorithm documentation.
///
/// # Theory
///
/// [Vertex](https://en.wikipedia.org/wiki/Vertex_(geometry))
///
/// # Example
///
/// ```
/// use path_navigator::components::*;
/// use path_navigator::vertex::*;
/// use path_navigator::data::*;
///
/// let connections: Vec<SphereConnection> = vec![SphereConnection::new(SpherePoint::new(0.00, 0.00),
/// SpherePoint::new(10.00, 24.00))];
/// let venus = CelestialObject::VENUS;
/// let vertex_buffer = VertexBuffer::new(connections, venus);
/// ```
///

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
struct VertexSpherePoint {
    coordinates: SpherePoint,
    graphs: Vec<GraphRelation>,
}

impl VertexSpherePoint {
    fn new(coordinates: SpherePoint) -> Self {
        let graphs = Vec::new();
        Self {coordinates, graphs}
    }

    fn has_point(&self, other: &SpherePoint) -> bool {
        self.coordinates == *other
    }
}

impl PartialEq for VertexSpherePoint {
    fn eq(&self, other: &Self) -> bool {
       self.coordinates == other.coordinates
    }
}

#[derive(Debug, Clone)]
pub struct VertexBuffer {
    celestial_object: CelestialObject,
    vector: Vec<VertexSpherePoint>,
}

impl VertexBuffer {
   pub fn new(connections: Vec<SphereConnection>, celestial_object: CelestialObject) -> Result<Self> {
        let vector = Vec::new();
        let mut vertex_buffer = Self {celestial_object, vector};
        if !vertex_buffer.is_connections_vec_correct(&connections) {
            return Err(Error::from_kind(ErrorKind::DataItemIncorrect));
        }
        connections.iter().for_each(|conn| vertex_buffer.append(conn.clone()));
        Ok(vertex_buffer)
    }

   fn len(&self) -> usize {
       self.vector.len()
   }

    fn is_connections_vec_correct(&self, connections: &Vec<SphereConnection>) -> bool {
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

    fn append(&mut self, connection: SphereConnection) {
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

    fn add(&mut self, coordinates: SpherePoint) -> usize {
        self.vector.push(VertexSpherePoint::new(coordinates));
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
    fn test_vertex_creation() {
        // given
        let mut connections_correct: Vec<SphereConnection> = Vec::new();
        let mut connections_incorrect: Vec<SphereConnection> = Vec::new();
        let first_point = SpherePoint::new(0.00, 0.00);
        let second_point = SpherePoint::new(1.0, 2.0);
        // when
        connections_correct.push(SphereConnection::new(first_point.clone(), second_point.clone()));
        connections_incorrect.push(SphereConnection::new(first_point.clone(), first_point.clone()));
        let vertex_buffer_correct = VertexBuffer::new(connections_correct, CelestialObject::SATURN);
        let vertex_buffer_incorrect = VertexBuffer::new(connections_incorrect, CelestialObject::SATURN);
        // then
        assert!(vertex_buffer_correct.is_ok());
        assert!(vertex_buffer_incorrect.is_err());
    }

    #[test]
    fn test_vertex_for_single_line() {
        // given
        let mut connections: Vec<SphereConnection> = Vec::new();
        let mut first_point = SpherePoint::new(0.00_f64, 0.00_f64);
        let mut second_point = SpherePoint::new(5.00_f64, 15.00_f64);
        for _ in 0..100 {
            let connection = SphereConnection::new(first_point.clone(), second_point.clone());
            connections.push(connection);
            first_point = second_point.clone();
            second_point.lat += 5.00_f64;
            second_point.lng += 15.00_f64;
        }
        // when
        let vertex_buffer = VertexBuffer::new(connections.clone(), CelestialObject::MARS);
        // then
        assert!(vertex_buffer.is_ok());
        assert_eq!(connections.len() + 1, vertex_buffer.unwrap().len());
    }
}

