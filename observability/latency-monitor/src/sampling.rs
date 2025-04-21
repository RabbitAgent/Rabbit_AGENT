pub struct AdaptiveSampler {
    target_rate: f32,
    current_prob: f32,
    ewma: f32,
}

impl AdaptiveSampler {
    pub fn should_sample(&mut self, latency: Duration) -> bool {
        let sample_value = latency.as_micros() as f32;
        
        // Update EWMA
        self.ewma = self.alpha * sample_value + (1.0 - self.alpha) * self.ewma;
        
        // Dynamic sampling probability
        let error = (sample_value - self.ewma).abs();
        self.current_prob = (self.target_rate / (1.0 + error)).clamp(0.01, 1.0);
        
        rand::thread_rng().gen_bool(self.current_prob as f64)
    }
}
