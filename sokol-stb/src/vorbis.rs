use std::fs::File;
use std::io;
use std::os::raw::c_int;
use std::ptr;

use memmap::MmapOptions;
use memmap::Mmap;
use std::slice::from_raw_parts;

mod ffi {
    use std::os::raw::c_char;
    use std::os::raw::c_int;
    use std::os::raw::c_uchar;
    use std::os::raw::c_uint;

    #[repr(C)]
    pub struct StbVorbis {
        _opaque: usize,
    }

    #[repr(C)]
    pub struct StbVorbisInfo {
        pub sample_rate: c_uint,
        pub channels: c_int,
        setup_memory_required: c_uint,
        setup_temp_memory_required: c_uint,
        temp_memory_required: c_uint,
        pub max_frame_size: c_int,
    }

    #[repr(C)]
    pub struct StbVorbisAlloc {
        alloc_buffer: *mut c_char,
        alloc_buffer_length_in_bytes: c_int,
    }

    extern {
        pub fn stb_vorbis_get_info(f: *mut StbVorbis) -> StbVorbisInfo;

        pub fn stb_vorbis_close(f: *mut StbVorbis);

        pub fn stb_vorbis_open_pushdata(datablock: *const c_uchar,
                                        datablock_length_in_bytes: c_int,
                                        datablock_memory_consumed_in_bytes: *mut c_int,
                                        error: *mut c_int,
                                        alloc_buffer: *const StbVorbisAlloc) -> *mut StbVorbis;

        pub fn stb_vorbis_decode_frame_pushdata(f: *mut StbVorbis,
                                                datablock: *const c_uchar,
                                                datablock_length_in_bytes: c_int,
                                                channels: *mut c_int,
                                                output: *mut *mut *mut f32,
                                                samples: *mut c_int) -> c_int;

        pub fn stb_vorbis_flush_pushdata(f: *mut StbVorbis);
    }
}

pub struct SAudioVorbis {
    mmap: Mmap,
    f: *mut ffi::StbVorbis,
    read_pos: usize,
    last_frame_decoded: *mut *mut f32,
    last_frame_samples: i32,
    last_frame_channels: i32,
    pub info: SAudioVorbisInfo,
}

pub struct SAudioVorbisInfo {
    pub sample_rate: u32,
    pub channels: i32,
    pub max_frame_size: i32,
}

const VORBIS_NO_ERROR: i32 = 0;
const VORBIS_NEED_MORE_DATA: i32 = 1;

/// Memory-maps a Vorbis .ogg file and prepares for streaming its audio data.
pub fn saudio_vorbis_open(path: &str) -> Result<SAudioVorbis, io::Error> {
    let file = File::open(path)?;

    let mmap = unsafe { MmapOptions::new().map(&file) }?;
    let mmap_size = mmap.len();

    let mut consumed = 0;
    let mut error = 0;

    let f = unsafe {
        ffi::stb_vorbis_open_pushdata(
            mmap[..].as_ptr(),
            mmap_size as c_int,
            &mut consumed,
            &mut error,
            ptr::null_mut(),
        )
    };

    if f == ptr::null_mut() || error != VORBIS_NO_ERROR {
        return Err(io::Error::new(io::ErrorKind::Other,
                                  if error == VORBIS_NEED_MORE_DATA {
                                      "insufficient buffer size"
                                  } else {
                                      "failed to open vorbis stream"
                                  }));
    }

    let info = unsafe {
        ffi::stb_vorbis_get_info(f)
    };

    Ok(SAudioVorbis {
        mmap,
        f,
        read_pos: consumed as usize,
        last_frame_decoded: ptr::null_mut(),
        last_frame_samples: 0,
        last_frame_channels: 0,
        info: SAudioVorbisInfo {
            sample_rate: info.sample_rate,
            channels: info.channels,
            max_frame_size: info.max_frame_size,
        },
    })
}

/// Closes the audio stream.
pub fn saudio_vorbis_close(stream: &SAudioVorbis) {
    unsafe {
        ffi::stb_vorbis_close(stream.f);
    }
}

/// Returns true if end of stream is reached.
///
/// You can use `saudio_vorbis_rewind()` to restart the stream.
pub fn saudio_vorbis_end_of_stream(stream: &SAudioVorbis) -> bool {
    let mmap: &Mmap = &stream.mmap;
    mmap.len() == stream.read_pos
}

/// Restarts the audio stream.
pub fn saudio_vorbis_rewind(stream: &mut SAudioVorbis) {
    stream.read_pos = 0;
    stream.last_frame_samples = 0;
    unsafe {
        ffi::stb_vorbis_flush_pushdata(stream.f);
    }
}

/// Decodes audio data.
///
/// This function decodes as many Vorbis frames as the provided output buffer
/// can hold, or the end of stream is reached.
///
/// The number of _samples per channel_ written to the output buffer is
/// returned, which equals `<return value> * output_channels` float values.
pub fn saudio_vorbis_decode(stream: &mut SAudioVorbis,
                            output_buffer: &mut [f32],
                            output_channels: i32) -> i32 {
    let mut samples_read = 0;
    let mut output_written = 0;
    let mut need_more_data = true;

    if stream.last_frame_samples > 0 {
        // left-over decoded data from last pass
        let decoded = unsafe {
            from_raw_parts(stream.last_frame_decoded, stream.last_frame_channels as usize)
        };
        let (more_data, written) = saudio_vorbis_mix(
            decoded,
            stream.last_frame_samples,
            stream.last_frame_channels,
            output_buffer,
            output_written,
            output_channels,
        );
        output_written += written;
        if !more_data || written == 0 {
            return 0;
        }
    }

    let mmap: &Mmap = &stream.mmap;
    let mut end_of_stream = mmap.len() == stream.read_pos;

    while !end_of_stream && (need_more_data || samples_read == 0) {
        let mmap_size = mmap.len() - stream.read_pos;

        let mut channels = 0;

        let (consumed, decoded) = unsafe {
            let mut output_ptr: *mut *mut f32 = ptr::null_mut();

            let consumed = ffi::stb_vorbis_decode_frame_pushdata(
                stream.f,
                mmap[stream.read_pos..].as_ptr(),
                mmap_size as c_int,
                &mut channels,
                &mut output_ptr,
                &mut samples_read,
            );

            let decoded = if samples_read != 0 {
                from_raw_parts(output_ptr, channels as usize)
            } else {
                &[]
            };

            stream.last_frame_decoded = output_ptr;
            stream.last_frame_samples = samples_read;
            stream.last_frame_channels = channels;

            (consumed, decoded)
        };

        if consumed == 0 && samples_read == 0 {
            // need more data
            // TODO shouldn't happen since we mmap the whole file
        } else if consumed > 0 && samples_read == 0 {
            // re-sync
            stream.read_pos += consumed as usize;
        } else {
            // decoded one frame of data
            stream.read_pos += consumed as usize;

            let (need_more, written) = saudio_vorbis_mix(
                decoded,
                samples_read,
                channels,
                output_buffer,
                output_written,
                output_channels,
            );

            need_more_data = need_more;
            output_written = written;
        }

        end_of_stream = mmap_size == 0;
    }

    output_written / output_channels
}

fn saudio_vorbis_mix(decoded: &[*mut f32],
                     decoded_samples: i32,
                     decoded_channels: i32,
                     output_buffer: &mut [f32],
                     output_pos: i32,
                     output_channels: i32) -> (bool, i32) {

    if decoded_channels != output_channels {
        unimplemented!("");
    }

    let dst_offset = output_pos as usize;
    let dst_frames = (output_buffer.len() - dst_offset) / output_channels as usize;

    let src_frames = decoded_samples as usize;

    if src_frames > dst_frames {
        // output buffer too small
        return (false, output_pos);
    }

    let src_channels: usize = decoded_channels as usize;
    let dst_channels: usize = output_channels as usize;

    for chan in 0..src_channels {
        let src_chan = unsafe { from_raw_parts(decoded[chan], src_frames) };
        let mut dst_idx = dst_offset + chan;
        for src_idx in 0..src_frames {
            let amp = src_chan[src_idx];
            output_buffer[dst_idx] = amp;
            dst_idx += dst_channels;
        }
    }

    let new_output_pos = output_pos as usize + src_frames * dst_channels;

    (true, new_output_pos as i32)
}
