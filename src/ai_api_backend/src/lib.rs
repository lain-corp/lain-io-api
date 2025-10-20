use candid::{CandidType, Deserialize};
use ic_llm::{ChatMessage, Model};

mod context;
mod personality;

use context::{RoomConfig, get_system_prompt_for_room, get_all_room_configs, get_enhanced_system_prompt_for_room};
use personality::{
    PersonalityEmbedding,
    UserMemory,
    store_personality_embedding,
    store_user_memory,
    get_all_personality_embeddings,
    search_personality_context,
    get_channel_personality_context
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

