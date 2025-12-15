# bevy-hexmesh

Render the surface of `input.mesh` with Bevy

```bash
cargo run
```

1. The hexmesh `input.mesh` is read
2. For each hexahedra, its deformation is measured (Scaled Jacobian)
3. The adjacency between cells is computed
4. The surface quad mesh is extracted, and its wireframe is stored as a set of edges
5. The surface is triangulated
6. Isolated vertices are removed
7. Remaining vertices are duplicated to have a single uv coordinate accross each triangle
8. The textured triangle mesh (the hexmesh surface) is rendered, alongside the quad mesh wireframe, with the Bevy engine