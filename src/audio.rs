#[cfg(not(target_arch = "wasm32"))]
use rodio::source::{Buffered, Source};
#[cfg(not(target_arch = "wasm32"))]
use rodio::{Decoder, OutputStream, OutputStreamHandle};
#[cfg(not(target_arch = "wasm32"))]
use std::io::Cursor;

const CLICK_WAV: &[u8] = include_bytes!("../assets/click.wav");
const GUITAR_WAV: &[u8] = include_bytes!("../assets/guitar_g4.wav");

#[cfg(not(target_arch = "wasm32"))]
pub struct SoundEngine {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    click_sample: Buffered<Decoder<Cursor<&'static [u8]>>>,
    guitar_sample: Buffered<Decoder<Cursor<&'static [u8]>>>,
}

#[cfg(not(target_arch = "wasm32"))]
impl SoundEngine {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible d'initialiser la sortie audio");
        let click_sample = Decoder::new(Cursor::new(CLICK_WAV))
            .expect("Erreur lors du décodage de click.wav")
            .buffered();
        let guitar_sample = Decoder::new(Cursor::new(GUITAR_WAV))
            .expect("Erreur lors du décodage de guitar_g4.wav")
            .buffered();

        Self {
            _stream,
            stream_handle,
            click_sample,
            guitar_sample,
        }
    }

    /// Joue la note de guitare Sol4
    pub fn play_guitar_note(&self, volume: f32) {
        if volume <= 0.0 {
            return;
        }
        let source = self.guitar_sample.clone().amplify(volume);
        let _ = self.stream_handle.play_raw(source.convert_samples());
    }

    /// Joue le clic du métronome
    pub fn play_click(&self, volume: f32) {
        if volume <= 0.0 {
            return;
        }
        let source = self.click_sample.clone().amplify(volume);
        let _ = self.stream_handle.play_raw(source.convert_samples());
    }
}

#[cfg(target_arch = "wasm32")]
pub struct SoundEngine {
    ctx: web_sys::AudioContext,
}

#[cfg(target_arch = "wasm32")]
impl SoundEngine {
    pub fn new() -> Self {
        let ctx = web_sys::AudioContext::new().unwrap();
        Self { ctx }
    }

    pub fn play_guitar_note(&self, volume: f32) {
        if volume <= 0.0 {
            return;
        }
        let osc = self.ctx.create_oscillator().unwrap();
        let gain = self.ctx.create_gain().unwrap();

        osc.frequency().set_value(392.0); // Sol4
        gain.gain().set_value(volume);

        let _ = osc.connect_with_audio_node(&gain);
        let _ = gain.connect_with_audio_node(&self.ctx.destination());

        let now = self.ctx.current_time();
        let _ = osc.start_with_when(now);
        let _ = osc.stop_with_when(now + 0.6); // 600ms
    }

    pub fn play_click(&self, volume: f32) {
        if volume <= 0.0 {
            return;
        }
        let osc = self.ctx.create_oscillator().unwrap();
        let gain = self.ctx.create_gain().unwrap();

        osc.frequency().set_value(1000.0);
        gain.gain().set_value(volume);

        let _ = osc.connect_with_audio_node(&gain);
        let _ = gain.connect_with_audio_node(&self.ctx.destination());

        let now = self.ctx.current_time();
        let _ = osc.start_with_when(now);
        let _ = osc.stop_with_when(now + 0.02); // 20ms
    }
}
