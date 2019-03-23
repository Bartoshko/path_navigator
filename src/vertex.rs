use crate::PartialEq;

#[derive(Debug, Clone)]
pub struct Point {
    lat: f64,
    lng: f64,
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

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.finish == other.finish
    }
}

impl Connection {
    fn new(start: Point, finish: Point) -> Self {
        Self {start, finish}
    }
}

#[derive(Debug, Clone)]
struct GraphRelation {
    vertex_index: usize,
    cost: f64,
}

#[derive(Debug, Clone)]
struct VertexPoint {
    coordinates: Point,
    graphs: Vec<GraphRelation>,
}

impl PartialEq for VertexPoint {
    fn eq(&self, other: &Self) -> bool {
       self.coordinates == other.coordinates
   }
}

impl VertexPoint {
    fn new(coordinates: Point) -> Self {
        let graphs = Vec::new();
        Self {coordinates, graphs}
    }

    fn push_relation(&mut self, graph_relation: GraphRelation) {
        self.graphs.push(graph_relation);
    }
}

pub struct VertexBuffer {
    vector: Vec<VertexPoint>
}

impl VertexBuffer {
   fn new(connections: Vec<Connection>) -> Self {
       // TODO: add coonections to vector
       let vector = Vec::new();
        Self {vector}
    }
}



//    fn append_to_vertex_matrix(&mut self, line: Line) {
//        let start_vertex_index_option: Option<usize> = self
//            .dijkstra_vertex_matrix
//            .iter()
//            .position(|r| r.is_equal(&line.start));
//        let end_vertex_index_option: Option<usize> = self
//            .dijkstra_vertex_matrix
//            .iter()
//            .position(|r| r.is_equal(&line.finish));
//        let start_vertex_index: i32 = match start_vertex_index_option {
//            Some(v) => v as i32,
//            None => self.add_new_vertex(line.start.copy()),
//        };
//        let end_vertex_index: i32 = match end_vertex_index_option {
//            Some(v) => v as i32,
//            None => self.add_new_vertex(line.finish.copy()),
//        };
//        let cost: f64 = line.length();
//        &mut self.update_vertex_matrix(&start_vertex_index, &end_vertex_index, &cost);
//        &mut self.update_vertex_matrix(&end_vertex_index, &start_vertex_index, &cost);
//}
