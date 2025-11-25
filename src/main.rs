use crate::hexmesh::*;
use crate::wavefront::*;
use bevy::{
    math::Vec3Swizzles,
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
    asset::RenderAssetUsages,
    render::{
        mesh::MeshRenderAssetPlugin,
        render_resource::{
            AsBindGroup, PrimitiveTopology
        },
    },
    mesh::Indices,
    color::palettes::basic::SILVER,
};

mod matrix;
mod vector;
mod hexmesh;
mod wavefront;

static INPUT_FILE: &str = "input.mesh";

// https://docs.rs/bevy/latest/bevy/mesh/struct.Mesh.html#manual-creation
fn create_simple_parallelogram() -> Mesh {
    // Create a new mesh using a triangle list topology, where each set of 3 vertices composes a triangle.
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        // Add 4 vertices, each with its own position attribute (coordinate in
        // 3D space), for each of the corners of the parallelogram.
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![[0.0, 0.0, 0.0], [1.0, 2.0, 0.0], [2.0, 2.0, 0.0], [1.0, 0.0, 0.0]]
        )
        // Assign a UV coordinate to each vertex.
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            vec![[0.0, 1.0], [0.5, 0.0], [1.0, 0.0], [0.5, 1.0]]
        )
        // Assign normals (everything points outwards)
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            vec![[0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]]
        )
        // After defining all the vertices and their attributes, build each triangle using the
        // indices of the vertices that make it up in a counter-clockwise order.
        .with_inserted_indices(Indices::U32(vec![
            // First triangle
            0, 3, 1,
            // Second triangle
            1, 3, 2
        ]))
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Mesh3d(meshes.add(create_simple_parallelogram())),
            MeshMaterial3d(materials.add(Color::from(SILVER)))
        ))
        .insert(Wireframe);

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 7., 14.0)
            .looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
    ));
}

fn main() {
    // let mut mesh: HexMesh = HexMesh::from_medit(INPUT_FILE);
    
    // mesh.compute_scaled_jacobian();
    // println!("Scaled Jacobians computed");

    // let triangles: Vec<(usize,usize,usize)> = mesh.triangulate_surface();

    // write_obj("surface.obj", &mesh.points, &triangles);

    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .run();
}
