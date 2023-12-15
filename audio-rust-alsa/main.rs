use alsa::{pcm::{PCM, HwParams, Format, Access}, Direction};
use std::{f32::consts::PI, error::Error};
//pcm : Audio playback and capture

//return ：Ok(T) or Err(E)
fn main() -> Result<(), Box<dyn Error>> {
    //init new PCM instance for audio playback. "default": default ALSA soundcard, false: non-blocking mode.
    let pcm = PCM::new("default", Direction::Playback, false)?;
    // create obj, conf hardware-specific parameters.
    let hwp = HwParams::any(&pcm)?;

    hwp.set_channels(2)?; //1: mono, 2: stereo
    hwp.set_rate(44100, alsa::ValueOr::Nearest)?;
    // if exact rate not available, use nearest available rate.
    hwp.set_format(Format::FloatLE)?;
    // floating point, little-endian
    hwp.set_access(Access::RWInterleaved)?;
    //conf PCM device for read-write access with interleaved samples
    pcm.hw_params(&hwp)?;
    //apply hw conf

    let period_size = hwp.get_period_size()? as usize;//nomber of frames/1 perio
    let mut buffer: Vec<f32> = vec![0.0; period_size];//filled with zeros
    let freq = 440.0; // Frequency of the sine wave
    let sample_rate = 44100.0;
    let mut phase:f32 = 0.0;
    let phase_step = 2.0 * PI * freq / sample_rate;

    while pcm.state() != alsa::pcm::State::Disconnected {
        // for sample in buffer.iter_mut() {
            // *sample = phase.sin();//mono
        for sample_chunk in buffer.chunks_mut(2) {
            let sample_value = phase.sin();
            sample_chunk[0] = sample_value;  // Left channel
            sample_chunk[1] = sample_value;  // Right channel            
            phase = (phase + phase_step) % (2.0 * PI);//wrapped around using modulo to avoid overflow.
        }

        pcm.io_f32()?.writei(&buffer)?;//buffer to PCM device 
    }

    Ok(())
}
