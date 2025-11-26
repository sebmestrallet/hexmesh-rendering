use crate::hexmesh::*;
use crate::wavefront::*;
use bevy::{
    math::Vec3Swizzles,
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
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
use bevy_panorbit_camera::{PanOrbitCamera,PanOrbitCameraPlugin};
use crate::trianglemesh::TriangleMesh;

mod matrix;
mod vector;
mod hexmesh;
mod wavefront;
mod trianglemesh;

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
        // After defining all the vertices and their attributes, build each triangle using the
        // indices of the vertices that make it up in a counter-clockwise order.
        .with_inserted_indices(Indices::U32(vec![
            // First triangle
            0, 3, 1,
            // Second triangle
            1, 3, 2
        ]))
}

fn create_simple_pyramid() -> Mesh {
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                [0.0, 0.0, 0.0], // 0
                [1.0, 0.0, 0.0], // 1
                [1.0, 1.0, 0.0], // 2
                [0.0, 1.0, 0.0], // 3
                [0.5, 0.5, 1.0]  // 4
            ]
        )
        .with_inserted_indices(Indices::U32(vec![
            0,2,1, // 1/2 bottom
            0,3,2, // 1/2 bottom
            0,1,4,
            1,2,4,
            2,3,4,
            3,0,4
        ]))
}

fn create_mesh_from(mesh: TriangleMesh) -> Mesh {
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            mesh.positions.clone()
        )
        .with_inserted_indices(Indices::U32(mesh.indices))
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let mut mesh: HexMesh = HexMesh::from_medit(INPUT_FILE);
    
    mesh.compute_scaled_jacobian();
    println!("Scaled Jacobians computed");

    let mut trianglemesh = mesh.triangulate_surface();
    trianglemesh.sanity_check();

    // trianglemesh.remove_isolated_vertices();
    // trianglemesh.sanity_check();

    write_obj("surface.obj", &trianglemesh);

    commands
        .spawn((
            Mesh3d(meshes.add(create_mesh_from(trianglemesh))),
            MeshMaterial3d(materials.add(Color::from(SILVER)))
        ))
        .insert(Wireframe);

    commands.spawn((
        Transform::from_xyz(0.0, 7., 14.0)
            .looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        PanOrbitCamera::default(),
    ));
}

fn toggle_wireframe(
    mut wireframe_config: ResMut<WireframeConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins((
            DefaultPlugins,
            WireframePlugin::default(),
            PanOrbitCameraPlugin
        ))
        .add_systems(Startup, startup)
        .add_systems(
            Update,
            (
                toggle_wireframe,
            ),
        )

        .run();
}
