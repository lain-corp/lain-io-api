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
  return IDL.Service({
    'chat' : IDL.Func([IDL.Vec(chat_message)], [IDL.Text], []),
  });
};
export const init = ({ IDL }) => { return []; };
