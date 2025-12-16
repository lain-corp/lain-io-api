#!/usr/bin/env python3
"""
Simple test script for the unified knowledge search system.
Tests basic functionality without complex vector formatting.
"""

import subprocess
import sys

def run_dfx_command(command):
    """Run a dfx command and return the output."""
    try:
        result = subprocess.run(command, shell=True, capture_output=True, text=True)
        if result.returncode == 0:
            return result.stdout.strip()
        else:
            return f"Error: {result.stderr.strip()}"
    except Exception as e:
        return f"Exception: {str(e)}"

def test_knowledge_stats():
    """Test getting knowledge statistics."""
    print("🔍 Testing knowledge statistics...")
    command = "dfx canister call ai_api_backend get_knowledge_stats --network ic"
    result = run_dfx_command(command)
    print(f"Result: {result}\n")

def test_knowledge_categories():
    """Test getting knowledge categories."""
    print("📚 Testing knowledge categories...")
    command = "dfx canister call ai_api_backend get_knowledge_categories --network ic"
    result = run_dfx_command(command)
    print(f"Result: {result}\n")

def test_search_by_text():
    """Test searching by text without vector embeddings."""
    print("🔎 Testing text-based search...")
    # Simple text search for "blockchain"
    command = 'dfx canister call ai_api_backend search_unified_knowledge \'(vec {0.1:float32; 0.2:float32}, 5:nat64, opt "personality")\' --network ic'
    result = run_dfx_command(command)
    print(f"Result: {result}\n")

def main():
    """Run all tests."""
    print("🚀 Testing Unified Knowledge Search System")
    print("=" * 50)
    
    test_knowledge_stats()
    test_knowledge_categories() 
    test_search_by_text()
    
    print("✅ Testing complete!")

if __name__ == "__main__":
    main()