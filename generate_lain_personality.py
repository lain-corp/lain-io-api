#!/usr/bin/env python3
"""
Lain Personality Embedding Generator
Generates embeddings for Lain's personality traits and memex-wiki documentation
Uploads embeddings to IC canister for unified knowledge retrieval
"""

from sentence_transformers import SentenceTransformer
import json
import subprocess
import time
import os
import re
from pathlib import Path
from typing import List, Dict, Tuple, Optional

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

# Memex-wiki content categorization patterns
WIKI_CATEGORIZATION = {
    "project-docs": {
        "patterns": [
            r"projects/.*\.md$",
            r"projects/README\.md$"
        ],
        "description": "Documentation for specific LainCorp projects"
    },
    "tech-guides": {
        "patterns": [
            r"icp-overview\.md$",
            r"development\.md$",
            r"architecture\.md$",
            r"deployment\.md$",
            r"best-practices\.md$"
        ],
        "description": "Technical guides and development documentation"
    },
    "meta-docs": {
        "patterns": [
            r"README\.md$",
            r"contributing\.md$",
            r"glossary\.md$",
            r"faq\.md$",
            r"resources\.md$"
        ],
        "description": "Meta documentation about the wiki and contribution guides"
    }
}

def parse_markdown_file(file_path: str) -> Dict[str, str]:
    """Parse markdown file and extract metadata and content sections"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Extract metadata from the beginning of the file
        metadata = {}
        content_sections = {}
        
        # Look for structured metadata (Status, Type, URL, etc.)
        metadata_patterns = {
            'status': r'\*\*Status\*\*:\s*(.+)',
            'type': r'\*\*Type\*\*:\s*(.+)',
            'url': r'\*\*URL\*\*:\s*(.+)',
            'platform': r'\*\*Platform\*\*:\s*(.+)'
        }
        
        for key, pattern in metadata_patterns.items():
            match = re.search(pattern, content)
            if match:
                metadata[key] = match.group(1).strip()
        
        # Extract title (first # heading)
        title_match = re.search(r'^#\s+(.+)$', content, re.MULTILINE)
        if title_match:
            metadata['title'] = title_match.group(1).strip()
        
        # Split content by major sections (## headings)
        sections = re.split(r'\n##\s+(.+)\n', content)
        
        # First section is before any ## heading (overview/intro)
        if sections[0].strip():
            content_sections['overview'] = sections[0].strip()
        
        # Process subsequent sections
        for i in range(1, len(sections), 2):
            if i + 1 < len(sections):
                section_title = sections[i].strip().lower().replace(' ', '_')
                section_content = sections[i + 1].strip()
                content_sections[section_title] = section_content
        
        return {
            'metadata': metadata,
            'sections': content_sections,
            'full_content': content
        }
    
    except Exception as e:
        print(f"Error parsing {file_path}: {e}")
        return {}

def categorize_wiki_content(file_path: str) -> str:
    """Categorize wiki content based on file path patterns"""
    relative_path = os.path.relpath(file_path, start="/Users/laincorp/LainCorp/memex-wiki/docs")
    
    for category, config in WIKI_CATEGORIZATION.items():
        for pattern in config["patterns"]:
            if re.search(pattern, relative_path):
                return category
    
    return "general-docs"

def extract_meaningful_content(content: str, max_chars: int = 1000) -> List[str]:
    """Extract meaningful chunks from content for embedding"""
    # Remove markdown formatting for cleaner embeddings
    cleaned = re.sub(r'```[\s\S]*?```', '', content)  # Remove code blocks
    cleaned = re.sub(r'`[^`]+`', '', cleaned)  # Remove inline code
    cleaned = re.sub(r'\[([^\]]+)\]\([^)]+\)', r'\1', cleaned)  # Convert links to text
    cleaned = re.sub(r'[#*_`]', '', cleaned)  # Remove markdown formatting
    cleaned = re.sub(r'\n+', ' ', cleaned)  # Replace newlines with spaces
    cleaned = ' '.join(cleaned.split())  # Normalize whitespace
    
    # Split into meaningful chunks
    chunks = []
    
    if len(cleaned) <= max_chars:
        chunks.append(cleaned)
    else:
        # Split by sentences, trying to keep chunks under max_chars
        sentences = re.split(r'[.!?]+', cleaned)
        current_chunk = ""
        
        for sentence in sentences:
            sentence = sentence.strip()
            if not sentence:
                continue
                
            if len(current_chunk) + len(sentence) <= max_chars:
                current_chunk += sentence + ". "
            else:
                if current_chunk:
                    chunks.append(current_chunk.strip())
                current_chunk = sentence + ". "
        
        if current_chunk:
            chunks.append(current_chunk.strip())
    
    return [chunk for chunk in chunks if len(chunk.strip()) > 50]  # Filter out very short chunks

def process_wiki_file(file_path: str) -> List[Dict]:
    """Process a single wiki markdown file into embedding records"""
    parsed = parse_markdown_file(file_path)
    if not parsed:
        return []
    
    category = categorize_wiki_content(file_path)
    file_name = os.path.basename(file_path)
    embeddings = []
    
    # Create embedding for overall file summary
    title = parsed['metadata'].get('title', file_name.replace('.md', ''))
    overview_text = ""
    
    if 'overview' in parsed['sections']:
        overview_text = parsed['sections']['overview']
    elif parsed['sections']:
        # Use first section if no overview
        first_section = list(parsed['sections'].values())[0]
        overview_text = first_section[:500] + "..." if len(first_section) > 500 else first_section
    
    if overview_text:
        # Create a comprehensive summary embedding
        summary_text = f"{title}: {overview_text}"
        content_chunks = extract_meaningful_content(summary_text, max_chars=800)
        
        for chunk in content_chunks:
            embedding_record = {
                "text": chunk,
                "category": f"wiki_{category}",
                "importance": 0.9,  # Wiki content is generally important
                "source_file": file_name,
                "content_type": "summary",
                "metadata": parsed['metadata']
            }
            embeddings.append(embedding_record)
    
    # Create embeddings for major sections
    important_sections = ['features', 'architecture', 'core_mission', 'key_concepts', 
                         'overview', 'getting_started', 'installation', 'usage']
    
    for section_name, section_content in parsed['sections'].items():
        if not section_content or len(section_content) < 100:
            continue
        
        # Determine importance based on section type
        importance = 0.8 if section_name.lower() in important_sections else 0.6
        
        content_chunks = extract_meaningful_content(section_content, max_chars=900)
        
        for chunk in content_chunks:
            embedding_record = {
                "text": f"{title} - {section_name.replace('_', ' ').title()}: {chunk}",
                "category": f"wiki_{category}",
                "importance": importance,
                "source_file": file_name,
                "content_type": f"section_{section_name}",
                "metadata": parsed['metadata']
            }
            embeddings.append(embedding_record)
    
    return embeddings

def process_memex_wiki(wiki_path: str = "/Users/laincorp/LainCorp/memex-wiki/docs") -> List[Dict]:
    """Process all markdown files in memex-wiki and generate embeddings"""
    print(f"🔍 Processing memex-wiki content from {wiki_path}...")
    
    if not os.path.exists(wiki_path):
        print(f"❌ Wiki path not found: {wiki_path}")
        return []
    
    all_embeddings = []
    markdown_files = []
    
    # Find all markdown files recursively
    for root, dirs, files in os.walk(wiki_path):
        for file in files:
            if file.endswith('.md') and not file.startswith('.'):
                full_path = os.path.join(root, file)
                markdown_files.append(full_path)
    
    print(f"📚 Found {len(markdown_files)} markdown files")
    
    for file_path in markdown_files:
        try:
            print(f"📄 Processing {os.path.basename(file_path)}...")
            file_embeddings = process_wiki_file(file_path)
            all_embeddings.extend(file_embeddings)
            print(f"  ✓ Generated {len(file_embeddings)} embeddings")
        except Exception as e:
            print(f"  ❌ Error processing {file_path}: {e}")
    
    print(f"📊 Generated {len(all_embeddings)} total wiki embeddings")
    return all_embeddings

def generate_wiki_embeddings(wiki_embeddings: List[Dict]) -> List[Dict]:
    """Generate embeddings for wiki content using the sentence transformer model"""
    print("🧠 Generating embeddings for wiki content...")
    
    processed_embeddings = []
    
    for i, item in enumerate(wiki_embeddings, 1):
        try:
            # Generate embedding vector
            embedding_vector = model.encode(item["text"]).tolist()
            
            # Create the structure expected by the canister
            embedding_record = {
                "text": item["text"],
                "embedding": embedding_vector,
                "channel_id": "#wiki",  # Special channel for wiki content
                "category": item["category"],
                "importance": item["importance"],
                "created_at": int(time.time()),
                "source_file": item.get("source_file", "unknown"),
                "content_type": item.get("content_type", "general")
            }
            
            processed_embeddings.append(embedding_record)
            
            if i % 10 == 0:
                print(f"  📈 Processed {i}/{len(wiki_embeddings)} wiki embeddings...")
                
        except Exception as e:
            print(f"  ❌ Error generating embedding for item {i}: {e}")
    
    return processed_embeddings

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
    print("🧠 Generating Lain's personality and memex-wiki embeddings...")
    
    all_embeddings = []
    
    # Generate personality embeddings for each channel
    print("\n📡 Processing personality data...")
    for channel_id, personality_data in LAIN_PERSONALITY.items():
        print(f"  Processing {channel_id}...")
        channel_embeddings = generate_embeddings_for_channel(channel_id, personality_data)
        all_embeddings.extend(channel_embeddings)
    
    print(f"✓ Generated {len(all_embeddings)} personality embeddings")
    
    # Process memex-wiki content
    print("\n📚 Processing memex-wiki content...")
    wiki_content = process_memex_wiki()
    if wiki_content:
        wiki_embeddings = generate_wiki_embeddings(wiki_content)
        all_embeddings.extend(wiki_embeddings)
        print(f"✓ Generated {len(wiki_embeddings)} wiki embeddings")
    else:
        print("⚠️  No wiki content processed")
    
    print(f"\n📊 Total embeddings generated: {len(all_embeddings)}")
    
    # Categorize embeddings for summary
    categories = {}
    for emb in all_embeddings:
        cat = emb['category']
        categories[cat] = categories.get(cat, 0) + 1
    
    print("\n📈 Embedding breakdown by category:")
    for category, count in sorted(categories.items()):
        print(f"  {category}: {count}")
    
    # Option to save to file first (for backup/inspection)
    output_file = "lain_personality_embeddings.json"
    with open(output_file, "w") as f:
        json.dump(all_embeddings, f, indent=2)
    print(f"\n💾 Saved embeddings to {output_file}")
    
    # Upload to canister
    upload_choice = input("\n🚀 Upload to IC canister? (y/n): ").lower().strip()
    if upload_choice in ['y', 'yes']:
        print("\n🚀 Uploading to IC canister individually...")
        successful_uploads = 0
        
        for i, embedding in enumerate(all_embeddings, 1):
            source_info = ""
            if 'source_file' in embedding:
                source_info = f" [{embedding['source_file']}]"
            
            print(f"📤 Uploading {i}/{len(all_embeddings)}: {embedding['text'][:50]}...{source_info}")
            
            if upload_single_embedding(embedding):
                successful_uploads += 1
            else:
                print(f"❌ Failed to upload embedding {i}")
                retry = input("Continue with remaining uploads? (y/n): ").lower().strip()
                if retry not in ['y', 'yes']:
                    break
        
        if successful_uploads == len(all_embeddings):
            print(f"🎉 Successfully uploaded all {successful_uploads} embeddings!")
        else:
            print(f"⚠️  Uploaded {successful_uploads}/{len(all_embeddings)} embeddings.")
    else:
        print("📁 Embeddings saved locally only. Run again to upload to canister.")

if __name__ == "__main__":
    main()