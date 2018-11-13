use sokol::app::*;
use sokol::audio::*;
use sokol::gfx::*;

const NUM_SAMPLES: usize = 32;

struct SAudio {
    even_odd: u32,
    sample_pos: i32,
    samples: [f32; NUM_SAMPLES],
}

impl SappCallbacks for SAudio {
    fn sapp_init(&mut self) {
        sg_setup(&SgDesc {
            ..Default::default()
        });
        saudio_setup(&SAudioDesc {
            ..Default::default()
        });
    }

    fn sapp_frame(&mut self) {
        let pass_action = SgPassAction {
            colors: vec![
                SgColorAttachmentAction {
                    action: SgAction::Clear,
                    val: [1.0, 0.5, 0.0, 1.0],
                }
            ],
            ..Default::default()
        };

        sg_begin_default_pass(&pass_action, sapp_width(), sapp_height());

        let num_frames = saudio_expect();
        let mut s: f32;
        for _i in 0..num_frames {
            if (self.even_odd & (1 << 5)) != 0 {
                s = 0.05;
            } else {
                s = -0.05;
            }
            self.even_odd += 1;
            self.samples[self.sample_pos as usize] = s;
            self.sample_pos += 1;
            if self.sample_pos == NUM_SAMPLES as i32 {
                self.sample_pos = 0;
                saudio_push(&self.samples, NUM_SAMPLES as i32);
            }
        }

        sg_end_pass();
        sg_commit();
    }

    fn sapp_cleanup(&mut self) {
        saudio_shutdown();
        sg_shutdown();
    }

    fn sapp_event(&mut self, _event: SappEvent) {}
}

/*impl SAudioCallbacks for SAudio {
    fn saudio_stream(&mut self, buffer: &mut [f32], num_frames: i32, num_channels: i32) {
        unimplemented!()
    }
}*/

fn main() {
    let saudio_app = SAudio {
        even_odd: 0,
        sample_pos: 0,
        samples: [0.0; NUM_SAMPLES],
    };

    let title = format!("saudio-sapp.rs ({:?})", sg_api());

    let exit_code = sapp_main(
        saudio_app,
        SappDesc {
            width: 800,
            height: 600,
            window_title: title,
            ..Default::default()
        });

    std::process::exit(exit_code);
}
