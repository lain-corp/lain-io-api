export const idlFactory = ({ IDL }) => {
  const chat_message = IDL.Variant({
    'tool' : IDL.Record({ 'content' : IDL.Text, 'tool_call_id' : IDL.Text }),
    'user' : IDL.Record({ 'content' : IDL.Text }),
    'assistant' : IDL.Record({
      'content' : IDL.Opt(IDL.Text),
      'tool_calls' : IDL.Vec(
        IDL.Record({
          'id' : IDL.Text,
          'function' : IDL.Record({
            'name' : IDL.Text,
            'arguments' : IDL.Vec(
              IDL.Record({ 'value' : IDL.Text, 'name' : IDL.Text })
            ),
          }),
        })
      ),
    }),
    'system' : IDL.Record({ 'content' : IDL.Text }),
  });
  const room_config = IDL.Record({
    'id' : IDL.Text,
    'name' : IDL.Text,
    'description' : IDL.Text,
  });
  const personality_embedding = IDL.Record({
    'channel_id' : IDL.Text,
    'text' : IDL.Text,
    'importance' : IDL.Float32,
    'created_at' : IDL.Nat64,
    'category' : IDL.Text,
    'embedding' : IDL.Vec(IDL.Float32),
  });
  return IDL.Service({
    'chat' : IDL.Func(
        [IDL.Vec(chat_message), IDL.Opt(IDL.Text)],
        [IDL.Text],
        [],
      ),
    'chat_default' : IDL.Func([IDL.Vec(chat_message)], [IDL.Text], []),
    'chat_with_rag' : IDL.Func(
        [IDL.Vec(chat_message), IDL.Opt(IDL.Text), IDL.Vec(IDL.Float32)],
        [IDL.Text],
        [],
      ),
    'get_available_rooms' : IDL.Func([], [IDL.Vec(room_config)], ['query']),
    'get_personality_embeddings' : IDL.Func(
        [],
        [IDL.Vec(personality_embedding)],
        ['query'],
      ),
    'search_personality' : IDL.Func(
        [IDL.Text, IDL.Vec(IDL.Float32)],
        [IDL.Vec(IDL.Text)],
        ['query'],
      ),
    'store_personality' : IDL.Func([personality_embedding], [IDL.Text], []),
    'store_personality_batch' : IDL.Func(
        [IDL.Vec(personality_embedding)],
        [IDL.Text],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
