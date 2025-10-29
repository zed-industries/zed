#!/opt/homebrew/bin/nu
# todo! delete-me

let tag_name = "v00.00.00-test";
let bookmark_git_hash = jj show -r gh-workflow-release@origin --template "commit_id" --no-patch;
git tag -a $tag_name -f -m "Test Tag" $bookmark_git_hash
git push --delete origin $tag_name
git push origin $tag_name
