use std::fs;
use std::collections::HashMap;
use crate::vector::*;
use crate::trianglemesh::TriangleMesh;

pub struct Hexahedron {
    pub vertices: [usize; 8], // 8 indices, for 8 vertices /!\ 0-based indices
}

impl Hexahedron {
    pub fn new(vertices: [usize; 8]) -> Hexahedron {
        Hexahedron { vertices: vertices }
    }

    pub const CORNER_SPLITING: [[usize; 4]; 8] = [
        // corner index, then its 3 ordered neighbors
        // MEDIT convention (.mesh files)
        // https://github.com/LIHPC-Computational-Geometry/validity-first-polycube-labeling/blob/main/include/geometry_hexahedra.h
        //        5-------6
        //       /|      /|
        //      / |     / |
        //     1-------2  |
        //     |  4----|--7
        //     | /     | /
        //     |/      |/
        //     0-------3
        [0,3,4,1],
        [1,5,2,0],
        [2,1,6,3],
        [3,7,0,2],
        [4,0,7,5],
        [5,6,1,4],
        [6,2,5,7],
        [7,4,3,6]
    ];

    pub const FACET_SPLITTING: [[usize; 5]; 6] = [
        // facet index, then its 4 ordered vertices
        // facets ordering: no particular order
        // vertices ordering: clockwise order when facet seen from outside the cell
        [0,0,1,2,3], // front
        [1,3,2,6,7], // right
        [2,7,6,5,4], // back
        [3,4,5,1,0], // left
        [4,4,0,3,7], // bottom
        [5,1,5,6,2], // tom
    ];
}

pub struct HexMesh {
    pub points: Vec<Vec3>,
    pub cells: Vec<Hexahedron>,
    pub scaled_jacobians: Option<Vec<f32>>,
    pub cell_adjacency: Vec<[Option<(usize,usize)>; 6]> // for each cell, for each of each facet, either a cell index & a local facet index, or None
}

impl HexMesh {
    #[allow(unused)]
    pub fn new() -> HexMesh {
        let points = Vec::new();
        let cells = Vec::new();
        HexMesh { points: points, cells: cells, scaled_jacobians: None, cell_adjacency: Vec::new() }
    }

    pub fn from_medit(file_name: &str) -> HexMesh {
        // MEDIT .mesh format for hex-meshes
        // /!\ in hexahedra definitions, vertex indices are 1-based
        // ---
        // MeshVersionFormatted 2
        // Dimension 3
        //
        // Vertices
        // <nb>
        // <x> <y> <z> 1
        // ...
        //
        // Hexahedra
        // <nb>
        // <v0> <v1> <v2> <v3> <v4> <v5> <v6> <v7> 1
        // ...
        // End

        let mut points = Vec::new();
        let mut cells = Vec::new();

        enum ParsingState {
            Header,
            VerticesNumber,
            Vertices,
            SectionTransition,
            CellsNumber,
            Cells,
            ParsingFinised
        }

        let mut state: ParsingState = ParsingState::Header;
        
        if let Ok(whole_file) = fs::read_to_string(file_name) {
            let lines = whole_file.split("\n");
            for line in lines {
                match state {
                    ParsingState::Header => {
                        if line == "Vertices" {
                            state = ParsingState::VerticesNumber;
                        }
                        // else: ignore this line
                        continue;
                    },
                    ParsingState::VerticesNumber => {
                        let nb_vertices: usize = line.to_string().parse::<usize>().unwrap();
                        points.reserve(nb_vertices); // preallocation
                        println!("Found nb_vertices = {nb_vertices}");
                        state = ParsingState::Vertices;
                        continue;
                    },
                    ParsingState::Vertices => {
                        if line == "" {
                            state = ParsingState::SectionTransition;
                        }
                        else {
                            let parts: Vec<&str> = line.split(" ").collect::<Vec<&str>>();
                            assert!(parts.len() == 4); // 4 parts: x, y, z and the value 1
                            let x: f32 = parts[0].to_string().parse::<f32>().unwrap();
                            let y: f32 = parts[1].to_string().parse::<f32>().unwrap();
                            let z: f32 = parts[2].to_string().parse::<f32>().unwrap();
                            points.push(Vec3{ x: x, y: y, z: z });
                        }
                        continue;
                    },
                    ParsingState::SectionTransition => {
                        if line == "Hexahedra" {
                            state = ParsingState::CellsNumber;
                        }
                        // else: ignore this line
                        continue;
                    },
                    ParsingState::CellsNumber => {
                        let nb_cells: usize = line.to_string().parse::<usize>().unwrap();
                        cells.reserve(nb_cells); // preallocation
                        println!("Found nb_cells = {nb_cells}");
                        state = ParsingState::Cells;
                        continue;
                    },
                    ParsingState::Cells => {
                        if line == "" {
                            state = ParsingState::ParsingFinised;
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
                            cells.push(Hexahedron::new([v0-1,v1-1,v2-1,v3-1,v4-1,v5-1,v6-1,v7-1])); // 1-based to 0-based indices
                        }
                        continue;
                    },
                    ParsingState::ParsingFinised => {
                        break;
                    }
                }
            }
            println!("points.len() = {}", points.len());
            println!("cells.len() = {}", cells.len());
            return HexMesh { points: points, cells: cells, scaled_jacobians: None, cell_adjacency: Vec::new() }
        }
        else {
            panic!("Unable to open file '{file_name}'");
        }
    }

    pub fn compute_scaled_jacobian(&mut self) {
        self.scaled_jacobians = Some(vec![0.0f32; self.cells.len()]); // fill with 0.0, self.cells.len() times
        for hex_index in 0..self.cells.len() { // for each cell (each hexahedron)
            let mut scaled_jacobian: f32 = 1.0;
            for hex_corner in 0..8 { // for each of the 8 vertices of the current hexahedron
                let mut v: [Vec3; 4] = [Vec3::ZERO, Vec3::ZERO, Vec3::ZERO, Vec3::ZERO];
                for i in 0..4 { // [0] will be the current vertex, and [1:3] its 3 neighboring corners
                    let which_corner = Hexahedron::CORNER_SPLITING[hex_corner][i];
                    let vertex_index = self.cells.get(hex_index).unwrap().vertices[which_corner];
                    v[i] = *self.points.get(vertex_index).unwrap_or_else(|| {
                            panic!("Cannot access `points` vec at {vertex_index}");
                        }
                    ); // get 3D coordinates of vertex at vertex_index
                }
                let n1: Vec3 = (v[1] - v[0]).normalized();
                let n2: Vec3 = (v[2] - v[0]).normalized();
                let n3: Vec3 = (v[3] - v[0]).normalized();
                scaled_jacobian = f32::min(scaled_jacobian,dot(&n3,&cross(&n1,&n2)));
            }
            *self.scaled_jacobians.as_mut().unwrap().get_mut(hex_index).unwrap() = scaled_jacobian; // update vector
        }
    }

    pub fn compute_cell_adjacency(&mut self) {
        let mut uniques_quads: HashMap<[usize; 4],Vec<(usize,usize)>> = HashMap::new();
        let mut v0: usize;
        let mut v1: usize;
        let mut v2: usize;
        let mut v3: usize;
        let mut four_vertices_of_the_facet: [usize; 4];
        for hex_index in 0..self.cells.len() { // for each cell (each hexahedron)
            let current_hex: &Hexahedron = self.cells.get(hex_index).unwrap();
            for facet_index in 0..6 { // for each facet of the current cell
                v0 = *current_hex.vertices.get(Hexahedron::FACET_SPLITTING[facet_index][1]).unwrap();
                v1 = *current_hex.vertices.get(Hexahedron::FACET_SPLITTING[facet_index][2]).unwrap();
                v2 = *current_hex.vertices.get(Hexahedron::FACET_SPLITTING[facet_index][3]).unwrap();
                v3 = *current_hex.vertices.get(Hexahedron::FACET_SPLITTING[facet_index][4]).unwrap();
                four_vertices_of_the_facet = [v0, v1, v2, v3];
                four_vertices_of_the_facet.sort();
                let existing_value: Option<&mut Vec<(usize,usize)>> = uniques_quads.get_mut(&four_vertices_of_the_facet);
                if let Some(value) = existing_value {
                    // this unique facet already exists in the hashmap
                    value.push((hex_index,facet_index));
                }
                else {
                    // this unique facet doesn't already exists in the hashmap
                    uniques_quads.insert(four_vertices_of_the_facet,vec![(hex_index,facet_index)]);
                }
            }
        }

        // parse again all facets of all cells
        self.cell_adjacency.clear();
        self.cell_adjacency.reserve(self.cells.len()); // preallocation
        for hex_index in 0..self.cells.len() {
            let current_hex: &Hexahedron = self.cells.get(hex_index).unwrap();
            let mut adjacent_cells: [Option<(usize,usize)>; 6] = [None,None,None,None,None,None];
            for facet_index in 0..6 {
                v0 = *current_hex.vertices.get(Hexahedron::FACET_SPLITTING[facet_index][1]).unwrap();
                v1 = *current_hex.vertices.get(Hexahedron::FACET_SPLITTING[facet_index][2]).unwrap();
                v2 = *current_hex.vertices.get(Hexahedron::FACET_SPLITTING[facet_index][3]).unwrap();
                v3 = *current_hex.vertices.get(Hexahedron::FACET_SPLITTING[facet_index][4]).unwrap();
                four_vertices_of_the_facet = [v0, v1, v2, v3];
                four_vertices_of_the_facet.sort();
                let existing_value: Option<&Vec<(usize,usize)>> = uniques_quads.get(&four_vertices_of_the_facet);
                let value = existing_value.unwrap(); // assert the facet is in the hashmap
                assert!(value.len() == 1 || value.len() == 2); // each unique facet is linked to 1 or 2 oriented facets
                // if value.len() == 1, nothing to do, adjacency is None at initialization
                if value.len() == 2 {
                    if value.get(0).unwrap().0 == hex_index {
                        adjacent_cells[facet_index] = Some(*value.get(1).unwrap());
                    }
                    else {
                        adjacent_cells[facet_index] = Some(*value.get(0).unwrap());
                    }
                }
            }
            self.cell_adjacency.push(adjacent_cells);
        }
        println!("cell_adjacency.len() = {}",self.cell_adjacency.len());

        println!("End of compute_cell_adjacency");
    }

    pub fn triangulate_surface(&mut self) -> (TriangleMesh,Vec<f32>) {
        if self.scaled_jacobians == None {
            self.compute_scaled_jacobian();
        }
        if self.cell_adjacency.is_empty() {
            self.compute_cell_adjacency();
        }
        let scaled_jacobians = self.scaled_jacobians.as_ref().unwrap();
        // extract surface of the mesh
        let mut trianglemesh = TriangleMesh::new();
        assert!(trianglemesh.positions.len() == 0);
        assert!(trianglemesh.indices.len() == 0);
        assert!(trianglemesh.edges.len() == 0);
        let mut per_triangle_scaled_jacobian: Vec<f32> = Vec::new();
        let mut v0: u32;
        let mut v1: u32;
        let mut v2: u32;
        let mut v3: u32;
        let mut e0: [u32; 2]; // edge between v0 and v1;
        let mut e1: [u32; 2]; // edge between v1 and v2;
        let mut e2: [u32; 2]; // edge between v2 and v3;
        let mut e3: [u32; 2]; // edge between v3 and v0;
        trianglemesh.positions.reserve(self.points.len());
        for vertex_index in 0..self.points.len() {
            let current_vertex = self.points.get(vertex_index).unwrap();
            trianglemesh.positions.push([
                current_vertex.x,
                current_vertex.y,
                current_vertex.z,
            ]);
        }
        assert!(trianglemesh.positions.len() == self.points.len());
        for hex_index in 0..self.cells.len() {
            let current_hex: &Hexahedron = self.cells.get(hex_index).unwrap();
            for facet_index in 0..6 {
                assert!(Hexahedron::FACET_SPLITTING[facet_index][0] == facet_index);
                let at_other_side = self.cell_adjacency.get(hex_index).unwrap().get(facet_index).unwrap();
                if *at_other_side == None {
                    // this facet (quad) is on the surface
                    // create 2 triangles, [v0,v2,v1] and [v0,v3,v2]
                    // v1 +-----+ v2
                    //    |  // |
                    //    | //  |
                    // v0 +-----+ v3
                    v0 = TryInto::<u32>::try_into(
                        *current_hex.vertices.get(Hexahedron::FACET_SPLITTING[facet_index][1]).unwrap()
                    ).unwrap();
                    v1 = TryInto::<u32>::try_into(
                        *current_hex.vertices.get(Hexahedron::FACET_SPLITTING[facet_index][2]).unwrap()
                    ).unwrap();
                    v2 = TryInto::<u32>::try_into(
                        *current_hex.vertices.get(Hexahedron::FACET_SPLITTING[facet_index][3]).unwrap()
                    ).unwrap();
                    v3 = TryInto::<u32>::try_into(
                        *current_hex.vertices.get(Hexahedron::FACET_SPLITTING[facet_index][4]).unwrap()
                    ).unwrap();

                    trianglemesh.indices.reserve(6); // 2 new triangles -> 3*2 = 6 indices
                    trianglemesh.indices.push(v0);
                    trianglemesh.indices.push(v2);
                    trianglemesh.indices.push(v1);

                    trianglemesh.indices.push(v0);
                    trianglemesh.indices.push(v3);
                    trianglemesh.indices.push(v2);

                    per_triangle_scaled_jacobian.reserve(2); // 2 new per-triangle scaled jacobian
                    per_triangle_scaled_jacobian.push(*scaled_jacobians.get(hex_index).unwrap());
                    per_triangle_scaled_jacobian.push(*scaled_jacobians.get(hex_index).unwrap());

                    // assemble oriented edges
                    e0 = [v0,v1];
                    e1 = [v1,v2];
                    e2 = [v2,v3];
                    e3 = [v3,v0];
                    // sort vertex indices inside edges -> unoriented edges
                    e0.sort();
                    e1.sort();
                    e2.sort();
                    e3.sort();

                    // insert unoriented edges in the hashset of unique edges
                    trianglemesh.edges.insert(e0);
                    trianglemesh.edges.insert(e1);
                    trianglemesh.edges.insert(e2);
                    trianglemesh.edges.insert(e3);
                }
            }
        }
        assert!(!trianglemesh.positions.is_empty());
        assert!(trianglemesh.indices.len() % 3 == 0);
        println!("End of triangulate_surface()");
        (trianglemesh,per_triangle_scaled_jacobian)
    }
}