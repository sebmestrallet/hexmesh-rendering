pub struct WireframeMesh {
    pub vertices: Vec<[f32; 3]>,
    pub edges: Vec<u32>
}

impl WireframeMesh {
    pub fn new() -> WireframeMesh {
        WireframeMesh { vertices: Vec::new(), edges: Vec::new() }
    }
}