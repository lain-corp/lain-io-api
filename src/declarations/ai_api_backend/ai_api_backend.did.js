export const idlFactory = ({ IDL }) => {
  const topic_interest = IDL.Record({
    'expertise_level' : IDL.Float32,
    'topic' : IDL.Text,
    'last_mentioned' : IDL.Nat64,
    'message_count' : IDL.Nat32,
    'engagement_score' : IDL.Float32,
    'first_mentioned' : IDL.Nat64,
  });
  const big_five_traits = IDL.Record({
    'conscientiousness' : IDL.Float32,
    'neuroticism' : IDL.Float32,
    'agreeableness' : IDL.Float32,
    'extraversion' : IDL.Float32,
    'openness' : IDL.Float32,
  });
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
  const user_profile = IDL.Record({
    'updated_at' : IDL.Nat64,
    'personality_traits' : big_five_traits,
    'interests' : IDL.Vec(topic_interest),
    'created_at' : IDL.Nat64,
    'user_id' : IDL.Text,
    'total_messages' : IDL.Nat32,
    'aggregated_embedding' : IDL.Vec(IDL.Float32),
    'conversation_count' : IDL.Nat32,
  });
  const room_config = IDL.Record({
    'id' : IDL.Text,
    'name' : IDL.Text,
    'description' : IDL.Text,
  });
  const category_info = IDL.Record({
    'count' : IDL.Nat32,
    'description' : IDL.Text,
    'category' : IDL.Text,
  });
  const knowledge_stats = IDL.Record({
    'categories' : IDL.Vec(category_info),
    'personality_embeddings' : IDL.Nat32,
    'wiki_embeddings' : IDL.Nat32,
    'total_embeddings' : IDL.Nat32,
  });
  const personality_embedding = IDL.Record({
    'channel_id' : IDL.Text,
    'text' : IDL.Text,
    'importance' : IDL.Float32,
    'created_at' : IDL.Nat64,
    'category' : IDL.Text,
    'embedding' : IDL.Vec(IDL.Float32),
  });
  const conversation_embedding = IDL.Record({
    'channel_id' : IDL.Text,
    'chunk_index' : IDL.Nat32,
    'conversation_text' : IDL.Text,
    'created_at' : IDL.Nat64,
    'user_id' : IDL.Text,
    'summary' : IDL.Text,
    'message_count' : IDL.Nat32,
    'embedding' : IDL.Vec(IDL.Float32),
  });
  const search_result = IDL.Record({
    'text' : IDL.Text,
    'content_type' : IDL.Text,
    'importance' : IDL.Float32,
    'source_info' : IDL.Text,
    'similarity' : IDL.Float32,
    'category' : IDL.Text,
  });
  return IDL.Service({
    'analyze_user_interests' : IDL.Func(
        [IDL.Text],
        [IDL.Vec(topic_interest)],
        ['query'],
      ),
    'analyze_user_personality' : IDL.Func(
        [IDL.Text],
        [IDL.Opt(big_five_traits)],
        ['query'],
      ),
    'calculate_user_similarity' : IDL.Func(
        [IDL.Text, IDL.Text],
        [IDL.Opt(IDL.Float32)],
        ['query'],
      ),
    'chat' : IDL.Func(
        [IDL.Vec(chat_message), IDL.Opt(IDL.Text)],
        [IDL.Text],
        [],
      ),
    'chat_default' : IDL.Func([IDL.Vec(chat_message)], [IDL.Text], []),
    'chat_with_knowledge' : IDL.Func(
        [
          IDL.Vec(chat_message),
          IDL.Opt(IDL.Text),
          IDL.Vec(IDL.Float32),
          IDL.Opt(IDL.Vec(IDL.Text)),
        ],
        [IDL.Text],
        [],
      ),
    'chat_with_rag' : IDL.Func(
        [IDL.Vec(chat_message), IDL.Opt(IDL.Text), IDL.Vec(IDL.Float32)],
        [IDL.Text],
        [],
      ),
    'chat_with_user_context' : IDL.Func(
        [
          IDL.Vec(chat_message),
          IDL.Text,
          IDL.Opt(IDL.Text),
          IDL.Vec(IDL.Float32),
        ],
        [IDL.Text],
        [],
      ),
    'create_user_profile' : IDL.Func([IDL.Text], [IDL.Opt(user_profile)], []),
    'get_all_user_profiles' : IDL.Func([], [IDL.Vec(user_profile)], ['query']),
    'get_available_rooms' : IDL.Func([], [IDL.Vec(room_config)], ['query']),
    'get_friendship_recommendations' : IDL.Func(
        [IDL.Text, IDL.Opt(IDL.Nat32)],
        [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Float32))],
        ['query'],
      ),
    'get_knowledge_categories' : IDL.Func(
        [],
        [IDL.Vec(category_info)],
        ['query'],
      ),
    'get_knowledge_stats' : IDL.Func([], [knowledge_stats], ['query']),
    'get_next_conversation_chunk_index' : IDL.Func(
        [IDL.Text, IDL.Text],
        [IDL.Nat32],
        ['query'],
      ),
    'get_personality_embeddings' : IDL.Func(
        [],
        [IDL.Vec(personality_embedding)],
        ['query'],
      ),
    'get_recent_user_conversations' : IDL.Func(
        [IDL.Text, IDL.Text, IDL.Opt(IDL.Nat32)],
        [IDL.Vec(IDL.Text)],
        ['query'],
      ),
    'get_user_conversation_stats' : IDL.Func(
        [IDL.Text, IDL.Text],
        [IDL.Nat32, IDL.Nat32],
        ['query'],
      ),
    'get_user_conversations' : IDL.Func(
        [IDL.Text, IDL.Text],
        [IDL.Vec(conversation_embedding)],
        ['query'],
      ),
    'get_user_profile_by_id' : IDL.Func(
        [IDL.Text],
        [IDL.Opt(user_profile)],
        ['query'],
      ),
    'search_knowledge_by_text' : IDL.Func(
        [IDL.Text, IDL.Opt(IDL.Vec(IDL.Text)), IDL.Opt(IDL.Nat32)],
        [IDL.Vec(search_result)],
        ['query'],
      ),
    'search_personality' : IDL.Func(
        [IDL.Text, IDL.Vec(IDL.Float32)],
        [IDL.Vec(IDL.Text)],
        ['query'],
      ),
    'search_unified_knowledge' : IDL.Func(
        [IDL.Vec(IDL.Float32), IDL.Opt(IDL.Vec(IDL.Text)), IDL.Opt(IDL.Nat32)],
        [IDL.Vec(search_result)],
        ['query'],
      ),
    'search_user_conversation_history' : IDL.Func(
        [IDL.Text, IDL.Text, IDL.Vec(IDL.Float32), IDL.Opt(IDL.Nat32)],
        [IDL.Vec(IDL.Text)],
        ['query'],
      ),
    'search_wiki_content' : IDL.Func(
        [IDL.Vec(IDL.Float32), IDL.Opt(IDL.Text), IDL.Opt(IDL.Nat32)],
        [IDL.Vec(search_result)],
        ['query'],
      ),
    'store_conversation_chunk' : IDL.Func(
        [conversation_embedding],
        [IDL.Text],
        [],
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
