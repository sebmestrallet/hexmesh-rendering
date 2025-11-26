use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

pub struct TriangleMesh {
    pub positions: Vec<[f32;3]>, // 3d coordinates of vertices
    pub uv: Vec<[f32;2]>, // 2d texture coordinates of vertices
    pub indices: Vec<u32>, // triangle definitions = vertex indices /!\ 0-based indices
    pub edges: HashSet<[u32;2]>, // edges definitions = vertex indices /!\ 0-based indices
}

impl TriangleMesh {
    pub fn new() -> TriangleMesh {
        TriangleMesh { positions: Vec::new(), uv: Vec::new(), indices: Vec::new(), edges: HashSet::new() }
    }

    pub fn write_obj(&self, file_name: &str) {
        // Wavefront .obj file
        // ---
        // v <x> <y> <z>
        // ...
        // usemtl Material_0
        // f <v0> <v1> <v2>
        // ...
        let mut file = File::create(file_name).unwrap();
        for vertex_index in 0..self.positions.len() {
            let current_vertex = self.positions.get(vertex_index).unwrap();
            let _ = file.write_all(format!("v {} {} {}\n",current_vertex[0], current_vertex[1], current_vertex[2]).as_bytes());
        }
        let _ = file.write_all(b"usemtl Material_0\n");
        let mut v0: u32;
        let mut v1: u32;
        let mut v2: u32;
        assert!(self.indices.len() % 3 ==0);
        for triangle_index in 0..self.indices.len()/3 {
            v0 = *self.indices.get(triangle_index*3+0).unwrap();
            v1 = *self.indices.get(triangle_index*3+1).unwrap();
            v2 = *self.indices.get(triangle_index*3+2).unwrap();
            let _ = file.write_all(format!("f {} {} {}\n", v0+1, v1+1, v2+1).as_bytes()); // /!\ 0-based to 1-based indices
        }
        for edge in self.edges.iter() {
            let _ = file.write_all(format!("l {} {}\n", edge[0]+1, edge[1]+1).as_bytes()); // /!\ 0-based to 1-based indices
        }
        println!("{file_name} written");
    }

    pub fn sanity_check(&self) {
        assert!(self.indices.len() % 3 == 0);
        let nb_triangles = self.indices.len()/3;
        let nb_vertices = self.positions.len() as u32;
        for triangle_index in 0..nb_triangles {
            for local_vertex_index in 0..3 {
                let vertex_index = *self.indices.get(triangle_index*3+local_vertex_index).unwrap();
                if vertex_index >= nb_vertices {
                    panic!("In triangle n°{}/{} at the local vertex n°{}, vertex index is {}, but they are only {} vertices",triangle_index,nb_triangles,local_vertex_index,vertex_index,nb_vertices);
                }
            }
        }
        for edge in &self.edges {
            for vertex_index in edge {
                assert!(*vertex_index < nb_vertices);
            }
        }
        println!("TriangleMesh sanity_check() OK");
    }

    pub fn remove_isolated_vertices(&mut self) {
        assert!(!self.positions.is_empty());
        assert!(!self.indices.is_empty());

        let mut old_to_new_indices: HashMap<u32,u32> = HashMap::new(); // map an old vertex index (index in `self.positions`) to a new vertex index (index in `new_vertices`)
        let mut new_vertices: Vec<[f32;3]> = Vec::with_capacity(self.positions.len()); // at most, no vertex is removed, and `new_vertices` will have the same size as `self.positions` (the old vertices)
        assert!(self.indices.len() % 3 == 0);
        for triangle_index in 0..self.indices.len()/3 {
            for local_vertex_index in 0..3 {
                let vertex_index = *self.indices.get(triangle_index*3+local_vertex_index).unwrap();
                if let Some(new_vertex_index_ref) = old_to_new_indices.get(&vertex_index) {
                    // we already attributed a new index to this old index
                    old_to_new_indices.insert(vertex_index, *new_vertex_index_ref);
                }
                else {
                    // this index is referenced, we must keep it
                    let vertex_index_as_usize = vertex_index as usize;
                    let coordinates: &[f32; 3] = self.positions.get(vertex_index_as_usize).unwrap();
                    let index_of_new_vertex: u32 = TryInto::<u32>::try_into(new_vertices.len()).unwrap();
                    new_vertices.push(*coordinates);
                    old_to_new_indices.insert(vertex_index, index_of_new_vertex);
                }
            }
        }
        // also parse edges, in case some vertices are only referenced by edges
        for edge in &self.edges {
            for vertex_index in edge {
                if let Some(new_vertex_index) = old_to_new_indices.get(vertex_index) {
                    // we already attributed a new index to this old index
                    old_to_new_indices.insert(*vertex_index, *new_vertex_index);
                }
                else {
                    // this index is referenced, we must keep it
                    new_vertices.push(*self.positions.get(*vertex_index as usize).unwrap());
                    old_to_new_indices.insert(*vertex_index, new_vertices.len().try_into().unwrap());
                }
            }
        }
        // update vertex definitions = positions
        self.positions = new_vertices;
        // create uv
        self.uv = vec![[1.0,1.0];self.positions.len()];
        // update triangle definitions = vertex indices
        assert!(self.indices.len() % 3 == 0);
        for triangle_index in 0..self.indices.len()/3 {
            for local_vertex_index in 0..3 {
                let vertex_index = *self.indices.get(triangle_index*3+local_vertex_index).unwrap();
                let vertex_index_ref = self.indices.get_mut(triangle_index*3+local_vertex_index).unwrap();
                *vertex_index_ref = *old_to_new_indices.get(&vertex_index).unwrap();
                assert!(*vertex_index_ref < self.positions.len().try_into().unwrap());
            }
        }
        // update edges definitions = vertex indices
        let mut new_edges: HashSet<[u32;2]> = HashSet::new();
        for old_edge in &self.edges {
            new_edges.insert([
                *old_to_new_indices.get(&old_edge[0]).unwrap(),
                *old_to_new_indices.get(&old_edge[1]).unwrap(),
            ]);
        }
        self.edges = new_edges;
        println!("End of remove_isolated_vertices()");
    }
}