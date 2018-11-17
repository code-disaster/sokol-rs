extern crate imgui_sys;
extern crate sokol;

use std::mem;
use std::os::raw::c_int;
use std::os::raw::c_uchar;
use std::ptr;

use imgui_sys::*;

use sokol::app::*;
use sokol::gfx::*;
use std::slice;

const IMGUI_MAX_VERTICES: usize = 1 << 16;
const IMGUI_MAX_INDICES: usize = IMGUI_MAX_VERTICES * 3;

pub struct ImGuiRenderer {
    //vertices: Vec<ImDrawVert>,
    //indices: Vec<u16>,
    vb: SgBuffer,
    ib: SgBuffer,
    font_image: SgImage,
    shader: SgShader,
    pipeline: SgPipeline,
    draw_state: SgDrawState,
}

impl Drop for ImGuiRenderer {
    fn drop(&mut self) {
        sg_destroy_buffer(self.vb);
        sg_destroy_buffer(self.ib);
        sg_destroy_image(self.font_image);
        sg_destroy_shader(self.shader);
        sg_destroy_pipeline(self.pipeline);
    }
}

pub fn imgui_create_context() {
    unsafe {
        let _ctx = igCreateContext(None, None);

        let io = &mut *igGetIO();

        ImFontAtlas_AddFontDefault(io.fonts, ptr::null());

        io.key_map[ImGuiKey::Tab as usize] = SAppKeycode::KeyTab as i32;
        io.key_map[ImGuiKey::LeftArrow as usize] = SAppKeycode::KeyLeft as i32;
        io.key_map[ImGuiKey::RightArrow as usize] = SAppKeycode::KeyRight as i32;
        io.key_map[ImGuiKey::UpArrow as usize] = SAppKeycode::KeyUp as i32;
        io.key_map[ImGuiKey::DownArrow as usize] = SAppKeycode::KeyDown as i32;
        io.key_map[ImGuiKey::PageUp as usize] = SAppKeycode::KeyPageUp as i32;
        io.key_map[ImGuiKey::PageDown as usize] = SAppKeycode::KeyPageDown as i32;
        io.key_map[ImGuiKey::Home as usize] = SAppKeycode::KeyHome as i32;
        io.key_map[ImGuiKey::End as usize] = SAppKeycode::KeyEnd as i32;
        io.key_map[ImGuiKey::Delete as usize] = SAppKeycode::KeyDelete as i32;
        io.key_map[ImGuiKey::Backspace as usize] = SAppKeycode::KeyBackspace as i32;
        //io.key_map[ImGuiKey::Space as usize] = SAppKeycode::KeySpace as i32;
        io.key_map[ImGuiKey::Enter as usize] = SAppKeycode::KeyEnter as i32;
        io.key_map[ImGuiKey::Escape as usize] = SAppKeycode::KeyEscape as i32;
        io.key_map[ImGuiKey::A as usize] = SAppKeycode::KeyA as i32;
        io.key_map[ImGuiKey::C as usize] = SAppKeycode::KeyC as i32;
        io.key_map[ImGuiKey::V as usize] = SAppKeycode::KeyV as i32;
        io.key_map[ImGuiKey::X as usize] = SAppKeycode::KeyX as i32;
        io.key_map[ImGuiKey::Y as usize] = SAppKeycode::KeyY as i32;
        io.key_map[ImGuiKey::Z as usize] = SAppKeycode::KeyZ as i32;
    }
}

pub fn imgui_setup() -> Box<ImGuiRenderer> {
    //
    // vertex & index buffers
    //
    //let vertices = Vec::with_capacity(IMGUI_MAX_VERTICES);
    //let indices = Vec::with_capacity(IMGUI_MAX_INDICES);

    let vb = sg_make_buffer(None::<&ImDrawVert>, &SgBufferDesc {
        size: IMGUI_MAX_VERTICES * mem::size_of::<ImDrawVert>(),
        buffer_type: SgBufferType::VertexBuffer,
        usage: SgUsage::Stream,
    });

    let ib = sg_make_buffer(None::<&i16>, &SgBufferDesc {
        size: IMGUI_MAX_INDICES * mem::size_of::<i16>(),
        buffer_type: SgBufferType::IndexBuffer,
        usage: SgUsage::Stream,
    });

    //
    // font texture
    //
    let mut font_width: c_int = 0;
    let mut font_height: c_int = 0;
    let mut font_pixels: *mut c_uchar = ptr::null_mut();
    let mut font_bytes_per_pixel: c_int = 0;

    unsafe {
        let io = &*igGetIO();
        ImFontAtlas_GetTexDataAsRGBA32(io.fonts,
                                       &mut font_pixels,
                                       &mut font_width,
                                       &mut font_height,
                                       &mut font_bytes_per_pixel);
    }

    println!("font: {}x{}x{}", font_width, font_height, font_bytes_per_pixel);
    let font_image_size = font_width * font_height * font_bytes_per_pixel;
    let font_image = unsafe { slice::from_raw_parts(font_pixels, font_image_size as usize) };

    let font_image = sg_make_image(&vec![(font_image, font_image_size)], &SgImageDesc {
        image_type: SgImageType::Texture2D,
        width: font_width,
        height: font_height,
        pixel_format: SgPixelFormat::RGBA8,
        wrap_u: SgWrap::ClampToEdge,
        wrap_v: SgWrap::ClampToEdge,
        min_filter: SgFilter::Nearest,
        mag_filter: SgFilter::Nearest,
        ..Default::default()
    });

    let shader = sg_make_shader(&SgShaderDesc {
        vs: SgShaderStageDesc {
            source: Some(
                include_str!("shader/imgui.vert.hlsl")
            ),
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
            source: Some(
                include_str!("shader/imgui.frag.hlsl")
            ),
            images: vec![
                SgShaderImageDesc {
                    name: "tex",
                    image_type: SgImageType::Texture2D,
                }
            ],
            ..Default::default()
        },
    });

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

    Box::new(ImGuiRenderer {
        vb,
        ib,
        font_image,
        shader,
        pipeline,
        draw_state,
    })
}

pub fn imgui_new_frame() {
    unsafe {
        let io = &mut *igGetIO();

        io.display_size.x = sapp_width() as f32;
        io.display_size.y = sapp_height() as f32;

        io.delta_time = 1.0/60.0;

        igNewFrame();

        igSetNextWindowPos(
            ImVec2 { x: 460.0, y: 20.0 },
            ImGuiCond::FirstUseEver,
            ImVec2 { x: 0.0, y: 0.0 }
        );

        let mut show = true;
        igShowDemoWindow(&mut show);
    }
}

pub fn imgui_draw(renderer: &Option<Box<ImGuiRenderer>>) {
    match renderer {
        None => {}
        Some(r) => {
            let draw_data = unsafe {
                igRender();
                &*igGetDrawData()
            };

            imgui_render_draw_data(&r, &draw_data);
        }
    };
}

fn imgui_render_draw_data(renderer: &ImGuiRenderer, draw_data: &ImDrawData) {
    if draw_data.cmd_lists_count == 0 {
        return;
    }

    let mut num_vertices = 0;
    let mut num_indices = 0;
    //let mut num_cmdlists = 0;

    unsafe {
        for cmd_list in draw_data.cmd_lists() {
            let cl = &**cmd_list;

            if (num_vertices + cl.vtx_buffer.size) > IMGUI_MAX_VERTICES as i32
                || (num_indices + cl.idx_buffer.size) > IMGUI_MAX_INDICES as i32 {
                break;
            }

            let vb_offs = sg_append_buffer(renderer.vb,
                                           &*cl.vtx_buffer.data,
                                           cl.vtx_buffer.size * 20);

            num_vertices += cl.vtx_buffer.size;

            let ib_offs = sg_append_buffer(renderer.ib,
                                           &*cl.idx_buffer.data,
                                           cl.idx_buffer.size * 2);

            num_indices += cl.idx_buffer.size;

            let draw_state = SgDrawState {
                pipeline: renderer.draw_state.pipeline,
                vertex_buffers: vec![renderer.draw_state.vertex_buffers[0]],
                vertex_buffer_offsets: vec![vb_offs],
                index_buffer: renderer.draw_state.index_buffer,
                index_buffer_offset: ib_offs,
                vs_images: vec![],
                fs_images: vec![renderer.draw_state.fs_images[0]],
            };

            sg_apply_draw_state(&draw_state);

            let uniforms = ImVec4 {
                x: sapp_width() as f32,
                y: sapp_height() as f32,
                z: 0.0,
                w: 0.0
            };

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

            //num_cmdlists += 1;
        }
    }
}