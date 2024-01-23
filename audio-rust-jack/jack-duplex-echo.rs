use jack::{AudioIn, AudioOut, Client, Control, ProcessHandler, ProcessScope};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

struct EchoEffect {
    input_port: jack::Port<AudioIn>,
    output_port: jack::Port<AudioOut>,
    running: Arc<AtomicBool>,
    delay_buffer: Vec<f32>,
    buffer_position: usize,
    delay_samples: usize,
    mix_ratio: f32,
}

impl ProcessHandler for EchoEffect {
    fn process(&mut self, _: &Client, ps: &ProcessScope) -> Control {
        let input = self.input_port.as_slice(ps);
        let output = self.output_port.as_mut_slice(ps);

        for (i, out_sample) in output.iter_mut().enumerate() {
            let in_sample = input[i];
            let delayed_sample = self.delay_buffer[self.buffer_position];
            self.delay_buffer[self.buffer_position] = in_sample;

            *out_sample = in_sample * (1.0 - self.mix_ratio) + delayed_sample * self.mix_ratio;

            self.buffer_position = (self.buffer_position + 1) % self.delay_samples;
        }

        if self.running.load(Ordering::SeqCst) {
            Control::Continue
        } else {
            Control::Quit
        }
    }
}

fn main() -> Result<(), jack::Error> {
    let (client, _status) = Client::new("echo_effect", jack::ClientOptions::empty())?;

    let running = Arc::new(AtomicBool::new(true));

    let input_port = client.register_port("input", AudioIn::default())?;
    let output_port = client.register_port("output", AudioOut::default())?;

    let sample_rate = client.sample_rate() as usize;
    let delay_samples = sample_rate / 2;  //0.5s
    let mix_ratio = 0.5;  

    let handler = EchoEffect {
        input_port,
        output_port,
        running: Arc::clone(&running),
        delay_buffer: vec![0.0; delay_samples],
        buffer_position: 0,
        delay_samples,
        mix_ratio,
    };

    let active_client = client.activate_async((), handler)?;

    while running.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(100000));
    }

    active_client.deactivate()?;

    Ok(())
}
