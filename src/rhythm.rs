use std::time::Duration;

pub trait RhythmController {
    /// Returns the wait time until the next beat is supposed to be initialised
    fn step(&mut self) -> Duration;
}

struct PolyRhythm {
    beat_time: Duration,
    pub accumulated: Duration,
}

impl PolyRhythm {
    pub const fn new(beat_time_ms: u64) -> Self {
        Self {
            beat_time: Duration::from_millis(beat_time_ms),
            accumulated: Duration::from_secs(0),
        }
    }
    pub fn step(&mut self, time: Duration) {
        self.accumulated = self.accumulated + time;
    }
    pub fn time_to_next(&self) -> Duration {
        if self.accumulated.is_zero() {
            return self.beat_time;
        }

        let mut diff = self.accumulated.clone();
        while diff > self.beat_time {
            diff -= self.beat_time;
        }

        return diff;
    }
}

pub struct PolyRhythmController {
    rhythms: Vec<PolyRhythm>,
}

impl PolyRhythmController {
    pub fn new(rhythms: &[u64]) -> Self {
        Self {
            rhythms: rhythms
                .iter()
                .map(|beat_time_s| PolyRhythm::new(*beat_time_s))
                .collect(),
        }
    }
}

impl RhythmController for PolyRhythmController {
    fn step(&mut self) -> Duration {
        let maybe_time_to_next_rhythm =
            self.rhythms
                .iter()
                .map(PolyRhythm::time_to_next)
                .fold(None, |min, next| {
                    return if let Some(min_val) = min {
                        if next < min_val {
                            Some(next)
                        } else {
                            min
                        }
                    } else {
                        Some(next)
                    };
                });
        if let Some(next_rhythm) = maybe_time_to_next_rhythm {
            self.rhythms
                .iter_mut()
                .for_each(|rhythm| rhythm.step(next_rhythm));
        }
        return maybe_time_to_next_rhythm.unwrap_or(Duration::new(0, 0));
    }
}
