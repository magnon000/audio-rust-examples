// use cpal::Devices;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::error::Error;
// use cpal_lib_test::waveforms::{sine_wave, sawtooth_wave};
use cpal_lib_test::waveforms;

fn main() -> Result<(), Box<dyn Error>> {
    let host = cpal::default_host();
    // let devices = host.output_devices()?;
    // println!("Output devices:");
    // for device in devices {
    //     println!("Device: {}", device.name()?);

    //     // match device.supported_output_configs() {
    //     //     Ok(formats) => {
    //     //         for format in formats {
    //     //             println!("  Format: {:?}", format);
    //     //         }
    //     //     },
    //     //     Err(e) => println!("  Error: {}", e),
    //     // }
    // }
    let device = host.default_output_device().expect("no output device available");
    let format: cpal::StreamConfig = device.default_output_config().expect("no output format available").config();

    let sample_rate = format.sample_rate.0 as f32; //1st in sample_rate
    let freq = 440f32;
    // let mut sine = sine_wave(sample_rate, freq);
    let mut square = waveforms::SquareWave::new(sample_rate, freq);
    // let mut sawtooth = sawtooth_wave(sample_rate, freq);

    let stream = device.build_output_stream(
        &format,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                // *sample = sine();//don't: `dyn FnMut() -> f32` cannot be sent between threads safely
                *sample = square.next_sample();
            }
        },
        //pourquoi on fait "pub struct SquareWave" dans le crate ?:
        //Box<dyn FnMut() -> f32>, does not automatically implement the Send trait. 
        //because dyn FnMut() -> f32 is a trait object, 
        //and Rust cannot guarantee that all possible implementations of this trait object 
        //are safe to send between threads.
        move |err| {
            eprintln!("an error occurred on the stream: {}", err);
        },
        None //blocking behavior
    )?;

    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(())
}
