use jack::{AudioOut, Client, Control, ProcessHandler, ProcessScope};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
// atomic types for thread-safe operations.
// Atomic Reference Counted pointers for shared ownership of values across multiple threads.
const AMPLITUDE: f32 = 0.1;

struct WhiteNoise {
    output_port1: jack::Port<AudioOut>,
    output_port2: jack::Port<AudioOut>,
    running: Arc<AtomicBool>,
}

impl ProcessHandler for WhiteNoise {
    fn process(&mut self, _: &Client, ps: &ProcessScope) -> Control {
        let out1 = self.output_port1.as_mut_slice(ps);
        let out2 = self.output_port2.as_mut_slice(ps);

        for i in 0..ps.n_frames() {
            let rand1: f32 = (0..4).map(|_| rand::random::<f32>()).sum::<f32>() - 2.0;
            let rand2: f32 = (0..4).map(|_| rand::random::<f32>()).sum::<f32>() - 2.0;
            let i_usize = i as usize; // Cast u32 to usize
            out1[i_usize] = AMPLITUDE * rand1;
            out2[i_usize] = AMPLITUDE * rand2;
        }

        if self.running.load(Ordering::SeqCst) {
            Control::Continue
        } else {
            Control::Quit
        }
    }
}

fn main() -> Result<(), jack::Error> {
    // let (client, _status) = Client::new("whitenoise", jack::ClientOptions::NO_START_SERVER)?;
    let (client, _status) = Client::new("whitenoise", jack::ClientOptions::empty())?;

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running); // Clone the Arc for the closure

    let output_port1 = client.register_port("output_1", AudioOut::default())?;
    let output_port2 = client.register_port("output_2", AudioOut::default())?;

    let handler = WhiteNoise {
        output_port1,
        output_port2,
        running: Arc::clone(&running), // Use the cloned Arc
    };

    let active_client = client.activate_async((), handler)?;

    // ctrlc::set_handler(move || {
    //     running_clone.store(false, Ordering::SeqCst);
    // })
    // .expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    active_client.deactivate()?;

    Ok(())
}

