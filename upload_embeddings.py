#!/usr/bin/env python3
"""
Upload embeddings to lain-io-api canister in small batches
"""

import json
import subprocess
import time

def format_for_dfx(embedding):
    """Format single embedding for dfx canister call"""
    # Format the embedding vector for Candid with explicit float32 type
    embedding_values = [f"{f}:float32" for f in embedding["embedding"]]
    embedding_str = "; ".join(embedding_values)
    
    # Escape quotes in text
    escaped_text = embedding["text"].replace('"', '\\"').replace('\n', ' ').replace('\r', ' ')
    
    record = f'''record {{
        text = "{escaped_text}";
        embedding = vec {{ {embedding_str} }};
        channel_id = "{embedding["channel_id"]}";
        category = "{embedding["category"]}";
        importance = {embedding["importance"]}:float32;
        created_at = {embedding["created_at"]}:nat64;
    }}'''
    
    return record

def upload_single_embedding(embedding, canister_name="ai_api_backend"):
    """Upload a single embedding to the canister"""
    formatted_data = f"({format_for_dfx(embedding)})"
    
    # Use the single embedding endpoint with IC network
    cmd = [
        "dfx", "canister", "call", "--network", "ic", canister_name, 
        "store_personality", formatted_data
    ]
    
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, check=True, timeout=30)
        return True, result.stdout.strip()
    except subprocess.TimeoutExpired:
        return False, "Timeout"
    except subprocess.CalledProcessError as e:
        return False, e.stderr
    except Exception as e:
        return False, str(e)

def main():
    print("📤 Loading embeddings from file...")
    
    with open("lain_personality_embeddings.json", "r") as f:
        embeddings = json.load(f)
    
    print(f"📊 Found {len(embeddings)} embeddings to upload")
    
    # Start from beginning or resume from a specific point
    start_index = int(input("Start from index (0 for beginning): ") or 0)
    
    successful_uploads = 0
    failed_uploads = 0
    
    for i in range(start_index, len(embeddings)):
        embedding = embeddings[i]
        
        # Show progress
        source_info = ""
        if 'source_file' in embedding:
            source_info = f" [{embedding['source_file']}]"
        
        print(f"\n📤 Uploading {i+1}/{len(embeddings)}: {embedding['text'][:50]}...{source_info}")
        
        success, message = upload_single_embedding(embedding)
        
        if success:
            successful_uploads += 1
            print(f"  ✅ Success")
            # Small delay to avoid overwhelming the canister
            time.sleep(0.5)
        else:
            failed_uploads += 1
            print(f"  ❌ Failed: {message}")
            
            # Ask user whether to continue
            if "Timeout" in message or "network" in message.lower():
                retry = input("  🔄 Network issue. Retry this embedding? (y/n/s=skip): ").lower().strip()
                if retry == 'y':
                    print("  🔄 Retrying...")
                    success, message = upload_single_embedding(embedding)
                    if success:
                        successful_uploads += 1
                        failed_uploads -= 1
                        print(f"  ✅ Retry successful")
                    else:
                        print(f"  ❌ Retry failed: {message}")
                elif retry == 's':
                    print(f"  ⏭️  Skipping embedding {i+1}")
                    continue
                else:
                    break
            else:
                # For other errors, ask if we should continue
                continue_choice = input("  ⚠️  Continue with next embedding? (y/n): ").lower().strip()
                if continue_choice not in ['y', 'yes']:
                    break
    
    print(f"\n📈 Upload Summary:")
    print(f"  ✅ Successful: {successful_uploads}")
    print(f"  ❌ Failed: {failed_uploads}")
    print(f"  📊 Total processed: {successful_uploads + failed_uploads}")
    
    if failed_uploads > 0:
        print(f"\n💡 To resume from where you left off, restart and enter index: {successful_uploads + start_index}")

if __name__ == "__main__":
    main()