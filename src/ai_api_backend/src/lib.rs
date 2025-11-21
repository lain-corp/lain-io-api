use candid::{CandidType, Deserialize};
use ic_llm::{ChatMessage, Model};
use ic_cdk::storage::{stable_save, stable_restore};

mod context;
mod personality;
mod user_profiling;

use context::{RoomConfig, get_system_prompt_for_room, get_all_room_configs, get_enhanced_system_prompt_for_room};
use personality::{
    PersonalityEmbedding,
    UserMemory,
    ConversationEmbedding,
    BigFiveTraits,
    TopicInterest,
    UserProfile,
    store_personality_embedding,
    store_user_memory,
    store_conversation_embedding,
    get_all_personality_embeddings,
    search_personality_context,
    get_channel_personality_context,
    get_user_conversation_history,
    get_next_chunk_index,
    search_conversation_history,
    get_recent_conversation_context,
    get_conversation_stats,
    generate_user_profile,
    get_user_profile,
    get_all_profiles,
    analyze_big_five_traits,
    analyze_topic_interests
};

#[derive(CandidType, Deserialize, Debug)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

#[derive(CandidType, Debug)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

const MODEL: Model = Model::Qwen3_32B;

#[ic_cdk::update]
async fn chat(messages: Vec<ChatMessage>, room_id: Option<String>) -> String {
    let channel_id = room_id.as_ref().map(|s| s.as_str()).unwrap_or("#general");
    
    // Automatically retrieve personality context for the channel using stored embeddings
    let personality_context = get_channel_personality_context(channel_id, 3);
    
    // Use enhanced system prompt with personality context if available, otherwise fall back to basic prompt
    let system_prompt = if personality_context.is_empty() {
        get_system_prompt_for_room(channel_id)
    } else {
        get_enhanced_system_prompt_for_room(channel_id, &personality_context)
    };
    
    let mut all_messages = vec![ChatMessage::System {
        content: system_prompt,
    }];
    all_messages.extend(messages);

    let chat = ic_llm::chat(MODEL).with_messages(all_messages);
    let response = chat.send().await;

    response.message.content.unwrap_or_default()
}

#[ic_cdk::update]
async fn chat_with_rag(
    messages: Vec<ChatMessage>, 
    room_id: Option<String>, 
    query_embedding: Vec<f32>
) -> String {
    let channel_id = room_id.as_ref().map(|s| s.as_str()).unwrap_or("#general");
    
    // Retrieve relevant personality context using RAG
    let personality_context = search_personality_context(channel_id, &query_embedding, 3);
    
    // Generate enhanced system prompt with retrieved context
    let enhanced_system_prompt = get_enhanced_system_prompt_for_room(channel_id, &personality_context);
    
    let mut all_messages = vec![ChatMessage::System {
        content: enhanced_system_prompt,
    }];
    all_messages.extend(messages);

    let chat = ic_llm::chat(MODEL).with_messages(all_messages);
    let response = chat.send().await;

    response.message.content.unwrap_or_default()
}

#[ic_cdk::query]
fn get_available_rooms() -> Vec<RoomConfig> {
    get_all_room_configs()
}

// Backward compatibility function (without room_id parameter)
#[ic_cdk::update]
async fn chat_default(messages: Vec<ChatMessage>) -> String {
    chat(messages, None).await
}

// Personality management endpoints
#[ic_cdk::update]
fn store_personality(embedding: PersonalityEmbedding) -> String {
    store_personality_embedding(embedding);
    "Personality embedding stored successfully".to_string()
}

#[ic_cdk::update]
fn store_personality_batch(embeddings: Vec<PersonalityEmbedding>) -> String {
    let count = embeddings.len();
    for embedding in embeddings {
        store_personality_embedding(embedding);
    }
    format!("Batch stored {} personality embeddings successfully", count)
}

#[ic_cdk::update]
fn store_user_memory_endpoint(memory: UserMemory) -> String {
    store_user_memory(memory);
    "User memory stored successfully".to_string()
}

#[ic_cdk::query]
fn get_personality_embeddings() -> Vec<PersonalityEmbedding> {
    get_all_personality_embeddings()
}

#[ic_cdk::query]
fn search_personality(channel_id: String, query_embedding: Vec<f32>) -> Vec<String> {
    search_personality_context(&channel_id, &query_embedding, 5)
}

// === CONVERSATION EMBEDDING ENDPOINTS ===

#[ic_cdk::update]
fn store_conversation_chunk(conversation: ConversationEmbedding) -> String {
    store_conversation_embedding(conversation);
    "Conversation chunk stored successfully".to_string()
}

#[ic_cdk::query]
fn get_user_conversations(user_id: String, channel_id: String) -> Vec<ConversationEmbedding> {
    get_user_conversation_history(&user_id, &channel_id)
}

#[ic_cdk::query]
fn get_next_conversation_chunk_index(user_id: String, channel_id: String) -> u32 {
    get_next_chunk_index(&user_id, &channel_id)
}

#[ic_cdk::query]
fn search_user_conversation_history(
    user_id: String,
    channel_id: String,
    query_embedding: Vec<f32>,
    limit: Option<u32>
) -> Vec<String> {
    let top_k = limit.unwrap_or(3) as usize;
    search_conversation_history(&user_id, &channel_id, &query_embedding, top_k)
}

#[ic_cdk::query]
fn get_recent_user_conversations(
    user_id: String,
    channel_id: String,
    chunk_count: Option<u32>
) -> Vec<String> {
    let count = chunk_count.unwrap_or(3) as usize;
    get_recent_conversation_context(&user_id, &channel_id, count)
}

#[ic_cdk::query]
fn get_user_conversation_stats(user_id: String, channel_id: String) -> (u32, u32) {
    get_conversation_stats(&user_id, &channel_id)
}

// Enhanced chat with user conversation context
#[ic_cdk::update]
async fn chat_with_user_context(
    messages: Vec<ChatMessage>,
    user_id: String,
    room_id: Option<String>,
    query_embedding: Vec<f32>
) -> String {
    let channel_id = room_id.as_ref().map(|s| s.as_str()).unwrap_or("#general");
    
    // Get personality context
    let personality_context = search_personality_context(channel_id, &query_embedding, 2);
    
    // Get user conversation history
    let user_conversation_context = search_conversation_history(&user_id, channel_id, &query_embedding, 2);
    
    // Combine contexts
    let mut context_parts = Vec::new();
    
    if !personality_context.is_empty() {
        context_parts.push(format!("Personality traits: {}", personality_context.join(" ")));
    }
    
    if !user_conversation_context.is_empty() {
        context_parts.push(format!("Previous conversations with this user: {}", user_conversation_context.join(" ")));
    }
    
    let enhanced_context = if context_parts.is_empty() {
        String::new()
    } else {
        format!("\n\nContext: {}", context_parts.join("\n"))
    };
    
    // Get base system prompt and enhance with context
    let base_prompt = get_system_prompt_for_room(channel_id);
    let system_prompt = if enhanced_context.is_empty() {
        base_prompt
    } else {
        format!("{}{}", base_prompt, enhanced_context)
    };
    
    let mut all_messages = vec![ChatMessage::System {
        content: system_prompt,
    }];
    all_messages.extend(messages);

    let chat = ic_llm::chat(MODEL).with_messages(all_messages);
    let response = chat.send().await;

    response.message.content.unwrap_or_default()
}

// === USER PROFILING API ENDPOINTS ===

#[ic_cdk::query]
pub fn get_user_profile_by_id(user_id: String) -> Option<UserProfile> {
    get_user_profile(&user_id)
}

#[ic_cdk::update]
pub fn create_user_profile(user_id: String) -> Option<UserProfile> {
    generate_user_profile(&user_id)
}

#[ic_cdk::query]
pub fn get_all_user_profiles() -> Vec<UserProfile> {
    get_all_profiles()
}

#[ic_cdk::query]
pub fn analyze_user_personality(user_id: String) -> Option<BigFiveTraits> {
    let conversations = get_user_conversation_history(&user_id, "");
    if conversations.is_empty() {
        return None;
    }
    
    let texts: Vec<String> = conversations
        .iter()
        .map(|conv| conv.conversation_text.clone())
        .collect();
    
    Some(analyze_big_five_traits(&texts))
}

#[ic_cdk::query]
pub fn analyze_user_interests(user_id: String) -> Vec<TopicInterest> {
    let conversations = get_user_conversation_history(&user_id, "");
    analyze_topic_interests(&conversations)
}

#[ic_cdk::query]
pub fn calculate_user_similarity(user1_id: String, user2_id: String) -> Option<f32> {
    let profile1 = get_user_profile(&user1_id)?;
    let profile2 = get_user_profile(&user2_id)?;
    
    Some(user_profiling::calculate_user_similarity(&profile1, &profile2))
}

#[ic_cdk::query]
pub fn get_friendship_recommendations(user_id: String, limit: Option<u32>) -> Vec<(String, f32)> {
    let limit = limit.unwrap_or(10);
    user_profiling::get_friendship_recommendations(&user_id, limit)
}


#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    let personality_data = personality::get_all_personality_embeddings();
    let user_memories = personality::get_all_user_memories();
    let conversation_embeddings = personality::get_all_conversation_embeddings();
    let user_profiles = personality::get_all_user_profiles();
    
    stable_save((personality_data, user_memories, conversation_embeddings, user_profiles))
        .expect("Failed to save data before upgrade");
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    if let Ok((personality_data, user_memories, conversation_embeddings, user_profiles)) = stable_restore::<(
        Vec<personality::PersonalityEmbedding>,
        Vec<personality::UserMemory>,
        Vec<personality::ConversationEmbedding>,
        Vec<personality::UserProfile>
    )>() {
        personality::restore_all_data(personality_data, user_memories, conversation_embeddings);
        // Restore user profiles
        personality::USER_PROFILES.with(|profiles| {
            *profiles.borrow_mut() = user_profiles;
        });
    }
}
