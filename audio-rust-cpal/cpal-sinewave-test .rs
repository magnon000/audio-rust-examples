use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("no output device available");
    let format = device.default_output_config().expect("no output format available").config();

    let sample_rate = format.sample_rate.0 as f32;
    let mut sample_clock = 0f32;
    let amplitude = 0.5f32;
    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        amplitude * (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
    };// another way to limit phase

    let stream = device.build_output_stream(
        &format,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                *sample = next_value();
            }
        },
        move |err| {
            eprintln!("an error occurred on the stream: {}", err);
        },
        None // None for blocking behavior
    )?;

    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(10000));

    Ok(())
}
