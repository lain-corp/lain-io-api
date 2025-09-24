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
export interface _SERVICE {
  'chat' : ActorMethod<[Array<chat_message>], string>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
