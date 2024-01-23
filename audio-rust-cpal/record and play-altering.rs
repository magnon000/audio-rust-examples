use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::error::Error;
use std::sync::{Arc, Mutex, Condvar};
use std::collections::VecDeque;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let host = cpal::default_host();

    let input_device = host.default_input_device().ok_or("No input device available")?;
    let input_format = input_device.default_input_config()?.config();

    let output_device = host.default_output_device().ok_or("No output device available")?;
    let output_format = output_device.default_output_config()?.config();

    let buffer_size = input_format.sample_rate.0 as usize;
    println!("buffer: {:?}'\nbuffer size: {}",input_format.sample_rate, buffer_size);
    let buffer = Arc::new((Mutex::new(VecDeque::with_capacity(buffer_size)), Condvar::new()));

    // Error handling functions
    let input_err_fn = |err| eprintln!("Input stream error: {}", err);
    let output_err_fn = |err| eprintln!("Output stream error: {}", err);

    // Input stream
    let buffer_clone = Arc::clone(&buffer);
    let input_stream = input_device.build_input_stream(
        &input_format,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let (lock, cvar) = &*buffer_clone;
            let mut buffer = lock.lock().unwrap();
            for &sample in data {
                buffer.push_back(sample);
                if buffer.len() == buffer_size {
                    cvar.notify_one();
                    break;
                }
            }
        },
        input_err_fn,
        None,
    )?;

    // Output stream
    let buffer_clone = Arc::clone(&buffer);
    let output_stream = output_device.build_output_stream(
        &output_format,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let (lock, cvar) = &*buffer_clone;
            let mut buffer = lock.lock().unwrap();
            while buffer.len() < buffer_size {
                buffer = cvar.wait(buffer).unwrap();
            }
            for sample in data.iter_mut() {
                *sample = buffer.pop_front().unwrap_or(0.0);
            }
        },
        output_err_fn,
        None,
    )?;

    input_stream.play()?;
    output_stream.play()?;

    std::thread::sleep(Duration::from_secs(5));

    drop(input_stream);
    drop(output_stream);

    Ok(())
}
