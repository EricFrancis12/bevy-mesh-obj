use crate::error::Error;
use std::{fmt, fs, path::PathBuf, str::FromStr};

#[derive(Eq, PartialEq)]
enum ObjToken {
    O,
    V,
    Vn,
    Vt,
    S,
    F,
}

impl fmt::Display for ObjToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_str = match self {
            ObjToken::O => "o",
            ObjToken::V => "v",
            ObjToken::Vn => "vn",
            ObjToken::Vt => "vt",
            ObjToken::S => "s",
            ObjToken::F => "f",
        };
        write!(f, "{}", token_str)
    }
}

impl FromStr for ObjToken {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "o" => Ok(ObjToken::O),
            "v" => Ok(ObjToken::V),
            "vn" => Ok(ObjToken::Vn),
            "vt" => Ok(ObjToken::Vt),
            "s" => Ok(ObjToken::S),
            "f" => Ok(ObjToken::F),
            _ => Err(Error::UnrecognizedToken(s.to_owned())),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn to_arr(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:.9}, {:.9}, {:.9}]", self.x, self.y, self.z)
    }
}

impl TryFrom<&[&str]> for Vertex {
    type Error = crate::error::Error;

    fn try_from(value: &[&str]) -> Result<Self, Self::Error> {
        match value {
            [x_str, y_str, z_str] => Ok(Self::new(
                x_str.parse::<f32>()?,
                y_str.parse::<f32>()?,
                z_str.parse::<f32>()?,
            )),
            [token_str, x_str, y_str, z_str]
                if ObjToken::from_str(token_str).ok() == Some(ObjToken::V) =>
            {
                Ok(Self::new(
                    x_str.parse::<f32>()?,
                    y_str.parse::<f32>()?,
                    z_str.parse::<f32>()?,
                ))
            }
            _ => Err(Error::InvalidVertexFormat),
        }
    }
}

impl FromStr for Vertex {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.split(" ").collect::<Vec<&str>>().as_slice())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Normal {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Normal {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn to_arr(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl fmt::Display for Normal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:.9}, {:.9}, {:.9}]", self.x, self.y, self.z)
    }
}

impl TryFrom<&[&str]> for Normal {
    type Error = crate::error::Error;

    fn try_from(value: &[&str]) -> Result<Self, Self::Error> {
        match value {
            [x_str, y_str, z_str] => Ok(Self::new(
                x_str.parse::<f32>()?,
                y_str.parse::<f32>()?,
                z_str.parse::<f32>()?,
            )),
            [token_str, x_str, y_str, z_str]
                if ObjToken::from_str(token_str).ok() == Some(ObjToken::Vn) =>
            {
                Ok(Self::new(
                    x_str.parse::<f32>()?,
                    y_str.parse::<f32>()?,
                    z_str.parse::<f32>()?,
                ))
            }
            _ => Err(Error::InvalidNormalFormat),
        }
    }
}

impl FromStr for Normal {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.split(" ").collect::<Vec<&str>>().as_slice())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UVTexture {
    pub h: f32,
    pub v: f32,
}

impl UVTexture {
    pub fn new(h: f32, v: f32) -> Self {
        Self { h, v }
    }

    pub fn to_arr(&self) -> [f32; 2] {
        [self.h, self.v]
    }
}

impl fmt::Display for UVTexture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:.9}, {:.9}]", self.h, self.v)
    }
}

impl TryFrom<&[&str]> for UVTexture {
    type Error = crate::error::Error;

    fn try_from(value: &[&str]) -> Result<Self, Self::Error> {
        match value {
            [h_str, v_str] => Ok(Self::new(h_str.parse::<f32>()?, v_str.parse::<f32>()?)),
            [token_str, h_str, v_str]
                if ObjToken::from_str(token_str).ok() == Some(ObjToken::Vt) =>
            {
                Ok(Self::new(h_str.parse::<f32>()?, v_str.parse::<f32>()?))
            }
            _ => Err(Error::InvalidUVTextureFormat),
        }
    }
}

impl FromStr for UVTexture {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.split(" ").collect::<Vec<&str>>().as_slice())
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Smoothing(pub u8);

impl fmt::Display for Smoothing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&[&str]> for Smoothing {
    type Error = crate::error::Error;

    fn try_from(value: &[&str]) -> Result<Self, Self::Error> {
        match value {
            [s] => Self::from_str(s),
            _ => Err(Error::InvalidSmoothingFormat),
        }
    }
}

impl FromStr for Smoothing {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.to_lowercase() == "off" {
            return Ok(Smoothing(0));
        }
        Ok(Smoothing(s.parse::<u8>()?))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FaceDefinition {
    pub vertex_index: usize,
    pub normal_index: usize,
    pub uv_texture_index: usize,
}

impl FaceDefinition {
    pub fn new(vertex_index: usize, normal_index: usize, uv_texture_index: usize) -> Self {
        Self {
            vertex_index,
            normal_index,
            uv_texture_index,
        }
    }
}

impl FromStr for FaceDefinition {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("/").collect();
        if let [v_str, vt_str, vn_str] = &parts[0..] {
            // subtracting 1 is necessary because .obj indexing starts at 1:
            let v = v_str.parse::<usize>()? - 1;
            let vt = vt_str.parse::<usize>()? - 1;
            let vn = vn_str.parse::<usize>()? - 1;

            return Ok(Self::new(v, vn, vt));
        }

        Err(Error::InvalidFaceDefinitionString)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Face {
    pub face_defs: Vec<FaceDefinition>,
}

impl Face {
    pub fn new(face_defs: Vec<FaceDefinition>) -> Self {
        Self { face_defs }
    }
}

impl fmt::Display for Face {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .face_defs
            .iter()
            .map(|fd| {
                format!(
                    "{},{},{}",
                    fd.vertex_index, fd.uv_texture_index, fd.normal_index,
                )
            })
            .collect::<Vec<String>>()
            .join(",");

        write!(f, "{}", s)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Obj3D {
    pub name: Option<String>,
    pub vertices: Vec<Vertex>,
    pub normals: Vec<Normal>,
    pub uv_textures: Vec<UVTexture>,
    pub smoothing: Smoothing,
    pub faces: Vec<Face>,
}

impl Obj3D {
    pub fn new(
        name: Option<String>,
        vertices: Vec<Vertex>,
        normals: Vec<Normal>,
        uv_textures: Vec<UVTexture>,
        smoothing: Smoothing,
        faces: Vec<Face>,
    ) -> Self {
        Self {
            name,
            vertices,
            normals,
            uv_textures,
            smoothing,
            faces,
        }
    }

    pub fn new_with_name(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            ..Default::default()
        }
    }

    pub fn parse_string(s: impl Into<String>) -> Result<Vec<Self>, Error> {
        let content = s.into();
        let mut objs: Vec<Self> = Vec::new();

        for line in &content.split("\n").collect::<Vec<&str>>() {
            let tokens: Vec<&str> = line.split(" ").map(|s| s.trim()).collect();
            let tail = &tokens[1..];

            if let Some(obj_token) = tokens.get(0).and_then(|s| ObjToken::from_str(s).ok()) {
                match obj_token {
                    ObjToken::O => {
                        if let [_, name] = tokens.as_slice() {
                            objs.push(Self::new_with_name(*name));
                        }
                    }
                    ObjToken::V => {
                        objs.last_mut()
                            .ok_or_else(|| Error::MissingObjectDeclaration)?
                            .vertices
                            .push(Vertex::try_from(tail)?);
                    }
                    ObjToken::Vn => {
                        objs.last_mut()
                            .ok_or_else(|| Error::MissingObjectDeclaration)?
                            .normals
                            .push(Normal::try_from(tail)?);
                    }
                    ObjToken::Vt => {
                        objs.last_mut()
                            .ok_or_else(|| Error::MissingObjectDeclaration)?
                            .uv_textures
                            .push(UVTexture::try_from(tail)?);
                    }
                    ObjToken::S => {
                        objs.last_mut()
                            .ok_or_else(|| Error::MissingObjectDeclaration)?
                            // TODO: handle possibility where there are multiple s tokens in a single object,
                            // instead of overriding the previous value?
                            .smoothing = Smoothing::try_from(tail)?
                    }
                    ObjToken::F => {
                        let obj = objs
                            .last_mut()
                            .ok_or_else(|| Error::MissingObjectDeclaration)?;

                        let face_defs = tail
                            .iter()
                            .map(|face_str| FaceDefinition::from_str(face_str))
                            .collect::<Result<_, _>>()?;

                        obj.faces.push(Face::new(face_defs));
                    }
                }
            }
        }

        Ok(objs)
    }

    pub fn parse_string_first(s: impl Into<String>) -> Result<Option<Self>, Error> {
        Self::parse_string(s).map(|objs| objs.first().cloned())
    }

    pub fn parse_string_single(s: impl Into<String>) -> Result<Self, Error> {
        let objs = Self::parse_string(s.into())?;
        if let [obj] = objs.as_slice() {
            return Ok(obj.clone());
        }
        Err(Error::ParseSingleObj(objs.len()))
    }

    pub fn parse(path: impl Into<PathBuf>) -> Result<Vec<Self>, Error> {
        let content = fs::read_to_string(path.into())?;
        Self::parse_string(content)
    }

    pub fn parse_first(path: impl Into<PathBuf>) -> Result<Option<Self>, Error> {
        let content = fs::read_to_string(path.into())?;
        Self::parse_string_first(content)
    }

    pub fn parse_single(path: impl Into<PathBuf>) -> Result<Self, Error> {
        let objs = Self::parse(path)?;
        if let [obj] = objs.as_slice() {
            return Ok(obj.clone());
        }
        Err(Error::ParseSingleObj(objs.len()))
    }
}

impl FromStr for Obj3D {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_string_single(s)
    }
}
