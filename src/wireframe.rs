/// A set of vertices with edges (each connecting two vertices)
pub struct WireframeMesh {
    pub vertices: Vec<[f32; 3]>,
    pub edges: Vec<u32> // a even number of vertex indices. each group of 2 encodes an edge
}

impl WireframeMesh {
    pub fn new() -> WireframeMesh {
        WireframeMesh { vertices: Vec::new(), edges: Vec::new() }
    }
}