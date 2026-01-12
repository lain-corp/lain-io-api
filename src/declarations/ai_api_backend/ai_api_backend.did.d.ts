import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface big_five_traits {
  'conscientiousness' : number,
  'neuroticism' : number,
  'agreeableness' : number,
  'extraversion' : number,
  'openness' : number,
}
/**
 * Category info for knowledge base
 */
export interface category_info {
  'count' : number,
  'description' : string,
  'category' : string,
}
export type chat_message = {
    'tool' : { 'content' : string, 'tool_call_id' : string }
  } |
  { 'user' : { 'content' : string } } |
  {
    'assistant' : {
      'content' : [] | [string],
      'tool_calls' : Array<
        {
          'id' : string,
          'function' : {
            'name' : string,
            'arguments' : Array<{ 'value' : string, 'name' : string }>,
          },
        }
      >,
    }
  } |
  { 'system' : { 'content' : string } };
export interface conversation_embedding {
  'channel_id' : string,
  'chunk_index' : number,
  'conversation_text' : string,
  'created_at' : bigint,
  'user_id' : string,
  'summary' : string,
  'message_count' : number,
  'embedding' : Array<number>,
}
/**
 * Knowledge base statistics
 */
export interface knowledge_stats {
  'categories' : Array<category_info>,
  'personality_embeddings' : number,
  'wiki_embeddings' : number,
  'total_embeddings' : number,
}
export interface personality_embedding {
  'channel_id' : string,
  'text' : string,
  'importance' : number,
  'created_at' : bigint,
  'category' : string,
  'embedding' : Array<number>,
}
export interface room_config {
  'id' : string,
  'name' : string,
  'description' : string,
}
/**
 * Search result type for unified knowledge search
 */
export interface search_result {
  'text' : string,
  'content_type' : string,
  'importance' : number,
  'source_info' : string,
  'similarity' : number,
  'category' : string,
}
export interface topic_interest {
  'expertise_level' : number,
  'topic' : string,
  'last_mentioned' : bigint,
  'message_count' : number,
  'engagement_score' : number,
  'first_mentioned' : bigint,
}
export interface user_profile {
  'updated_at' : bigint,
  'personality_traits' : big_five_traits,
  'interests' : Array<topic_interest>,
  'created_at' : bigint,
  'user_id' : string,
  'total_messages' : number,
  'aggregated_embedding' : Array<number>,
  'conversation_count' : number,
}
export interface _SERVICE {
  'analyze_user_interests' : ActorMethod<[string], Array<topic_interest>>,
  'analyze_user_personality' : ActorMethod<[string], [] | [big_five_traits]>,
  'calculate_user_similarity' : ActorMethod<[string, string], [] | [number]>,
  'chat' : ActorMethod<[Array<chat_message>, [] | [string]], string>,
  'chat_default' : ActorMethod<[Array<chat_message>], string>,
  'chat_with_knowledge' : ActorMethod<
    [Array<chat_message>, [] | [string], Array<number>, [] | [Array<string>]],
    string
  >,
  'chat_with_rag' : ActorMethod<
    [Array<chat_message>, [] | [string], Array<number>],
    string
  >,
  'chat_with_user_context' : ActorMethod<
    [Array<chat_message>, string, [] | [string], Array<number>],
    string
  >,
  'create_user_profile' : ActorMethod<[string], [] | [user_profile]>,
  'get_all_user_profiles' : ActorMethod<[], Array<user_profile>>,
  'get_available_rooms' : ActorMethod<[], Array<room_config>>,
  'get_friendship_recommendations' : ActorMethod<
    [string, [] | [number]],
    Array<[string, number]>
  >,
  'get_knowledge_categories' : ActorMethod<[], Array<category_info>>,
  'get_knowledge_stats' : ActorMethod<[], knowledge_stats>,
  'get_next_conversation_chunk_index' : ActorMethod<[string, string], number>,
  'get_personality_embeddings' : ActorMethod<[], Array<personality_embedding>>,
  'get_recent_user_conversations' : ActorMethod<
    [string, string, [] | [number]],
    Array<string>
  >,
  'get_user_conversation_stats' : ActorMethod<
    [string, string],
    [number, number]
  >,
  'get_user_conversations' : ActorMethod<
    [string, string],
    Array<conversation_embedding>
  >,
  /**
   * User Profiling API
   */
  'get_user_profile_by_id' : ActorMethod<[string], [] | [user_profile]>,
  /**
   * Text-based search (NO EMBEDDING REQUIRED - uses keyword matching)
   */
  'search_knowledge_by_text' : ActorMethod<
    [string, [] | [Array<string>], [] | [number]],
    Array<search_result>
  >,
  'search_personality' : ActorMethod<[string, Array<number>], Array<string>>,
  /**
   * Unified Knowledge Search API (searches across all personality + wiki embeddings)
   */
  'search_unified_knowledge' : ActorMethod<
    [Array<number>, [] | [Array<string>], [] | [number]],
    Array<search_result>
  >,
  'search_user_conversation_history' : ActorMethod<
    [string, string, Array<number>, [] | [number]],
    Array<string>
  >,
  'search_wiki_content' : ActorMethod<
    [Array<number>, [] | [string], [] | [number]],
    Array<search_result>
  >,
  'store_conversation_chunk' : ActorMethod<[conversation_embedding], string>,
  'store_personality' : ActorMethod<[personality_embedding], string>,
  'store_personality_batch' : ActorMethod<
    [Array<personality_embedding>],
    string
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
