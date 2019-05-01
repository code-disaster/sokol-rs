#[derive(Debug, PartialEq)]
pub enum SgApi {
    Direct3D11,
    Metal,
    OpenGL33,
}

#[cfg(gfx = "d3d11")]
pub fn sg_api() -> SgApi {
    SgApi::Direct3D11
}

#[cfg(gfx = "metal")]
pub fn sg_api() -> SgApi {
    SgApi::Metal
}

#[cfg(gfx = "glcore33")]
pub fn sg_api() -> SgApi {
    SgApi::OpenGL33
}
