use bevy::{
    prelude::*,
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    asset::{RenderAssetUsages,Handle},
    render::render_resource::PrimitiveTopology,
    mesh::Indices,
    image::Image,
    color::palettes::basic::BLACK,
    prelude::Vec3
};
use bevy_panorbit_camera::{PanOrbitCamera,PanOrbitCameraPlugin};
use crate::hexmesh::*;
use crate::trianglemesh::TriangleMesh;
use crate::wireframe::WireframeMesh;

mod matrix;
mod vector;
mod hexmesh;
mod trianglemesh;
mod wireframe;

static INPUT_FILE: &str = "input.mesh";

fn create_mesh_from(mesh: TriangleMesh) -> Mesh {
    // https://docs.rs/bevy/latest/bevy/mesh/struct.Mesh.html#manual-creation
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            mesh.positions
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            mesh.uv
        )
}

fn create_lines_from(wireframe: WireframeMesh) -> Mesh {
    Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::default())
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            wireframe.vertices
        )
        .with_inserted_indices(
            Indices::U32(wireframe.edges)
        )
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

    let wireframe_mesh = trianglemesh.create_wireframe_mesh();

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
            Mesh3d(meshes.add(create_lines_from(wireframe_mesh))),
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
