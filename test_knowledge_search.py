#!/usr/bin/env python3
"""
Test script for the unified knowledge search functionality
"""

import json
import subprocess
from sentence_transformers import SentenceTransformer

# Load the same model used by the canister
model = SentenceTransformer('all-MiniLM-L6-v2')

def generate_query_embedding(query_text):
    """Generate embedding for a query"""
    return model.encode(query_text).tolist()

def test_unified_search(query_text, categories=None, limit=5):
    """Test the unified knowledge search"""
    print(f"🔍 Searching for: '{query_text}'")
    
    # Generate query embedding
    query_embedding = generate_query_embedding(query_text)
    
    # Format embedding for dfx call
    embedding_str = "; ".join([f"{f}:float32" for f in query_embedding])
    
    # Prepare dfx command
    if categories:
        categories_str = "; ".join([f'"{cat}"' for cat in categories])
        cmd = [
            "dfx", "canister", "call", "--network", "ic", "ai_api_backend",
            "search_unified_knowledge",
            f"(vec {{{embedding_str}}}; opt vec {{{categories_str}}}; opt {limit}:nat32)"
        ]
    else:
        cmd = [
            "dfx", "canister", "call", "--network", "ic", "ai_api_backend",
            "search_unified_knowledge", 
            f"(vec {{{embedding_str}}}; null; opt {limit}:nat32)"
        ]
    
    print(f"💫 Running search...")
    
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        print(f"✅ Search successful!")
        print(f"📊 Results:")
        print(result.stdout)
        return result.stdout
    except subprocess.CalledProcessError as e:
        print(f"❌ Search failed: {e.stderr}")
        return None

def test_wiki_search(query_text, content_type=None, limit=3):
    """Test the wiki-specific search"""
    print(f"📚 Wiki search for: '{query_text}'")
    
    # Generate query embedding
    query_embedding = generate_query_embedding(query_text)
    
    # Format embedding for dfx call
    embedding_str = "; ".join([f"{f}:float32" for f in query_embedding])
    
    # Prepare dfx command
    if content_type:
        cmd = [
            "dfx", "canister", "call", "--network", "ic", "ai_api_backend",
            "search_wiki_content",
            f"(vec {{{embedding_str}}}; opt \"{content_type}\"; opt {limit}:nat32)"
        ]
    else:
        cmd = [
            "dfx", "canister", "call", "--network", "ic", "ai_api_backend",
            "search_wiki_content", 
            f"(vec {{{embedding_str}}}; null; opt {limit}:nat32)"
        ]
    
    print(f"📖 Running wiki search...")
    
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        print(f"✅ Wiki search successful!")
        print(f"📊 Results:")
        print(result.stdout)
        return result.stdout
    except subprocess.CalledProcessError as e:
        print(f"❌ Wiki search failed: {e.stderr}")
        return None

def main():
    print("🚀 Testing Unified Knowledge Search System")
    print("=" * 50)
    
    # Test 1: General knowledge search
    print("\\n🧪 Test 1: General search about 'blockchain technology'")
    test_unified_search("blockchain technology and decentralized systems")
    
    print("\\n" + "="*50)
    
    # Test 2: Personality-focused search  
    print("\\n🧪 Test 2: Personality search about 'programming preferences'")
    test_unified_search("programming languages and technical preferences", 
                       categories=["technical_preference", "programming_philosophy"])
    
    print("\\n" + "="*50)
    
    # Test 3: Wiki-specific search
    print("\\n🧪 Test 3: Wiki search about 'lain.ai project'")
    test_wiki_search("autonomous AI governance and decision making", 
                    content_type="project-docs")
    
    print("\\n" + "="*50)
    
    # Test 4: Technical documentation search
    print("\\n🧪 Test 4: Technical guide search about 'ICP development'")
    test_wiki_search("Internet Computer development setup and deployment", 
                    content_type="tech-guides")
    
    print("\\n🎉 All tests completed!")
    print("\\n📝 Summary:")
    print("- Unified search combines personality traits + wiki knowledge")
    print("- Category filtering allows targeted searches")
    print("- Wiki-specific search focuses on documentation")
    print("- Results ranked by similarity × importance score")

if __name__ == "__main__":
    main()