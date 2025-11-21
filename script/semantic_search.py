import os
import glob
import time
import sys
import hashlib
import json
from google import genai
from google.genai import types

try:
    from rich.console import Console
    from rich.markdown import Markdown
    from rich.panel import Panel
    RICH_AVAILABLE = True
except ImportError:
    RICH_AVAILABLE = False

def calculate_md5(file_path):
    """Calculates the MD5 hash of a file."""
    hash_md5 = hashlib.md5()
    with open(file_path, "rb") as f:
        for chunk in iter(lambda: f.read(4096), b""):
            hash_md5.update(chunk)
    return hash_md5.hexdigest()

def main():
    # 1. Initialize Client
    api_key = os.environ.get("GOOGLE_API_KEY")
    if not api_key:
        print("Error: GOOGLE_API_KEY environment variable not set.")
        print("Please export it: export GOOGLE_API_KEY='your_key_here'")
        sys.exit(1)
    
    print("Initializing Gemini Client...")
    client = genai.Client(api_key=api_key)

    # 2. Discover Files
    # Assuming script is run from project root, otherwise adjust path
    docs_path = "docs/src"
    if not os.path.exists(docs_path):
        # Fallback if run from inside script/
        if os.path.exists("../docs/src"):
            docs_path = "../docs/src"
        else:
            print(f"Error: Could not find docs path at {docs_path}")
            sys.exit(1)

    md_files = glob.glob(f"{docs_path}/**/*.md", recursive=True)
    if not md_files:
        print(f"No markdown files found in {docs_path}")
        sys.exit(1)
    
    # LIMIT for PoC to avoid long wait times
    # Ensure key-bindings.md is included if present
    # priority_files = [f for f in md_files if "key-bindings.md" in f]
    # other_files = [f for f in md_files if "key-bindings.md" not in f]
    # md_files = priority_files + other_files
    
    # if len(md_files) > 20:
    #     print(f"Found {len(md_files)} files. Limiting to first 20 for PoC speed.")
    #     md_files = md_files[:20]

    print(f"Processing {len(md_files)} markdown files...")

    # 3. Manage Store
    store_name = "Zed Docs Prototype"
    store_id_file = ".zed_docs_store_id"
    file_hashes_file = ".zed_docs_hashes.json"
    store = None
    created_new = False
    
    # A. Try loading from local cache
    if os.path.exists(store_id_file):
        with open(store_id_file, "r") as f:
            cached_id = f.read().strip()
        if cached_id:
            try:
                print(f"Attempting to load store from cache: {cached_id}")
                store = client.file_search_stores.get(name=cached_id)
                print(f"Loaded store: {store.name}")
            except Exception:
                print("Cached store not found or invalid.")
                store = None

    # B. If not found in cache, try finding by display name (fallback)
    if not store:
        print("Checking for existing File Search Store by name...")
        for s in client.file_search_stores.list():
            if hasattr(s, 'display_name') and s.display_name == store_name:
                store = s
                print(f"Found existing store by name: {store.name}")
                # Update cache
                with open(store_id_file, "w") as f:
                    f.write(store.name)
                break
    
    # C. If still not found, create new
    if not store:
        print("Creating new File Search Store...")
        try:
            # Try to set display_name if the SDK supports it in the create payload
            # We'll try passing the config object if possible, or just bare create
            # For the python SDK 'google-genai', exact syntax varies, but let's try basic first
            store = client.file_search_stores.create()
            print(f"Store created: {store.name}")
            created_new = True
            
            # Persist ID immediately
            with open(store_id_file, "w") as f:
                f.write(store.name)
                
        except Exception as e:
            print(f"Failed to create store: {e}")
            sys.exit(1)

    # 4. Upload Files (Incremental)
    # We track file hashes to avoid re-uploading unchanged files.
    file_hashes = {}
    if not created_new and os.path.exists(file_hashes_file):
        try:
            with open(file_hashes_file, "r") as f:
                file_hashes = json.load(f)
        except Exception:
            print("Warning: Could not load hash cache. Re-indexing may occur.")
            file_hashes = {}
    elif created_new:
        # If we created a new store, we must upload everything, so ignore old hashes
        file_hashes = {}

    print("Checking files for updates...")
    
    uploaded_count = 0
    skipped_count = 0
    failed_count = 0
    
    # Track current hashes to clean up deleted files from cache later (optional cleanup)
    new_file_hashes = {}

    for file_path in md_files:
        try:
            current_hash = calculate_md5(file_path)
            new_file_hashes[file_path] = current_hash
            
            if file_path in file_hashes and file_hashes[file_path] == current_hash:
                skipped_count += 1
                continue

            print(f"Uploading {file_path}...")
            upload_op = client.file_search_stores.upload_to_file_search_store(
                file_search_store_name=store.name,
                file=file_path
            )
            
            while not upload_op.done:
                time.sleep(0.5)
                upload_op = client.operations.get(upload_op)
            
            uploaded_count += 1
            if uploaded_count % 5 == 0:
                    print(f"Uploaded {uploaded_count} new/modified files...")

        except Exception as e:
            print(f"Failed to upload {file_path}: {e}")
            failed_count += 1
            # If upload failed, don't update the hash in the new map so we retry next time?
            # Or keep old hash? Let's just not add it to new_file_hashes if we want strict sync,
            # but simpler to just keep the loop going. 
            # If we fail, we likely won't query correctly anyway.

    # Save the new state
    # We merge old and new to avoid losing history of files that might have been temporarily skipped?
    # Actually, strictly speaking, we should only keep what exists now.
    # But for simplicity, let's just save what we processed.
    with open(file_hashes_file, "w") as f:
        json.dump(new_file_hashes, f, indent=2)

    print(f"Finished processing. Uploaded: {uploaded_count}, Skipped: {skipped_count}, Failed: {failed_count}")


    # 5. Query
    if len(sys.argv) > 1:
        query = " ".join(sys.argv[1:])
    else:
        query = "How do I configure key bindings?"
        
    print(f"\nAsking question: '{query}'")
    
    try:
        response = client.models.generate_content(
            model='gemini-2.5-flash',
            contents=query,
            config=types.GenerateContentConfig(
                tools=[types.Tool(
                    file_search=types.FileSearch(
                        file_search_store_names=[store.name]
                    )
                )]
            )
        )

        # 6. Output
        if RICH_AVAILABLE:
            console = Console()
            console.print("\n[bold green]Response:[/bold green]")
            console.print(Panel(Markdown(response.text), title="Gemini Response", border_style="blue"))
        else:
            print("\nResponse:")
            print(response.text)
            print("\n(Tip: Install 'rich' library for prettier output: pip install rich)")
        
        # Print citations/grounding
        if response.candidates[0].grounding_metadata:
            if RICH_AVAILABLE:
                 console.print("\n[bold yellow]Sources used:[/bold yellow]")
            else:
                 print("\nSources used:")

            chunks = response.candidates[0].grounding_metadata.grounding_chunks
            if chunks:
                seen_sources = set()
                for chunk in chunks:
                    if chunk.retrieved_context:
                         title = chunk.retrieved_context.title
                         if title not in seen_sources:
                             if RICH_AVAILABLE:
                                 console.print(f"- [cyan]{title}[/cyan]")
                             else:
                                 print(f"- {title}")
                             seen_sources.add(title)
            else:
                print("No specific grounding chunks returned.")
        else:
             print("\nNo grounding metadata returned.")

    except Exception as e:
        print(f"Error during generation: {e}")

    print("\nDemo complete. Note: You may want to delete the store manually if not needed.")

if __name__ == "__main__":
    main()
