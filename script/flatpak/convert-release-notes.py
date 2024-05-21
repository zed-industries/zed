import re
import requests
import sys
import json

def convert_line(line: str, wrap_in_code: bool) -> (bool, bool, bool, str):
    line = line.strip()
    if not line:
        return (True, False, False, "")
    
    line = re.sub(r'<', '&lt;', line)
    line = re.sub(r'>', '&gt;', line)

    is_code_fence = re.search(r'^```\w*', line)
    wrap_in_code = wrap_in_code or is_code_fence
    if not wrap_in_code:
        line = re.sub(r'(`[^`]*`)', lambda match: f'<code>{match.group(1)}</code>', line)
    
    line = re.sub(r'\(\[#\d+\]\([\w|\d|:|\/|\.|\-|_]*\)\)', "", line)
    line = re.sub(r'\s(\.)$', lambda match: match.group(1), line.strip())
    
    if re.search(r'\[[\w|\d|:|\/|\.|\-|_]*\]\([\w|\d|:|\/|\.|\-|_]*\)', line):
        return (True, False, False, "")

    match line[0]:
        case "-":
            return (False, True, False, f"    <li>{line[1:].strip()}</li>")
        case "#":
            return (False, False, False, f"<p><em>{line}</em></p>")
        case _:
            return (False, False, is_code_fence, f"<li><code>    {line}</code></li>" if wrap_in_code else f"<p>{line}</p>")


def get_release_info(tag, channel):
    url = f"https://api.github.com/repos/zed-industries/zed/releases/tags/{tag}"
    response = requests.get(url)

    if response.status_code == 200:
        release_info = response.json()
        version = tag.removeprefix("v")
        date = release_info["published_at"]

        indent = " " * 8
        print(f"{indent}<release version=\"{version}\" date=\"{date}\">")
        print(f"{indent}    <description>")

        in_list = False
        in_code_fence = False
        for line in release_info["body"].splitlines():
            empty, is_list, is_code_fence, content = convert_line(line, in_code_fence)
            if empty:
                continue

            if not in_list and is_list:
                print(f"{indent}        <ul>")
            elif in_list and not is_list:
                print(f"{indent}        </ul>")
            in_list = is_list

            if is_code_fence:
                if in_code_fence:
                    print(f"{indent}        {content}")
                    print(f"{indent}        </ul>")
                    in_code_fence = False
                    continue
                else:
                    print(f"{indent}        <ul>") # we make code blocks list because there is no way to force a single line break
                    in_code_fence = True
            
            print(f"{indent}        {content}")
        if in_list:
            print(f"{indent}        </ul>")

        print(f"{indent}    </description>")
        print(f"{indent}    <url>https://github.com/zed-industries/zed/releases/tag/{tag}</url>")
        print(f"{indent}</release>")
    else:
        print(f"Failed to fetch release info for tag '{tag}'. Status code: {response.status_code}")

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: python script.py <release_tag> <release_channel>")
        sys.exit(1)

    release_tag = sys.argv[1]
    release_channel = sys.argv[2]

    get_release_info(release_tag, release_channel)
