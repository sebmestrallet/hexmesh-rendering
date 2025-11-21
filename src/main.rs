use std::fs;

const INPUT_FILE: &str = "input.mesh";

// MEDIT .mesh format for hex-meshes
// ---
// MeshVersionFormatted 2
// Dimension 3

// Vertices
// <nb>
// x y z 1
// ...
//
// Hexahedra
// <nb>
// v0 v1 v2 v3 v4 v5 v6 v7 1
// 
// End

#[derive(Debug, Clone, Copy)]
struct Point3D {
    x: f32,
    y: f32,
    z: f32
}

struct Hexahedra {
    //   MEDIT convention (.mesh files)
    //        5-------6
    //       /|      /|
    //      / |     / |
    //     1-------2  |
    //     |  4----|--7
    //     | /     | /
    //     |/      |/
    //     0-------3
    vertices: [usize; 8] // 8 indices, for 8 vertices
}

struct HexMesh {
    points: Vec<Point3D>,
    cells: Vec<Hexahedra>
}

impl HexMesh {
    pub fn new() -> HexMesh {
        let points = Vec::new();
        let cells = Vec::new();
        HexMesh { points: points, cells: cells }
    }
}


enum State {
    Header,
    VerticesNumber,
    Vertices,
    SectionTransition,
    CellsNumber,
    Cells,
    ParsingFinised
}

fn main() {
    let mut state: State = State::Header;
    let mut mesh: HexMesh = HexMesh::new();
    if let Ok(whole_file) = fs::read_to_string(INPUT_FILE) {
        let lines = whole_file.split("\n");
        for line in lines {
            match state {
                State::Header => {
                    if line == "Vertices" {
                        state = State::VerticesNumber;
                    }
                    // else: ignore this line
                    continue;
                },
                State::VerticesNumber => {
                    let nb_vertices: usize = line.to_string().parse::<usize>().unwrap();
                    mesh.points.reserve(nb_vertices); // preallocation
                    println!("Found nb_vertices = {nb_vertices}");
                    state = State::Vertices;
                    continue;
                },
                State::Vertices => {
                    if line == "" {
                        state = State::SectionTransition;
                    }
                    else {
                        let parts: Vec<&str> = line.split(" ").collect::<Vec<&str>>();
                        assert!(parts.len() == 4); // 4 parts: x, y, z and the value 1
                        // let current_point: &mut Point3D = mesh.points.get_mut(vertices_counter).unwrap();
                        let x: f32 = parts[0].to_string().parse::<f32>().unwrap();
                        let y: f32 = parts[1].to_string().parse::<f32>().unwrap();
                        let z: f32 = parts[2].to_string().parse::<f32>().unwrap();
                        mesh.points.push(Point3D{ x: x, y: y, z: z });
                    }
                    continue;
                },
                State::SectionTransition => {
                    if line == "Hexahedra" {
                        state = State::CellsNumber;
                    }
                    // else: ignore this line
                    continue;
                },
                State::CellsNumber => {
                    let nb_cells: usize = line.to_string().parse::<usize>().unwrap();
                    mesh.cells.reserve(nb_cells); // preallocation
                    println!("Found nb_cells = {nb_cells}");
                    state = State::Cells;
                    continue;
                },
                State::Cells => {
                    if line == "" {
                        state = State::ParsingFinised;
                    }
                    else {
                        let parts: Vec<&str> = line.split(" ").collect::<Vec<&str>>();
                        assert!(parts.len() == 9); // 9 parts: v0 to v7 and the value 1
                        let v0: usize = parts[0].to_string().parse::<usize>().unwrap();
                        let v1: usize = parts[1].to_string().parse::<usize>().unwrap();
                        let v2: usize = parts[2].to_string().parse::<usize>().unwrap();
                        let v3: usize = parts[3].to_string().parse::<usize>().unwrap();
                        let v4: usize = parts[4].to_string().parse::<usize>().unwrap();
                        let v5: usize = parts[5].to_string().parse::<usize>().unwrap();
                        let v6: usize = parts[6].to_string().parse::<usize>().unwrap();
                        let v7: usize = parts[7].to_string().parse::<usize>().unwrap();
                        mesh.cells.push(Hexahedra { vertices: [v0,v1,v2,v3,v4,v5,v6,v7] });
                    }
                    continue;
                },
                State::ParsingFinised => {
                    break;
                }
            }
        }
        println!("mesh.points.len() = {}", mesh.points.len());
        println!("mesh.cells.len() = {}", mesh.cells.len());
    }
}
