extern crate sokol;
extern crate sokol_stb;

use std::env;

use sokol::app::*;
use sokol::audio::*;
use sokol::gfx::*;
use sokol_stb::vorbis::*;

const NUM_SAMPLES: usize = 44800 * 2;

struct SAudio {
    samples: Box<[f32; NUM_SAMPLES]>,
    audio_stream: Option<SAudioVorbis>,
}

impl SApp for SAudio {
    fn sapp_init(&mut self) {
        sg_setup(&SgDesc {
            ..Default::default()
        });

        let path = env::args().nth(1);
        if path.is_some() {
            self.audio_stream = match saudio_vorbis_open(&path.unwrap()) {
                Err(_) => None,
                Ok(s) => Some(s)
            };
        }

        saudio_setup(SAudioDesc {
            sample_rate: 44800,
            num_channels: 2,
            use_stream_cb: !self.audio_stream.is_some(),
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

                let buffer_size_requested = ((num_frames * num_channels) as usize).min(NUM_SAMPLES);
                let buffer_requested = &mut (*buffer)[..buffer_size_requested];

                if num_frames > 0 {
                    let frames_decoded = saudio_vorbis_decode(stream, buffer_requested, num_channels);
                    if frames_decoded != 0 {
                        saudio_push(buffer_requested, frames_decoded);
                    }
                }

                if saudio_vorbis_end_of_stream(stream) {
                    saudio_vorbis_rewind(stream);
                }
            }
        };

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

    fn sapp_event(&mut self, event: SAppEvent) {
        if event.event_type == SAppEventType::Char {
            if event.char_code == 'r' as u32 {
                match &mut self.audio_stream {
                    None => {}
                    Some(stream) => saudio_vorbis_rewind(stream)
                }
            }
        }
    }

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
        samples: Box::new([0.0; NUM_SAMPLES]),
        audio_stream: None,
    };

    let title = format!("saudio-sapp.rs ({:?})", sg_api());

    let exit_code = sapp_run(
        saudio_app,
        SAppDesc {
            width: 800,
            height: 600,
            window_title: title,
            ..Default::default()
        });

    std::process::exit(exit_code);
}
