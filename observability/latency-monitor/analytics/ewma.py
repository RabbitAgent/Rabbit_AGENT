import numpy as np

class AdaptiveEWMA:
    def __init__(self, alpha=0.3, threshold=1.5):
        self.value = None
        self.alpha = alpha
        self.threshold = threshold
        self.variance = 0.0
    
    def update(self, new_value: float) -> float:
        if self.value is None:
            self.value = new_value
            self.variance = 0.0
            return new_value
        
        residual = new_value - self.value
        self.value += self.alpha * residual
        self.variance = (1 - self.alpha) * (self.variance + self.alpha * residual**2)
        
        # Dynamic alpha adjustment
        if abs(residual) > self.threshold * np.sqrt(self.variance):
            self.alpha = min(0.7, self.alpha * 1.1)
        else:
            self.alpha = max(0.1, self.alpha * 0.9)
        
        return self.value
    
    def forecast(self, steps: int) -> list:
        return [self.value * (1 + self.alpha)**i for i in range(steps)]
