# SIMD-Accelerated 8-Bit Quantized Linear Layer Inference

## 1. Inference Pipeline
To evaluate policy networks without GPU dependencies, Local-Jev employs an embedded INT8 feed-forward layer:
- **Weight Matrix Quantization:** Pre-trained float32 policy weights symmetrically quantized to `int8` with scale factor $S_W$.
- **Vector Arithmetic:** Uses AVX2/AVX-512 `_mm256_maddubs_epi16` instructions to compute 32 multiply-accumulate operations per single CPU cycle.
- **Zero Heap Allocations:** Intermediate layer activation scratchpads live in stack-allocated fixed arrays.
