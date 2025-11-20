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

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ConversationEmbedding {
    pub user_id: String,        // Principal ID of the user
    pub channel_id: String,     // Channel where conversation happened
    pub conversation_text: String,  // The 10-message conversation chunk
    pub embedding: Vec<f32>,    // Vector representation of the conversation
    pub message_count: u32,     // Number of messages in this chunk
    pub chunk_index: u32,       // Sequential chunk number (0, 1, 2, ...)
    pub created_at: u64,        // When this chunk was stored
    pub summary: String,        // Brief summary of the conversation chunk
}

// Storage for personality embeddings (stable memory)
thread_local! {
    static PERSONALITY_EMBEDDINGS: std::cell::RefCell<Vec<PersonalityEmbedding>> = std::cell::RefCell::new(Vec::new());
    static USER_MEMORIES: std::cell::RefCell<Vec<UserMemory>> = std::cell::RefCell::new(Vec::new());
    static CONVERSATION_EMBEDDINGS: std::cell::RefCell<Vec<ConversationEmbedding>> = std::cell::RefCell::new(Vec::new());
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

// === CONVERSATION EMBEDDING FUNCTIONS ===

/// Store a conversation embedding chunk
pub fn store_conversation_embedding(conversation: ConversationEmbedding) {
    CONVERSATION_EMBEDDINGS.with(|conversations| {
        conversations.borrow_mut().push(conversation);
    });
}

/// Get all conversation embeddings for a specific user and channel
pub fn get_user_conversation_history(user_id: &str, channel_id: &str) -> Vec<ConversationEmbedding> {
    CONVERSATION_EMBEDDINGS.with(|conversations| {
        conversations.borrow()
            .iter()
            .filter(|conv| conv.user_id == user_id && conv.channel_id == channel_id)
            .cloned()
            .collect()
    })
}

/// Get the next chunk index for a user in a specific channel
pub fn get_next_chunk_index(user_id: &str, channel_id: &str) -> u32 {
    CONVERSATION_EMBEDDINGS.with(|conversations| {
        conversations.borrow()
            .iter()
            .filter(|conv| conv.user_id == user_id && conv.channel_id == channel_id)
            .map(|conv| conv.chunk_index)
            .max()
            .unwrap_or(0) + 1
    })
}

/// Search conversation history using semantic similarity
pub fn search_conversation_history(
    user_id: &str,
    channel_id: &str,
    query_embedding: &[f32],
    top_k: usize
) -> Vec<String> {
    CONVERSATION_EMBEDDINGS.with(|conversations| {
        let mut scored_conversations: Vec<(f32, ConversationEmbedding)> = conversations.borrow()
            .iter()
            .filter(|conv| conv.user_id == user_id && conv.channel_id == channel_id)
            .map(|conv| {
                let similarity = cosine_similarity(query_embedding, &conv.embedding);
                (similarity, conv.clone())
            })
            .collect();

        // Sort by similarity score (descending)
        scored_conversations.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        // Return top_k conversation summaries/texts
        scored_conversations
            .into_iter()
            .take(top_k)
            .map(|(_, conv)| if conv.summary.is_empty() { 
                conv.conversation_text 
            } else { 
                conv.summary 
            })
            .collect()
    })
}

/// Get recent conversation context for a user (last N chunks)
pub fn get_recent_conversation_context(
    user_id: &str,
    channel_id: &str,
    chunk_count: usize
) -> Vec<String> {
    CONVERSATION_EMBEDDINGS.with(|conversations| {
        let mut user_conversations: Vec<ConversationEmbedding> = conversations.borrow()
            .iter()
            .filter(|conv| conv.user_id == user_id && conv.channel_id == channel_id)
            .cloned()
            .collect();

        // Sort by chunk_index (most recent first)
        user_conversations.sort_by(|a, b| b.chunk_index.cmp(&a.chunk_index));

        // Return the most recent chunks
        user_conversations
            .into_iter()
            .take(chunk_count)
            .map(|conv| if conv.summary.is_empty() { 
                conv.conversation_text 
            } else { 
                conv.summary 
            })
            .collect()
    })
}

/// Get conversation statistics for a user
pub fn get_conversation_stats(user_id: &str, channel_id: &str) -> (u32, u32) {
    CONVERSATION_EMBEDDINGS.with(|conversations| {
        let borrowed_conversations = conversations.borrow();
        let user_conversations: Vec<&ConversationEmbedding> = borrowed_conversations
            .iter()
            .filter(|conv| conv.user_id == user_id && conv.channel_id == channel_id)
            .collect();

        let chunk_count = user_conversations.len() as u32;
        let total_messages = user_conversations
            .iter()
            .map(|conv| conv.message_count)
            .sum::<u32>();

        (chunk_count, total_messages)
    })
}
// Functions for upgrade persistence
pub fn get_all_user_memories() -> Vec<UserMemory> {
    USER_MEMORIES.with(|memories| memories.borrow().clone())
}

pub fn get_all_conversation_embeddings() -> Vec<ConversationEmbedding> {
    CONVERSATION_EMBEDDINGS.with(|embeddings| embeddings.borrow().clone())
}

pub fn restore_all_data(
    personality_data: Vec<PersonalityEmbedding>,
    user_memories: Vec<UserMemory>,
    conversation_embeddings: Vec<ConversationEmbedding>
) {
    PERSONALITY_EMBEDDINGS.with(|embeddings| {
        *embeddings.borrow_mut() = personality_data;
    });
    
    USER_MEMORIES.with(|memories| {
        *memories.borrow_mut() = user_memories;
    });
    
    CONVERSATION_EMBEDDINGS.with(|embeddings| {
        *embeddings.borrow_mut() = conversation_embeddings;
    });
}
