use prometheus::{Counter, Registry};

pub fn register_metrics(registry: &Registry) {
    let requests = Counter::new(
        "api_requests_total", 
        "Total API requests"
    ).unwrap();
    
    registry.register(Box::new(requests)).unwrap();
}
