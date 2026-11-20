/// 4-bit scalar quantized vector for ultra-compact edge memory.
#[derive(Debug, Clone, PartialEq)]
pub struct Int4Vector {
    pub scale: f32,
    pub min: f32,
    pub packed_bytes: Vec<u8>,
    pub original_len: usize,
}

impl Int4Vector {
    pub fn quantize(floats: &[f32]) -> Self {
        if floats.is_empty() {
            return Self { scale: 1.0, min: 0.0, packed_bytes: Vec::new(), original_len: 0 };
        }
        let min = floats.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = floats.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let scale = if max > min { (max - min) / 15.0 } else { 1.0 };

        let mut packed = Vec::with_capacity(floats.len().div_ceil(2));
        for chunk in floats.chunks(2) {
            let q0 = (((chunk[0] - min) / scale).round() as u8).min(15);
            let q1 = if chunk.len() > 1 {
                (((chunk[1] - min) / scale).round() as u8).min(15)
            } else {
                0
            };
            packed.push((q1 << 4) | (q0 & 0x0F));
        }

        Self { scale, min, packed_bytes: packed, original_len: floats.len() }
    }

    pub fn dequantize(&self) -> Vec<f32> {
        let mut result = Vec::with_capacity(self.original_len);
        for &byte in &self.packed_bytes {
            let q0 = byte & 0x0F;
            result.push(self.min + (q0 as f32) * self.scale);
            if result.len() < self.original_len {
                let q1 = (byte >> 4) & 0x0F;
                result.push(self.min + (q1 as f32) * self.scale);
            }
        }
        result
    }
}
