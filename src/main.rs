use crate::hexmesh::*;
use bevy::{
    prelude::*,
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    asset::{RenderAssetUsages,Handle},
    render::{
        render_resource::PrimitiveTopology,
    },
    mesh::Indices,
    image::Image,
    color::palettes::basic::BLACK,
    prelude::Vec3
};
use bevy_panorbit_camera::{PanOrbitCamera,PanOrbitCameraPlugin};

use crate::trianglemesh::TriangleMesh;

mod matrix;
mod vector;
mod hexmesh;
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
    let mut res = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            mesh.positions
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            mesh.uv
        );
    res
}

fn create_lines_from(positions: Vec<[f32; 3]>,indices: Vec<u32>) -> Mesh {
    let mut res = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::default())
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            positions
        )
        .with_inserted_indices(Indices::U32(indices));
    res
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    
    let parula_texture_handle: Handle<Image> = asset_server.load("parula.png"); // width=32px, height=1px

    // https://bevy.org/examples/3d-rendering/texture/
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(parula_texture_handle.clone()),
        alpha_mode: AlphaMode::Opaque,
        unlit: true,
        ..default()
    });

    let mut mesh: HexMesh = HexMesh::from_medit(INPUT_FILE);
    
    mesh.compute_scaled_jacobian();
    println!("Scaled Jacobians computed");

    let (mut trianglemesh,per_triangle_scaled_jacobian): (TriangleMesh, Vec<f32>) = mesh.triangulate_surface();
    trianglemesh.sanity_check();

    trianglemesh.remove_isolated_vertices();
    trianglemesh.sanity_check();

    // trianglemesh.write_obj("surface.obj");

    let (wireframe_positions, wireframe_indices) = trianglemesh.create_wireframe_mesh();

    let bounding_box = trianglemesh.bounding_box();
    println!("Bounding box {:?}",bounding_box);

    trianglemesh.duplicate_vertices(&per_triangle_scaled_jacobian);

    commands
        .spawn((
            Mesh3d(meshes.add(create_mesh_from(trianglemesh))),
            MeshMaterial3d(material_handle)
        ))
        .insert(Transform::from_xyz(
            -(bounding_box[0].1-bounding_box[0].0) / 2.0,
            -(bounding_box[1].1-bounding_box[1].0) / 2.0,
            -(bounding_box[2].1-bounding_box[2].0) / 2.0,
        ));

    commands
        .spawn((
            Mesh3d(meshes.add(create_lines_from(wireframe_positions, wireframe_indices))),
            MeshMaterial3d(materials.add(StandardMaterial  {
                base_color: BLACK.into(),
                ..Default::default()
        })),
        ))
        .insert(Transform::from_xyz(
            -(bounding_box[0].1-bounding_box[0].0) / 2.0,
            -(bounding_box[1].1-bounding_box[1].0) / 2.0,
            -(bounding_box[2].1-bounding_box[2].0) / 2.0,
        ))
        .insert(Wireframe);

    commands.spawn(
            PanOrbitCamera::default()
        ).insert(Transform::from_xyz(
            10.0,
            10.0,
            10.0,
        )
        .looking_at(Vec3::new(
            0.0,
            0.0,
            0.0,
        ), Vec3::Y),
);
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(WireframeConfig {
            global: false,
            default_color: BLACK.into(),
        })
        .add_plugins((
            DefaultPlugins,
            WireframePlugin::default(),
            PanOrbitCameraPlugin
        ))
        .add_systems(Startup, startup)
        .run();
}
