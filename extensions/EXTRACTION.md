# Extracting an extension to dedicated repo

These are some notes of how to extract an extension from the main zed repository and generate a new repository which preserves the history as best as possible. In the this example we will be extracting the `ruby` extension, substitute as appropriate.

## Pre-requisites

Install [git-filter-repo](https://github.com/newren/git-filter-repo/blob/main/INSTALL.md):

```
brew install git-filter-repo
```

## Process

1. Create an expressions.txt file somewhere (e.g. `~/projects/expressions.txt`)

```
ruby: ==>
extension: ==>
chore: ==>
zed_extension_api: ==>
regex:(?<![\[a-zA-Z0-9])(#[0-9]{3,5})==>zed-industries/zed\1
```

This file takes the form of `patern==>replacement`, where the replacement is optional.
Note whitespace matters so `ruby: ==>` is removing the `ruby:` prefix from a commit messages and adding a space after `==> ` means the replacement begins with a space. Regex capture groups are numbered `\1`, `\2`, etc.

See: [Git Filter Repo Docs](https://htmlpreview.github.io/?https://github.com/newren/git-filter-repo/blob/docs/html/git-filter-repo.html) for more.

2. Create a clean clone the zed repository, delete tags, delete branches and do the work.

> **Note**
> If you get `zsh: command not found: #` errors, run:
> `setopt interactive_comments && echo "setopt interactive_comments" >> ~/.zshrc`

```sh
LANGNAME=ruby
rm -rf $LANGNAME
git clone --single-branch --no-tags git@github.com:zed-industries/zed.git $LANGNAME
cd $LANGNAME

# This removes the LICENSE symlink
git filter-repo --invert-paths --path extensions/$LANGNAME/LICENSE-APACHE

git filter-repo \
    --use-mailmap \
    --subdirectory-filter extensions/$LANGNAME/ \
    --path LICENSE-APACHE \
    --replace-message ~/projects/expressions.txt
```

3. Review the commits.

This is your last chance to make any modifications.
If you don't fix it now, it'll be wrong forever.

For example, a previous commit message was `php/ruby: bump version to 0.0.5`
which was replaced with `php/bump version to 0.0.5`
so I added a new line to expressions.txt with `php/==>`
and next run it became `bump version to 0.0.5`.

4. [Optional] Generate tags

You can always add tags later, but it's a nice touch.

Show you all commits that mention a version number:

```sh
git log --grep="(\d+\.\d+\.\d+)" --perl-regexp --oneline --reverse
```

Then just:

```
git tag v0.0.2 abcd1234
git tag v0.0.3 deadbeef
```

Usually the initial extraction didn't mention a version number so you can just do that one manually.

4. [Optional] Add a README.md and commit.

5. Push to the new repo

Create a new empty repo on github under the [zed-extensions](https://github.com/organizations/zed-extensions/repositories/new) organization.

```
git remote add origin git@github.com:zed-extensions/$LANGNAME
git push origin main --tags
git branch --set-upstream-to=origin/main main
```

6. Setup the new repository:

- Go to the repository settings:
  - Disable Wikis
  - Uncheck "Allow Merge Commits"
  - Check "Allow Squash Merging"
    - Default commit message: "Pull request title and description"

7. Publish a new version of the extension.

```
OLD_VERSION=$(grep '^version = ' extension.toml | cut -d'"' -f2)
NEW_VERSION=$(echo "$OLD_VERSION" | awk -F. '{$NF = $NF + 1;} 1' OFS=.)
echo $OLD_VERSION $NEW_VERSION
perl -i -pe "s/$OLD_VERSION/$NEW_VERSION/" extension.toml

# if there's rust code, update this too.
test -f Cargo.toml && perl -i -pe "s/$OLD_VERSION/$NEW_VERSION/" cargo.toml
test -f Cargo.toml && cargo check

# commit and push
git add -u
git checkout -b "bump_${NEW_VERSION}"
git commit -m "Bump to v${NEW_VERSION}"
git push
gh pr create --title "Bump to v${NEW_VERSION}" --web

# merge PR in web interface
git checkout main
git pull
git tag v${NEW_VERSION}
git push origin v${NEW_VERSION}
```

7. In zed repository, `rm -rf extension/langname` and push a PR.

8. Update extensions repository:

```sh
cd ../extensions
git checkout main
git pull
git submodule init
git submodule update
git status

git checkout -b ${LANGNAME}_v${NEW_VERSION}
git submodule add https://github.com/zed-extensions/${LANGNAME}.git extensions/${LANGNAME}
pnpm sort-extensions

# edit extensions.toml:
# - bump version
# - change `submodule` from `extensions/zed` to new path
# - remove `path` line all together

git add extensions.toml .gitmodules extensions/${LANGNAME}
git diff --cached
git commit -m "Bump ${LANGNAME} to v${NEW_VERSION}"
git push
```

Create PR and reference the Zed PR with removal from tree.