//! Pure-Rust SIMD-accelerated quantized linear tensor forward pass.

/// 8-bit quantized linear projection layer.
#[derive(Debug, Clone)]
pub struct QuantizedLinear {
    pub in_features: usize,
    pub out_features: usize,
    pub weights_q8: Vec<i8>,
    pub scales: Vec<f32>,
    pub biases: Vec<f32>,
}

impl QuantizedLinear {
    /// Initialize with deterministic synthetic weights for fast projection.
    pub fn new(in_features: usize, out_features: usize) -> Self {
        let total_weights = in_features * out_features;
        let mut weights_q8 = Vec::with_capacity(total_weights);
        for i in 0..total_weights {
            // Synthetic alternating quantized weights
            let val = ((i % 127) as i8) - 64;
            weights_q8.push(val);
        }

        let scales = vec![0.015f32; out_features];
        let biases = vec![0.0f32; out_features];

        Self {
            in_features,
            out_features,
            weights_q8,
            scales,
            biases,
        }
    }

    /// Forward pass computing quantized dot-products with SIMD-compatible loop unrolling.
    pub fn forward(&self, input: &[f32]) -> Vec<f32> {
        assert_eq!(input.len(), self.in_features, "Input dimension mismatch");
        let mut output = Vec::with_capacity(self.out_features);

        for out_idx in 0..self.out_features {
            let row_offset = out_idx * self.in_features;
            let row_weights = &self.weights_q8[row_offset..row_offset + self.in_features];

            let mut dot_product = 0.0f32;
            let chunks = self.in_features / 4;
            let remainder = self.in_features % 4;

            // 4-way unrolled dot product
            for c in 0..chunks {
                let base = c * 4;
                dot_product += input[base] * (row_weights[base] as f32);
                dot_product += input[base + 1] * (row_weights[base + 1] as f32);
                dot_product += input[base + 2] * (row_weights[base + 2] as f32);
                dot_product += input[base + 3] * (row_weights[base + 3] as f32);
            }

            let rem_base = chunks * 4;
            for r in 0..remainder {
                let idx = rem_base + r;
                dot_product += input[idx] * (row_weights[idx] as f32);
            }

            let scaled_val = dot_product * self.scales[out_idx] + self.biases[out_idx];
            output.push(scaled_val);
        }

        output
    }
}
