extern crate alsa;
use alsa::pcm::{PCM, HwParams, Access, Format};
use alsa::{Direction, ValueOr};
use std::error::Error;

fn init(name: Option<&str>) -> Result<(), Box<dyn Error>> {
    // Open the default sound device
    let pcm = PCM::new(name.unwrap_or("default"), Direction::Playback, false)?;

    // Set hardware parameters
    let hwp = HwParams::any(&pcm)?;
    hwp.set_access(Access::RWInterleaved)?;
    hwp.set_format(Format::s16())?;
    hwp.set_rate(44100, ValueOr::Nearest)?;
    hwp.set_channels(2)?;
    pcm.hw_params(&hwp)?;

    // Display information
    println!("Init: Buffer size = {} frames.", hwp.get_buffer_size()?);
    // println!("Init: Significant bits for linear samples = {}", hwp.get_sbits()?);

    // Prepare interface for use
    pcm.prepare()?;

    println!("Audio device has been prepared for use.");
    Ok(())
}

fn uninit(pcm: PCM) -> Result<(), Box<dyn Error>> {
    // Close the sound device
    drop(pcm);
    println!("Audio device has been uninitialized.");
    Ok(())
}

fn main() {
    match init(None) {
        Ok(_) => println!("Initialization successful"),
        Err(e) => eprintln!("Error during initialization: {}", e),
    }

    // Assuming `init` was successful and `pcm` is the PCM instance
    // uninit(pcm);
}
