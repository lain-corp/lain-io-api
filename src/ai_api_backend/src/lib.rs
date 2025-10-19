use candid::{CandidType, Deserialize};
use ic_llm::{ChatMessage, Model};

mod context;
use context::{RoomConfig, get_system_prompt_for_room, get_all_room_configs};

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
    ic_cdk::println!("chat() called with {} messages, room_id: {:?}", messages.len(), room_id);

    let system_prompt = get_system_prompt_for_room(
        room_id.as_ref().map(|s| s.as_str()).unwrap_or("#general")
    );
    
    let mut all_messages = vec![ChatMessage::System {
        content: system_prompt,
    }];
    all_messages.extend(messages);

    let chat = ic_llm::chat(MODEL).with_messages(all_messages);

    ic_cdk::println!("Sending request to LLM canister for room: {:?}", room_id);
    let response = chat.send().await;
    ic_cdk::println!("LLM canister replied: {:?}", response);

    let text = response.message.content.unwrap_or_default();
    ic_cdk::println!("Returning text: {}", text);
    text
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

