use std::mem;

use cgmath::*;
use cgmath::conv::*;

use sokol::app::*;
use sokol::gfx::*;

const SAMPLE_COUNT: i32 = 4;

#[derive(Default)]
struct Cube {
    draw_state: SgDrawState,
    rx: f32,
    ry: f32,
}

impl SappCallbacks for Cube {
    fn sapp_init(&mut self) {
        sg_setup(&SgDesc {
            ..Default::default()
        });

        let vertices: [f32; 168] = [
            -1.0, -1.0, -1.0, 1.0, 0.0, 0.0, 1.0,
            1.0, -1.0, -1.0, 1.0, 0.0, 0.0, 1.0,
            1.0, 1.0, -1.0, 1.0, 0.0, 0.0, 1.0,
            -1.0, 1.0, -1.0, 1.0, 0.0, 0.0, 1.0,
            -1.0, -1.0, 1.0, 0.0, 1.0, 0.0, 1.0,
            1.0, -1.0, 1.0, 0.0, 1.0, 0.0, 1.0,
            1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0,
            -1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0,
            -1.0, -1.0, -1.0, 0.0, 0.0, 1.0, 1.0,
            -1.0, 1.0, -1.0, 0.0, 0.0, 1.0, 1.0,
            -1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0,
            -1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 1.0,
            1.0, -1.0, -1.0, 1.0, 0.5, 0.0, 1.0,
            1.0, 1.0, -1.0, 1.0, 0.5, 0.0, 1.0,
            1.0, 1.0, 1.0, 1.0, 0.5, 0.0, 1.0,
            1.0, -1.0, 1.0, 1.0, 0.5, 0.0, 1.0,
            -1.0, -1.0, -1.0, 0.0, 0.5, 1.0, 1.0,
            -1.0, -1.0, 1.0, 0.0, 0.5, 1.0, 1.0,
            1.0, -1.0, 1.0, 0.0, 0.5, 1.0, 1.0,
            1.0, -1.0, -1.0, 0.0, 0.5, 1.0, 1.0,
            -1.0, 1.0, -1.0, 1.0, 0.0, 0.5, 1.0,
            -1.0, 1.0, 1.0, 1.0, 0.0, 0.5, 1.0,
            1.0, 1.0, 1.0, 1.0, 0.0, 0.5, 1.0,
            1.0, 1.0, -1.0, 1.0, 0.0, 0.5, 1.0
        ];

        let vbuf = sg_make_buffer(
            &vertices,
            &SgBufferDesc {
                size: mem::size_of_val(&vertices) as i32,
                buffer_type: SgBufferType::VertexBuffer,
                usage: SgUsage::Immutable,
            },
        );

        let indices: [u16; 36] = [
            0, 1, 2, 0, 2, 3,
            6, 5, 4, 7, 6, 4,
            8, 9, 10, 8, 10, 11,
            14, 13, 12, 15, 14, 12,
            16, 17, 18, 16, 18, 19,
            22, 21, 20, 23, 22, 20
        ];

        let ibuf = sg_make_buffer(
            &indices,
            &SgBufferDesc {
                size: mem::size_of_val(&indices) as i32,
                buffer_type: SgBufferType::IndexBuffer,
                ..Default::default()
            },
        );

        let (vs_src, fs_src) = match sg_api() {
            SgApi::Direct3D11 => (
                "cbuffer params: register(b0) {
                  float4x4 mvp;
                };
                struct vs_in {
                  float4 pos: POS;
                  float4 color: COLOR0;
                };
                struct vs_out {
                  float4 color: COLOR0;
                  float4 pos: SV_Position;
                };
                vs_out main(vs_in inp) {
                  vs_out outp;
                  outp.pos = mul(mvp, inp.pos);
                  outp.color = inp.color;
                  return outp;
                }",
                "float4 main(float4 color: COLOR0): SV_Target0 {
                  return color;
                }"
            ),
            SgApi::Metal => (
                "#include <metal_stdlib>
                using namespace metal;
                struct params_t {
                  float4x4 mvp;
                };
                struct vs_in {
                  float4 position [[attribute(0)]];
                  float4 color [[attribute(1)]];
                };
                struct vs_out {
                  float4 pos [[position]];
                  float4 color;
                };
                vertex vs_out _main(vs_in in [[stage_in]], constant params_t& params [[buffer(0)]]) {
                  vs_out out;
                  out.pos = params.mvp * in.position;
                  out.color = in.color;
                  return out;
                }",
                "#include <metal_stdlib>
                using namespace metal;
                fragment float4 _main(float4 color [[stage_in]]) {
                  return color;
                }"
            ),
            SgApi::OpenGL33 => (
                "#version 330
                uniform mat4 mvp;
                in vec4 position;
                in vec4 color0;
                out vec4 color;
                void main() {
                  gl_Position = mvp * position;
                  color = color0;
                }",
                "#version 330
                in vec4 color;
                out vec4 frag_color;
                void main() {
                  frag_color = color;
                }"
            )
        };

        let shd = sg_make_shader(
            &SgShaderDesc {
                vs: SgShaderStageDesc {
                    source: vs_src,
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
                    source: fs_src,
                    ..Default::default()
                },
            },
        );

        let pip = sg_make_pipeline(
            &SgPipelineDesc {
                layout: SgLayoutDesc {
                    buffers: vec!(
                        SgBufferLayoutDesc {
                            stride: 28,
                            ..Default::default()
                        }
                    ),
                    attrs: vec!(
                        SgVertexAttrDesc {
                            name: "position",
                            sem_name: "POS",
                            format: SgVertexFormat::Float3,
                            ..Default::default()
                        },
                        SgVertexAttrDesc {
                            name: "color0",
                            sem_name: "COLOR",
                            format: SgVertexFormat::Float4,
                            ..Default::default()
                        },
                    ),
                },
                shader: shd,
                index_type: SgIndexType::U16,
                depth_stencil: SgDepthStencilState {
                    depth_compare_func: SgCompareFunc::LessEqual,
                    depth_write_enabled: true,
                    ..Default::default()
                },
                rasterizer: SgRasterizerState {
                    cull_mode: SgCullMode::Back,
                    sample_count: SAMPLE_COUNT,
                    ..Default::default()
                },
                ..Default::default()
            }
        );

        self.draw_state = SgDrawState {
            pipeline: pip,
            vertex_buffers: vec!(vbuf),
            index_buffer: ibuf,
            ..Default::default()
        };
    }

    fn sapp_frame(&mut self) {
        let pass_action =
            SgPassAction::color(
                SgColorAttachmentAction::clear(
                    [0.25, 0.5, 0.75, 1.0]
                )
            );

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

        let mvp = array4x4(view_proj * model);

        sg_begin_default_pass(&pass_action, sapp_width(), sapp_height());
        sg_apply_draw_state(&self.draw_state);
        sg_apply_uniform_block(
            SgShaderStage::VS,
            0,
            &mvp,
            64);
        sg_draw(0, 36, 1);
        sg_end_pass();
        sg_commit();
    }

    fn sapp_cleanup(&mut self) {
        sg_shutdown();
    }
}

fn main() {
    let cube: Cube = Cube {
        ..Default::default()
    };

    let title = format!("cube-sapp.rs ({:?})", sg_api());

    let exit_code = sapp_main(
        cube,
        SappDesc {
            width: 800,
            height: 600,
            window_title: title,
        },
    );

    std::process::exit(exit_code);
}
