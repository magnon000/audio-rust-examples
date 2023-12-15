use jack::{AudioOut, Client, Control, ProcessHandler, ProcessScope};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const AMPLITUDE: f32 = 0.5;

struct SineWave {
    output_port1: jack::Port<AudioOut>,
    output_port2: jack::Port<AudioOut>,
    running: Arc<AtomicBool>,
    phase: f64,
    sample_rate: f64,
}

impl ProcessHandler for SineWave {
    fn process(&mut self, _: &Client, ps: &ProcessScope) -> Control {
        let out1 = self.output_port1.as_mut_slice(ps);
        let out2 = self.output_port2.as_mut_slice(ps);

        let phase_increment = 2.0 * std::f64::consts::PI * 440.0 / self.sample_rate;

        for i in 0..ps.n_frames() {
            let i_usize = i as usize;
            let sample = (self.phase.sin() as f32) * AMPLITUDE; 
            out1[i_usize] = sample;
            out2[i_usize] = sample;

            self.phase += phase_increment;
            if self.phase > 2.0 * std::f64::consts::PI {
                self.phase -= 2.0 * std::f64::consts::PI;  // limit between 0 and 2π
            }
        }

        if self.running.load(Ordering::SeqCst) {
            Control::Continue
        } else {
            Control::Quit
        }
    }
}

fn main() -> Result<(), jack::Error> {
    let (client, _status) = Client::new("sinewave", jack::ClientOptions::empty())?;  // Create a new JACK client

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);  // Clone the Arc for the closure

    let output_port1 = client.register_port("output_1", AudioOut::default())?;
    let output_port2 = client.register_port("output_2", AudioOut::default())?;

    let handler = SineWave {
        output_port1,
        output_port2,
        running: Arc::clone(&running),
        phase: 0.0,  // Initialize phase
        sample_rate: client.sample_rate() as f64,  // Get sample rate from JACK client
    };

    let active_client = client.activate_async((), handler)?;  // Activate the client with the handler

    // ctrlc::set_handler(move || {
    //     running_clone.store(false, Ordering::SeqCst);  // Set up Ctrl-C handler
    // })
    // .expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    active_client.deactivate()?;

    Ok(())
}
