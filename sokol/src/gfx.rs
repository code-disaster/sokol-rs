use std::os::raw::c_void;

mod ffi {
    use std::ffi::CString;
    use std::os::raw::c_char;
    use std::os::raw::c_int;
    use std::os::raw::c_void;
    use std::ptr::null;

    use crate::app::ffi::*;

    const _SG_INVALID_ID: usize = 0;
    const _SG_NUM_SHADER_STAGES: usize = 2;
    const SG_NUM_INFLIGHT_FRAMES: usize = 2;
    const SG_MAX_COLOR_ATTACHMENTS: usize = 4;
    const SG_MAX_SHADERSTAGE_BUFFERS: usize = 4;
    const SG_MAX_SHADERSTAGE_IMAGES: usize = 12;
    const SG_MAX_SHADERSTAGE_UBS: usize = 4;
    const SG_MAX_UB_MEMBERS: usize = 16;
    const SG_MAX_VERTEX_ATTRIBUTES: usize = 16;
    const SG_MAX_MIPMAPS: usize = 16;
    const _SG_MAX_TEXTUREARRAY_LAYERS: usize = 128;

    #[repr(C)]
    pub struct SgPassAction {
        _start_canary: u32,
        colors: [super::SgColorAttachmentAction; SG_MAX_COLOR_ATTACHMENTS],
        depth: super::SgDepthAttachmentAction,
        stencil: super::SgStencilAttachmentAction,
        _end_canary: u32,
    }

    impl SgPassAction {
        pub fn make(pass_action: &super::SgPassAction) -> SgPassAction {
            let mut pa = SgPassAction {
                _start_canary: 0,
                colors: Default::default(),
                depth: pass_action.depth,
                stencil: pass_action.stencil,
                _end_canary: 0,
            };

            for (idx, color_action) in pass_action.colors.iter().enumerate() {
                pa.colors[idx] = *color_action;
            }

            pa
        }
    }

    #[repr(C)]
    #[derive(Default)]
    pub struct SgDrawState {
        _start_canary: u32,
        pipeline: super::SgPipeline,
        vertex_buffers: [super::SgBuffer; SG_MAX_SHADERSTAGE_BUFFERS],
        vertex_buffer_offsets: [c_int; SG_MAX_SHADERSTAGE_BUFFERS],
        index_buffer: super::SgBuffer,
        index_buffer_offset: c_int,
        vs_images: [super::SgImage; SG_MAX_SHADERSTAGE_IMAGES],
        fs_images: [super::SgImage; SG_MAX_SHADERSTAGE_IMAGES],
        _end_canary: u32,
    }

    impl SgDrawState {
        pub fn make(draw_state: &super::SgDrawState) -> SgDrawState {
            let mut ds = SgDrawState {
                pipeline: (*draw_state).pipeline,
                index_buffer: (*draw_state).index_buffer,
                index_buffer_offset: (*draw_state).index_buffer_offset,
                ..Default::default()
            };

            Self::collect_buffers(&mut ds, draw_state);

            ds
        }

        fn collect_buffers(draw_state: &mut SgDrawState,
                           src: &super::SgDrawState) {
            for (idx, vb) in src.vertex_buffers.iter().enumerate() {
                draw_state.vertex_buffers[idx] = *vb;
            }

            for (idx, vb_offs) in src.vertex_buffer_offsets.iter().enumerate() {
                draw_state.vertex_buffer_offsets[idx] = *vb_offs;
            }

            for (idx, img) in src.vs_images.iter().enumerate() {
                draw_state.vs_images[idx] = *img;
            }

            for (idx, img) in src.fs_images.iter().enumerate() {
                draw_state.fs_images[idx] = *img;
            }
        }
    }

    #[repr(C)]
    pub struct SgDesc {
        _start_canary: u32,
        desc: super::SgDesc,
        gl_force_gles2: bool,
        mtl_device: *const c_void,
        mtl_renderpass_descriptor_cb: unsafe extern fn() -> *const c_void,
        mtl_drawable_cb: unsafe extern fn() -> *const c_void,
        mtl_global_uniform_buffer_size: c_int,
        mtl_sampler_cache_size: c_int,
        d3d11_device: *const c_void,
        d3d11_device_context: *const c_void,
        d3d11_render_target_view_cb: unsafe extern fn() -> *const c_void,
        d3d11_depth_stencil_view_cb: unsafe extern fn() -> *const c_void,
        _end_canary: u32,
    }

    impl SgDesc {
        pub fn make(desc: &super::SgDesc) -> SgDesc {
            unsafe {
                SgDesc {
                    _start_canary: 0,
                    desc: *desc,
                    gl_force_gles2: false,
                    mtl_device: sapp_metal_get_device(),
                    mtl_renderpass_descriptor_cb: sapp_metal_get_renderpass_descriptor,
                    mtl_drawable_cb: sapp_metal_get_drawable,
                    mtl_global_uniform_buffer_size: 0,
                    mtl_sampler_cache_size: 0,
                    d3d11_device: sapp_d3d11_get_device(),
                    d3d11_device_context: sapp_d3d11_get_device_context(),
                    d3d11_render_target_view_cb: sapp_d3d11_get_render_target_view,
                    d3d11_depth_stencil_view_cb: sapp_d3d11_get_depth_stencil_view,
                    _end_canary: 0,
                }
            }
        }
    }

    #[repr(C)]
    pub struct SgBufferDesc {
        _start_canary: u32,
        desc: super::SgBufferDesc,
        content: *const c_void,
        gl_buffers: [u32; SG_NUM_INFLIGHT_FRAMES],
        mtl_buffers: [*const c_void; SG_NUM_INFLIGHT_FRAMES],
        d3d11_buffer: *const c_void,
        _end_canary: u32,
    }

    impl SgBufferDesc {
        pub fn make<T>(content: &T, desc: &super::SgBufferDesc) -> SgBufferDesc {
            let ptr = content as *const T;

            SgBufferDesc {
                _start_canary: 0,
                desc: *desc,
                content: ptr as *const c_void,
                gl_buffers: [0, 0],
                mtl_buffers: [null(), null()],
                d3d11_buffer: null(),
                _end_canary: 0,
            }
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    struct SgSubImageContent {
        ptr: *const c_void,
        size: c_int,
    }

    impl Default for SgSubImageContent {
        fn default() -> Self {
            Self {
                ptr: null(),
                size: 0,
            }
        }
    }

    #[repr(C)]
    struct SgImageContent {
        subimage: [SgSubImageContent; 6 * SG_MAX_MIPMAPS],
    }

    impl Default for SgImageContent {
        fn default() -> Self {
            Self {
                subimage: [
                    SgSubImageContent {
                        ..Default::default()
                    }; 96
                ]
            }
        }
    }

    #[repr(C)]
    pub struct SgImageDesc {
        _start_canary: u32,
        image_type: super::SgImageType,
        render_target: bool,
        width: c_int,
        height: c_int,
        depth_or_layers: c_int,
        num_mipmaps: c_int,
        usage: super::SgUsage,
        pixel_format: super::SgPixelFormat,
        sample_count: c_int,
        min_filter: super::SgFilter,
        mag_filter: super::SgFilter,
        wrap_u: super::SgWrap,
        wrap_v: super::SgWrap,
        wrap_w: super::SgWrap,
        max_anisotropy: u32,
        min_lod: f32,
        max_lod: f32,
        content: SgImageContent,
        gl_textures: [u32; SG_NUM_INFLIGHT_FRAMES],
        mtl_textures: [*const c_void; SG_NUM_INFLIGHT_FRAMES],
        d3d11_texture: *const c_void,
        _end_canary: u32,
    }

    impl SgImageDesc {
        pub fn make<T>(content: &Vec<(T, i32)>, desc: &super::SgImageDesc) -> SgImageDesc {
            let mut img = SgImageDesc {
                _start_canary: 0,
                image_type: desc.image_type,
                render_target: desc.render_target,
                width: desc.width,
                height: desc.height,
                depth_or_layers: desc.depth_or_layers,
                num_mipmaps: desc.num_mipmaps,
                usage: desc.usage,
                pixel_format: desc.pixel_format,
                sample_count: desc.sample_count,
                min_filter: desc.min_filter,
                mag_filter: desc.mag_filter,
                wrap_u: desc.wrap_u,
                wrap_v: desc.wrap_v,
                wrap_w: desc.wrap_w,
                max_anisotropy: desc.max_anisotropy,
                min_lod: desc.min_lod,
                max_lod: desc.max_lod,
                content: Default::default(),
                gl_textures: [0; SG_NUM_INFLIGHT_FRAMES],
                mtl_textures: [null(); SG_NUM_INFLIGHT_FRAMES],
                d3d11_texture: null(),
                _end_canary: 0,
            };

            for (idx, (data, size)) in content.iter().enumerate() {
                let ptr = data as *const T;

                img.content.subimage[idx] = SgSubImageContent {
                    ptr: ptr as *const c_void,
                    size: *size as i32,
                };
            }

            img
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    struct SgShaderUniformDesc {
        name: *const c_char,
        uniform_type: super::SgUniformType,
        array_count: c_int,
    }

    impl Default for SgShaderUniformDesc {
        fn default() -> SgShaderUniformDesc {
            SgShaderUniformDesc {
                name: null(),
                uniform_type: super::SgUniformType::_Invalid,
                array_count: 0,
            }
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    struct SgShaderUniformBlockDesc {
        size: c_int,
        uniforms: [SgShaderUniformDesc; SG_MAX_UB_MEMBERS],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    struct SgShaderImageDesc {
        name: *const c_char,
        image_type: super::SgImageType,
    }

    impl Default for SgShaderImageDesc {
        fn default() -> Self {
            SgShaderImageDesc {
                name: null(),
                image_type: super::SgImageType::_Default,
            }
        }
    }

    #[repr(C)]
    struct SgShaderStageDesc {
        source: *const c_char,
        byte_code: *const u8,
        byte_code_size: c_int,
        entry: *const c_char,
        uniform_blocks: [SgShaderUniformBlockDesc; SG_MAX_SHADERSTAGE_UBS],
        images: [SgShaderImageDesc; SG_MAX_SHADERSTAGE_IMAGES],
    }

    impl Default for SgShaderStageDesc {
        fn default() -> Self {
            SgShaderStageDesc {
                source: null(),
                byte_code: null(),
                byte_code_size: 0,
                entry: null(),
                uniform_blocks: [
                    Default::default(); SG_MAX_SHADERSTAGE_UBS
                ],
                images: [
                    Default::default(); SG_MAX_SHADERSTAGE_IMAGES
                ],
            }
        }
    }

    #[repr(C)]
    pub struct SgShaderDesc {
        _start_canary: u32,
        vs: SgShaderStageDesc,
        fs: SgShaderStageDesc,
        _end_canary: u32,
    }

    impl SgShaderDesc {
        pub fn make(desc: &super::SgShaderDesc) -> SgShaderDesc {
            let vs_code = CString::new(desc.vs.source).unwrap();
            let fs_code = CString::new(desc.fs.source).unwrap();

            let mut shd = SgShaderDesc {
                _start_canary: 0,
                vs: SgShaderStageDesc {
                    source: vs_code.into_raw(),
                    byte_code: null(),
                    byte_code_size: 0,
                    entry: null(),
                    ..Default::default()
                },
                fs: SgShaderStageDesc {
                    source: fs_code.into_raw(),
                    byte_code: null(),
                    byte_code_size: 0,
                    entry: null(),
                    ..Default::default()
                },
                _end_canary: 0,
            };

            SgShaderDesc::collect_uniform_blocks(&mut shd.vs, &desc.vs.uniform_blocks);
            SgShaderDesc::collect_images(&mut shd.vs, &desc.vs.images);

            SgShaderDesc::collect_uniform_blocks(&mut shd.fs, &desc.fs.uniform_blocks);
            SgShaderDesc::collect_images(&mut shd.fs, &desc.fs.images);

            shd
        }

        fn collect_uniforms(desc: &mut SgShaderUniformBlockDesc,
                            src: &Vec<super::SgShaderUniformDesc>) {
            for (idx, u) in src.iter().enumerate() {
                let dst = &mut desc.uniforms[idx];

                let name = CString::new(u.name).unwrap();

                dst.name = name.into_raw();
                dst.uniform_type = u.uniform_type;
                dst.array_count = u.array_count;
            }
        }

        fn collect_uniform_blocks(desc: &mut SgShaderStageDesc,
                                  src: &Vec<super::SgShaderUniformBlockDesc>) {
            for (idx, ub) in src.iter().enumerate() {
                let dst = &mut desc.uniform_blocks[idx];
                dst.size = ub.size;
                SgShaderDesc::collect_uniforms(dst, &ub.uniforms);
            }
        }

        fn collect_images(desc: &mut SgShaderStageDesc,
                          src: &Vec<super::SgShaderImageDesc>) {
            for (idx, img) in src.iter().enumerate() {
                let dst = &mut desc.images[idx];

                let name = CString::new(img.name).unwrap();

                dst.name = name.into_raw();
                dst.image_type = img.image_type;
            }
        }
    }

    #[repr(C)]
    pub struct SgVertexAttrDesc {
        name: *const c_char,
        sem_name: *const c_char,
        sem_index: c_int,
        buffer_index: c_int,
        offset: c_int,
        format: super::SgVertexFormat,
    }

    impl Default for SgVertexAttrDesc {
        fn default() -> Self {
            SgVertexAttrDesc {
                name: null(),
                sem_name: null(),
                sem_index: 0,
                buffer_index: 0,
                offset: 0,
                format: super::SgVertexFormat::_Invalid,
            }
        }
    }

    #[repr(C)]
    #[derive(Default)]
    pub struct SgLayoutDesc {
        buffers: [super::SgBufferLayoutDesc; SG_MAX_SHADERSTAGE_BUFFERS],
        attrs: [SgVertexAttrDesc; SG_MAX_VERTEX_ATTRIBUTES],
    }

    #[repr(C)]
    pub struct SgBlendState {
        enabled: bool,
        src_factor_rgb: super::SgBlendFactor,
        dst_factor_rgb: super::SgBlendFactor,
        op_rgb: super::SgBlendOp,
        src_factor_alpha: super::SgBlendFactor,
        dst_factor_alpha: super::SgBlendFactor,
        op_alpha: super::SgBlendOp,
        color_write_mask: u8,
        color_attachment_count: c_int,
        color_format: super::SgPixelFormat,
        depth_format: super::SgPixelFormat,
        blend_color: [f32; 4],
    }

    #[repr(C)]
    pub struct SgPipelineDesc {
        _start_canary: u32,
        layout: SgLayoutDesc,
        shader: super::SgShader,
        primitive_type: super::SgPrimitiveType,
        index_type: super::SgIndexType,
        depth_stencil: super::SgDepthStencilState,
        blend: SgBlendState,
        rasterizer: super::SgRasterizerState,
        _end_canary: u32,
    }

    impl SgPipelineDesc {
        pub fn make(desc: &super::SgPipelineDesc) -> SgPipelineDesc {
            let blend = desc.blend;

            let mut pip = SgPipelineDesc {
                _start_canary: 0,
                layout: Default::default(),
                shader: (*desc).shader,
                primitive_type: (*desc).primitive_type,
                index_type: (*desc).index_type,
                depth_stencil: (*desc).depth_stencil,
                blend: SgBlendState {
                    enabled: blend.enabled,
                    src_factor_rgb: blend.src_factor_rgb,
                    dst_factor_rgb: blend.dst_factor_rgb,
                    op_rgb: blend.op_rgb,
                    src_factor_alpha: blend.src_factor_alpha,
                    dst_factor_alpha: blend.dst_factor_alpha,
                    op_alpha: blend.op_alpha,
                    color_write_mask: blend.color_write_mask.bits(),
                    color_attachment_count: blend.color_attachment_count,
                    color_format: blend.color_format,
                    depth_format: blend.depth_format,
                    blend_color: blend.blend_color,
                },
                rasterizer: (*desc).rasterizer,
                _end_canary: 0,
            };

            SgPipelineDesc::collect_layout_buffers(&mut pip.layout, &desc.layout.buffers);
            SgPipelineDesc::collect_layout_attrs(&mut pip.layout, &desc.layout.attrs);

            pip
        }

        fn collect_layout_buffers(desc: &mut SgLayoutDesc,
                                  src: &Vec<super::SgBufferLayoutDesc>) {
            for (idx, buf) in src.iter().enumerate() {
                desc.buffers[idx] = super::SgBufferLayoutDesc {
                    stride: buf.stride,
                    step_func: buf.step_func,
                    step_rate: buf.step_rate,
                };
            }
        }

        fn collect_layout_attrs(desc: &mut SgLayoutDesc,
                                src: &Vec<super::SgVertexAttrDesc>) {
            for (idx, attr) in src.iter().enumerate() {
                let name = CString::new(attr.name).unwrap();
                let sem_name = CString::new(attr.sem_name).unwrap();

                desc.attrs[idx] = SgVertexAttrDesc {
                    name: name.into_raw(),
                    sem_name: sem_name.into_raw(),
                    sem_index: attr.sem_index,
                    buffer_index: attr.buffer_index,
                    offset: attr.offset,
                    format: attr.format,
                };
            }
        }
    }

    #[repr(C)]
    pub struct SgPassDesc {
        _start_canary: u32,
        color_attachments: [super::SgAttachmentDesc; SG_MAX_COLOR_ATTACHMENTS],
        depth_stencil_attachment: super::SgAttachmentDesc,
        _end_canary: u32,
    }

    impl SgPassDesc {
        pub fn make(desc: &super::SgPassDesc) -> SgPassDesc {
            let mut pass = SgPassDesc {
                _start_canary: 0,
                color_attachments: Default::default(),
                depth_stencil_attachment: desc.depth_stencil_attachment,
                _end_canary: 0,
            };

            for (idx, att) in desc.color_attachments.iter().enumerate() {
                pass.color_attachments[idx] = *att;
            }

            pass
        }
    }

    extern {
        pub fn sg_setup(desc: *const SgDesc);
        pub fn sg_shutdown();
        pub fn sg_isvalid() -> bool;
        pub fn sg_query_feature(feature: super::SgFeature) -> bool;
        pub fn sg_reset_state_cache();

        pub fn sg_make_buffer(desc: *const SgBufferDesc) -> super::SgBuffer;
        pub fn sg_make_image(desc: *const SgImageDesc) -> super::SgImage;
        pub fn sg_make_shader(desc: *const SgShaderDesc) -> super::SgShader;
        pub fn sg_make_pipeline(desc: *const SgPipelineDesc) -> super::SgPipeline;
        pub fn sg_make_pass(desc: *const SgPassDesc) -> super::SgPass;

        pub fn sg_destroy_buffer(buf: super::SgBuffer);
        pub fn sg_destroy_image(img: super::SgImage);
        pub fn sg_destroy_shader(shd: super::SgShader);
        pub fn sg_destroy_pipeline(pip: super::SgPipeline);
        pub fn sg_destroy_pass(pass: super::SgPass);

        pub fn sg_apply_draw_state(ds: *const SgDrawState);
        pub fn sg_apply_uniform_block(stage: super::SgShaderStage,
                                      ub_index: c_int,
                                      data: *const c_void,
                                      num_bytes: c_int);
        pub fn sg_draw(base_element: c_int,
                       num_elements: c_int,
                       num_instances: c_int);

        pub fn sg_begin_default_pass(pass_action: *const SgPassAction,
                                     width: c_int,
                                     height: c_int);
        pub fn sg_begin_pass(pass: super::SgPass,
                             pass_action: *const SgPassAction);
        pub fn sg_apply_viewport(x: c_int, y: c_int,
                                 width: c_int, height: c_int,
                                 origin_top_left: bool);
        pub fn sg_end_pass();

        pub fn sg_commit();
    }
}

/*
    resource id typedefs
*/

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct SgBuffer {
    id: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct SgImage {
    id: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct SgShader {
    id: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct SgPipeline {
    id: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct SgPass {
    id: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct SgContext {
    id: i32,
}

/*
    enums
*/

#[derive(Debug)]
pub enum SgApi {
    Direct3D11,
    Metal,
    OpenGL33,
}

#[repr(C)]
pub enum SgFeature {
    Instancing,
    TextureCompressionDXT,
    TextureCompressionPVRTC,
    TextureCompressionATC,
    TextureCompressionETC2,
    TextureFloat,
    TextureHalfFloat,
    OriginBottomLeft,
    OriginTopLeft,
    MSAARenderTargets,
    PackedVertexFormat102,
    MultipleRenderTarget,
    ImageType3D,
    ImageTypeArray,
}

#[repr(C)]
pub enum SgResourceState {
    Initial,
    Alloc,
    Valid,
    Failed,
    Invalid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SgUsage {
    _Default,
    Immutable,
    Dynamic,
    Stream,
}

impl Default for SgUsage {
    fn default() -> Self {
        SgUsage::_Default
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SgBufferType {
    _Default,
    VertexBuffer,
    IndexBuffer,
}

impl Default for SgBufferType {
    fn default() -> Self {
        SgBufferType::_Default
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SgIndexType {
    _Default,
    None,
    U16,
    U32,
}

impl Default for SgIndexType {
    fn default() -> Self {
        SgIndexType::_Default
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SgImageType {
    _Default,
    _2D,
    Cube,
    _3D,
    Array,
}

impl Default for SgImageType {
    fn default() -> Self {
        SgImageType::_Default
    }
}

#[repr(C)]
pub enum SgCubeFace {
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
}

#[repr(C)]
pub enum SgShaderStage {
    VS,
    FS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SgPixelFormat {
    _Default,
    None,
    RGBA8,
    RGB8,
    RGBA4,
    R5G6B5,
    R5G5B5A1,
    R10G10B10A2,
    RGBA32F,
    RGBA16F,
    R32F,
    R16F,
    L8,
    DXT1,
    DXT3,
    DXT5,
    DEPTH,
    DEPTHSTENCIL,
    PVRTC2RGB,
    PVRTC4RGB,
    PVRTC2RGBA,
    PVRTC4RGBA,
    ETC2RGB8,
    ETC2SRGB8,
}

impl Default for SgPixelFormat {
    fn default() -> Self {
        SgPixelFormat::_Default
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SgPrimitiveType {
    _Default,
    Points,
    Lines,
    LineStrip,
    Triangles,
    TriangleStrip,
}

impl Default for SgPrimitiveType {
    fn default() -> Self {
        SgPrimitiveType::_Default
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SgFilter {
    _Default,
    Nearest,
    Linear,
    NearestMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapNearest,
    LinearMipmapLinear,
}

impl Default for SgFilter {
    fn default() -> Self {
        SgFilter::_Default
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SgWrap {
    _Default,
    Repeat,
    ClampToEdge,
    MirroredRepeat,
}

impl Default for SgWrap {
    fn default() -> Self {
        SgWrap::_Default
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SgVertexFormat {
    _Invalid,
    Float,
    Float2,
    Float3,
    Float4,
    Byte4,
    Byte4N,
    UByte4,
    UByte4N,
    Short2,
    Short2N,
    Short4,
    Short4N,
    UINT10N2,
}

impl Default for SgVertexFormat {
    fn default() -> Self {
        SgVertexFormat::_Invalid
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SgVertexStep {
    _Default,
    PerVertex,
    PerInstance,
}

impl Default for SgVertexStep {
    fn default() -> Self {
        SgVertexStep::_Default
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SgUniformType {
    _Invalid,
    Float,
    Float2,
    Float3,
    Float4,
    Mat4,
}

impl Default for SgUniformType {
    fn default() -> Self {
        SgUniformType::_Invalid
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SgCullMode {
    _Default,
    None,
    Front,
    Back,
}

impl Default for SgCullMode {
    fn default() -> Self {
        SgCullMode::_Default
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SgFaceWinding {
    _Default,
    CCW,
    CW,
}

impl Default for SgFaceWinding {
    fn default() -> Self {
        SgFaceWinding::_Default
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SgCompareFunc {
    _Default,
    Never,
    Less,
    Equal,
    LessEqual,
    Greater,
    NotEqual,
    GreaterEqual,
    Always,
}

impl Default for SgCompareFunc {
    fn default() -> Self {
        SgCompareFunc::_Default
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SgStencilOp {
    _Default,
    Keep,
    Zero,
    Replace,
    IncrClamp,
    DecrClamp,
    Invert,
    IncrWrap,
    DecrWrap,
}

impl Default for SgStencilOp {
    fn default() -> Self {
        SgStencilOp::_Default
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SgBlendFactor {
    _Default,
    Zero,
    One,
    SrcColor,
    OneMinusSrcColor,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstColor,
    OneMinusDstColor,
    DstAlpha,
    OneMinusDstAlpha,
    SrcAlphaSaturated,
    BlendColor,
    OneMinusBlendColor,
    BlendAlpha,
    OneMinusBlendAlpha,
}

impl Default for SgBlendFactor {
    fn default() -> Self {
        SgBlendFactor::_Default
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SgBlendOp {
    _Default,
    Add,
    Subtract,
    ReverseSubtract,
}

impl Default for SgBlendOp {
    fn default() -> Self {
        SgBlendOp::_Default
    }
}

bitflags! {
    #[derive(Default)]
    pub struct SgColorMask: u8 {
        const None = 0x10;
        const R = 0x01;
        const G = 0x02;
        const B = 0x04;
        const A = 0x08;
        const RGB = 0x07;
        const RGBA = 0x0f;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SgAction {
    _Default,
    Clear,
    Load,
    DontCare,
}

impl Default for SgAction {
    fn default() -> SgAction {
        SgAction::_Default
    }
}

/*
    structs
*/

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct SgColorAttachmentAction {
    pub action: SgAction,
    pub val: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct SgDepthAttachmentAction {
    pub action: SgAction,
    pub val: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct SgStencilAttachmentAction {
    pub action: SgAction,
    pub val: u8,
}

#[derive(Default)]
pub struct SgPassAction {
    pub colors: Vec<SgColorAttachmentAction>,
    pub depth: SgDepthAttachmentAction,
    pub stencil: SgStencilAttachmentAction,
}

#[derive(Default)]
pub struct SgDrawState {
    pub pipeline: SgPipeline,
    pub vertex_buffers: Vec<SgBuffer>,
    pub vertex_buffer_offsets: Vec<i32>,
    pub index_buffer: SgBuffer,
    pub index_buffer_offset: i32,
    pub vs_images: Vec<SgImage>,
    pub fs_images: Vec<SgImage>,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct SgDesc {
    pub buffer_pool_size: i32,
    pub image_pool_size: i32,
    pub shader_pool_size: i32,
    pub pipeline_pool_size: i32,
    pub pass_pool_size: i32,
    pub context_pool_size: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct SgBufferDesc {
    pub size: i32,
    pub buffer_type: SgBufferType,
    pub usage: SgUsage,
}

#[derive(Default)]
pub struct SgImageDesc {
    pub image_type: SgImageType,
    pub render_target: bool,
    pub width: i32,
    pub height: i32,
    pub depth_or_layers: i32,
    pub num_mipmaps: i32,
    pub usage: SgUsage,
    pub pixel_format: SgPixelFormat,
    pub sample_count: i32,
    pub min_filter: SgFilter,
    pub mag_filter: SgFilter,
    pub wrap_u: SgWrap,
    pub wrap_v: SgWrap,
    pub wrap_w: SgWrap,
    pub max_anisotropy: u32,
    pub min_lod: f32,
    pub max_lod: f32,
}

#[derive(Default)]
pub struct SgShaderUniformDesc<'a> {
    pub name: &'a str,
    pub uniform_type: SgUniformType,
    pub array_count: i32,
}

#[derive(Default)]
pub struct SgShaderUniformBlockDesc<'a> {
    pub size: i32,
    pub uniforms: Vec<SgShaderUniformDesc<'a>>,
}

#[derive(Default)]
pub struct SgShaderImageDesc<'a> {
    pub name: &'a str,
    pub image_type: SgImageType,
}

#[derive(Default)]
pub struct SgShaderStageDesc<'a> {
    pub source: &'a str,
    pub uniform_blocks: Vec<SgShaderUniformBlockDesc<'a>>,
    pub images: Vec<SgShaderImageDesc<'a>>,
}

pub struct SgShaderDesc<'a> {
    pub vs: SgShaderStageDesc<'a>,
    pub fs: SgShaderStageDesc<'a>,
}

#[repr(C)]
#[derive(Default)]
pub struct SgBufferLayoutDesc {
    pub stride: i32,
    pub step_func: SgVertexStep,
    pub step_rate: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct SgStencilState {
    pub fail_op: SgStencilOp,
    pub depth_fail_op: SgStencilOp,
    pub pass_op: SgStencilOp,
    pub compare_func: SgCompareFunc,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct SgDepthStencilState {
    pub stencil_front: SgStencilState,
    pub stencil_back: SgStencilState,
    pub depth_compare_func: SgCompareFunc,
    pub depth_write_enabled: bool,
    pub stencil_enabled: bool,
    pub stencil_read_mask: u8,
    pub stencil_write_mask: u8,
    pub stencil_ref: u8,
}

#[derive(Copy, Clone, Default)]
pub struct SgBlendState {
    pub enabled: bool,
    pub src_factor_rgb: SgBlendFactor,
    pub dst_factor_rgb: SgBlendFactor,
    pub op_rgb: SgBlendOp,
    pub src_factor_alpha: SgBlendFactor,
    pub dst_factor_alpha: SgBlendFactor,
    pub op_alpha: SgBlendOp,
    pub color_write_mask: SgColorMask,
    pub color_attachment_count: i32,
    pub color_format: SgPixelFormat,
    pub depth_format: SgPixelFormat,
    pub blend_color: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct SgRasterizerState {
    pub alpha_to_coverage_enabled: bool,
    pub cull_mode: SgCullMode,
    pub face_winding: SgFaceWinding,
    pub sample_count: i32,
    pub depth_bias: f32,
    pub depth_bias_slope_scale: f32,
    pub depth_bias_clamp: f32,
}

#[derive(Default)]
pub struct SgVertexAttrDesc<'a> {
    pub name: &'a str,
    pub sem_name: &'a str,
    pub sem_index: i32,
    pub buffer_index: i32,
    pub offset: i32,
    pub format: SgVertexFormat,
}

#[derive(Default)]
pub struct SgLayoutDesc<'a> {
    pub buffers: Vec<SgBufferLayoutDesc>,
    pub attrs: Vec<SgVertexAttrDesc<'a>>,
}

#[derive(Default)]
pub struct SgPipelineDesc<'a> {
    pub shader: SgShader,
    pub layout: SgLayoutDesc<'a>,
    pub primitive_type: SgPrimitiveType,
    pub index_type: SgIndexType,
    pub depth_stencil: SgDepthStencilState,
    pub blend: SgBlendState,
    pub rasterizer: SgRasterizerState,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union SgAttachmentDescValue {
    pub face: i32,
    pub layer: i32,
    pub slice: i32,
}

impl Default for SgAttachmentDescValue {
    fn default() -> Self {
        Self {
            face: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct SgAttachmentDesc {
    pub image: SgImage,
    pub mip_level: i32,
    pub u: SgAttachmentDescValue,
}

#[derive(Default)]
pub struct SgPassDesc {
    pub color_attachments: Vec<SgAttachmentDesc>,
    pub depth_stencil_attachment: SgAttachmentDesc,
}

/*
    functions
*/

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

pub fn sg_setup(desc: &SgDesc) {
    unsafe {
        ffi::sg_setup(&ffi::SgDesc::make(desc));
    }
}

pub fn sg_shutdown() {
    unsafe {
        ffi::sg_shutdown();
    }
}

pub fn sg_isvalid() -> bool {
    unsafe {
        ffi::sg_isvalid()
    }
}

pub fn sg_query_feature(feature: SgFeature) -> bool {
    unsafe {
        ffi::sg_query_feature(feature)
    }
}

pub fn sg_reset_state_cache() {
    unsafe {
        ffi::sg_reset_state_cache();
    }
}

pub fn sg_make_buffer<T>(content: &T, desc: &SgBufferDesc) -> SgBuffer {
    unsafe {
        ffi::sg_make_buffer(&ffi::SgBufferDesc::make(content, desc))
    }
}

pub fn sg_make_image<T>(content: &Vec<(T, i32)>, desc: &SgImageDesc) -> SgImage {
    unsafe {
        ffi::sg_make_image(&ffi::SgImageDesc::make(content, desc))
    }
}

pub fn sg_make_shader(desc: &SgShaderDesc) -> SgShader {
    unsafe {
        ffi::sg_make_shader(&ffi::SgShaderDesc::make(desc))
    }
}

pub fn sg_make_pipeline(desc: &SgPipelineDesc) -> SgPipeline {
    unsafe {
        ffi::sg_make_pipeline(&ffi::SgPipelineDesc::make(desc))
    }
}

pub fn sg_make_pass(desc: &SgPassDesc) -> SgPass {
    unsafe {
        ffi::sg_make_pass(&ffi::SgPassDesc::make(desc))
    }
}

pub fn sg_destroy_buffer(buf: SgBuffer) {
    unsafe {
        ffi::sg_destroy_buffer(buf);
    }
}

pub fn sg_destroy_image(img: SgImage) {
    unsafe {
        ffi::sg_destroy_image(img);
    }
}

pub fn sg_destroy_shader(shd: SgShader) {
    unsafe {
        ffi::sg_destroy_shader(shd);
    }
}

pub fn sg_destroy_pipeline(pip: SgPipeline) {
    unsafe {
        ffi::sg_destroy_pipeline(pip);
    }
}

pub fn sg_destroy_pass(pass: SgPass) {
    unsafe {
        ffi::sg_destroy_pass(pass);
    }
}

pub fn sg_apply_draw_state(ds: &SgDrawState) {
    unsafe {
        ffi::sg_apply_draw_state(&ffi::SgDrawState::make(ds));
    }
}

pub fn sg_apply_uniform_block<T>(stage: SgShaderStage,
                                 ub_index: i32,
                                 data: &T,
                                 num_bytes: i32) {
    let ptr = data as *const T;

    unsafe {
        ffi::sg_apply_uniform_block(stage,
                                    ub_index,
                                    ptr as *const c_void,
                                    num_bytes);
    }
}

pub fn sg_draw(base_element: i32,
               num_elements: i32,
               num_instances: i32) {
    unsafe {
        ffi::sg_draw(base_element, num_elements, num_instances);
    }
}

pub fn sg_begin_default_pass(pass_action: &SgPassAction, width: i32, height: i32) {
    let action = ffi::SgPassAction::make(pass_action);
    unsafe {
        ffi::sg_begin_default_pass(&action, width, height);
    }
}

pub fn sg_begin_pass(pass: &SgPass,
                     pass_action: &SgPassAction) {
    let action = ffi::SgPassAction::make(pass_action);
    unsafe {
        ffi::sg_begin_pass(pass.clone(), &action);
    }
}

pub fn sg_apply_viewport(x: i32, y: i32,
                         width: i32, height: i32,
                         origin_top_left: bool) {
    unsafe {
        ffi::sg_apply_viewport(x, y, width, height, origin_top_left);
    }
}

pub fn sg_end_pass() {
    unsafe {
        ffi::sg_end_pass();
    }
}

pub fn sg_commit() {
    unsafe {
        ffi::sg_commit();
    }
}
