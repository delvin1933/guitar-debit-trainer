#[cfg(not(target_arch = "wasm32"))]
use rodio::source::{Buffered, Source};
#[cfg(not(target_arch = "wasm32"))]
use rodio::{Decoder, OutputStream, OutputStreamHandle};
#[cfg(not(target_arch = "wasm32"))]
use std::io::Cursor;

const CLICK_HIGH_WAV: &[u8] = include_bytes!("../assets/click_high.wav");
const CLICK_LOW_WAV: &[u8] = include_bytes!("../assets/click_low.wav");
const GUITAR_WAV: &[u8] = include_bytes!("../assets/guitar_g4.wav");

#[cfg(not(target_arch = "wasm32"))]
pub struct SoundEngine {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    click_high_sample: Buffered<Decoder<Cursor<&'static [u8]>>>,
    click_low_sample: Buffered<Decoder<Cursor<&'static [u8]>>>,
    guitar_sample: Buffered<Decoder<Cursor<&'static [u8]>>>,
}

#[cfg(not(target_arch = "wasm32"))]
impl SoundEngine {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible d'initialiser la sortie audio");
        let click_high_sample = Decoder::new(Cursor::new(CLICK_HIGH_WAV))
            .expect("Erreur lors du décodage de click_high.wav")
            .buffered();
        let click_low_sample = Decoder::new(Cursor::new(CLICK_LOW_WAV))
            .expect("Erreur lors du décodage de click_low.wav")
            .buffered();
        let guitar_sample = Decoder::new(Cursor::new(GUITAR_WAV))
            .expect("Erreur lors du décodage de guitar_g4.wav")
            .buffered();

        Self {
            _stream,
            stream_handle,
            click_high_sample,
            click_low_sample,
            guitar_sample,
        }
    }

    pub fn resume(&self) {}

    /// Joue la note de guitare Sol4
    pub fn play_guitar_note(&self, volume: f32) {
        if volume <= 0.0 {
            return;
        }
        let source = self.guitar_sample.clone().amplify(volume);
        let _ = self.stream_handle.play_raw(source.convert_samples());
    }

    /// Joue le clic du métronome (TIC aigu si is_accent = true, TOC grave sinon)
    pub fn play_click(&self, volume: f32, is_accent: bool) {
        if volume <= 0.0 {
            return;
        }
        let sample = if is_accent {
            &self.click_high_sample
        } else {
            &self.click_low_sample
        };
        let source = sample.clone().amplify(volume);
        let _ = self.stream_handle.play_raw(source.convert_samples());
    }
}

#[cfg(target_arch = "wasm32")]
pub struct SoundEngine {
    ctx: web_sys::AudioContext,
    click_high_buffer: web_sys::AudioBuffer,
    click_low_buffer: web_sys::AudioBuffer,
    guitar_buffer: web_sys::AudioBuffer,
}

#[cfg(target_arch = "wasm32")]
fn create_audio_buffer(ctx: &web_sys::AudioContext, wav_bytes: &[u8]) -> web_sys::AudioBuffer {
    let mut pos = 12; // Skip 'RIFF', taille et 'WAVE'
    let mut sample_rate = 44100u32;
    let mut samples = Vec::new();

    while pos + 8 <= wav_bytes.len() {
        let chunk_id = &wav_bytes[pos..pos + 4];
        let chunk_size = u32::from_le_bytes(wav_bytes[pos + 4..pos + 8].try_into().unwrap()) as usize;
        pos += 8;

        if chunk_id == b"fmt " && chunk_size >= 8 {
            sample_rate = u32::from_le_bytes(wav_bytes[pos + 4..pos + 8].try_into().unwrap());
        } else if chunk_id == b"data" {
            let data_end = (pos + chunk_size).min(wav_bytes.len());
            for chunk in wav_bytes[pos..data_end].chunks_exact(2) {
                let val = i16::from_le_bytes([chunk[0], chunk[1]]);
                samples.push(val as f32 / 32768.0);
            }
        }
        pos += chunk_size;
    }

    let buffer = ctx
        .create_buffer(1, samples.len() as u32, sample_rate as f32)
        .expect("Erreur création AudioBuffer");
    buffer
        .copy_to_channel(&mut samples, 0)
        .expect("Erreur copie données AudioBuffer");
    buffer
}

#[cfg(target_arch = "wasm32")]
impl SoundEngine {
    pub fn new() -> Self {
        let ctx = web_sys::AudioContext::new().unwrap();
        let click_high_buffer = create_audio_buffer(&ctx, CLICK_HIGH_WAV);
        let click_low_buffer = create_audio_buffer(&ctx, CLICK_LOW_WAV);
        let guitar_buffer = create_audio_buffer(&ctx, GUITAR_WAV);
        Self {
            ctx,
            click_high_buffer,
            click_low_buffer,
            guitar_buffer,
        }
    }

    pub fn resume(&self) {
        if self.ctx.state() == web_sys::AudioContextState::Suspended {
            let _ = self.ctx.resume();
        }
    }

    pub fn play_guitar_note(&self, volume: f32) {
        if volume <= 0.0 {
            return;
        }
        self.resume();

        let source = self.ctx.create_buffer_source().unwrap();
        source.set_buffer(Some(&self.guitar_buffer));

        let gain = self.ctx.create_gain().unwrap();
        gain.gain().set_value(volume);

        let _ = source.connect_with_audio_node(&gain);
        let _ = gain.connect_with_audio_node(&self.ctx.destination());

        let _ = source.start();
    }

    pub fn play_click(&self, volume: f32, is_accent: bool) {
        if volume <= 0.0 {
            return;
        }
        self.resume();

        let buffer = if is_accent {
            &self.click_high_buffer
        } else {
            &self.click_low_buffer
        };

        let source = self.ctx.create_buffer_source().unwrap();
        source.set_buffer(Some(buffer));

        let gain = self.ctx.create_gain().unwrap();
        gain.gain().set_value(volume);

        let _ = source.connect_with_audio_node(&gain);
        let _ = gain.connect_with_audio_node(&self.ctx.destination());

        let _ = source.start();
    }
}
