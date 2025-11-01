mod storage;
mod types;

use candid::Principal;
use ic_cdk::{caller, query, update};
use types::{ApiResponse, Friend, FriendRequest, FriendRequestStatus, UserProfile, UserSearchResult, BlockedUser, ChatMessage, UserDataSync, SyncResponse};

// ============ USER REGISTRY METHODS ============

#[update]
fn register_user(display_name: String, avatar_base64: Option<String>, bio: Option<String>) -> ApiResponse<UserProfile> {
    let principal = caller();
    
    // Check if user already registered
    let existing = storage::USER_PROFILES.with(|profiles| {
        profiles.borrow().get(&principal)
    });
    
    if existing.is_some() {
        return ApiResponse::error("User already registered".to_string());
    }
    
    let profile = UserProfile {
        principal,
        display_name,
        avatar_base64,
        bio,
        created_at: ic_cdk::api::time(),
    };
    
    storage::USER_PROFILES.with(|profiles| {
        profiles.borrow_mut().insert(principal, profile.clone());
    });
    
    ApiResponse::success(profile)
}

#[query]
fn search_users(query: String) -> ApiResponse<Vec<UserSearchResult>> {
    let query_lower = query.to_lowercase();
    
    let results = storage::USER_PROFILES.with(|profiles| {
        profiles.borrow()
            .iter()
            .filter(|(_, profile)| {
                profile.display_name.to_lowercase().contains(&query_lower)
            })
            .take(50) // Limit to 50 results to avoid exceeding ICP's 3.1MB response limit
            .map(|(_, profile)| UserSearchResult {
                principal: profile.principal,
                display_name: profile.display_name.clone(),
                created_at: profile.created_at,
            })
            .collect::<Vec<_>>()
    });
    
    ApiResponse::success(results)
}

#[query]
fn get_user_by_principal(principal: Principal) -> ApiResponse<UserProfile> {
    match storage::USER_PROFILES.with(|profiles| profiles.borrow().get(&principal)) {
        Some(profile) => ApiResponse::success(profile),
        None => ApiResponse::error("User not found".to_string()),
    }
}

#[query]
fn get_all_users() -> ApiResponse<Vec<UserProfile>> {
    let users = storage::USER_PROFILES.with(|profiles| {
        profiles.borrow().iter().map(|(_, profile)| profile).collect()
    });
    
    ApiResponse::success(users)
}

#[update]
fn update_profile(
    display_name: Option<String>,
    avatar_base64: Option<String>,
    bio: Option<String>,
) -> ApiResponse<()> {
    let caller_principal = caller();
    
    // Load existing user profile
    let mut user = match storage::USER_PROFILES.with(|profiles| profiles.borrow().get(&caller_principal)) {
        Some(user) => user,
        None => return ApiResponse::error("User not registered".to_string()),
    };
    
    // Update fields if provided
    if let Some(name) = display_name {
        user.display_name = name;
    }
    if let Some(avatar) = avatar_base64 {
        user.avatar_base64 = Some(avatar);
    }
    if let Some(bio_text) = bio {
        user.bio = Some(bio_text);
    }
    
    // Save updated profile
    storage::USER_PROFILES.with(|profiles| {
        profiles.borrow_mut().insert(caller_principal, user);
    });
    
    ApiResponse::success(())
}

#[query]
fn is_display_name_taken(display_name: String) -> ApiResponse<bool> {
    let display_name_lower = display_name.to_lowercase();
    let caller_principal = caller();
    
    let is_taken = storage::USER_PROFILES.with(|profiles| {
        profiles.borrow()
            .iter()
            .any(|(principal, profile)| {
                // Allow the current user to keep their own display name
                if principal == caller_principal {
                    return false;
                }
                // Check if any other user has this display name (case-insensitive)
                profile.display_name.to_lowercase() == display_name_lower
            })
    });
    
    ApiResponse::success(is_taken)
}

// ============ FRIENDS MANAGEMENT METHODS ============

#[update]
fn add_friend(friend_principal: Principal) -> ApiResponse<()> {
    let caller_principal = caller();
    
    // Validate friend exists
    let friend_profile = storage::USER_PROFILES.with(|profiles| {
        profiles.borrow().get(&friend_principal)
    });
    
    let friend_profile = match friend_profile {
        Some(p) => p,
        None => return ApiResponse::error("Friend user not found".to_string()),
    };
    
    // Check if blocked
    let is_blocked = storage::BLOCKED_USERS.with(|blocked| {
        blocked.borrow().contains_key(&(caller_principal, friend_principal)) ||
        blocked.borrow().contains_key(&(friend_principal, caller_principal))
    });
    
    if is_blocked {
        return ApiResponse::error("Cannot add friend: user is blocked".to_string());
    }
    
    // Create Friend entry
    let friend = Friend {
        principal: friend_profile.principal,
        display_name: friend_profile.display_name.clone(),
        avatar_base64: friend_profile.avatar_base64.clone(),
        added_at: ic_cdk::api::time(),
    };
    
    // Add bidirectional friendship
    storage::FRIENDS.with(|friends| {
        let mut friends = friends.borrow_mut();
        friends.insert((caller_principal, friend_principal), friend.clone());
        
        // Reverse direction
        let caller_profile = storage::USER_PROFILES.with(|profiles| {
            profiles.borrow().get(&caller_principal).unwrap()
        });
        
        let reverse_friend = Friend {
            principal: caller_profile.principal,
            display_name: caller_profile.display_name,
            avatar_base64: caller_profile.avatar_base64,
            added_at: ic_cdk::api::time(),
        };
        
        friends.insert((friend_principal, caller_principal), reverse_friend);
    });
    
    ApiResponse::success(())
}

#[update]
fn remove_friend(friend_principal: Principal) -> ApiResponse<()> {
    let caller_principal = caller();
    
    storage::FRIENDS.with(|friends| {
        let mut friends = friends.borrow_mut();
        friends.remove(&(caller_principal, friend_principal));
        friends.remove(&(friend_principal, caller_principal));
    });
    
    ApiResponse::success(())
}

#[query]
fn get_friends() -> ApiResponse<Vec<Friend>> {
    let caller_principal = caller();
    
    let friends = storage::FRIENDS.with(|friends| {
        friends.borrow()
            .iter()
            .filter(|((user_principal, _), _)| *user_principal == caller_principal)
            .map(|(_, friend)| friend)
            .collect()
    });
    
    ApiResponse::success(friends)
}

#[query]
fn is_friend(principal: Principal) -> ApiResponse<bool> {
    let caller_principal = caller();
    
    let is_friend = storage::FRIENDS.with(|friends| {
        friends.borrow().contains_key(&(caller_principal, principal))
    });
    
    ApiResponse::success(is_friend)
}

// ============ FRIEND REQUESTS METHODS ============

#[update]
fn send_friend_request(to_principal: Principal) -> ApiResponse<FriendRequest> {
    let from_principal = caller();
    
    // Validate users exist
    let from_profile = storage::USER_PROFILES.with(|profiles| {
        profiles.borrow().get(&from_principal)
    });
    let to_profile = storage::USER_PROFILES.with(|profiles| {
        profiles.borrow().get(&to_principal)
    });
    
    let from_profile = match from_profile {
        Some(p) => p,
        None => return ApiResponse::error("Sender not registered".to_string()),
    };
    let to_profile = match to_profile {
        Some(p) => p,
        None => return ApiResponse::error("Recipient not found".to_string()),
    };
    
    // Check if already friends
    let already_friends = storage::FRIENDS.with(|friends| {
        friends.borrow().contains_key(&(from_principal, to_principal))
    });
    
    if already_friends {
        return ApiResponse::error("Already friends".to_string());
    }
    
    // Check if blocked
    let is_blocked = storage::BLOCKED_USERS.with(|blocked| {
        blocked.borrow().contains_key(&(to_principal, from_principal))
    });
    
    if is_blocked {
        return ApiResponse::error("Cannot send friend request: you are blocked".to_string());
    }
    
    // Check for existing pending request in both directions
    let (existing_request, reverse_request) = storage::FRIEND_REQUESTS.with(|requests| {
        let borrowed = requests.borrow();
        let existing = borrowed.iter().find(|(_, req)| {
            req.from_principal == from_principal && 
            req.to_principal == to_principal && 
            req.status == FriendRequestStatus::Pending
        }).map(|(_, req)| req);
        
        let reverse = borrowed.iter().find(|(_, req)| {
            req.from_principal == to_principal && 
            req.to_principal == from_principal && 
            req.status == FriendRequestStatus::Pending
        }).map(|(_, req)| req);
        
        (existing, reverse)
    });
    
    if existing_request.is_some() {
        return ApiResponse::error("Friend request already sent".to_string());
    }
    
    if reverse_request.is_some() {
        return ApiResponse::error("This user has already sent you a friend request. Check your pending requests.".to_string());
    }
    
    // Create request
    let request_id = format!("{}_{}", from_principal.to_text(), ic_cdk::api::time());
    let request = FriendRequest {
        id: request_id.clone(),
        from_principal,
        from_display_name: from_profile.display_name,
        to_principal,
        to_display_name: to_profile.display_name,
        status: FriendRequestStatus::Pending,
        created_at: ic_cdk::api::time(),
    };
    
    storage::FRIEND_REQUESTS.with(|requests| {
        requests.borrow_mut().insert(request_id, request.clone());
    });
    
    ApiResponse::success(request)
}

#[update]
fn accept_friend_request(request_id: String) -> ApiResponse<()> {
    let caller_principal = caller();
    
    // Get and validate request
    let request = storage::FRIEND_REQUESTS.with(|requests| {
        requests.borrow().get(&request_id)
    });
    
    let mut request = match request {
        Some(r) => r,
        None => return ApiResponse::error("Friend request not found".to_string()),
    };
    
    if request.to_principal != caller_principal {
        return ApiResponse::error("Not authorized to accept this request".to_string());
    }
    
    if request.status != FriendRequestStatus::Pending {
        return ApiResponse::error("Request is not pending".to_string());
    }
    
    // Create bidirectional friendship
    let result = add_friend(request.from_principal);
    if !result.success {
        return result;
    }
    
    // Update request status
    request.status = FriendRequestStatus::Accepted;
    storage::FRIEND_REQUESTS.with(|requests| {
        requests.borrow_mut().insert(request_id, request);
    });
    
    ApiResponse::success(())
}

#[update]
fn reject_friend_request(request_id: String) -> ApiResponse<()> {
    let caller_principal = caller();
    
    let request = storage::FRIEND_REQUESTS.with(|requests| {
        requests.borrow().get(&request_id)
    });
    
    let mut request = match request {
        Some(r) => r,
        None => return ApiResponse::error("Friend request not found".to_string()),
    };
    
    if request.to_principal != caller_principal {
        return ApiResponse::error("Not authorized to reject this request".to_string());
    }
    
    if request.status != FriendRequestStatus::Pending {
        return ApiResponse::error("Request is not pending".to_string());
    }
    
    request.status = FriendRequestStatus::Rejected;
    storage::FRIEND_REQUESTS.with(|requests| {
        requests.borrow_mut().insert(request_id, request);
    });
    
    ApiResponse::success(())
}

#[query]
fn get_friend_requests() -> ApiResponse<Vec<FriendRequest>> {
    let caller_principal = caller();
    
    let requests = storage::FRIEND_REQUESTS.with(|requests| {
        requests.borrow()
            .iter()
            .filter(|(_, req)| {
                req.to_principal == caller_principal && 
                req.status == FriendRequestStatus::Pending
            })
            .map(|(_, req)| req)
            .collect()
    });
    
    ApiResponse::success(requests)
}

#[query]
fn get_sent_requests() -> ApiResponse<Vec<FriendRequest>> {
    let caller_principal = caller();
    
    let requests = storage::FRIEND_REQUESTS.with(|requests| {
        requests.borrow()
            .iter()
            .filter(|(_, req)| {
                req.from_principal == caller_principal && 
                req.status == FriendRequestStatus::Pending
            })
            .map(|(_, req)| req)
            .collect()
    });
    
    ApiResponse::success(requests)
}

// ============ BLOCKING METHODS ============

#[update]
fn block_user(blocked_principal: Principal) -> ApiResponse<()> {
    let blocker_principal = caller();
    
    // Validate blocked user exists
    let blocked_profile = storage::USER_PROFILES.with(|profiles| {
        profiles.borrow().get(&blocked_principal)
    });
    
    let blocked_profile = match blocked_profile {
        Some(p) => p,
        None => return ApiResponse::error("User not found".to_string()),
    };
    
    // Remove from friends if exists
    storage::FRIENDS.with(|friends| {
        let mut friends = friends.borrow_mut();
        friends.remove(&(blocker_principal, blocked_principal));
        friends.remove(&(blocked_principal, blocker_principal));
    });
    
    // Add to blocked
    let blocked_user = BlockedUser {
        principal: blocked_principal,
        display_name: blocked_profile.display_name,
        blocked_at: ic_cdk::api::time(),
    };
    
    storage::BLOCKED_USERS.with(|blocked| {
        blocked.borrow_mut().insert((blocker_principal, blocked_principal), blocked_user);
    });
    
    ApiResponse::success(())
}

#[update]
fn unblock_user(blocked_principal: Principal) -> ApiResponse<()> {
    let blocker_principal = caller();
    
    storage::BLOCKED_USERS.with(|blocked| {
        blocked.borrow_mut().remove(&(blocker_principal, blocked_principal));
    });
    
    ApiResponse::success(())
}

#[query]
fn get_blocked_users() -> ApiResponse<Vec<BlockedUser>> {
    let caller_principal = caller();
    
    let blocked = storage::BLOCKED_USERS.with(|blocked| {
        blocked.borrow()
            .iter()
            .filter(|((blocker, _), _)| *blocker == caller_principal)
            .map(|(_, user)| user)
            .collect()
    });
    
    ApiResponse::success(blocked)
}

#[query]
fn is_blocked(principal: Principal) -> ApiResponse<bool> {
    let caller_principal = caller();
    
    let is_blocked = storage::BLOCKED_USERS.with(|blocked| {
        blocked.borrow().contains_key(&(caller_principal, principal)) ||
        blocked.borrow().contains_key(&(principal, caller_principal))
    });
    
    ApiResponse::success(is_blocked)
}

// ============ DATA SYNC METHODS ============

#[update]
fn sync_user_data(chat_messages: Vec<ChatMessage>) -> ApiResponse<SyncResponse> {
    let caller_principal = caller();
    let now = ic_cdk::api::time();
    
    // Debug: Log incoming messages
    ic_cdk::println!("=== SYNC DEBUG START ===");
    ic_cdk::println!("Received {} messages for principal: {}", chat_messages.len(), caller_principal);
    for (i, msg) in chat_messages.iter().enumerate() {
        ic_cdk::println!("Message {}: id={}, text={}, sender={}, timestamp={}, channel={:?}", 
            i, msg.id, msg.text, msg.sender, msg.timestamp, msg.channel);
    }
    
    // Create or update user data sync
    let user_data = UserDataSync {
        chat_messages: chat_messages.clone(),
        profile: storage::USER_PROFILES.with(|profiles| {
            profiles.borrow().get(&caller_principal)
        }),
        last_sync: now,
    };
    
    let messages_count = user_data.chat_messages.len() as u32;
    ic_cdk::println!("Created UserDataSync with {} messages", messages_count);
    
    // Store the sync data
    storage::USER_DATA_SYNC.with(|sync_data| {
        sync_data.borrow_mut().insert(caller_principal, user_data);
    });
    
    // Debug: Verify storage
    let stored_data = storage::USER_DATA_SYNC.with(|sync_data| {
        sync_data.borrow().get(&caller_principal)
    });
    
    match stored_data {
        Some(data) => {
            ic_cdk::println!("Stored data verification: {} messages", data.chat_messages.len());
            for (i, msg) in data.chat_messages.iter().enumerate() {
                ic_cdk::println!("Stored message {}: id={}, text={}, sender={}", 
                    i, msg.id, msg.text, msg.sender);
            }
        },
        None => ic_cdk::println!("ERROR: No data found after storage!"),
    }
    ic_cdk::println!("=== SYNC DEBUG END ===");
    
    let response = SyncResponse {
        success: true,
        messages_synced: messages_count,
        last_sync: now,
    };
    
    ApiResponse::success(response)
}

#[query]
fn get_user_data_sync() -> ApiResponse<UserDataSync> {
    let caller_principal = caller();
    
    match storage::USER_DATA_SYNC.with(|sync_data| {
        sync_data.borrow().get(&caller_principal)
    }) {
        Some(data) => ApiResponse::success(data),
        None => ApiResponse::error("No sync data found for user".to_string()),
    }
}

#[query]
fn get_user_chat_messages(channel: Option<String>) -> ApiResponse<Vec<ChatMessage>> {
    let caller_principal = caller();
    
    match storage::USER_DATA_SYNC.with(|sync_data| {
        sync_data.borrow().get(&caller_principal)
    }) {
        Some(data) => {
            let filtered_messages: Vec<ChatMessage> = if let Some(channel_filter) = channel {
                data.chat_messages.into_iter()
                    .filter(|msg| msg.channel.as_ref() == Some(&channel_filter))
                    .collect()
            } else {
                data.chat_messages
            };
            ApiResponse::success(filtered_messages)
        },
        None => ApiResponse::success(vec![]),
    }
}

#[query]
fn debug_get_user_chat_messages(user_principal: Principal, channel: Option<String>) -> ApiResponse<Vec<ChatMessage>> {
    ic_cdk::println!("Debug: Getting messages for principal: {} with channel: {:?}", user_principal.to_text(), channel);
    
    match storage::USER_DATA_SYNC.with(|sync_data| {
        sync_data.borrow().get(&user_principal)
    }) {
        Some(data) => {
            ic_cdk::println!("Debug: Found sync data with {} messages", data.chat_messages.len());
            
            let filtered_messages: Vec<ChatMessage> = if let Some(channel_filter) = channel {
                ic_cdk::println!("Debug: Filtering by channel: {}", channel_filter);
                let filtered: Vec<ChatMessage> = data.chat_messages.into_iter()
                    .filter(|msg| {
                        let matches = msg.channel.as_ref() == Some(&channel_filter);
                        ic_cdk::println!("Debug: Message {} channel {:?} matches {}: {}", msg.id, msg.channel, channel_filter, matches);
                        matches
                    })
                    .collect();
                ic_cdk::println!("Debug: After filtering: {} messages", filtered.len());
                filtered
            } else {
                ic_cdk::println!("Debug: No channel filter, returning all {} messages", data.chat_messages.len());
                data.chat_messages
            };
            
            // Log first few messages for debugging
            for (i, msg) in filtered_messages.iter().take(3).enumerate() {
                ic_cdk::println!("Debug: Message {}: id={}, text={}, sender={}, channel={:?}", 
                    i, msg.id, msg.text.chars().take(50).collect::<String>(), msg.sender, msg.channel);
            }
            
            ApiResponse::success(filtered_messages)
        },
        None => {
            ic_cdk::println!("Debug: No sync data found for principal: {}", user_principal.to_text());
            ApiResponse::success(vec![])
        }
    }
}

// ============ ADMIN METHODS ============

#[query]
fn debug_get_all_friend_requests() -> ApiResponse<Vec<FriendRequest>> {
    // Get ALL friend requests regardless of status or user (for debugging)
    let all_requests = storage::FRIEND_REQUESTS.with(|requests| {
        requests.borrow()
            .iter()
            .map(|(_, req)| req)
            .collect()
    });
    
    ApiResponse::success(all_requests)
}

#[update]
fn clear_all_friend_requests() -> ApiResponse<()> {
    storage::FRIEND_REQUESTS.with(|requests| {
        requests.borrow_mut().clear_new();
    });
    
    ApiResponse::success(())
}

#[update]
fn clear_all_data() -> ApiResponse<()> {
    // Clear all user profiles
    storage::USER_PROFILES.with(|profiles| {
        profiles.borrow_mut().clear_new();
    });
    
    // Clear all friends
    storage::FRIENDS.with(|friends| {
        friends.borrow_mut().clear_new();
    });
    
    // Clear all friend requests
    storage::FRIEND_REQUESTS.with(|requests| {
        requests.borrow_mut().clear_new();
    });
    
    // Clear all blocked users
    storage::BLOCKED_USERS.with(|blocked| {
        blocked.borrow_mut().clear_new();
    });
    
    // Clear all user data sync
    storage::USER_DATA_SYNC.with(|sync_data| {
        sync_data.borrow_mut().clear_new();
    });
    
    ApiResponse::success(())
}

#[query]
fn debug_get_all_sync_data() -> ApiResponse<Vec<(String, UserDataSync)>> {
    let all_sync_data = storage::USER_DATA_SYNC.with(|sync_data| {
        sync_data.borrow()
            .iter()
            .map(|(principal, data)| (principal.to_text(), data))
            .collect()
    });
    
    ApiResponse::success(all_sync_data)
}
