//! sokol_imgui::gfx - Dear ImGui renderer using the sokol::gfx API

extern crate imgui_sys;
extern crate sokol;

use std::mem;
use std::os::raw::c_int;
use std::os::raw::c_uchar;
use std::ptr;

use imgui_sys::*;

use sokol::app::*;
use sokol::gfx::*;

#[derive(Default)]
pub struct SgImGui {
    vb: SgBuffer,
    ib: SgBuffer,
    font_image: SgImage,
    shader: SgShader,
    pipeline: SgPipeline,
    draw_state: SgDrawState,
}

pub fn sg_imgui_setup(max_vertices: usize) -> SgImGui {
    //
    // vertex & index buffers
    //
    let vb = sg_make_buffer(SG_BUFFER_CONTENT_NONE, &SgBufferDesc {
        size: max_vertices * mem::size_of::<ImDrawVert>(),
        buffer_type: SgBufferType::VertexBuffer,
        usage: SgUsage::Stream,
    });

    let ib = sg_make_buffer(SG_BUFFER_CONTENT_NONE, &SgBufferDesc {
        size: max_vertices * 3 * mem::size_of::<i16>(),
        buffer_type: SgBufferType::IndexBuffer,
        usage: SgUsage::Stream,
    });

    //
    // font texture
    //
    let mut font_width: c_int = 0;
    let mut font_height: c_int = 0;
    let mut font_pixels: *mut c_uchar = ptr::null_mut();

    unsafe {
        let io = &*igGetIO();
        ImFontAtlas_GetTexDataAsRGBA32(io.fonts,
                                       &mut font_pixels,
                                       &mut font_width,
                                       &mut font_height,
                                       ptr::null_mut());
    }

    let font_image_size = font_width * font_height * 4;
    let subimages = vec![(font_pixels as *const u8, font_image_size)];

    let font_image = sg_make_image(
        Some(&subimages),
        &SgImageDesc {
            image_type: SgImageType::Texture2D,
            width: font_width,
            height: font_height,
            pixel_format: SgPixelFormat::RGBA8,
            wrap_u: SgWrap::ClampToEdge,
            wrap_v: SgWrap::ClampToEdge,
            min_filter: SgFilter::Nearest,
            mag_filter: SgFilter::Nearest,
            ..Default::default()
        },
    );

    //
    // shader
    //
    let api = sg_api();
    let shader = sg_make_shader(&SgShaderDesc {
        vs: SgShaderStageDesc {
            source: match api {
                SgApi::OpenGL33 => Some(include_str!("shader/imgui.vert.glsl")),
                _ => None,
            },
            byte_code: match api {
                SgApi::Direct3D11 => Some(include_bytes!("shader/imgui.vert.fxc")),
                SgApi::Metal => Some(include_bytes!("shader/imgui.vert.metallib")),
                _ => None,
            },
            entry: if api == SgApi::Metal { Some("main0") } else { None },
            uniform_blocks: vec![
                SgShaderUniformBlockDesc {
                    size: 16,
                    uniforms: vec![
                        SgShaderUniformDesc {
                            name: "Uniforms",
                            uniform_type: SgUniformType::Float4,
                            array_count: 1,
                        },
                    ],
                },
            ],
            ..Default::default()
        },
        fs: SgShaderStageDesc {
            source: match api {
                SgApi::OpenGL33 => Some(include_str!("shader/imgui.frag.glsl")),
                _ => None,
            },
            byte_code: match api {
                SgApi::Direct3D11 => Some(include_bytes!("shader/imgui.frag.fxc")),
                SgApi::Metal => Some(include_bytes!("shader/imgui.frag.metallib")),
                _ => None,
            },
            entry: if api == SgApi::Metal { Some("main0") } else { None },
            images: vec![
                SgShaderImageDesc {
                    name: "tex",
                    image_type: SgImageType::Texture2D,
                }
            ],
            ..Default::default()
        },
    });

    //
    // pipeline and draw state
    //
    let pipeline = sg_make_pipeline(&SgPipelineDesc {
        shader,
        layout: SgLayoutDesc {
            buffers: vec![
                SgBufferLayoutDesc {
                    stride: mem::size_of::<ImDrawVert>(),
                    ..Default::default()
                }
            ],
            attrs: vec![
                SgVertexAttrDesc {
                    name: "position",
                    sem_name: "TEXCOORD",
                    sem_index: 0,
                    format: SgVertexFormat::Float2,
                    ..Default::default()
                },
                SgVertexAttrDesc {
                    name: "texcoord0",
                    sem_name: "TEXCOORD",
                    sem_index: 1,
                    format: SgVertexFormat::Float2,
                    ..Default::default()
                },
                SgVertexAttrDesc {
                    name: "color0",
                    sem_name: "TEXCOORD",
                    sem_index: 2,
                    format: SgVertexFormat::UByte4N,
                    ..Default::default()
                },
            ],
        },
        index_type: SgIndexType::UInt16,
        depth_stencil: SgDepthStencilState {
            depth_compare_func: SgCompareFunc::Always,
            depth_write_enabled: true,
            ..Default::default()
        },
        blend: SgBlendState {
            enabled: true,
            src_factor_rgb: SgBlendFactor::SrcAlpha,
            dst_factor_rgb: SgBlendFactor::OneMinusSrcAlpha,
            color_write_mask: SgColorMask::RGB,
            color_attachment_count: 0,
            ..Default::default()
        },
        rasterizer: SgRasterizerState {
            cull_mode: SgCullMode::None,
            ..Default::default()
        },
        ..Default::default()
    });

    let draw_state = SgDrawState {
        vertex_buffers: vec![vb],
        index_buffer: ib,
        fs_images: vec![font_image],
        pipeline,
        ..Default::default()
    };

    SgImGui {
        vb,
        ib,
        font_image,
        shader,
        pipeline,
        draw_state,
    }
}

pub fn sg_imgui_shutdown(ui: &SgImGui) {
    sg_destroy_buffer(ui.vb);
    sg_destroy_buffer(ui.ib);
    sg_destroy_image(ui.font_image);
    sg_destroy_shader(ui.shader);
    sg_destroy_pipeline(ui.pipeline);
}

pub fn sg_imgui_draw(ui: &SgImGui) {
    let draw_data = unsafe {
        igRender();
        &*igGetDrawData()
    };

    if draw_data.cmd_lists_count == 0 {
        return;
    }

    let uniforms = ImVec4 {
        x: sapp_width() as f32,
        y: sapp_height() as f32,
        z: 0.0,
        w: 0.0,
    };

    unsafe {
        for cmd_list in draw_data.cmd_lists() {
            let cl = &**cmd_list;

            let vb_offs = sg_append_buffer(ui.vb,
                                           &*cl.vtx_buffer.data,
                                           cl.vtx_buffer.size * 20);

            let ib_offs = sg_append_buffer(ui.ib,
                                           &*cl.idx_buffer.data,
                                           cl.idx_buffer.size * 2);

            if sg_query_buffer_overflow(ui.vb)
                || sg_query_buffer_overflow(ui.ib) {
                continue;
            }

            let draw_state = SgDrawState {
                pipeline: ui.draw_state.pipeline,
                vertex_buffers: vec![ui.draw_state.vertex_buffers[0]],
                vertex_buffer_offsets: vec![vb_offs],
                index_buffer: ui.draw_state.index_buffer,
                index_buffer_offset: ib_offs,
                vs_images: vec![],
                fs_images: vec![ui.draw_state.fs_images[0]],
            };

            sg_apply_draw_state(&draw_state);

            sg_apply_uniform_block(SgShaderStage::Vertex, 0, &uniforms, 16);

            let mut base_element: i32 = 0;
            for cmd in cl.cmd_buffer.as_slice() {
                sg_apply_scissor_rect(cmd.clip_rect.x as i32,
                                      cmd.clip_rect.y as i32,
                                      (cmd.clip_rect.z - cmd.clip_rect.x) as i32,
                                      (cmd.clip_rect.w - cmd.clip_rect.y) as i32,
                                      true);

                sg_draw(base_element, cmd.elem_count as i32, 1);

                base_element += cmd.elem_count as i32;
            }
        }
    }
}
