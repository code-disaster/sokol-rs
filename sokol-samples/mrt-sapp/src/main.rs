use std::mem;

use cgmath::*;
use cgmath::conv::*;

use sokol::app::*;
use sokol::gfx::*;

const MSAA_SAMPLES: i32 = 4;

struct MRT {
    offscreen_pass_desc: SgPassDesc,
    offscreen_pass: SgPass,
    offscreen_draw_state: SgDrawState,
    fsq_draw_state: SgDrawState,
    dbg_draw_state: SgDrawState,
    offscreen_pass_action: SgPassAction,
    default_pass_action: SgPassAction,
    rx: f32,
    ry: f32,
}

impl MRT {
    fn create_offscreen_pass(&mut self, width: i32, height: i32) {
        sg_destroy_pass(self.offscreen_pass);
        for att in &self.offscreen_pass_desc.color_attachments {
            sg_destroy_image(att.image);
        }
        sg_destroy_image(self.offscreen_pass_desc.depth_stencil_attachment.image);

        let offscreen_sample_count = if sg_query_feature(SgFeature::MSAARenderTargets) {
            MSAA_SAMPLES
        } else {
            1
        };

        let empty: Vec<(u8, i32)> = vec![];

        let color_img_desc = SgImageDesc {
            render_target: true,
            width,
            height,
            min_filter: SgFilter::Linear,
            mag_filter: SgFilter::Linear,
            wrap_u: SgWrap::ClampToEdge,
            wrap_v: SgWrap::ClampToEdge,
            sample_count: offscreen_sample_count,
            ..Default::default()
        };
        let depth_img_desc = SgImageDesc {
            pixel_format: SgPixelFormat::DEPTH,
            ..color_img_desc
        };
        self.offscreen_pass_desc = SgPassDesc {
            color_attachments: vec![
                SgAttachmentDesc {
                    image: sg_make_image(&empty, &color_img_desc),
                    ..Default::default()
                },
                SgAttachmentDesc {
                    image: sg_make_image(&empty, &color_img_desc),
                    ..Default::default()
                },
                SgAttachmentDesc {
                    image: sg_make_image(&empty, &color_img_desc),
                    ..Default::default()
                },
            ],
            depth_stencil_attachment: SgAttachmentDesc {
                image: sg_make_image(&empty, &depth_img_desc),
                ..Default::default()
            },
        };
        self.offscreen_pass = sg_make_pass(&self.offscreen_pass_desc);

        self.fsq_draw_state.fs_images.clear();
        for att in &self.offscreen_pass_desc.color_attachments {
            self.fsq_draw_state.fs_images.push(att.image);
        }
    }
}

impl SappCallbacks for MRT {
    fn sapp_init(&mut self) {
        sg_setup(&SgDesc {
            ..Default::default()
        });

        self.create_offscreen_pass(sapp_width(), sapp_height());

        let cube_vertices: [f32; 96] = [
            -1.0, -1.0, -1.0, 1.0,
            1.0, -1.0, -1.0, 1.0,
            1.0, 1.0, -1.0, 1.0,
            -1.0, 1.0, -1.0, 1.0,
            -1.0, -1.0, 1.0, 0.8,
            1.0, -1.0, 1.0, 0.8,
            1.0, 1.0, 1.0, 0.8,
            -1.0, 1.0, 1.0, 0.8,
            -1.0, -1.0, -1.0, 0.6,
            -1.0, 1.0, -1.0, 0.6,
            -1.0, 1.0, 1.0, 0.6,
            -1.0, -1.0, 1.0, 0.6,
            1.0, -1.0, -1.0, 0.4,
            1.0, 1.0, -1.0, 0.4,
            1.0, 1.0, 1.0, 0.4,
            1.0, -1.0, 1.0, 0.4,
            -1.0, -1.0, -1.0, 0.5,
            -1.0, -1.0, 1.0, 0.5,
            1.0, -1.0, 1.0, 0.5,
            1.0, -1.0, -1.0, 0.5,
            -1.0, 1.0, -1.0, 0.7,
            -1.0, 1.0, 1.0, 0.7,
            1.0, 1.0, 1.0, 0.7,
            1.0, 1.0, -1.0, 0.7,
        ];

        let cube_vbuf = sg_make_buffer(
            &cube_vertices,
            &SgBufferDesc {
                size: mem::size_of_val(&cube_vertices) as i32,
                ..Default::default()
            },
        );

        let cube_indices: [u16; 36] = [
            0, 1, 2, 0, 2, 3,
            6, 5, 4, 7, 6, 4,
            8, 9, 10, 8, 10, 11,
            14, 13, 12, 15, 14, 12,
            16, 17, 18, 16, 18, 19,
            22, 21, 20, 23, 22, 20
        ];

        let cube_ibuf = sg_make_buffer(
            &cube_indices,
            &SgBufferDesc {
                size: mem::size_of_val(&cube_indices) as i32,
                buffer_type: SgBufferType::IndexBuffer,
                ..Default::default()
            },
        );

        let (cube_vs_src, cube_fs_src) = match sg_api() {
            SgApi::Direct3D11 => (
                "",
                ""
            ),
            SgApi::Metal => (
                "",
                ""
            ),
            SgApi::OpenGL33 => (
                "#version 330
                uniform mat4 mvp;
                in vec4 position;
                in float bright0;
                out float bright;
                void main() {
                  gl_Position = mvp * position;
                  bright = bright0;
                }",
                "#version 330
                in float bright;
                layout(location=0) out vec4 frag_color_0;
                layout(location=1) out vec4 frag_color_1;
                layout(location=2) out vec4 frag_color_2;
                void main() {
                  frag_color_0 = vec4(bright, 0.0, 0.0, 1.0);
                  frag_color_1 = vec4(0.0, bright, 0.0, 1.0);
                  frag_color_2 = vec4(0.0, 0.0, bright, 1.0);
                }"
            )
        };

        let cube_shd = sg_make_shader(
            &SgShaderDesc {
                vs: SgShaderStageDesc {
                    source: cube_vs_src,
                    uniform_blocks: vec!(
                        SgShaderUniformBlockDesc {
                            size: 64,
                            uniforms: vec!(
                                SgShaderUniformDesc {
                                    name: "mvp",
                                    uniform_type: SgUniformType::Mat4,
                                    ..Default::default()
                                }
                            ),
                        }
                    ),
                    ..Default::default()
                },
                fs: SgShaderStageDesc {
                    source: cube_fs_src,
                    ..Default::default()
                },
            },
        );

        let cube_pip = sg_make_pipeline(
            &SgPipelineDesc {
                layout: SgLayoutDesc {
                    buffers: vec!(
                        SgBufferLayoutDesc {
                            stride: 16,
                            ..Default::default()
                        }
                    ),
                    attrs: vec!(
                        SgVertexAttrDesc {
                            name: "position",
                            sem_name: "POS",
                            format: SgVertexFormat::Float3,
                            offset: 0,
                            ..Default::default()
                        },
                        SgVertexAttrDesc {
                            name: "bright0",
                            sem_name: "BRIGHT",
                            format: SgVertexFormat::Float,
                            offset: 12,
                            ..Default::default()
                        },
                    ),
                },
                shader: cube_shd,
                index_type: SgIndexType::U16,
                depth_stencil: SgDepthStencilState {
                    depth_compare_func: SgCompareFunc::LessEqual,
                    depth_write_enabled: true,
                    ..Default::default()
                },
                blend: SgBlendState {
                    color_attachment_count: 3,
                    color_format: SgPixelFormat::RGBA8,
                    depth_format: SgPixelFormat::DEPTH,
                    ..Default::default()
                },
                rasterizer: SgRasterizerState {
                    cull_mode: SgCullMode::Back,
                    sample_count: MSAA_SAMPLES,
                    ..Default::default()
                },
                ..Default::default()
            }
        );

        self.offscreen_draw_state = SgDrawState {
            pipeline: cube_pip,
            vertex_buffers: vec!(cube_vbuf),
            index_buffer: cube_ibuf,
            ..Default::default()
        };

        let quad_vertices: [f32; 8] = [
            0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0
        ];

        let quad_vbuf = sg_make_buffer(
            &quad_vertices,
            &SgBufferDesc {
                size: mem::size_of_val(&quad_vertices) as i32,
                ..Default::default()
            },
        );

        let (fsq_vs_src, fsq_fs_src) = match sg_api() {
            SgApi::Direct3D11 => (
                "",
                ""
            ),
            SgApi::Metal => (
                "",
                ""
            ),
            SgApi::OpenGL33 => (
                "#version 330
                uniform vec2 offset;
                in vec2 pos;
                out vec2 uv0;
                out vec2 uv1;
                out vec2 uv2;
                void main() {
                  gl_Position = vec4(pos*2.0-1.0, 0.5, 1.0);
                  uv0 = pos + vec2(offset.x, 0.0);
                  uv1 = pos + vec2(0.0, offset.y);
                  uv2 = pos;
                }",
                "#version 330
                uniform sampler2D tex0;
                uniform sampler2D tex1;
                uniform sampler2D tex2;
                in vec2 uv0;
                in vec2 uv1;
                in vec2 uv2;
                out vec4 frag_color;
                void main() {
                  vec3 c0 = texture(tex0, uv0).xyz;
                  vec3 c1 = texture(tex1, uv1).xyz;
                  vec3 c2 = texture(tex2, uv2).xyz;
                  frag_color = vec4(c0 + c1 + c2, 1.0);
                }"
            )
        };

        let fsq_shd = sg_make_shader(
            &SgShaderDesc {
                vs: SgShaderStageDesc {
                    source: fsq_vs_src,
                    uniform_blocks: vec!(
                        SgShaderUniformBlockDesc {
                            size: 8,
                            uniforms: vec!(
                                SgShaderUniformDesc {
                                    name: "offset",
                                    uniform_type: SgUniformType::Float2,
                                    ..Default::default()
                                }
                            ),
                        }
                    ),
                    ..Default::default()
                },
                fs: SgShaderStageDesc {
                    source: fsq_fs_src,
                    images: vec![
                        SgShaderImageDesc {
                            name: "tex0",
                            image_type: SgImageType::_2D,
                        },
                        SgShaderImageDesc {
                            name: "tex1",
                            image_type: SgImageType::_2D,
                        },
                        SgShaderImageDesc {
                            name: "tex2",
                            image_type: SgImageType::_2D,
                        },
                    ],
                    ..Default::default()
                },
            },
        );

        let fsq_pip = sg_make_pipeline(
            &SgPipelineDesc {
                layout: SgLayoutDesc {
                    attrs: vec![
                        SgVertexAttrDesc {
                            name: "pos",
                            sem_name: "POSITION",
                            format: SgVertexFormat::Float2,
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                },
                shader: fsq_shd,
                primitive_type: SgPrimitiveType::TriangleStrip,
                rasterizer: SgRasterizerState {
                    sample_count: MSAA_SAMPLES,
                    ..Default::default()
                },
                ..Default::default()
            }
        );

        self.fsq_draw_state = SgDrawState {
            pipeline: fsq_pip,
            vertex_buffers: vec!(quad_vbuf),
            fs_images: vec![
                self.offscreen_pass_desc.color_attachments[0].image,
                self.offscreen_pass_desc.color_attachments[1].image,
                self.offscreen_pass_desc.color_attachments[2].image,
            ],
            ..Default::default()
        };

        let (dbg_vs_src, dbg_fs_src) = match sg_api() {
            SgApi::Direct3D11 => (
                "",
                ""
            ),
            SgApi::Metal => (
                "",
                ""
            ),
            SgApi::OpenGL33 => (
                "#version 330
                in vec2 pos;
                out vec2 uv;
                void main() {
                  gl_Position = vec4(pos*2.0-1.0, 0.5, 1.0);
                  uv = pos;
                }",
                "#version 330
                uniform sampler2D tex;
                in vec2 uv;
                out vec4 frag_color;
                void main() {
                  frag_color = vec4(texture(tex,uv).xyz, 1.0);
                }"
            )
        };

        self.dbg_draw_state = SgDrawState {
            pipeline: sg_make_pipeline(&SgPipelineDesc {
                layout: SgLayoutDesc {
                    attrs: vec![
                        SgVertexAttrDesc {
                            name: "pos",
                            sem_name: "POSITION",
                            format: SgVertexFormat::Float2,
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                },
                primitive_type: SgPrimitiveType::TriangleStrip,
                shader: sg_make_shader(&SgShaderDesc {
                    vs: SgShaderStageDesc {
                        source: dbg_vs_src,
                        ..Default::default()
                    },
                    fs: SgShaderStageDesc {
                        source: dbg_fs_src,
                        images: vec![
                            SgShaderImageDesc {
                                name: "tex",
                                image_type: SgImageType::_2D,
                            },
                        ],
                        ..Default::default()
                    },
                }),
                rasterizer: SgRasterizerState {
                    sample_count: MSAA_SAMPLES,
                    ..Default::default()
                },
                ..Default::default()
            }),
            vertex_buffers: vec![quad_vbuf],
            ..Default::default()
        };
    }

    fn sapp_frame(&mut self) {
        let w: f32 = sapp_width() as f32;
        let h: f32 = sapp_height() as f32;

        let proj = perspective(Deg(60.0), w / h, 0.01, 10.0);
        let view = Matrix4::look_at(
            Point3 { x: 0.0, y: 1.5, z: 6.0 },
            Point3 { x: 0.0, y: 0.0, z: 0.0 },
            Vector3 { x: 0.0, y: 1.0, z: 0.0 },
        );
        let view_proj: Matrix4<f32> = proj * view;

        self.rx += 1.0;
        self.ry += 2.0;
        let rxm = Matrix4::from_angle_x(Deg(self.rx));
        let rym = Matrix4::from_angle_y(Deg(self.ry));
        let model = rxm * rym;

        let mvp: [[f32; 4]; 4] = array4x4(view_proj * model);

        sg_begin_pass(&self.offscreen_pass, &self.offscreen_pass_action);
        sg_apply_draw_state(&self.offscreen_draw_state);
        sg_apply_uniform_block(SgShaderStage::VS, 0, &mvp, 64);
        sg_draw(0, 36, 1);
        sg_end_pass();

        let offset: [f32; 2] = [
            (self.rx * 0.01).sin() * 0.1, (self.ry * 0.01).sin() * 0.1
        ];

        sg_begin_default_pass(&self.default_pass_action, sapp_width(), sapp_height());
        sg_apply_draw_state(&self.fsq_draw_state);
        sg_apply_uniform_block(SgShaderStage::VS, 0, &offset, 8);
        sg_draw(0, 4, 1);

        for i in 0..3 {
            sg_apply_viewport(i * 100, 0, 100, 100, false);
            self.dbg_draw_state.fs_images = vec![self.offscreen_pass_desc.color_attachments[i as usize].image];
            sg_apply_draw_state(&self.dbg_draw_state);
            sg_draw(0, 4, 1);
        }

        sg_end_pass();
        sg_commit();
    }

    fn sapp_cleanup(&mut self) {
        sg_shutdown();
    }
}

fn main() {
    let mrt_app = MRT {
        offscreen_pass_desc: Default::default(),
        offscreen_pass: Default::default(),
        offscreen_draw_state: Default::default(),
        fsq_draw_state: Default::default(),
        dbg_draw_state: Default::default(),
        offscreen_pass_action: SgPassAction {
            colors: vec!(
                SgColorAttachmentAction {
                    action: SgAction::Clear,
                    val: [0.25, 0.0, 0.0, 1.0],
                },
                SgColorAttachmentAction {
                    action: SgAction::Clear,
                    val: [0.0, 0.25, 0.0, 1.0],
                },
                SgColorAttachmentAction {
                    action: SgAction::Clear,
                    val: [0.0, 0.0, 0.25, 1.0],
                },
            ),
            ..Default::default()
        },
        default_pass_action: SgPassAction {
            ..Default::default()
        },
        rx: 0.0,
        ry: 0.0,
    };

    let title = format!("mrt-sapp.rs ({:?})", sg_api());

    let exit_code = sapp_main(
        mrt_app,
        SappDesc {
            width: 800,
            height: 600,
            sample_count: MSAA_SAMPLES,
            window_title: title,
            ..Default::default()
        });

    std::process::exit(exit_code);
}
