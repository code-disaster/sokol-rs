extern crate nalgebra_glm as glm;
extern crate sokol;

use std::f32::consts::PI;
use std::mem;

use sokol::app::*;
use sokol::gfx::*;

const SAMPLE_COUNT: i32 = 4;

#[derive(Default)]
struct Cube {
    pipeline: SgPipeline,
    bindings: SgBindings,
    rx: f32,
    ry: f32,
}

impl SApp for Cube {
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
            Some(&vertices),
            &SgBufferDesc {
                size: mem::size_of_val(&vertices),
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
            Some(&indices),
            &SgBufferDesc {
                size: mem::size_of_val(&indices),
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
                    source: Some(vs_src),
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
                    source: Some(fs_src),
                    ..Default::default()
                },
            },
        );

        self.pipeline = sg_make_pipeline(
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
                index_type: SgIndexType::UInt16,
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

        self.bindings = SgBindings {
            vertex_buffers: vec!(vbuf),
            index_buffer: ibuf,
            ..Default::default()
        };
    }

    fn sapp_frame(&mut self) {
        let pass_action = SgPassAction {
            colors: vec!(
                SgColorAttachmentAction {
                    action: SgAction::Clear,
                    val: [0.25, 0.5, 0.75, 1.0],
                }
            ),
            ..Default::default()
        };

        let w: f32 = sapp_width() as f32;
        let h: f32 = sapp_height() as f32;

        let proj = glm::perspective(w / h, 60.0 * PI / 180.0, 0.01, 10.0);
        let view = glm::look_at(
            &glm::vec3(0.0, 1.5, 6.0),
            &glm::vec3(0.0, 0.0, 0.0),
            &glm::vec3(0.0, 1.0, 0.0),
        );
        let view_proj = proj * view;

        self.rx += 1.0;
        self.ry += 2.0;
        let rxm = glm::rotation(self.rx * PI / 180.0, &glm::vec3(1.0, 0.0, 0.0));
        let rym = glm::rotation(self.ry * PI / 180.0, &glm::vec3(0.0, 1.0, 0.0));
        let model = rxm * rym;

        let mvp: [[f32; 4]; 4] = (view_proj * model).into();

        sg_begin_default_pass(&pass_action, sapp_width(), sapp_height());
        sg_apply_pipeline(self.pipeline);
        sg_apply_bindings(&self.bindings);
        sg_apply_uniforms(
            SgShaderStage::Vertex,
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

    fn sapp_event(&mut self, _event: SAppEvent) {}
}

fn main() {
    let cube: Cube = Cube {
        ..Default::default()
    };

    let title = format!("cube-sapp.rs ({:?})", sg_api());

    let exit_code = sapp_run(
        cube,
        SAppDesc {
            width: 800,
            height: 600,
            sample_count: SAMPLE_COUNT,
            window_title: title,
            ..Default::default()
        },
    );

    std::process::exit(exit_code);
}
