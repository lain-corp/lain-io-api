use candid::{CandidType, Deserialize};
use std::collections::HashMap;

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

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct BigFiveTraits {
    pub openness: f32,          // 0.0-1.0: openness to experience
    pub conscientiousness: f32, // 0.0-1.0: organization and discipline  
    pub extraversion: f32,      // 0.0-1.0: social energy and enthusiasm
    pub agreeableness: f32,     // 0.0-1.0: cooperation and empathy
    pub neuroticism: f32,       // 0.0-1.0: emotional instability
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct TopicInterest {
    pub topic: String,          // Topic name (e.g., "technology", "art")
    pub engagement_score: f32,  // 0.0-1.0: how engaged with this topic
    pub message_count: u32,     // Number of messages about this topic
    pub expertise_level: f32,   // 0.0-1.0: estimated expertise
    pub first_mentioned: u64,   // When first discussed
    pub last_mentioned: u64,    // Most recent discussion
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct UserProfile {
    pub user_id: String,
    pub personality_traits: BigFiveTraits,
    pub interests: Vec<TopicInterest>,
    pub aggregated_embedding: Vec<f32>, // Average of all conversation embeddings
    pub conversation_count: u32,
    pub total_messages: u32,
    pub created_at: u64,
    pub updated_at: u64,
}

// New structures for unified knowledge search
#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct SearchResult {
    pub text: String,
    pub similarity: f32,
    pub category: String,
    pub importance: f32,
    pub source_info: String,  // For wiki: file name, for personality: channel
    pub content_type: String, // For wiki: section type, for personality: trait type
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct CategoryInfo {
    pub category: String,
    pub count: u32,
    pub description: String,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct KnowledgeStats {
    pub total_embeddings: u32,
    pub personality_embeddings: u32,
    pub wiki_embeddings: u32,
    pub categories: Vec<CategoryInfo>,
}

// Storage for personality embeddings (stable memory)
thread_local! {
    static PERSONALITY_EMBEDDINGS: std::cell::RefCell<Vec<PersonalityEmbedding>> = std::cell::RefCell::new(Vec::new());
    static USER_MEMORIES: std::cell::RefCell<Vec<UserMemory>> = std::cell::RefCell::new(Vec::new());
    static CONVERSATION_EMBEDDINGS: std::cell::RefCell<Vec<ConversationEmbedding>> = std::cell::RefCell::new(Vec::new());
    pub static USER_PROFILES: std::cell::RefCell<Vec<UserProfile>> = std::cell::RefCell::new(Vec::new());
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

pub fn get_all_user_profiles() -> Vec<UserProfile> {
    USER_PROFILES.with(|profiles| profiles.borrow().clone())
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

// === USER PROFILING ANALYSIS FUNCTIONS ===

/// Extract Big Five personality traits from conversation text
pub fn analyze_big_five_traits(conversation_texts: &[String]) -> BigFiveTraits {
    let combined_text = conversation_texts.join(" ").to_lowercase();
    
    // Openness: curiosity, creativity, intellectual interests
    let openness_markers = ["curious", "wonder", "imagine", "creative", "art", "new", "different", 
        "explore", "discover", "unique", "abstract", "philosophy", "novel", "innovative"];
    let openness_score = calculate_trait_score(&combined_text, &openness_markers);
    
    // Conscientiousness: organization, discipline, responsibility
    let conscientiousness_markers = ["organize", "plan", "schedule", "responsibility", "careful", 
        "detail", "precise", "thorough", "systematic", "disciplined", "reliable", "punctual"];
    let conscientiousness_score = calculate_trait_score(&combined_text, &conscientiousness_markers);
    
    // Extraversion: social energy, enthusiasm, assertiveness
    let extraversion_markers = ["excited", "enthusiastic", "social", "party", "meet", "talk", 
        "outgoing", "energetic", "assertive", "confident", "leader", "group"];
    let extraversion_score = calculate_trait_score(&combined_text, &extraversion_markers);
    
    // Agreeableness: cooperation, trust, empathy
    let agreeableness_markers = ["help", "kind", "empathy", "understand", "support", "care", 
        "cooperative", "trust", "compassion", "gentle", "generous", "considerate"];
    let agreeableness_score = calculate_trait_score(&combined_text, &agreeableness_markers);
    
    // Neuroticism: emotional instability, anxiety, stress
    let neuroticism_markers = ["anxious", "worry", "stress", "nervous", "upset", "emotional", 
        "unstable", "moody", "insecure", "fearful", "tense", "overwhelmed"];
    let neuroticism_score = calculate_trait_score(&combined_text, &neuroticism_markers);
    
    BigFiveTraits {
        openness: openness_score,
        conscientiousness: conscientiousness_score,
        extraversion: extraversion_score,
        agreeableness: agreeableness_score,
        neuroticism: neuroticism_score,
    }
}

/// Calculate trait score based on keyword frequency
fn calculate_trait_score(text: &str, markers: &[&str]) -> f32 {
    let word_count = text.split_whitespace().count() as f32;
    if word_count == 0.0 {
        return 0.5; // Default neutral score
    }
    
    let marker_count: f32 = markers
        .iter()
        .map(|marker| text.matches(marker).count() as f32)
        .sum();
    
    // Normalize to 0.0-1.0 range
    let raw_score = marker_count / word_count * 100.0; // Scale up for better resolution
    (raw_score.min(1.0).max(0.0) + 0.5).min(1.0) // Add baseline and cap at 1.0
}

/// Extract topic interests from conversation content
pub fn analyze_topic_interests(conversations: &[ConversationEmbedding]) -> Vec<TopicInterest> {
    let mut topic_stats: HashMap<String, (f32, u32, u64, u64)> = HashMap::new(); // (engagement, count, first, last)
    
    // Define topic keywords
    let topics = vec![
        ("technology", vec!["code", "programming", "computer", "software", "ai", "tech", "algorithm", "data"]),
        ("art", vec!["art", "painting", "drawing", "creative", "design", "aesthetic", "visual", "gallery"]),
        ("music", vec!["music", "song", "band", "album", "instrument", "melody", "concert", "rhythm"]),
        ("philosophy", vec!["philosophy", "meaning", "existence", "consciousness", "reality", "ethics", "moral"]),
        ("science", vec!["science", "research", "experiment", "theory", "discovery", "physics", "biology"]),
        ("relationships", vec!["love", "friend", "relationship", "family", "emotion", "feelings", "dating"]),
        ("gaming", vec!["game", "play", "gaming", "video", "console", "strategy", "rpg", "adventure"]),
        ("books", vec!["book", "read", "novel", "author", "story", "literature", "writing", "chapter"]),
        ("movies", vec!["movie", "film", "cinema", "actor", "director", "plot", "scene", "hollywood"]),
        ("food", vec!["food", "cook", "recipe", "restaurant", "taste", "flavor", "cuisine", "meal"])
    ];
    
    for conversation in conversations {
        let text_lower = conversation.conversation_text.to_lowercase();
        let timestamp = conversation.created_at;
        
        for (topic, keywords) in &topics {
            let mut topic_mentions = 0;
            let mut engagement_score = 0.0;
            
            for keyword in keywords {
                let count = text_lower.matches(keyword).count();
                topic_mentions += count;
                engagement_score += count as f32;
            }
            
            if topic_mentions > 0 {
                let entry = topic_stats.entry(topic.to_string()).or_insert((0.0, 0, timestamp, timestamp));
                entry.0 += engagement_score;
                entry.1 += topic_mentions as u32;
                entry.2 = entry.2.min(timestamp); // First mention
                entry.3 = entry.3.max(timestamp); // Last mention
            }
        }
    }
    
    topic_stats
        .into_iter()
        .map(|(topic, (engagement, count, first, last))| {
            let normalized_engagement = (engagement / conversations.len() as f32).min(1.0);
            let expertise_level = (count as f32 / 20.0).min(1.0); // Normalize to 0-1
            
            TopicInterest {
                topic,
                engagement_score: normalized_engagement,
                message_count: count,
                expertise_level,
                first_mentioned: first,
                last_mentioned: last,
            }
        })
        .filter(|interest| interest.engagement_score > 0.01) // Only include meaningful interests
        .collect()
}

/// Generate aggregated embedding for a user from their conversation embeddings
pub fn generate_user_embedding(user_id: &str) -> Vec<f32> {
    CONVERSATION_EMBEDDINGS.with(|conversations| {
        let borrowed_conversations = conversations.borrow();
        let user_conversations: Vec<&ConversationEmbedding> = borrowed_conversations
            .iter()
            .filter(|conv| conv.user_id == user_id)
            .collect();
            
        if user_conversations.is_empty() {
            return vec![0.0; 384]; // Return zero vector if no conversations
        }
        
        let embedding_dim = user_conversations[0].embedding.len();
        let mut aggregated = vec![0.0; embedding_dim];
        
        // Weight recent conversations more heavily (exponential decay)
        let now = ic_cdk::api::time();
        let mut total_weight = 0.0;
        
        for conversation in &user_conversations {
            // Calculate time-based weight (more recent = higher weight)
            let age_days = ((now - conversation.created_at) / (24 * 60 * 60 * 1_000_000_000)) as f32;
            let weight = (-age_days / 30.0).exp(); // 30-day half-life
            total_weight += weight;
            
            for (i, &value) in conversation.embedding.iter().enumerate() {
                aggregated[i] += value * weight;
            }
        }
        
        // Normalize by total weight
        if total_weight > 0.0 {
            for value in aggregated.iter_mut() {
                *value /= total_weight;
            }
        }
        
        // L2 normalize the final embedding
        let magnitude: f32 = aggregated.iter().map(|&x| x * x).sum::<f32>().sqrt();
        if magnitude > 0.0 {
            for value in aggregated.iter_mut() {
                *value /= magnitude;
            }
        }
        
        aggregated
    })
}

/// Generate or update a user profile from their conversation data
pub fn generate_user_profile(user_id: &str) -> Option<UserProfile> {
    let conversations = get_user_conversation_history(user_id, ""); // Get all channels
    
    if conversations.len() < 3 {
        return None; // Not enough data for reliable profiling
    }
    
    let conversation_texts: Vec<String> = conversations
        .iter()
        .map(|conv| conv.conversation_text.clone())
        .collect();
    
    let personality_traits = analyze_big_five_traits(&conversation_texts);
    let interests = analyze_topic_interests(&conversations);
    let aggregated_embedding = generate_user_embedding(user_id);
    
    let conversation_count = conversations.len() as u32;
    let total_messages: u32 = conversations.iter().map(|conv| conv.message_count).sum();
    let now = ic_cdk::api::time();
    
    let profile = UserProfile {
        user_id: user_id.to_string(),
        personality_traits,
        interests,
        aggregated_embedding,
        conversation_count,
        total_messages,
        created_at: now,
        updated_at: now,
    };
    
    // Store or update the profile
    USER_PROFILES.with(|profiles| {
        let mut borrowed_profiles = profiles.borrow_mut();
        
        // Remove existing profile if it exists
        borrowed_profiles.retain(|p| p.user_id != user_id);
        
        // Add new profile
        borrowed_profiles.push(profile.clone());
    });
    
    Some(profile)
}

/// Get user profile by ID
pub fn get_user_profile(user_id: &str) -> Option<UserProfile> {
    USER_PROFILES.with(|profiles| {
        profiles.borrow()
            .iter()
            .find(|profile| profile.user_id == user_id)
            .cloned()
    })
}

/// Get all user profiles
pub fn get_all_profiles() -> Vec<UserProfile> {
    USER_PROFILES.with(|profiles| profiles.borrow().clone())
}

// === UNIFIED KNOWLEDGE SEARCH FUNCTIONS ===

/// Search across both personality and wiki embeddings with unified ranking
pub fn search_unified_knowledge(
    query_embedding: &[f32], 
    categories: Option<Vec<String>>, 
    limit: usize
) -> Vec<SearchResult> {
    let mut all_results = Vec::new();
    
    PERSONALITY_EMBEDDINGS.with(|embeddings| {
        let borrowed_embeddings = embeddings.borrow();
        
        for embedding in borrowed_embeddings.iter() {
            // Filter by categories if specified
            if let Some(ref cats) = categories {
                if !cats.contains(&embedding.category) && !cats.iter().any(|cat| embedding.category.starts_with(cat)) {
                    continue;
                }
            }
            
            let similarity = cosine_similarity(query_embedding, &embedding.embedding);
            let source_info = if embedding.channel_id == "#wiki" {
                // Extract source file from the text or use a default
                if let Some(start) = embedding.text.find('[') {
                    if let Some(end) = embedding.text.find(']') {
                        embedding.text[start+1..end].to_string()
                    } else {
                        "wiki".to_string()
                    }
                } else {
                    embedding.channel_id.clone()
                }
            } else {
                embedding.channel_id.clone()
            };
            
            // Determine content type based on embedding fields
            let content_type = if embedding.channel_id == "#wiki" {
                // Try to extract content type from category
                if embedding.category.starts_with("wiki_") {
                    embedding.category[5..].to_string()
                } else {
                    "documentation".to_string()
                }
            } else {
                embedding.category.clone()
            };
            
            all_results.push(SearchResult {
                text: embedding.text.clone(),
                similarity,
                category: embedding.category.clone(),
                importance: embedding.importance,
                source_info,
                content_type,
            });
        }
    });
    
    // Sort by combined score: similarity * importance
    all_results.sort_by(|a, b| {
        let score_a = a.similarity * a.importance;
        let score_b = b.similarity * b.importance;
        score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
    });
    
    all_results.into_iter().take(limit).collect()
}

/// Search specifically for wiki content with optional filtering
pub fn search_wiki_content(
    query_embedding: &[f32], 
    content_type: Option<String>, 
    limit: usize
) -> Vec<SearchResult> {
    let wiki_categories: Vec<String> = if let Some(ct) = content_type {
        vec![format!("wiki_{}", ct)]
    } else {
        vec!["wiki_".to_string()]
    };
    
    search_unified_knowledge(query_embedding, Some(wiki_categories), limit)
}

/// Get available knowledge categories with counts
pub fn get_knowledge_categories() -> Vec<CategoryInfo> {
    let mut category_counts: HashMap<String, u32> = HashMap::new();
    
    PERSONALITY_EMBEDDINGS.with(|embeddings| {
        let borrowed_embeddings = embeddings.borrow();
        
        for embedding in borrowed_embeddings.iter() {
            *category_counts.entry(embedding.category.clone()).or_insert(0) += 1;
        }
    });
    
    let mut categories = Vec::new();
    for (category, count) in category_counts {
        let description = if category.starts_with("wiki_") {
            match category.as_str() {
                "wiki_project-docs" => "Documentation for specific LainCorp projects",
                "wiki_tech-guides" => "Technical guides and development documentation", 
                "wiki_meta-docs" => "Meta documentation and contribution guides",
                _ => "Wiki documentation content"
            }
        } else {
            match category.as_str() {
                "core_belief" => "Fundamental worldview and philosophical beliefs",
                "communication_style" => "Preferred ways of interacting and communicating",
                "technical_preference" => "Technology opinions and technical choices",
                "social_trait" => "Social behavior patterns and characteristics",
                "work_habit" => "Development practices and work behaviors",
                "artistic_taste" => "Creative preferences and aesthetic opinions",
                "music_preference" => "Musical tastes and audio preferences",
                _ => "Personality trait or preference"
            }
        }.to_string();
        
        categories.push(CategoryInfo {
            category,
            count,
            description,
        });
    }
    
    // Sort by count (descending)
    categories.sort_by(|a, b| b.count.cmp(&a.count));
    categories
}

/// Get overall knowledge base statistics
pub fn get_knowledge_stats() -> KnowledgeStats {
    let categories = get_knowledge_categories();
    let mut total_embeddings = 0;
    let mut personality_embeddings = 0;
    let mut wiki_embeddings = 0;
    
    for category in &categories {
        total_embeddings += category.count;
        if category.category.starts_with("wiki_") {
            wiki_embeddings += category.count;
        } else {
            personality_embeddings += category.count;
        }
    }
    
    KnowledgeStats {
        total_embeddings,
        personality_embeddings,
        wiki_embeddings,
        categories,
    }
}
