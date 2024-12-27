use crate::obj::{Face, FaceDefinition, Normal, Obj3D, Smoothing, UVTexture, Vertex};

#[test]
fn test_obj3d_new() {
    let name = String::from("Plane");
    let vertices = vec![
        Vertex::new(1.0, 2.0, 3.0),
        Vertex::new(4.0, 5.0, 6.0),
        Vertex::new(7.0, 8.0, 9.0),
    ];
    let normals = vec![
        Normal::new(10.0, 11.0, 12.0),
        Normal::new(13.0, 14.0, 15.0),
        Normal::new(16.0, 17.0, 18.0),
    ];
    let uv_textures = vec![
        UVTexture::new(1.0, 2.0),
        UVTexture::new(3.0, 4.0),
        UVTexture::new(5.0, 6.0),
    ];
    let smoothing = Smoothing(1);
    let faces = vec![Face::new(vec![
        FaceDefinition::new(1, 2, 3),
        FaceDefinition::new(4, 5, 6),
        FaceDefinition::new(7, 8, 9),
    ])];

    let o = Obj3D::new(
        Some(name.clone()),
        vertices.clone(),
        normals.clone(),
        uv_textures.clone(),
        smoothing.clone(),
        faces.clone(),
    );

    assert_eq!(o.name, Some(name));
    assert_eq!(o.vertices, vertices);
    assert_eq!(o.normals, normals);
    assert_eq!(o.uv_textures, uv_textures);
    assert_eq!(o.smoothing, smoothing);
    assert_eq!(o.faces, faces);
}

#[test]
fn test_obj3d_new_with_name() {
    let name = String::from("Plane");
    let o = Obj3D::new_with_name(&name);

    assert_eq!(o.name, Some(name));
    assert_eq!(o.vertices, Vec::new());
    assert_eq!(o.normals, Vec::new());
    assert_eq!(o.uv_textures, Vec::new());
    assert_eq!(o.smoothing, Smoothing(0));
    assert_eq!(o.faces, Vec::new());
}

#[test]
fn test_obj3d_parse_string() {
    let objs = Obj3D::parse_string(include_str!("../../assets/wall_with_door_gap.obj")).unwrap();

    assert_eq!(objs.len(), 1);

    for o in objs {
        assert_eq!(o.name, Some("Plane".to_owned()));
        // TODO: ...
        // assert_eq!(o.vertices,);
        // assert_eq!(o.normals,);
        // assert_eq!(o.uv_textures,);
        assert_eq!(o.smoothing, Smoothing(0));
        // assert_eq!(o.faces, );
    }
}

#[test]
fn test_obj3d_parse_string_first() {
    let o = Obj3D::parse_string_first(include_str!("../../assets/wall_with_door_gap.obj"))
        .unwrap()
        .unwrap();

    assert_eq!(o.name, Some("Plane".to_owned()));
    // TODO: ...
    // assert_eq!(o.vertices,);
    // assert_eq!(o.normals,);
    // assert_eq!(o.uv_textures,);
    assert_eq!(o.smoothing, Smoothing(0));
    // assert_eq!(o.faces, );
}

#[test]
fn test_obj3d_parse_string_single() {
    let o =
        Obj3D::parse_string_single(include_str!("../../assets/wall_with_door_gap.obj")).unwrap();

    assert_eq!(o.name, Some("Plane".to_owned()));
    // TODO: ...
    // assert_eq!(o.vertices,);
    // assert_eq!(o.normals,);
    // assert_eq!(o.uv_textures,);
    assert_eq!(o.smoothing, Smoothing(0));
    // assert_eq!(o.faces, );
}

#[test]
fn test_obj3d_parse() {
    let objs = Obj3D::parse("assets/wall_with_door_gap.obj").unwrap();

    assert_eq!(objs.len(), 1);

    for o in objs {
        assert_eq!(o.name, Some("Plane".to_owned()));
        // TODO: ...
        // assert_eq!(o.vertices,);
        // assert_eq!(o.normals,);
        // assert_eq!(o.uv_textures,);
        assert_eq!(o.smoothing, Smoothing(0));
        // assert_eq!(o.faces, );
    }
}

#[test]
fn test_obj3d_parse_first() {
    let o = Obj3D::parse_first("assets/wall_with_door_gap.obj")
        .unwrap()
        .unwrap();

    assert_eq!(o.name, Some("Plane".to_owned()));
    // TODO: ...
    // assert_eq!(o.vertices,);
    // assert_eq!(o.normals,);
    // assert_eq!(o.uv_textures,);
    assert_eq!(o.smoothing, Smoothing(0));
    // assert_eq!(o.faces, );
}

#[test]
fn test_obj3d_parse_single() {
    let o = Obj3D::parse_single("assets/wall_with_door_gap.obj").unwrap();

    assert_eq!(o.name, Some("Plane".to_owned()));
    // TODO: ...
    // assert_eq!(o.vertices,);
    // assert_eq!(o.normals,);
    // assert_eq!(o.uv_textures,);
    assert_eq!(o.smoothing, Smoothing(0));
    // assert_eq!(o.faces, );
}
