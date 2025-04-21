use linfa::prelude::*;
use linfa_elasticnet::ElasticNet;

pub struct LatencyPredictor {
    model: ElasticNet<f64>,
    window_size: usize,
}

impl LatencyPredictor {
    pub fn new(training_data: &[f64], window: usize) -> Self {
        let dataset = create_rolling_window_dataset(training_data, window);
        let model = ElasticNet::params()
            .penalty(0.5)
            .ratio(0.5)
            .fit(&dataset)
            .unwrap();
        
        Self { model, window_size: window }
    }

    pub fn predict_next(&self, recent_values: &[f64]) -> Option<f64> {
        if recent_values.len() != self.window_size {
            return None;
        }
        
        let x = Array1::from(recent_values.to_vec()).insert_axis(Axis(0));
        Some(self.model.predict(&x).get(0).cloned().unwrap())
    }

    fn create_rolling_window_dataset(data: &[f64], window: usize) -> Dataset<f64, f64, Ix1> {
        let mut samples = Vec::new();
        let mut targets = Vec::new();
        
        for i in window..data.len() {
            samples.push(data[i - window..i].to_vec());
            targets.push(data[i]);
        }
        
        Dataset::new(
            Array2::from_shape_vec((samples.len(), window), samples.into_iter().flatten().collect()).unwrap(),
            Array1::from(targets),
        )
    }
}
