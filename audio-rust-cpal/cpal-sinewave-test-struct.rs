use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::error::Error;

struct SineWave {
    sample_clock: f32,
    sample_rate: f32,
    amplitude: f32,
}

impl SineWave {
    fn new(sample_rate: f32, amplitude: f32) -> Self {
        SineWave {
            sample_clock: 0.0,
            sample_rate,
            amplitude,
        }
    }

    fn next_value(&mut self) -> f32 {
        self.sample_clock = (self.sample_clock + 1.0) % self.sample_rate;
        (self.sample_clock * 440.0 * 2.0 * std::f32::consts::PI / self.sample_rate).sin() * self.amplitude
    }
}

struct AudioOutput {
    device: cpal::Device,
    format: cpal::StreamConfig,
}

impl AudioOutput {
    fn new() -> Result<Self, Box<dyn Error>> {
        let host = cpal::default_host();
        let device = host.default_output_device().ok_or("no output device available")?;
        let format = device.default_output_config()?.config();
        Ok(AudioOutput { device, format })
    }

    fn play_sine_wave(&self, mut sine_wave: SineWave) -> Result<(), Box<dyn Error>> {
        let err_fn = |err| eprintln!("an error occurred on the audio stream: {}", err);
        let stream = self.device.build_output_stream(
            &self.format,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for sample in data.iter_mut() {
                    *sample = sine_wave.next_value();
                }
            },
            err_fn,
            None
        )?;
        stream.play()?;
        std::thread::sleep(std::time::Duration::from_secs(1));
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let sine_wave = SineWave::new(44100.0, 0.5);
    let audio_output = AudioOutput::new()?;
    audio_output.play_sine_wave(sine_wave)?;
    Ok(())
}