extern crate sokol;
extern crate sokol_stb;

use sokol::app::SApp;
use sokol::app::sapp_height;
use sokol::app::sapp_main;
use sokol::app::sapp_width;
use sokol::app::SAppDesc;
use sokol::app::SAppEvent;
use sokol::app::SAppEventType;
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
use sokol_stb::vorbis::saudio_vorbis_rewind;
use sokol_stb::vorbis::SAudioVorbis;

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

                let buffer_size_requested = ((num_frames * num_channels) as usize).min(NUM_SAMPLES);
                let buffer_requested = &mut (*buffer)[..buffer_size_requested];

                if num_frames > 0 {
                    let frames_decoded = saudio_vorbis_decode(stream, buffer_requested, num_channels);
                    if frames_decoded != 0 {
                        saudio_push(buffer_requested, frames_decoded);
                    }
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
