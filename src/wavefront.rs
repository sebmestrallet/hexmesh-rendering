use std::fs::File;
use std::io::prelude::*;
use crate::trianglemesh::*;

// Wavefront .obj file
// ---
// v <x> <y> <z>
// ...
// usemtl Material_0
// f <v0> <v1> <v2>
// ...

pub fn write_obj(file_name: &str, mesh: &TriangleMesh) {
    let mut file = File::create(file_name).unwrap();
    for vertex_index in 0..mesh.positions.len() {
        let current_vertex = mesh.positions.get(vertex_index).unwrap();
        let _ = file.write_all(format!("v {} {} {}\n",current_vertex[0], current_vertex[1], current_vertex[2]).as_bytes());
    }
    let _ = file.write_all(b"usemtl Material_0\n");
    let mut v0: u32;
    let mut v1: u32;
    let mut v2: u32;
    assert!(mesh.indices.len() % 3 ==0);
    for triangle_index in 0..mesh.indices.len()/3 {
        v0 = *mesh.indices.get(triangle_index*3+0).unwrap();
        v1 = *mesh.indices.get(triangle_index*3+1).unwrap();
        v2 = *mesh.indices.get(triangle_index*3+2).unwrap();
        let _ = file.write_all(format!("f {} {} {}\n", v0, v1, v2).as_bytes()); // /!\ 1-based indices
    }
    for edge in mesh.edges.iter() {
        let _ = file.write_all(format!("l {} {}\n", edge[0], edge[1]).as_bytes()); // /!\ 1-based indices
    }
    println!("{file_name} written");
}