# Bevy Mesh Obj

This crate provides utilities to parse Wavefront `.obj` files, handle 3D object data, and generate [bevy](https://github.com/bevyengine/bevy) meshes from them.

## Features

- **Parsing `.obj` files:** Parse `.obj` files into an internal representation of vertices, normals, texture coordinates, faces, and smoothing information.
- **Handle 3D object data:** The crate includes types for representing vertices (`Vertex`), normals (`Normal`), texture coordinates (`UVTexture`), smoothing groups (`Smoothing`), and faces (`Face`).
- **Generate meshes:** Create Bevy `Mesh` objects from parsed `.obj` files, which can be used in 3D rendering applications.


## Installation

Add the **bevy_mesh_obj** crate: 

```bash
cargo add bevy_third_person_camera
```

## Limitations

To ensure proper functionality, all .obj files must NOT contain any shared vertecies, and all objects must be triangulated (all faces must be triangles). See `assets/wall_with_door_gap.blend` for an example how to achieve this in Blender before exporting to .obj format.

## Usage

### Parsing .obj files

The crate provides several methods to parse .obj files either from a string or a file. Below are examples of how to use the provided functions.

#### Parse from a file
```rust
let obj: Obj3D = Obj3D::parse_single("path/to/file.obj").unwrap();
```

#### Parse multiple objects from a file

```rust
let objs: Vec<Obj3D> = Obj3D::parse("path/to/file.obj").unwrap();
```

#### Parse the nth object

```rust
let obj: Obj3D = Obj3D::parse_n("path/to/file.obj", 2).unwrap();
```

#### Parse the first object

```rust
let obj: Obj3D = Obj3D::parse_first("path/to/file.obj").unwrap();
```

### Writing to a file

You can write the Obj3D object back to an .obj file:

```rust
let obj: Obj3D = Obj3D::parse_single("path/to/file.obj").unwrap();
obj.write_to_file("path/to/output.obj").unwrap();
```

### Generating Bevy Mesh

You can use the `mesh_from_obj!()` macro to generate a Bevy `Mesh` directly from an .obj file, that you can use in your game or application:

```rust
use bevy::prelude::Mesh;
use bevy_mesh_obj::mesh_from_obj;

fn new_mesh() -> Mesh {
    mesh_from_obj!("path/to/file.obj")
}
```

## File Format

This crate handles parsing the following tokens from an .obj file:

- `o`: Object name
- `v`: Vertex position (x, y, z)
- `vn`: Vertex normal (x, y, z)
- `vt`: Texture coordinates (u, v)
- `s`: Smoothing group
- `f`: Face definitions, which are made up of indices to vertices, normals, and texture coordinates

Here is an example of a basic .obj file:

```
# Name
o Cube

# Vertices
v 1.0 1.0 1.0
v -1.0 1.0 1.0
v -1.0 -1.0 1.0
v 1.0 -1.0 1.0
v 1.0 1.0 -1.0
v -1.0 1.0 -1.0
v -1.0 -1.0 -1.0
v 1.0 -1.0 -1.0

# Normals
vn 0.0 0.0 1.0
vn 0.0 0.0 -1.0
vn 1.0 0.0 0.0
vn -1.0 0.0 0.0
vn 0.0 1.0 0.0
vn 0.0 -1.0 0.0

# Texture Coordinates
vt 0.0 0.0
vt 1.0 0.0
vt 1.0 1.0
vt 0.0 1.0

# Faces (using 1-based index notation)
f 1/1/1 2/2/1 3/3/1 4/4/1  # Front face
f 5/1/2 6/2/2 7/3/2 8/4/2  # Back face
f 1/1/3 2/2/3 6/3/5 5/4/5  # Top face
f 4/1/5 3/2/5 7/3/5 8/4/5  # Bottom face
f 1/1/4 4/2/4 8/3/4 5/4/4  # Right face
f 2/1/6 3/2/6 7/3/6 6/4/6  # Left face
```

- `o` Cube defines the object name.
- `v` lines define vertices.
- `vn` lines define vertex normals.
- `vt` lines define texture coordinates.
- `f` lines define faces using vertex/texture/normal indices.

## License

[MIT](https://mit-license.org)


## Contributing
I would be more than happy to accept your contribution if you have a valuable addition to this project. Please fork this repository, create a branch, and submit a pull request.
