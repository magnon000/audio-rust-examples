use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let host = cpal::default_host();

    let input_device = host.default_input_device().expect("No input device available");
    let input_format = input_device.default_input_config().expect("No input format available").config();

    let output_device = host.default_output_device().expect("No output device available");
    let output_format = output_device.default_output_config().expect("No output format available").config();

    let recorded_samples = Arc::new(Mutex::new(Vec::new()));//slice or static

    let recorded_samples_clone = Arc::clone(&recorded_samples);
    let err_fn = move |err| eprintln!("An error occurred on the stream: {}", err);

    let input_stream = input_device.build_input_stream(
        &input_format,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut recorded = recorded_samples_clone.lock().unwrap();
            recorded.extend_from_slice(data);
        },
        err_fn.clone(),
        None,
    )?;
    input_stream.play()?;
    std::thread::sleep(Duration::from_secs(3));
    drop(input_stream);

    let mut position = 0;

    let output_stream = output_device.build_output_stream(
        &output_format,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let recorded = recorded_samples.lock().unwrap();
            for sample in data.iter_mut() {
                if position < recorded.len() {
                    *sample = recorded[position];
                    position += 1;
                } else {
                    *sample = 0.0; //silence if no more samples
                }
            }
        },
        err_fn,
        None,
    )?;
    output_stream.play()?;
    std::thread::sleep(Duration::from_secs(3));
    drop(output_stream);

    Ok(())
}
