pub fn sine_wave(sample_rate: f32, frequency: f32) -> Box<dyn FnMut() -> f32> {
    let mut sample_clock = 0f32;
    Box::new(move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * frequency * 2.0 * std::f32::consts::PI / sample_rate).sin()
    })
}

pub struct SquareWave {
    sample_clock: f32,
    sample_rate: f32,
    frequency: f32,
}

impl SquareWave {
    pub fn new(sample_rate: f32, frequency: f32) -> Self {
        SquareWave {
            sample_clock: 0.0,
            sample_rate,
            frequency,
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        self.sample_clock = (self.sample_clock + 1.0) % self.sample_rate;
        let period = self.sample_rate / self.frequency;
        if (self.sample_clock % period) < (period / 2.0) {
            1.0
        } else {
            -1.0
        }
    }
}


pub fn sawtooth_wave(sample_rate: f32, frequency: f32) -> Box<dyn FnMut() -> f32> {
    let mut sample_clock = 0f32;
    let period = sample_rate / frequency;
    Box::new(move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        2.0 * (sample_clock % period) / period - 1.0
    })
}
