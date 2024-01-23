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

    // let buffer_size = input_format.buffer_size;
    // // default input size = output size
    // let recorded_samples = Arc::new(Mutex::new(vec![0.0f32; buffer_size]));
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
    std::thread::sleep(Duration::from_secs(5));
    drop(input_stream);

    // let recorded_samples_clone = Arc::clone(&recorded_samples);
    let mut position = 0;

    // let echo_delay_samples = 44100/2; // 1/2 second delay at 44.1kHz
    // let echo_decay = 0.5;

    // let output_stream = output_device.build_output_stream(
    //     &output_format,
    //     move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
    //         let recorded = recorded_samples.lock().unwrap();

    //         for sample in data.iter_mut() {
    //             if position < recorded.len() {
    //                 let echo_position = if position >= echo_delay_samples {
    //                     position - echo_delay_samples
    //                 } else {
    //                     0
    //                 };

    //                 // let echo_sample = if echo_position < recorded.len() {
    //                 //     recorded[echo_position] * echo_decay
    //                 // } else {
    //                 //     0.0
    //                 // };

    //                 let echo_sample = recorded[echo_position] * echo_decay;

    //                 *sample = recorded[position] + echo_sample;
    //                 position += 1;
    //             } else {
    //                 *sample = 0.0; //silence if no more samples
    //                 // let echo_position = if position >= echo_delay_samples {
    //                 //     position - echo_delay_samples
    //                 // } else {
    //                 //     0
    //                 // };
    //                 // let echo_sample = recorded[echo_position] * echo_decay;
    //                 // *sample = echo_sample;
    //                 // position += 1;
    //             }
    //         }
    //     },
    //     err_fn,
    //     None,
    // )?;
    let echo_delays_samples = vec![44100, 88200];
    let echo_decays = vec![0.5, 0.25]; 
    let output_stream = output_device.build_output_stream(
        &output_format,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let recorded = recorded_samples.lock().unwrap();

            for sample in data.iter_mut() {
                if position < recorded.len() {
                    let mut echo_sample = recorded[position];

                    for (&delay, &decay) in echo_delays_samples.iter().zip(echo_decays.iter()) {
                        let echo_position = if position >= delay {
                            position - delay
                        } else {
                            continue;
                        };

                        if echo_position < recorded.len() {
                            echo_sample += recorded[echo_position] * decay;
                        }
                    }

                    *sample = echo_sample;
                    position += 1;
                } else {
                    *sample = 0.0;
                }
            }
        },
        err_fn,
        None,
    )?;
    output_stream.play()?;
    std::thread::sleep(Duration::from_secs(10));
    drop(output_stream);

    Ok(())
}
