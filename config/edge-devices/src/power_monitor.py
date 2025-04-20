import psutil

class EnergyEstimator:
    @staticmethod
    def estimate_inference_energy(model_size_mb: float, ops_count: int) -> float:
        base_consumption = 0.18  # Watts per MB
        dynamic_consumption = 0.0002  # Watts per OP
        return (model_size_mb * base_consumption) + (ops_count * dynamic_consumption)
