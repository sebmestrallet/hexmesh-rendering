use crate::hexmesh::*;
use crate::wavefront::*;
use crate::wgpu::*;

mod matrix;
mod vector;
mod hexmesh;
mod wavefront;
mod wgpu;

static INPUT_FILE: &str = "input.mesh";

fn main() {
    // let mut mesh: HexMesh = HexMesh::from_medit(INPUT_FILE);
    
    // mesh.compute_scaled_jacobian();
    // println!("Scaled Jacobians computed");

    // let triangles: Vec<(usize,usize,usize)> = mesh.triangulate_surface();

    // write_obj("surface.obj", &mesh.points, &triangles);

    let _ = run();
}
