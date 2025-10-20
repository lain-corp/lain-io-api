use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct PersonalityEmbedding {
    pub text: String,           // The original personality text/memory
    pub embedding: Vec<f32>,    // Vector representation
    pub channel_id: String,     // Which channel this belongs to
    pub category: String,       // "experience", "preference", "opinion", etc.
    pub importance: f32,        // How important this memory is (0.0-1.0)
    pub created_at: u64,        // Timestamp
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct UserMemory {
    pub user_id: String,        // Principal ID
    pub text: String,           // The memory about the user
    pub embedding: Vec<f32>,    // Vector representation
    pub channel_id: String,     // Where this memory was formed
    pub memory_type: String,    // "preference", "skill", "interaction", etc.
    pub created_at: u64,        // When this was learned
}

// Storage for personality embeddings (stable memory)
thread_local! {
    static PERSONALITY_EMBEDDINGS: std::cell::RefCell<Vec<PersonalityEmbedding>> = std::cell::RefCell::new(Vec::new());
    static USER_MEMORIES: std::cell::RefCell<Vec<UserMemory>> = std::cell::RefCell::new(Vec::new());
}

/// Store a personality embedding (called from frontend)
pub fn store_personality_embedding(embedding: PersonalityEmbedding) {
    PERSONALITY_EMBEDDINGS.with(|embeddings| {
        embeddings.borrow_mut().push(embedding);
    });
}

/// Store a user memory (called when learning about users)
pub fn store_user_memory(memory: UserMemory) {
    USER_MEMORIES.with(|memories| {
        memories.borrow_mut().push(memory);
    });
}

/// Retrieve personality embeddings for a specific channel
pub fn get_personality_embeddings(channel_id: &str) -> Vec<PersonalityEmbedding> {
    PERSONALITY_EMBEDDINGS.with(|embeddings| {
        embeddings.borrow()
            .iter()
            .filter(|e| e.channel_id == channel_id)
            .cloned()
            .collect()
    })
}

/// Get all personality embeddings (for debugging/inspection)
pub fn get_all_personality_embeddings() -> Vec<PersonalityEmbedding> {
    PERSONALITY_EMBEDDINGS.with(|embeddings| {
        embeddings.borrow().clone()
    })
}

/// Simple cosine similarity calculation
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        return 0.0;
    }
    
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    
    dot_product / (norm_a * norm_b)
}

/// Search for relevant personality context based on query embedding
pub fn search_personality_context(channel_id: &str, query_embedding: &[f32], top_k: usize) -> Vec<String> {
    let embeddings = get_personality_embeddings(channel_id);
    
    let mut scored_embeddings: Vec<(f32, &PersonalityEmbedding)> = embeddings
        .iter()
        .map(|emb| (cosine_similarity(query_embedding, &emb.embedding), emb))
        .collect();
    
    // Sort by similarity score (descending)
    scored_embeddings.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    
    // Return top_k results
    scored_embeddings
        .into_iter()
        .take(top_k)
        .map(|(_, emb)| emb.text.clone())
        .collect()
}

/// Get channel-specific personality context without needing query embeddings
/// Returns the most important personality traits for a given channel
pub fn get_channel_personality_context(channel_id: &str, top_k: usize) -> Vec<String> {
    let embeddings = get_personality_embeddings(channel_id);
    
    // Sort by importance score (descending) and return top traits
    let mut sorted_embeddings: Vec<&PersonalityEmbedding> = embeddings.iter().collect();
    sorted_embeddings.sort_by(|a, b| b.importance.partial_cmp(&a.importance).unwrap_or(std::cmp::Ordering::Equal));
    
    sorted_embeddings
        .into_iter()
        .take(top_k)
        .map(|emb| emb.text.clone())
        .collect()
}

/// Search for relevant user memories
pub fn search_user_memories(user_id: &str, query_embedding: &[f32], top_k: usize) -> Vec<String> {
    USER_MEMORIES.with(|memories| {
        let borrowed_memories = memories.borrow();
        let user_memories: Vec<_> = borrowed_memories
            .iter()
            .filter(|m| m.user_id == user_id)
            .collect();

        let mut scored_memories: Vec<(f32, &UserMemory)> = user_memories
            .iter()
            .map(|mem| (cosine_similarity(query_embedding, &mem.embedding), *mem))
            .collect();

        // Sort by similarity score (descending)
        scored_memories.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        // Return top_k results
        scored_memories
            .into_iter()
            .take(top_k)
            .map(|(_, mem)| mem.text.clone())
            .collect()
    })
}

/// Enhanced context retrieval that combines personality and user memories
pub fn get_enhanced_context(
    channel_id: &str, 
    user_id: Option<&str>, 
    query_embedding: &[f32]
) -> (Vec<String>, Vec<String>) {
    // Get personality context for the channel
    let personality_context = search_personality_context(channel_id, query_embedding, 3);
    
    // Get user-specific context if user_id is provided
    let user_context = if let Some(uid) = user_id {
        search_user_memories(uid, query_embedding, 2)
    } else {
        Vec::new()
    };
    
    (personality_context, user_context)
}