import subprocess
import json
import http.client
import mimetypes
import os

def get_text_files():
    text_files = []
    # List all files tracked by Git
    git_files_proc = subprocess.run(['git', 'ls-files'], stdout=subprocess.PIPE, text=True)
    for file in git_files_proc.stdout.strip().split('\n'):
        # Check MIME type for each file
        mime_check_proc = subprocess.run(['file', '--mime', file], stdout=subprocess.PIPE, text=True)
        if 'text' in mime_check_proc.stdout:
            text_files.append(file)

    print(f"File count: {len(text_files)}")

    return text_files

def get_file_contents(file):
    # Read file content
    with open(file, 'r') as f:
        return f.read()


def main():
    GEMINI_API_KEY = os.environ.get('GEMINI_API_KEY')

    # Your prompt
    prompt = "Document the data types and dataflow in this codebase in preparation to port a streaming implementation to rust:\n\n"
    # Fetch all text files
    text_files = get_text_files()
    code_blocks = []
    for file in text_files:
        file_contents = get_file_contents(file)
        # Create a code block for each text file
        code_blocks.append(f"\n`{file}`\n\n```{file_contents}```\n")

    # Construct the JSON payload
    payload = json.dumps({
        "contents": [{
            "parts": [{
                "text": prompt + "".join(code_blocks)
            }]
        }]
    })

    # Prepare the HTTP connection
    conn = http.client.HTTPSConnection("generativelanguage.googleapis.com")

    # Define headers
    headers = {
        'Content-Type': 'application/json',
        'Content-Length': str(len(payload))
    }

    # Output the content length in bytes
    print(f"Content Length in kilobytes: {len(payload.encode('utf-8')) / 1024:.2f} KB")


    # Send a request to count the tokens
    conn.request("POST", f"/v1beta/models/gemini-1.5-pro-latest:countTokens?key={GEMINI_API_KEY}", body=payload, headers=headers)
    # Get the response
    response = conn.getresponse()
    if response.status == 200:
        token_count = json.loads(response.read().decode('utf-8')).get('totalTokens')
        print(f"Token count: {token_count}")
    else:
        print(f"Failed to get token count. Status code: {response.status}, Response body: {response.read().decode('utf-8')}")


    # Prepare the HTTP connection
    conn = http.client.HTTPSConnection("generativelanguage.googleapis.com")
    conn.request("GET", f"/v1beta/models/gemini-1.5-pro-latest:streamGenerateContent?key={GEMINI_API_KEY}", body=payload, headers=headers)

    # Get the response in a streaming manner
    response = conn.getresponse()
    if response.status == 200:
        print("Successfully sent the data to the API.")
        # Read the response in chunks
        while chunk := response.read(4096):
            print(chunk.decode('utf-8'))
    else:
        print(f"Failed to send the data to the API. Status code: {response.status}, Response body: {response.read().decode('utf-8')}")

    # Close the connection
    conn.close()

if __name__ == "__main__":
    main()
