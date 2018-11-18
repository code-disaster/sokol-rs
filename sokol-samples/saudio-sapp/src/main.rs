extern crate sokol;
extern crate sokol_stb;

use sokol::app::SApp;
use sokol::app::sapp_height;
use sokol::app::sapp_main;
use sokol::app::sapp_width;
use sokol::app::SAppDesc;
use sokol::app::SAppEvent;
use sokol::audio::saudio_channels;
use sokol::audio::saudio_expect;
use sokol::audio::saudio_push;
use sokol::audio::saudio_setup;
use sokol::audio::saudio_shutdown;
use sokol::audio::SAudioDesc;
use sokol::gfx::sg_api;
use sokol::gfx::sg_begin_default_pass;
use sokol::gfx::sg_commit;
use sokol::gfx::sg_end_pass;
use sokol::gfx::sg_setup;
use sokol::gfx::sg_shutdown;
use sokol::gfx::SgAction;
use sokol::gfx::SgColorAttachmentAction;
use sokol::gfx::SgDesc;
use sokol::gfx::SgPassAction;
use sokol_stb::vorbis::saudio_vorbis_close;
use sokol_stb::vorbis::saudio_vorbis_decode;
use sokol_stb::vorbis::saudio_vorbis_open;
use sokol_stb::vorbis::SAudioVorbis;

const NUM_SAMPLES: usize = 4096;

struct SAudio {
    even_odd: u32,
    sample_pos: i32,
    samples: Box<[f32; NUM_SAMPLES]>,
    audio_stream: Option<SAudioVorbis>,
}

impl SApp for SAudio {
    fn sapp_init(&mut self) {
        sg_setup(&SgDesc {
            ..Default::default()
        });

        self.audio_stream = match saudio_vorbis_open("test.ogg") {
            Err(_) => None,
            Ok(s) => Some(s)
        };

        saudio_setup(&SAudioDesc {
            sample_rate: 44800,
            num_channels: 2,
            use_stream_cb: false,
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

        //
        // this block is only used if use_stream_cb = false (push mode)
        //
        match &mut self.audio_stream {
            None => {}
            Some(stream) => {
                let num_frames = saudio_expect();
                let num_channels = saudio_channels();
                let buffer = &mut self.samples.as_mut();

                let mut frames_pushed = 0;
                while frames_pushed < num_frames {
                    let frames_decoded = saudio_vorbis_decode(stream, *buffer, num_channels);
                    if frames_decoded == 0 {
                        break;
                    }
                    saudio_push(*buffer, frames_decoded);
                    frames_pushed += frames_decoded;
                }
            }
        };

        /*let num_frames = saudio_expect();
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
        }*/

        sg_end_pass();
        sg_commit();
    }

    fn sapp_cleanup(&mut self) {
        saudio_shutdown();
        match &self.audio_stream {
            None => {}
            Some(s) => saudio_vorbis_close(s),
        };
        sg_shutdown();
    }

    fn sapp_event(&mut self, _event: SAppEvent) {}

    fn saudio_stream(&mut self, buffer: &mut [f32], num_frames: i32, _num_channels: i32) {
        //
        // this function is only called if use_stream_cb = true (callback mode)
        //
        let mut s: f32;
        let mut even_odd = 0;
        let mut sample_pos = 0;
        for _i in 0..num_frames {
            if (even_odd & (1 << 5)) != 0 {
                s = 0.05;
            } else {
                s = -0.05;
            }
            even_odd += 1;
            buffer[sample_pos as usize] = s;
            sample_pos += 1;
        }
    }
}

fn main() {
    let saudio_app = SAudio {
        even_odd: 0,
        sample_pos: 0,
        samples: Box::new([0.0; NUM_SAMPLES]),
        audio_stream: None,
    };

    let title = format!("saudio-sapp.rs ({:?})", sg_api());

    let exit_code = sapp_main(
        saudio_app,
        SAppDesc {
            width: 800,
            height: 600,
            window_title: title,
            ..Default::default()
        });

    std::process::exit(exit_code);
}
