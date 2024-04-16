# Zed Releases

Zed currently maintains two public releases for macOS:

- [Stable](https://zed.dev/download). This is the primary version that people download and use.
- [Preview](https://zed.dev/releases/preview), which receives updates a week ahead of stable for early adopters.

Typically we cut a new minor release every Wednesday. The current Preview becomes Stable, and the new Preview contains everything on main up until that point.

If bugs are found and fixed during the week, they may be cherry-picked into the release branches and so new patch versions for preview and stable can become available throughout the week.

## Wednesday release process

You will need write access to the Zed repository to do this:

- Checkout `main` and ensure your working copy is clean.
- Run `./script/bump-zed-minor-versions` and push the tags
  and branches as instructed.
- Wait for the builds to appear at https://github.com/zed-industries/zed/releases (typically takes around 30 minutes)
- Copy the release notes from the previous Preview release(s) to the current Stable release.
- Write new release notes for Preview. `/script/get-preview-channel-changes` can help with this, but you'll need to edit and format the output to make it good.
- Download the artifacts for each release and test that you can run them locally.
- Publish the releases.

## Patch release process

If your PR fixes a panic or a crash, you should cherry-pick it to the current stable and preview branches. If your PR fixes a regression in recently released code, you should cherry-pick it to the appropriate branch.

You will need write access to the Zed repository to do this:

- Cherry pick them onto the correct branch. You can either do this manually, or leave a comment of the form `/cherry-pick v0.XXX.x` on the PR, and the GitHub bot should do it for you.
- Run `./script/trigger-release {preview|stable}`
- Wait for the builds to appear at https://github.com/zed-industries/zed/releases (typically takes around 30 minutes)
- Add release notes using the `Release notes:` section of each cherry-picked PR.
- Download the artifacts for each release and test that you can run them locally.
- Publish the release.

## Nightly release process

- Merge your changes to main
- Run `./script/trigger-release {nightly}`
