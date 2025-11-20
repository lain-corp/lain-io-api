import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

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
export interface _SERVICE {
  'chat' : ActorMethod<[Array<chat_message>, [] | [string]], string>,
  'chat_default' : ActorMethod<[Array<chat_message>], string>,
  'chat_with_rag' : ActorMethod<
    [Array<chat_message>, [] | [string], Array<number>],
    string
  >,
  'chat_with_user_context' : ActorMethod<
    [Array<chat_message>, string, [] | [string], Array<number>],
    string
  >,
  'get_available_rooms' : ActorMethod<[], Array<room_config>>,
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
  'search_personality' : ActorMethod<[string, Array<number>], Array<string>>,
  'search_user_conversation_history' : ActorMethod<
    [string, string, Array<number>, [] | [number]],
    Array<string>
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
