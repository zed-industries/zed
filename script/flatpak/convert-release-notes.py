import re
import requests
import sys
import textwrap
import os

def clean_line(line: str, in_code_fence: bool) -> str:
    line = re.sub(r"<", "&lt;", line)
    line = re.sub(r">", "&gt;", line)
    line = re.sub(r"\(\[(#\d+)\]\([\w|\d\:|\/|\.|\-|_]*\)\)", lambda match: f"[{match.group(1)}]", line)
    line = re.sub(r"\[(#\d+)\]\([\w|\d\:|\/|\.|\-|_]*\)", lambda match: f"[{match.group(1)}]", line)
    if not in_code_fence:
        line = line.strip()

    return line


def convert_body(body: str) -> str:
    formatted = ""

    in_code_fence = False
    in_list = False
    for line in body.splitlines():
        line = clean_line(line, in_code_fence)
        if not line:
            continue
        if re.search(r'\[[\w|\d|:|\/|\.|\-|_]*\]\([\w|\d|:|\/|\.|\-|_]*\)', line):
            continue
        line = re.sub(r"(?<!\`)`([^`\n]+)`(?!`)", lambda match: f"<code>{match.group(1)}</code>", line)

        contains_code_fence = bool(re.search(r"```", line))
        is_list = bool(re.search(r"^-\s*", line))

        if in_list and not is_list:
            formatted += "</ul>\n"
        if (not in_code_fence and contains_code_fence) or (not in_list and is_list):
            formatted += "<ul>\n"
        in_list = is_list
        in_code_fence = contains_code_fence != in_code_fence

        if is_list:
            line = re.sub(r"^-\s*", "", line)
            line = f"    <li>{line}</li>"
        elif in_code_fence or contains_code_fence:
            line = f"    <li><code>    {line}</code></li>"
        else:
            line = f"<p>{line}</p>"
        formatted += f"{line}\n"

        if (not in_code_fence and contains_code_fence):
            formatted += "</ul>\n"
    if in_code_fence or in_list:
        formatted += "</ul>\n"
    return formatted

def get_release_info(tag: str):
    url = f"https://api.github.com/repos/zed-industries/zed/releases/tags/{tag}"
    response = requests.get(url)
    if response.status_code == 200:
        return response.json()
    else:
        print(f"Failed to fetch release info for tag '{tag}'. Status code: {response.status_code}")
        quit()


if __name__ == "__main__":
    os.chdir(sys.path[0])

    if len(sys.argv) != 3:
        print("Usage: python convert-release-notes.py <tag> <channel>")
        sys.exit(1)

    tag = sys.argv[1]
    channel = sys.argv[2]

    release_info = get_release_info(tag)
    body = convert_body(release_info["body"])
    version = tag.removeprefix("v").removesuffix("-pre")
    date = release_info["published_at"]

    release_info_str = f"<release version=\"{version}\" date=\"{date}\">\n"
    release_info_str += f"    <description>\n"
    release_info_str += textwrap.indent(body, " " * 8)
    release_info_str += f"    </description>\n"
    release_info_str += f"    <url>https://github.com/zed-industries/zed/releases/tag/{tag}</url>\n"
    release_info_str += "</release>\n"

    channel_releases_file = f"../../crates/zed/resources/flatpak/release-info/{channel}"
    with open(channel_releases_file) as f:
        old_release_info = f.read()
    with open(channel_releases_file, "w") as f:
        f.write(textwrap.indent(release_info_str, " " * 8) + old_release_info)
    print(f"Added release notes from {tag} to '{channel_releases_file}'")
