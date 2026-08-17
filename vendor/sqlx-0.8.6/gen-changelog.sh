# Requires Github CLI and `jq`
# Usage: `./gen-changelog.sh YYYY-mm-dd`
# Generates changelog entries for all PRs merged on or after the given date.
set -e

PULLS='[]'
CURSOR='null'

MIN_MERGED_AT=$(date --date="$1" +%s)

while true
do
  # Use the GraphQL API to paginate merged pull requests.
  # The REST API doesn't allow filtering only merged pull requests.
  # We scan all merged pull requests from the beginning because it's not unheard of to have a very old PR finally get
  # merged; e.g. #1081, merged a year and a half after it was opened.
  if [ "$CURSOR" != "null" ];
  then
    PAGE=$(gh api graphql -f after="$CURSOR" -f query='query($after: String) {
               repository(owner: "launchbadge", name: "sqlx") {
                 pullRequests(first:100,orderBy: {field:CREATED_AT, direction:ASC},states:MERGED, after: $after) {
                   nodes {
                     number
                     author { login }
                     title
                     url
                     mergedAt
                   }
                   pageInfo {
                     hasNextPage
                     endCursor
                   }
                 }
               }
             }');
  else
        PAGE=$(gh api graphql -f query='query {
                   repository(owner: "launchbadge", name: "sqlx") {
                     pullRequests(first:100,orderBy: {field:CREATED_AT, direction:ASC},states:MERGED) {
                       nodes {
                         number
                         author { login }
                         title
                         url
                         mergedAt
                       }
                       pageInfo {
                         hasNextPage
                         endCursor
                       }
                     }
                   }
                 }');
  fi

  CURSOR=$(echo "$PAGE" | jq -r '.data.repository.pullRequests.pageInfo.endCursor');

  HAS_NEXT_PAGE=$(echo "$PAGE" | jq '.data.repository.pullRequests.pageInfo.hasNextPage');

  PULLS=$(echo "$PAGE" | jq "$PULLS + (.data.repository.pullRequests.nodes | map(select(.mergedAt | fromdate >= $MIN_MERGED_AT)))");

  # can't use `"$CURSOR" == 'null'` because the last page still gives a valid cursor
  if ! $HAS_NEXT_PAGE; then break; fi;
done

COUNT=$(echo "$PULLS" | jq "length");

echo "Found $COUNT pull requests merged on or after $1\n"

if [ -z $COUNT ]; then exit 0; fi;

echo "Entries:"
echo "$PULLS" | jq -r 'map("* [[#\(.number)]]: \(.title) [[@\(.author.login)]]") | join("\n")'

echo "\nLinks:"
echo "$PULLS" | jq -r 'map("[#\(.number)]: \(.url)") | join("\n")'

echo "\nNew Authors:"
DUPE_AUTHORS=''

# Generate link entries for new authors at the end of the changelog.
echo "$PULLS" | jq -r '.[].author.login' | while read author; do
  author_url="https://github.com/$author"
  author_entry="[@$author]: $author_url"

  # Check if the entry already exists in the changelog or in our list of new authors.
  if grep -qF "$author_entry" CHANGELOG.md || echo "$DUPE_AUTHORS" | grep -qF "$author_entry";
  then continue;
  fi;

  DUPE_AUTHORS="$DUPE_AUTHORS$author_entry\n"
  echo $author_entry
done
