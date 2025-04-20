import onnx
from onnxruntime.transformers import optimizer
from onnxruntime.transformers.fusion_options import FusionOptions

class DynamicDistiller:
    def __init__(self, base_model_path: str):
        self.base_model = onnx.load(base_model_path)
        
    def adapt_model(self, target_precision: str, device_profile: dict) -> bytes:
        opt = FusionOptions('bert')
        opt.enable_embed_layer_norm = False
        
        optimized_model = optimizer.optimize_model(
            self.base_model,
            model_type='bert',
            num_heads=12,
            hidden_size=768,
            optimization_options=opt
        )
        
        if target_precision == 'int8':
            return self.quantize(optimized_model, device_profile)
        elif target_precision == 'float16':
            return optimized_model.convert_float_to_float16()
        else:
            return optimized_model.model.SerializeToString()

    def quantize(self, model, device_caps):
        from onnxruntime.quantization import quantize_dynamic
        return quantize_dynamic(
            model.SerializeToString(),
            activation_type=QuantType.QInt8 if device_caps['cpu'] else QuantType.QUInt8
        )
