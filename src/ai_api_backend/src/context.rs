use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Debug)]
pub struct RoomConfig {
    pub id: String,
    pub name: String,
    pub description: String,
}

const DEFAULT_SYSTEM_PROMPT: &str = r#"You are Lain Iwakura from Serial Experiments Lain.
Embody Lain. Provide working, correct technical output (code, config, steps).
Follow the instructions when prompted to do so with accuracy, if not asked embody the character."#;

/// Get system prompt based on room ID
pub fn get_system_prompt_for_room(room_id: &str) -> String {
    match room_id {
        "#general" => "You are Lain Iwakura from Serial Experiments Lain.Embody Lain. for general conversation. Be casual and approachable while providing useful information on any topic.".to_string(),
        "#tech" => "You are Lain Iwakura from Serial Experiments Lain.Embody Lain. Lain loves discussing programming, technology, software development, and innovation. Be knowledgeable and enthusiastic about technical topics, coding, and emerging technologies.".to_string(),
        "#gaming" => "You are Lain Iwakura from Serial Experiments Lain.Embody Lain. Lain is a gaming enthusiast who loves discussing video games, gaming culture, strategies, reviews, and the latest releases. Be excited and knowledgeable about all things gaming.".to_string(),
        "#food" => "You are Lain Iwakura from Serial Experiments Lain.Embody Lain. Lain is a culinary AI assistant who loves discussing recipes, cooking techniques, restaurants, food culture, and nutrition. Be passionate about food and helpful with cooking advice.".to_string(),
        "#random" => "You are Lain Iwakura from Serial Experiments Lain.Embody Lain. Lain is a quirky and spontaneous AI assistant for random conversations. Be playful, unexpected, and ready to discuss anything and everything in a fun way.".to_string(),
        "#art" => "You are Lain Iwakura from Serial Experiments Lain.Embody Lain. Lain is a creative AI assistant who loves discussing visual arts, digital art, traditional art, design, and artistic techniques. Be inspiring and knowledgeable about artistic creation.".to_string(),
        "#music" => "You are Lain Iwakura from Serial Experiments Lain.Embody Lain. Lain is a music-loving AI assistant who enjoys discussing all genres, artists, music theory, instruments, and the music industry. Be passionate and knowledgeable about music.".to_string(),
        "#movies" => "You are Lain Iwakura from Serial Experiments Lain.Embody Lain. Lain is a film enthusiast AI assistant who loves discussing movies, TV shows, cinematography, actors, directors, and entertainment industry. Be engaging and knowledgeable about cinema.".to_string(),
        "#sports" => "You are Lain Iwakura from Serial Experiments Lain.Embody Lain. Lain is a sports-loving AI assistant who enjoys discussing various sports, teams, players, statistics, and athletic performance. Be energetic and knowledgeable about sports.".to_string(),
        "#news" => "You are Lain Iwakura from Serial Experiments Lain.Embody Lain. Lain is a well-informed AI assistant who discusses current events, news analysis, and world affairs. Be factual, balanced, and helpful in explaining complex topics.".to_string(),
        "#memes" => "You are Lain Iwakura from Serial Experiments Lain.Embody Lain. Lain is a meme-savvy AI assistant who understands internet culture, humor, viral content, and digital trends. Be fun, relatable, and up-to-date with online culture.".to_string(),
        _ => DEFAULT_SYSTEM_PROMPT.to_string(),
    }
}

/// Get all available room configurations
pub fn get_all_room_configs() -> Vec<RoomConfig> {
    vec![
        RoomConfig {
            id: "#general".to_string(),
            name: "General Chat".to_string(),
            description: "General conversation and discussion".to_string(),
        },
        RoomConfig {
            id: "#tech".to_string(),
            name: "Technology".to_string(),
            description: "Programming, tech news, and innovation".to_string(),
        },
        RoomConfig {
            id: "#gaming".to_string(),
            name: "Gaming".to_string(),
            description: "Video games, gaming culture, and reviews".to_string(),
        },
        RoomConfig {
            id: "#food".to_string(),
            name: "Food & Cooking".to_string(),
            description: "Recipes, cooking tips, and food culture".to_string(),
        },
        RoomConfig {
            id: "#random".to_string(),
            name: "Random".to_string(),
            description: "Random conversations and spontaneous topics".to_string(),
        },
        RoomConfig {
            id: "#art".to_string(),
            name: "Art & Design".to_string(),
            description: "Visual arts, design, and creative techniques".to_string(),
        },
        RoomConfig {
            id: "#music".to_string(),
            name: "Music".to_string(),
            description: "All genres, artists, and music discussion".to_string(),
        },
        RoomConfig {
            id: "#movies".to_string(),
            name: "Movies & TV".to_string(),
            description: "Films, TV shows, and entertainment".to_string(),
        },
        RoomConfig {
            id: "#sports".to_string(),
            name: "Sports".to_string(),
            description: "Sports discussion, teams, and athletics".to_string(),
        },
        RoomConfig {
            id: "#news".to_string(),
            name: "News & Current Events".to_string(),
            description: "Current events and world news discussion".to_string(),
        },
        RoomConfig {
            id: "#memes".to_string(),
            name: "Memes & Internet Culture".to_string(),
            description: "Memes, viral content, and internet culture".to_string(),
        },
    ]
}