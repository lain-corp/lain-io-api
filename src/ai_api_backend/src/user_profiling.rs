use candid::{CandidType, Deserialize};
use crate::personality::{UserProfile, ConversationEmbedding, BigFiveTraits, TopicInterest};

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ConversationStyle {
    pub formality: f32,        // 0.0 = very casual, 1.0 = very formal
    pub emotiveness: f32,      // 0.0 = very logical, 1.0 = very emotional  
    pub verbosity: f32,        // 0.0 = very concise, 1.0 = very verbose
    pub politeness: f32,       // 0.0 = rude, 1.0 = very polite
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct PersonalityTraits {
    pub openness: f32,         // 0.0-1.0
    pub conscientiousness: f32, // 0.0-1.0
    pub extraversion: f32,     // 0.0-1.0  
    pub agreeableness: f32,    // 0.0-1.0
    pub neuroticism: f32,      // 0.0-1.0
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct InteractionPatterns {
    pub response_time_pattern: f32,    // How quickly they respond (normalized)
    pub conversation_initiation: f32,   // How often they start conversations  
    pub topic_switching: f32,           // How often they change topics
    pub question_asking: f32,           // How often they ask questions
}

impl UserProfile {
    /// Calculate multi-dimensional similarity between two user profiles
    pub fn calculate_similarity(&self, other: &UserProfile) -> f32 {
        // 1. Semantic similarity using aggregated embeddings (35% weight)
        let semantic_similarity = calculate_cosine_similarity(&self.aggregated_embedding, &other.aggregated_embedding);
        
        // 2. Personality trait similarity (25% weight)  
        let personality_similarity = calculate_personality_similarity(&self.personality_traits, &other.personality_traits);
        
        // 3. Interest overlap (20% weight)
        let interest_similarity = calculate_interest_overlap(&self.interests, &other.interests);
        
        // 4. Conversation style similarity (15% weight)
        let conversations_self = crate::personality::get_user_conversation_history(&self.user_id, "");
        let conversations_other = crate::personality::get_user_conversation_history(&other.user_id, ""); 
        let style_similarity = calculate_style_similarity(&conversations_self, &conversations_other);
        
        // 5. Interaction patterns (5% weight)
        let interaction_similarity = calculate_interaction_similarity(&conversations_self, &conversations_other);
        
        // Weighted combination
        let overall_similarity = semantic_similarity * 0.35 +
            personality_similarity * 0.25 +
            interest_similarity * 0.20 +
            style_similarity * 0.15 +
            interaction_similarity * 0.05;
            
        overall_similarity.min(1.0).max(0.0)
    }
}

/// Calculate cosine similarity between two embedding vectors
fn calculate_cosine_similarity(vec1: &[f32], vec2: &[f32]) -> f32 {
    if vec1.len() != vec2.len() {
        return 0.0;
    }
    
    let dot_product: f32 = vec1.iter().zip(vec2.iter()).map(|(a, b)| a * b).sum();
    let magnitude1: f32 = vec1.iter().map(|x| x * x).sum::<f32>().sqrt();
    let magnitude2: f32 = vec2.iter().map(|x| x * x).sum::<f32>().sqrt();
    
    if magnitude1 == 0.0 || magnitude2 == 0.0 {
        return 0.0;
    }
    
    (dot_product / (magnitude1 * magnitude2)).max(-1.0).min(1.0)
}

/// Calculate personality trait similarity using Big Five
fn calculate_personality_similarity(traits1: &BigFiveTraits, traits2: &BigFiveTraits) -> f32 {
    let openness_diff = (traits1.openness - traits2.openness).abs();
    let conscientiousness_diff = (traits1.conscientiousness - traits2.conscientiousness).abs();
    let extraversion_diff = (traits1.extraversion - traits2.extraversion).abs();
    let agreeableness_diff = (traits1.agreeableness - traits2.agreeableness).abs();
    let neuroticism_diff = (traits1.neuroticism - traits2.neuroticism).abs();
    
    let avg_diff = (openness_diff + conscientiousness_diff + extraversion_diff + agreeableness_diff + neuroticism_diff) / 5.0;
    1.0 - avg_diff // Convert difference to similarity
}

/// Calculate interest overlap between two users
fn calculate_interest_overlap(interests1: &[TopicInterest], interests2: &[TopicInterest]) -> f32 {
    if interests1.is_empty() || interests2.is_empty() {
        return 0.0;
    }
    
    let mut total_similarity = 0.0;
    let mut comparison_count = 0;
    
    for interest1 in interests1 {
        for interest2 in interests2 {
            if interest1.topic == interest2.topic {
                // Same topic - calculate engagement similarity
                let engagement_similarity = 1.0 - (interest1.engagement_score - interest2.engagement_score).abs();
                total_similarity += engagement_similarity * (interest1.engagement_score + interest2.engagement_score) / 2.0;
                comparison_count += 1;
            }
        }
    }
    
    if comparison_count == 0 {
        return 0.0;
    }
    
    total_similarity / comparison_count as f32
}

/// Calculate conversation style similarity
fn calculate_style_similarity(conversations1: &[ConversationEmbedding], conversations2: &[ConversationEmbedding]) -> f32 {
    let style1 = analyze_conversation_style(conversations1);
    let style2 = analyze_conversation_style(conversations2);
    
    let formality_similarity = 1.0 - (style1.formality - style2.formality).abs();
    let emotiveness_similarity = 1.0 - (style1.emotiveness - style2.emotiveness).abs();
    let verbosity_similarity = 1.0 - (style1.verbosity - style2.verbosity).abs();
    let politeness_similarity = 1.0 - (style1.politeness - style2.politeness).abs();
    
    (formality_similarity + emotiveness_similarity + verbosity_similarity + politeness_similarity) / 4.0
}

/// Analyze conversation style from conversation embeddings
fn analyze_conversation_style(conversations: &[ConversationEmbedding]) -> ConversationStyle {
    if conversations.is_empty() {
        return ConversationStyle {
            formality: 0.5,
            emotiveness: 0.5, 
            verbosity: 0.5,
            politeness: 0.5,
        };
    }
    
    let formality = analyze_formality(conversations);
    let emotiveness = analyze_emotiveness(conversations);
    let verbosity = analyze_verbosity(conversations);
    let politeness = analyze_politeness(conversations);
    
    ConversationStyle {
        formality,
        emotiveness,
        verbosity,
        politeness,
    }
}

/// Calculate interaction patterns similarity  
fn calculate_interaction_similarity(conversations1: &[ConversationEmbedding], conversations2: &[ConversationEmbedding]) -> f32 {
    let patterns1 = analyze_interaction_patterns(conversations1);
    let patterns2 = analyze_interaction_patterns(conversations2);
    
    let response_similarity = 1.0 - (patterns1.response_time_pattern - patterns2.response_time_pattern).abs();
    let initiation_similarity = 1.0 - (patterns1.conversation_initiation - patterns2.conversation_initiation).abs();
    let switching_similarity = 1.0 - (patterns1.topic_switching - patterns2.topic_switching).abs();
    let question_similarity = 1.0 - (patterns1.question_asking - patterns2.question_asking).abs();
    
    (response_similarity + initiation_similarity + switching_similarity + question_similarity) / 4.0
}

/// Analyze interaction patterns from conversation embeddings
fn analyze_interaction_patterns(conversations: &[ConversationEmbedding]) -> InteractionPatterns {
    if conversations.is_empty() {
        return InteractionPatterns {
            response_time_pattern: 0.5,
            conversation_initiation: 0.5,
            topic_switching: 0.5,
            question_asking: 0.5,
        };
    }
    
    // Simple heuristics based on conversation metadata
    let avg_message_length = conversations.iter()
        .map(|conv| conv.conversation_text.len())
        .sum::<usize>() as f32 / conversations.len() as f32;
    
    let question_count = conversations.iter()
        .map(|conv| conv.conversation_text.matches('?').count())
        .sum::<usize>() as f32;
    
    let total_messages: f32 = conversations.iter()
        .map(|conv| conv.message_count as f32)
        .sum();
        
    InteractionPatterns {
        response_time_pattern: 0.5, // Placeholder - would need timestamp analysis
        conversation_initiation: (conversations.len() as f32 / 10.0).min(1.0), // Rough heuristic
        topic_switching: (avg_message_length / 100.0).min(1.0), // Longer messages suggest fewer topic switches
        question_asking: (question_count / total_messages).min(1.0),
    }
}

/// Simple heuristics for analyzing conversation characteristics
fn analyze_formality(conversations: &[ConversationEmbedding]) -> f32 {
    // Count formal vs informal words
    let formal_markers = ["please", "thank you", "would you", "could you", "I would appreciate"];
    let informal_markers = ["hey", "yeah", "yep", "nah", "gonna", "wanna", "lol"];
    
    let mut formal_count = 0;
    let mut informal_count = 0;
    
    for conversation in conversations {
        let text_lower = conversation.conversation_text.to_lowercase();
        formal_count += formal_markers.iter().map(|marker| text_lower.matches(marker).count()).sum::<usize>();
        informal_count += informal_markers.iter().map(|marker| text_lower.matches(marker).count()).sum::<usize>();
    }
    
    let total = formal_count + informal_count;
    if total == 0 {
        return 0.5; // Default neutral
    }
    
    formal_count as f32 / total as f32
}

fn analyze_emotiveness(conversations: &[ConversationEmbedding]) -> f32 {
    let emotional_markers = ["feel", "emotion", "love", "hate", "excited", "sad", "happy", "angry", "!"];
    let logical_markers = ["analyze", "logic", "reason", "evidence", "data", "fact", "conclude"];
    
    let mut emotional_count = 0;
    let mut logical_count = 0;
    
    for conversation in conversations {
        let text_lower = conversation.conversation_text.to_lowercase();
        emotional_count += emotional_markers.iter().map(|marker| text_lower.matches(marker).count()).sum::<usize>();
        logical_count += logical_markers.iter().map(|marker| text_lower.matches(marker).count()).sum::<usize>();
    }
    
    let total = emotional_count + logical_count;
    if total == 0 {
        return 0.5;
    }
    
    emotional_count as f32 / total as f32
}

fn analyze_verbosity(conversations: &[ConversationEmbedding]) -> f32 {
    if conversations.is_empty() {
        return 0.5;
    }
    
    let avg_length = conversations.iter()
        .map(|conv| conv.conversation_text.len())
        .sum::<usize>() as f32 / conversations.len() as f32;
    
    // Normalize to 0-1 scale (assume 500 chars is very verbose)
    (avg_length / 500.0).min(1.0)
}

fn analyze_politeness(conversations: &[ConversationEmbedding]) -> f32 {
    let polite_markers = ["please", "thank", "sorry", "excuse me", "pardon"];
    let impolite_markers = ["shut up", "stupid", "idiot", "whatever"];
    
    let mut polite_count = 0;
    let mut impolite_count = 0;
    
    for conversation in conversations {
        let text_lower = conversation.conversation_text.to_lowercase();
        polite_count += polite_markers.iter().map(|marker| text_lower.matches(marker).count()).sum::<usize>();
        impolite_count += impolite_markers.iter().map(|marker| text_lower.matches(marker).count()).sum::<usize>();
    }
    
    let total = polite_count + impolite_count;
    if total == 0 {
        return 0.7; // Default slightly polite
    }
    
    polite_count as f32 / total as f32
}

/// Public API function for calculating similarity between users
pub fn calculate_user_similarity(profile1: &UserProfile, profile2: &UserProfile) -> f32 {
    profile1.calculate_similarity(profile2)
}

/// Get friendship recommendations for a user
pub fn get_friendship_recommendations(user_id: &str, limit: u32) -> Vec<(String, f32)> {
    use crate::personality::get_all_profiles;
    
    let target_profile = match crate::personality::get_user_profile(user_id) {
        Some(profile) => profile,
        None => return Vec::new(),
    };
    
    let all_profiles = get_all_profiles();
    let mut similarities: Vec<(String, f32)> = all_profiles
        .iter()
        .filter(|profile| profile.user_id != user_id) // Exclude self
        .map(|profile| {
            let similarity = calculate_user_similarity(&target_profile, profile);
            (profile.user_id.clone(), similarity)
        })
        .collect();
    
    // Sort by similarity (highest first)
    similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    
    // Return top recommendations
    similarities.into_iter().take(limit as usize).collect()
}