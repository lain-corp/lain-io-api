import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface ApiResponse {
  'data' : [] | [{}],
  'error' : [] | [string],
  'success' : boolean,
}
export interface ApiResponseBool {
  'data' : [] | [boolean],
  'error' : [] | [string],
  'success' : boolean,
}
export interface ApiResponseFriendRequest {
  'data' : [] | [FriendRequest],
  'error' : [] | [string],
  'success' : boolean,
}
export interface ApiResponseUserProfile {
  'data' : [] | [UserProfile],
  'error' : [] | [string],
  'success' : boolean,
}
export interface ApiResponseVecBlockedUser {
  'data' : [] | [Array<BlockedUser>],
  'error' : [] | [string],
  'success' : boolean,
}
export interface ApiResponseVecFriend {
  'data' : [] | [Array<Friend>],
  'error' : [] | [string],
  'success' : boolean,
}
export interface ApiResponseVecFriendRequest {
  'data' : [] | [Array<FriendRequest>],
  'error' : [] | [string],
  'success' : boolean,
}
export interface ApiResponseVecUserProfile {
  'data' : [] | [Array<UserProfile>],
  'error' : [] | [string],
  'success' : boolean,
}
export interface BlockedUser {
  'principal' : Principal,
  'display_name' : string,
  'blocked_at' : bigint,
}
export interface Friend {
  'principal' : Principal,
  'added_at' : bigint,
  'display_name' : string,
  'avatar_base64' : [] | [string],
}
export interface FriendRequest {
  'id' : string,
  'status' : FriendRequestStatus,
  'to_principal' : Principal,
  'to_display_name' : string,
  'created_at' : bigint,
  'from_principal' : Principal,
  'from_display_name' : string,
}
export type FriendRequestStatus = { 'Rejected' : null } |
  { 'Accepted' : null } |
  { 'Pending' : null };
export interface UserProfile {
  'bio' : [] | [string],
  'principal' : Principal,
  'created_at' : bigint,
  'display_name' : string,
  'avatar_base64' : [] | [string],
}
export interface _SERVICE {
  'accept_friend_request' : ActorMethod<[string], ApiResponse>,
  'add_friend' : ActorMethod<[Principal], ApiResponse>,
  'admin_clear_database' : ActorMethod<[], ApiResponse>,
  'block_user' : ActorMethod<[Principal], ApiResponse>,
  'clear_all_friend_requests' : ActorMethod<[], ApiResponse>,
  'debug_get_all_friend_requests' : ActorMethod<
    [],
    ApiResponseVecFriendRequest
  >,
  'get_all_users' : ActorMethod<[], ApiResponseVecUserProfile>,
  'get_blocked_users' : ActorMethod<[], ApiResponseVecBlockedUser>,
  'get_friend_requests' : ActorMethod<[], ApiResponseVecFriendRequest>,
  'get_friends' : ActorMethod<[], ApiResponseVecFriend>,
  'get_sent_requests' : ActorMethod<[], ApiResponseVecFriendRequest>,
  'get_user_by_principal' : ActorMethod<[Principal], ApiResponseUserProfile>,
  'is_blocked' : ActorMethod<[Principal], ApiResponseBool>,
  'is_display_name_taken' : ActorMethod<[string], ApiResponseBool>,
  'is_friend' : ActorMethod<[Principal], ApiResponseBool>,
  'register_user' : ActorMethod<
    [string, [] | [string], [] | [string]],
    ApiResponseUserProfile
  >,
  'reject_friend_request' : ActorMethod<[string], ApiResponse>,
  'remove_friend' : ActorMethod<[Principal], ApiResponse>,
  'search_users' : ActorMethod<[string], ApiResponseVecUserProfile>,
  'send_friend_request' : ActorMethod<[Principal], ApiResponseFriendRequest>,
  'unblock_user' : ActorMethod<[Principal], ApiResponse>,
  'update_profile' : ActorMethod<
    [[] | [string], [] | [string], [] | [string]],
    ApiResponse
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
