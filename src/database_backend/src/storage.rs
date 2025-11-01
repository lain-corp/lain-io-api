use candid::Principal;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;

use crate::types::{BlockedUser, Friend, FriendRequest, UserProfile, UserDataSync};

type Memory = VirtualMemory<DefaultMemoryImpl>;

// Memory IDs for different storage structures
const USER_PROFILES_MEM_ID: MemoryId = MemoryId::new(0);
const FRIENDS_MEM_ID: MemoryId = MemoryId::new(1);
const FRIEND_REQUESTS_MEM_ID: MemoryId = MemoryId::new(2);
const BLOCKED_USERS_MEM_ID: MemoryId = MemoryId::new(3);
const USER_DATA_SYNC_MEM_ID: MemoryId = MemoryId::new(4);

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // User registry: Principal -> UserProfile
    pub static USER_PROFILES: RefCell<StableBTreeMap<Principal, UserProfile, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(USER_PROFILES_MEM_ID)),
        )
    );

    // Friends: (user_principal, friend_principal) -> Friend
    pub static FRIENDS: RefCell<StableBTreeMap<(Principal, Principal), Friend, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(FRIENDS_MEM_ID)),
        )
    );

    // Friend requests: request_id -> FriendRequest
    pub static FRIEND_REQUESTS: RefCell<StableBTreeMap<String, FriendRequest, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(FRIEND_REQUESTS_MEM_ID)),
        )
    );

    // Blocked users: (blocker_principal, blocked_principal) -> BlockedUser
    pub static BLOCKED_USERS: RefCell<StableBTreeMap<(Principal, Principal), BlockedUser, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(BLOCKED_USERS_MEM_ID)),
        )
    );

    // User data sync: Principal -> UserDataSync
    pub static USER_DATA_SYNC: RefCell<StableBTreeMap<Principal, UserDataSync, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(USER_DATA_SYNC_MEM_ID)),
        )
    );
}
