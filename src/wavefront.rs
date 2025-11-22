use std::fs::File;
use std::io::prelude::*;
use crate::vector::*;

// Wavefront .obj file
// ---
// v <x> <y> <z>
// ...
// usemtl Material_0
// f <v0> <v1> <v2>
// ...

pub fn write_obj(file_name: &str, points: &Vec<Vec3>, triangles: &Vec<(usize,usize,usize)>) {
    let mut file = File::create(file_name).unwrap();
    for vertex_index in 0..points.len() {
        let current_vertex = points.get(vertex_index).unwrap();
        let _ = file.write_all(format!("v {} {} {}\n",current_vertex.x, current_vertex.y, current_vertex.z).as_bytes());
    }
    let _ = file.write_all(b"usemtl Material_0\n");
    for triangle_index in 0..triangles.len() {
        let current_triangle = triangles.get(triangle_index).unwrap();
        let _ = file.write_all(format!("f {} {} {}\n",current_triangle.0, current_triangle.1, current_triangle.2).as_bytes());
    }
    println!("{file_name} written");
}