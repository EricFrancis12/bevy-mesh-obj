#[macro_export]
macro_rules! parse_objs {
    ($file_path:expr) => {
        crate::obj::Obj3D::parse_string(include_str!($file_path)).unwrap()
    };
}

#[macro_export]
macro_rules! parse_obj {
    ($file_path:expr) => {
        crate::obj::Obj3D::parse_string_single(include_str!($file_path)).unwrap()
    };
}

#[macro_export]
macro_rules! parse_n_objs {
    ($file_path:expr, $n:expr) => {
        crate::obj::Obj3D::parse_string_n(include_str!($file_path), $n).unwrap()
    };
}

#[macro_export]
macro_rules! parse_first_obj {
    ($file_path:expr) => {
        parse_n_objs!($file_path, 1)
    };
}

#[macro_export]
macro_rules! mesh_fn_from_obj {
    ($file_path:expr) => {
        || {
            let o = crate::parse_obj!($file_path);

            let mut positions = Vec::new();
            let mut uv = Vec::new();
            let mut normals = Vec::new();
            let mut indeces = Vec::new();

            let mut i = 0;

            for face in &o.faces {
                for fd in &face.face_defs {
                    positions.push(o.vertices[fd.vertex_index].to_arr());
                    uv.push(o.uv_textures[fd.uv_texture_index].to_arr());
                    normals.push(o.normals[fd.normal_index].to_arr());
                    indeces.push(i);

                    i += 1;
                }
            }

            bevy::prelude::Mesh::new(
                bevy::render::mesh::PrimitiveTopology::TriangleList,
                bevy::render::render_asset::RenderAssetUsages::MAIN_WORLD
                    | bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,
            )
            .with_inserted_attribute(bevy::prelude::Mesh::ATTRIBUTE_POSITION, positions)
            .with_inserted_attribute(bevy::prelude::Mesh::ATTRIBUTE_UV_0, uv)
            .with_inserted_attribute(bevy::prelude::Mesh::ATTRIBUTE_NORMAL, normals)
            .with_inserted_indices(bevy::render::mesh::Indices::U32(indeces))
        }
    };
}

#[macro_export]
macro_rules! mesh_from_obj {
    ($file_path:expr) => {
        crate::mesh_fn_from_obj!($file_path)()
    };
}
