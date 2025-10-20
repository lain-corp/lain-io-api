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
  'get_personality_embeddings' : ActorMethod<
    [],
    Array<
      {
        'content' : string,
        'trait_type' : string,
        'importance' : number,
        'embedding' : Array<number>,
        'channel' : string,
      }
    >
  >,
  'store_personality' : ActorMethod<
    [string, string, string, number, Array<number>],
    undefined
  >,
  'store_personality_batch' : ActorMethod<
    [
      Array<
        {
          'content' : string,
          'trait_type' : string,
          'importance' : number,
          'embedding' : Array<number>,
          'channel' : string,
        }
      >,
    ],
    undefined
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
