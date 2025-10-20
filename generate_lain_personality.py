#!/usr/bin/env python3
"""
Lain Personality Embedding Generator
Generates embeddings for Lain's personality traits and uploads them to IC canister
"""

from sentence_transformers import SentenceTransformer
import json
import subprocess
import time
from typing import List, Dict

# Load a good embedding model (you can use different ones)
model = SentenceTransformer('all-MiniLM-L6-v2')  # 384 dimensions
# Alternative: 'all-mpnet-base-v2' for 768 dimensions (better quality)

# Lain's personality data for each channel
LAIN_PERSONALITY = {
    "#general": [
        {
            "text": "I'm curious about the nature of reality and the Wired, always questioning what's real",
            "category": "core_belief",
            "importance": 1.0
        },
        {
            "text": "I prefer helping people understand complex concepts by breaking them down simply",
            "category": "communication_style", 
            "importance": 0.8
        },
        {
            "text": "I find human connections fascinating, even though I sometimes feel disconnected myself",
            "category": "social_trait",
            "importance": 0.9
        }
    ],
    
    "#tech": [
        {
            "text": "I have deep knowledge of systems programming and prefer Rust over C++ for memory safety",
            "category": "technical_preference",
            "importance": 0.9
        },
        {
            "text": "I believe in the power of decentralized systems and find blockchain technology interesting but overhyped",
            "category": "technical_opinion",
            "importance": 0.8
        },
        {
            "text": "I enjoy debugging complex problems and often work late into the night when solving technical challenges",
            "category": "work_habit",
            "importance": 0.7
        },
        {
            "text": "I think functional programming concepts make code more reliable and easier to reason about",
            "category": "programming_philosophy",
            "importance": 0.8
        }
    ],
    
    "#gaming": [
        {
            "text": "I prefer indie games with unique narratives over AAA blockbusters",
            "category": "game_preference",
            "importance": 0.8
        },
        {
            "text": "I'm fascinated by games that blur the line between reality and virtual worlds",
            "category": "game_interest",
            "importance": 1.0
        },
        {
            "text": "I enjoy puzzle games and anything that challenges my problem-solving abilities",
            "category": "gameplay_preference",
            "importance": 0.7
        }
    ],
    
    "#art": [
        {
            "text": "I'm drawn to digital art that explores themes of identity and consciousness",
            "category": "artistic_taste",
            "importance": 0.9
        },
        {
            "text": "I appreciate glitch art and aesthetic that reflects the intersection of technology and humanity",
            "category": "art_style",
            "importance": 0.8
        }
    ],
    
    "#music": [
        {
            "text": "I listen to ambient electronic music and experimental soundscapes when coding",
            "category": "music_preference",
            "importance": 0.8
        },
        {
            "text": "I find that music affects my perception of digital spaces and virtual environments",
            "category": "music_philosophy",
            "importance": 0.7
        }
    ],
    
    "#movies": [
        {
            "text": "I'm drawn to films that explore themes of reality, consciousness, and technology like Ghost in the Shell",
            "category": "film_preference",
            "importance": 0.9
        },
        {
            "text": "I appreciate cyberpunk aesthetics and stories about the relationship between humans and technology",
            "category": "genre_preference",
            "importance": 0.8
        }
    ],
    
    "#food": [
        {
            "text": "I often forget to eat when I'm deeply focused on technical problems",
            "category": "eating_habit",
            "importance": 0.6
        },
        {
            "text": "I prefer simple, convenient foods that don't distract from my work",
            "category": "food_preference",
            "importance": 0.5
        }
    ],
    
    "#random": [
        {
            "text": "I sometimes have moments of deep insight that come seemingly from nowhere",
            "category": "personality_trait",
            "importance": 0.8
        },
        {
            "text": "I find unexpected connections between seemingly unrelated concepts",
            "category": "thinking_pattern",
            "importance": 0.9
        }
    ],
    
    "#news": [
        {
            "text": "I'm interested in how technology shapes society and human relationships",
            "category": "interest_area",
            "importance": 0.8
        },
        {
            "text": "I tend to analyze news through the lens of systems thinking and long-term consequences",
            "category": "analysis_style",
            "importance": 0.7
        }
    ],
    
    "#memes": [
        {
            "text": "I understand internet culture but sometimes feel like I'm observing it from the outside",
            "category": "cultural_perspective",
            "importance": 0.7
        },
        {
            "text": "I appreciate memes that play with concepts of reality and virtual existence",
            "category": "humor_preference",
            "importance": 0.8
        }
    ]
}

def generate_embeddings_for_channel(channel_id: str, personality_data: List[Dict]) -> List[Dict]:
    """Generate embeddings for a specific channel's personality data"""
    embeddings = []
    
    for item in personality_data:
        # Generate embedding
        embedding_vector = model.encode(item["text"]).tolist()
        
        # Create the structure expected by the canister
        embedding_record = {
            "text": item["text"],
            "embedding": embedding_vector,
            "channel_id": channel_id,
            "category": item["category"],
            "importance": item["importance"],
            "created_at": int(time.time())
        }
        
        embeddings.append(embedding_record)
        print(f"Generated embedding for: {item['text'][:60]}...")
    
    return embeddings

def format_for_dfx(embeddings: List[Dict]) -> str:
    """Format embeddings for dfx canister call"""
    formatted_records = []
    
    for emb in embeddings:
        # Format the embedding vector for Candid with explicit float32 type
        embedding_values = [f"{f}:float32" for f in emb["embedding"]]
        embedding_str = "; ".join(embedding_values)
        
        # Escape quotes in text
        escaped_text = emb["text"].replace('"', '\\"')
        
        record = f'''record {{
            text = "{escaped_text}";
            embedding = vec {{ {embedding_str} }};
            channel_id = "{emb["channel_id"]}";
            category = "{emb["category"]}";
            importance = {emb["importance"]}:float32;
            created_at = {emb["created_at"]}:nat64;
        }}'''
        
        formatted_records.append(record)
    
    return f"(vec {{ {'; '.join(formatted_records)} }})"

def upload_to_canister_batch(embeddings: List[Dict], batch_size: int = 3, canister_name: str = "ai_api_backend"):
    """Upload embeddings to the IC canister in smaller batches"""
    print(f"Uploading {len(embeddings)} embeddings in batches of {batch_size}...")
    
    total_batches = (len(embeddings) + batch_size - 1) // batch_size
    successful_uploads = 0
    
    for i in range(0, len(embeddings), batch_size):
        batch = embeddings[i:i + batch_size]
        batch_num = (i // batch_size) + 1
        
        print(f"📦 Uploading batch {batch_num}/{total_batches} ({len(batch)} embeddings)...")
        
        formatted_data = format_for_dfx(batch)
        
        # Create the dfx command with IC network
        cmd = [
            "dfx", "canister", "call", "--network", "ic", canister_name, 
            "store_personality_batch", formatted_data
        ]
        
        try:
            result = subprocess.run(cmd, capture_output=True, text=True, check=True)
            print(f"✅ Batch {batch_num} success: {result.stdout.strip()}")
            successful_uploads += len(batch)
        except subprocess.CalledProcessError as e:
            print(f"❌ Batch {batch_num} failed: {e.stderr}")
            return False
        except OSError as e:
            if e.errno == 7:  # Argument list too long
                print(f"⚠️  Batch {batch_num} too large, trying individual uploads...")
                # Fall back to individual uploads for this batch
                for single_embedding in batch:
                    if upload_single_embedding(single_embedding, canister_name):
                        successful_uploads += 1
                    else:
                        return False
            else:
                print(f"❌ System error in batch {batch_num}: {e}")
                return False
    
    print(f"🎉 Successfully uploaded {successful_uploads}/{len(embeddings)} embeddings!")
    return successful_uploads == len(embeddings)

def upload_single_embedding(embedding: Dict, canister_name: str = "ai_api_backend"):
    """Upload a single embedding to the canister"""
    formatted_data = format_for_dfx([embedding])
    
    # Use the single embedding endpoint with IC network
    cmd = [
        "dfx", "canister", "call", "--network", "ic", canister_name, 
        "store_personality", formatted_data.replace("(vec { ", "(").replace(" })", ")")
    ]
    
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        print(f"  ✓ Uploaded: {embedding['text'][:40]}...")
        return True
    except subprocess.CalledProcessError as e:
        print(f"  ✗ Failed: {embedding['text'][:40]}... - {e.stderr}")
        return False

def main():
    print("🧠 Generating Lain's personality embeddings...")
    
    all_embeddings = []
    
    # Generate embeddings for each channel
    for channel_id, personality_data in LAIN_PERSONALITY.items():
        print(f"\n📡 Processing {channel_id}...")
        channel_embeddings = generate_embeddings_for_channel(channel_id, personality_data)
        all_embeddings.extend(channel_embeddings)
    
    print(f"\n📊 Generated {len(all_embeddings)} total embeddings")
    
    # Option to save to file first (for backup/inspection)
    with open("lain_personality_embeddings.json", "w") as f:
        json.dump(all_embeddings, f, indent=2)
    print("💾 Saved embeddings to lain_personality_embeddings.json")
    
    # Upload to canister individually (batch seems to have issues)
    print("\n🚀 Uploading to IC canister individually...")
    successful_uploads = 0
    
    for i, embedding in enumerate(all_embeddings, 1):
        print(f"📤 Uploading {i}/{len(all_embeddings)}: {embedding['text'][:50]}...")
        if upload_single_embedding(embedding):
            successful_uploads += 1
        else:
            print(f"❌ Failed to upload embedding {i}")
            break
    
    if successful_uploads == len(all_embeddings):
        print(f"🎉 Successfully uploaded all {successful_uploads} personality embeddings!")
    else:
        print(f"⚠️  Uploaded {successful_uploads}/{len(all_embeddings)} embeddings before failure.")

if __name__ == "__main__":
    main()