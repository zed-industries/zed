# Extracting an extension to dedicated repo

These are some notes of how to extract an extension from the main zed repository and generate a new repository which preserves the history as best as possible.  In the this example we will be extracting the `ruby` extension, substitute as appropriate.

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
Note whitespace matters so `ruby: ==>` is removing the `ruby:` prefix from a commit messages and adding a space after `==> ` means the replacement begins with a space.  Regex capture groups are numbered `\1`, `\2`, etc.

See: [Git Filter Repo Docs](https://htmlpreview.github.io/?https://github.com/newren/git-filter-repo/blob/docs/html/git-filter-repo.html) for more.

2. Create a clean clone the zed repository, delete tags, delete branches and do the work.

> **Note**
> If you get `zsh: command not found: #` errors, run:
> `setopt interactive_comments && echo "setopt interactive_comments" >> ~/.zshrc`

```sh
rm -rf zed3
git clone --single-branch --no-tags git@github.com:zed-industries/zed.git zed3
cd zed3

# This removes the LICENSE symlink
git filter-repo --invert-paths --path extensions/ruby/LICENSE-APACHE

git filter-repo \
    --use-mailmap \
    --subdirectory-filter extensions/ruby/ \
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
git log --grep="(\d+\.\d+\.\d+\.)" --perl-regexp --oneline --reverse
```

Then just:
```
git tag v0.0.2 abcd1234
git tag v0.0.3 deadbeef
```

Usually the initial extraction didn't mention a version number so you can just do that one manually.

4. Push to the new repo

Create a new empty repo on github under the [zed-extensions](https://github.com/zed-extensions) organization.

```
git remote add origin git@github.com:zed-extensions/ruby
git push origin main --tags
```

5. [Optional]
