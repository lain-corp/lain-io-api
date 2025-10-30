use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

// UserProfile matches TypeScript interface
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct UserProfile {
    pub principal: Principal,
    pub display_name: String,
    pub avatar_base64: Option<String>,
    pub bio: Option<String>,
    pub created_at: u64,
}

impl Storable for UserProfile {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

// Friend matches TypeScript interface
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Friend {
    pub principal: Principal,
    pub display_name: String,
    pub avatar_base64: Option<String>,
    pub added_at: u64,
}

impl Storable for Friend {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

// FriendRequest matches TypeScript interface
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct FriendRequest {
    pub id: String,
    pub from_principal: Principal,
    pub from_display_name: String,
    pub from_avatar_base64: Option<String>,
    pub to_principal: Principal,
    pub to_display_name: String,
    pub status: FriendRequestStatus,
    pub created_at: u64,
}

impl Storable for FriendRequest {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

// FriendRequestStatus enum
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FriendRequestStatus {
    Pending,
    Accepted,
    Rejected,
}

// BlockedUser matches TypeScript interface
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct BlockedUser {
    pub principal: Principal,
    pub display_name: String,
    pub blocked_at: u64,
}

impl Storable for BlockedUser {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

// Response types for API
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(msg: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg),
        }
    }
}
