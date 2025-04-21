use quantiles::ckms::CKMS;
use std::time::{Duration, Instant};

pub struct LatencyHistogram {
    ckms: CKMS<f64>,
    circular_buffer: Vec<Duration>,
    buf_index: usize,
    capacity: usize,
}

impl LatencyHistogram {
    pub fn new(reservoir_size: usize, error: f64) -> Self {
        Self {
            ckms: CKMS::new(error),
            circular_buffer: vec![Duration::ZERO; reservoir_size],
            buf_index: 0,
            capacity: reservoir_size,
        }
    }

    pub fn record(&mut self, latency: Duration) {
        let ms = latency.as_secs_f64() * 1000.0;
        
        // Update CKMS sketch
        self.ckms.insert(ms);
        
        // Store raw sample
        self.circular_buffer[self.buf_index] = latency;
        self.buf_index = (self.buf_index + 1) % self.capacity;
    }

    pub fn quantile(&self, q: f64) -> Option<Duration> {
        self.ckms.query(q).map(|(_, v)| Duration::from_secs_f64(v / 1000.0))
    }

    pub fn stats(&self) -> LatencyStats {
        let samples: Vec<f64> = self.circular_buffer.iter()
            .map(|d| d.as_secs_f64() * 1000.0)
            .filter(|&v| v > 0.0)
            .collect();
        
        LatencyStats::from_samples(&samples)
    }
}

pub struct LatencyStats {
    pub mean: f64,
    pub std_dev: f64,
    pub min: f64,
    pub max: f64,
}

impl LatencyStats {
    fn from_samples(samples: &[f64]) -> Self {
        let count = samples.len() as f64;
        let sum: f64 = samples.iter().sum();
        let mean = sum / count;
        
        let variance: f64 = samples.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / count;
        
        Self {
            mean,
            std_dev: variance.sqrt(),
            min: samples.iter().fold(f64::INFINITY, |a, &b| a.min(b)),
            max: samples.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)),
        }
    }
}
