export const idlFactory = ({ IDL }) => {
  const ApiResponse = IDL.Record({
    'data' : IDL.Opt(IDL.Record({})),
    'error' : IDL.Opt(IDL.Text),
    'success' : IDL.Bool,
  });
  const UserProfile = IDL.Record({
    'bio' : IDL.Opt(IDL.Text),
    'principal' : IDL.Principal,
    'created_at' : IDL.Nat64,
    'display_name' : IDL.Text,
    'avatar_base64' : IDL.Opt(IDL.Text),
  });
  const ApiResponseVecUserProfile = IDL.Record({
    'data' : IDL.Opt(IDL.Vec(UserProfile)),
    'error' : IDL.Opt(IDL.Text),
    'success' : IDL.Bool,
  });
  const BlockedUser = IDL.Record({
    'principal' : IDL.Principal,
    'display_name' : IDL.Text,
    'blocked_at' : IDL.Nat64,
  });
  const ApiResponseVecBlockedUser = IDL.Record({
    'data' : IDL.Opt(IDL.Vec(BlockedUser)),
    'error' : IDL.Opt(IDL.Text),
    'success' : IDL.Bool,
  });
  const FriendRequestStatus = IDL.Variant({
    'Rejected' : IDL.Null,
    'Accepted' : IDL.Null,
    'Pending' : IDL.Null,
  });
  const FriendRequest = IDL.Record({
    'id' : IDL.Text,
    'status' : FriendRequestStatus,
    'to_principal' : IDL.Principal,
    'to_display_name' : IDL.Text,
    'created_at' : IDL.Nat64,
    'from_principal' : IDL.Principal,
    'from_display_name' : IDL.Text,
  });
  const ApiResponseVecFriendRequest = IDL.Record({
    'data' : IDL.Opt(IDL.Vec(FriendRequest)),
    'error' : IDL.Opt(IDL.Text),
    'success' : IDL.Bool,
  });
  const Friend = IDL.Record({
    'principal' : IDL.Principal,
    'added_at' : IDL.Nat64,
    'display_name' : IDL.Text,
    'avatar_base64' : IDL.Opt(IDL.Text),
  });
  const ApiResponseVecFriend = IDL.Record({
    'data' : IDL.Opt(IDL.Vec(Friend)),
    'error' : IDL.Opt(IDL.Text),
    'success' : IDL.Bool,
  });
  const ApiResponseUserProfile = IDL.Record({
    'data' : IDL.Opt(UserProfile),
    'error' : IDL.Opt(IDL.Text),
    'success' : IDL.Bool,
  });
  const ApiResponseBool = IDL.Record({
    'data' : IDL.Opt(IDL.Bool),
    'error' : IDL.Opt(IDL.Text),
    'success' : IDL.Bool,
  });
  const ApiResponseFriendRequest = IDL.Record({
    'data' : IDL.Opt(FriendRequest),
    'error' : IDL.Opt(IDL.Text),
    'success' : IDL.Bool,
  });
  return IDL.Service({
    'accept_friend_request' : IDL.Func([IDL.Text], [ApiResponse], []),
    'add_friend' : IDL.Func([IDL.Principal], [ApiResponse], []),
    'block_user' : IDL.Func([IDL.Principal], [ApiResponse], []),
    'clear_all_friend_requests' : IDL.Func([], [ApiResponse], []),
    'get_all_users' : IDL.Func([], [ApiResponseVecUserProfile], ['query']),
    'get_blocked_users' : IDL.Func([], [ApiResponseVecBlockedUser], ['query']),
    'get_friend_requests' : IDL.Func(
        [],
        [ApiResponseVecFriendRequest],
        ['query'],
      ),
    'get_friends' : IDL.Func([], [ApiResponseVecFriend], ['query']),
    'get_sent_requests' : IDL.Func(
        [],
        [ApiResponseVecFriendRequest],
        ['query'],
      ),
    'get_user_by_principal' : IDL.Func(
        [IDL.Principal],
        [ApiResponseUserProfile],
        ['query'],
      ),
    'is_blocked' : IDL.Func([IDL.Principal], [ApiResponseBool], ['query']),
    'is_friend' : IDL.Func([IDL.Principal], [ApiResponseBool], ['query']),
    'register_user' : IDL.Func(
        [IDL.Text, IDL.Opt(IDL.Text), IDL.Opt(IDL.Text)],
        [ApiResponseUserProfile],
        [],
      ),
    'reject_friend_request' : IDL.Func([IDL.Text], [ApiResponse], []),
    'remove_friend' : IDL.Func([IDL.Principal], [ApiResponse], []),
    'search_users' : IDL.Func(
        [IDL.Text],
        [ApiResponseVecUserProfile],
        ['query'],
      ),
    'send_friend_request' : IDL.Func(
        [IDL.Principal],
        [ApiResponseFriendRequest],
        [],
      ),
    'unblock_user' : IDL.Func([IDL.Principal], [ApiResponse], []),
    'update_profile' : IDL.Func(
        [IDL.Opt(IDL.Text), IDL.Opt(IDL.Text), IDL.Opt(IDL.Text)],
        [ApiResponse],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
