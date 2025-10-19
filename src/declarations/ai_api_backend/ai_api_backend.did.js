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
  return IDL.Service({
    'chat' : IDL.Func(
        [IDL.Vec(chat_message), IDL.Opt(IDL.Text)],
        [IDL.Text],
        [],
      ),
    'chat_default' : IDL.Func([IDL.Vec(chat_message)], [IDL.Text], []),
    'get_available_rooms' : IDL.Func([], [IDL.Vec(room_config)], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
