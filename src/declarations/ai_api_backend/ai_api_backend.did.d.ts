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
  'get_available_rooms' : ActorMethod<[], Array<room_config>>,
  'get_personality_embeddings' : ActorMethod<[], Array<personality_embedding>>,
  'search_personality' : ActorMethod<[string, Array<number>], Array<string>>,
  'store_personality' : ActorMethod<[personality_embedding], string>,
  'store_personality_batch' : ActorMethod<
    [Array<personality_embedding>],
    string
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
