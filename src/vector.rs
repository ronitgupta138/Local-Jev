//! Fast in-memory cosine vector context index for prompt states.

/// Document entry stored in the vector store.
#[derive(Debug, Clone)]
pub struct VectorDoc {
    pub id: String,
    pub vector: Vec<f32>,
    pub metadata: String,
}

/// In-memory vector index computing cosine similarity in sub-millisecond time.
#[derive(Debug, Default)]
pub struct VectorContextStore {
    docs: Vec<VectorDoc>,
}

impl VectorContextStore {
    pub fn new() -> Self {
        Self { docs: Vec::new() }
    }

    /// Add an embedding vector document.
    pub fn add(&mut self, id: impl Into<String>, vector: Vec<f32>, metadata: impl Into<String>) {
        self.docs.push(VectorDoc {
            id: id.into(),
            vector,
            metadata: metadata.into(),
        });
    }

    /// Search top-K nearest matches by cosine similarity.
    pub fn search(&self, query: &[f32], top_k: usize) -> Vec<(&VectorDoc, f32)> {
        if self.docs.is_empty() || query.is_empty() {
            return Vec::new();
        }

        let mut scored: Vec<(&VectorDoc, f32)> = self
            .docs
            .iter()
            .map(|doc| {
                let sim = cosine_similarity(query, &doc.vector);
                (doc, sim)
            })
            .collect();

        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(top_k);
        scored
    }

    pub fn len(&self) -> usize {
        self.docs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.docs.is_empty()
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let mut dot = 0.0f32;
    let mut norm_a = 0.0f32;
    let mut norm_b = 0.0f32;

    let len = a.len().min(b.len());
    for i in 0..len {
        dot += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }

    if norm_a <= 0.0 || norm_b <= 0.0 {
        0.0
    } else {
        dot / (norm_a.sqrt() * norm_b.sqrt())
    }
}
