use std::f32::MIN;
use std::fs;
use std::ops::{Div, Sub};

const INPUT_FILE: &str = "input.mesh";

// MEDIT .mesh format for hex-meshes
// /!\ in hexahedra definitions, vertex indices are 1-based
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

fn det2x2(a11: &f32, a12: &f32, a21: &f32, a22: &f32) -> f32 {
    a11*a22-a12*a21
}

#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}

fn dot(v0: &Vec3, v1: &Vec3) -> f32 {
    v0.x * v1.x + v0.y * v1.y + v0.z * v1.z
}

fn cross(v0: &Vec3, v1: &Vec3) -> Vec3 {
    Vec3 {
        x: det2x2(&v0.y, &v1.y, &v0.z, &v1.z),
        y: det2x2(&v0.z, &v1.z, &v0.x, &v1.x),
        z: det2x2(&v0.x, &v1.x, &v0.y, &v1.y)
    }
}

impl Vec3 {
    const ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

    fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x: x, y: y, z: z}
    }

    fn length(&self) -> f32 {
        return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
    }

    fn normalized(self) -> Vec3 {
        self / self.length()
    }

    fn dot(self, rhs: &Vec3) -> f32 {
        dot(&self,rhs)
    }

    fn cross(self, rhs: &Vec3) -> Vec3 {
        cross(&self,rhs)
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs}
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

// https://github.com/LIHPC-Computational-Geometry/validity-first-polycube-labeling/blob/main/include/geometry_hexahedra.h
// MEDIT convention (.mesh files)
//        5-------6
//       /|      /|
//      / |     / |
//     1-------2  |
//     |  4----|--7
//     | /     | /
//     |/      |/
//     0-------3
static HEX_CORNER_SPLITING: [[usize; 4]; 8] = [
    // corner index, then its 3 ordered neighbors
    [0,3,4,1],
    [1,5,2,0],
    [2,1,6,3],
    [3,7,0,2],
    [4,0,7,5],
    [5,6,1,4],
    [6,2,5,7],
    [7,4,3,6]
];

struct Hexahedra {
    vertices: [usize; 8], // 8 indices, for 8 vertices
}

impl Hexahedra {
    fn new(vertices: [usize; 8]) -> Hexahedra {
        Hexahedra { vertices: vertices }
    }
}

struct HexMesh {
    points: Vec<Vec3>,
    cells: Vec<Hexahedra>,
    scaled_jacobians: Option<Vec<f32>>
}

impl HexMesh {
    pub fn new() -> HexMesh {
        let points = Vec::new();
        let cells = Vec::new();
        HexMesh { points: points, cells: cells, scaled_jacobians: None }
    }

    fn compute_scaled_jacobian(&self) {
        let mut all_values: Vec<f32> = Vec::new();
        all_values.resize(self.cells.len(),0.0); // fill with 0.0, self.cells.len() times
        for hex_index in 0..self.cells.len() { // for each cell (each hexahedron)
            let mut scaled_jacobian: f32 = 1.0;
            for hex_corner in 0..8 { // for each of the 8 vertices of the current hexahedron
                let mut v: [Vec3; 4] = [Vec3::ZERO, Vec3::ZERO, Vec3::ZERO, Vec3::ZERO];
                for i in 0..4 { // [0] will be the current vertex, and [1:3] its 3 neighboring corners
                    let which_corner = HEX_CORNER_SPLITING[hex_corner][i];
                    let vertex_index = self.cells.get(hex_index).unwrap().vertices[which_corner];
                    v[i] = *self.points.get(vertex_index-1).unwrap_or_else(|| {
                            panic!("Cannot access `points` vec at {vertex_index}");
                        }
                    ); // get 3D coordinates of vertex at vertex_index
                }
                let n1: Vec3 = (v[1] - v[0]).normalized();
                let n2: Vec3 = (v[2] - v[0]).normalized();
                let n3: Vec3 = (v[3] - v[0]).normalized();
                scaled_jacobian = f32::min(scaled_jacobian,dot(&n3,&cross(&n1,&n2)));
            }
            *all_values.get_mut(hex_index).unwrap() = scaled_jacobian; // update vector
        }
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
                        let x: f32 = parts[0].to_string().parse::<f32>().unwrap();
                        let y: f32 = parts[1].to_string().parse::<f32>().unwrap();
                        let z: f32 = parts[2].to_string().parse::<f32>().unwrap();
                        mesh.points.push(Vec3{ x: x, y: y, z: z });
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
                        mesh.cells.push(Hexahedra::new([v0,v1,v2,v3,v4,v5,v6,v7]));
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
        mesh.compute_scaled_jacobian();
        println!("Scaled Jacobians computed");
    }
}
