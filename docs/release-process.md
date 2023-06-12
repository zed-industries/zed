# Zed's Release Process

The process to create and ship a Zed release

## Overview

### Release Channels

Users of Zed can choose between two _release channels_ - 'Stable' and 'Preview'. Most people use Stable, but Preview exists so that the Zed team and other early-adopters can test new features before they are released to our general user-base.

### Weekly (Minor) Releases

We normally publish new releases of Zed on Wednesdays, for both the Stable and Preview channels. For each of these releases, we bump Zed's _minor_ version number.

For the Preview channel, we build the new release based on what's on the `main` branch. For the Stable channel, we build the new release based on the last Preview release.

### Hotfix (Patch) Releases

When we find a _regression_ in Zed (a bug that wasn't present in an earlier version), or find a significant bug in a newly-released feature, we typically publish a hotfix release. For these releases, we bump Zed's _patch_ version number.

### Server Deployments

Often, changes in the Zed app require corresponding changes in the `collab` server. At the currente stage of our copmany, we don't attempt to keep our server backwards-compatible with older versions of the app. Instead, when making a change, we simply bump Zed's _protocol version_ number (in the `rpc` crate), which causes the server to recognize that it isn't compatible with earlier versions of the Zed app.

This means that when releasing a new version of Zed that has changes to the RPC protocol, we need to deploy a new version of the `collab` server at the same time.

## Instructions

### Publishing a Minor Release

1. Announce your intent to publish a new version in Discord. This gives other people a chance to raise concerns or postpone the release if they want to get something merged before publishing a new version.
1. Open your terminal and `cd` into your local copy of Zed. Checkout `main` and perform a `git pull` to ensure you have the latest version.
1. Run the following command, which will update two git branches and two git tags (one for each release channel):

   ```
   script/bump-zed-minor-versions
   ```

1. The script will make local changes only, and print out a shell command that you can use to push all of these branches and tags.
1. Pushing the two new tags will trigger two CI builds that, when finished, will create two draft releases (Stable and Preview) containing `Zed.dmg` files.
1. Now you need to write the release notes for the Stable and Preview releases. For the Stable release, you can just copy the release notes from the last week's Preview release, plus any hotfixes that were published on the Preview channel since then. Some of the hotfixes may not be relevant for the Stable release notes, if they were fixing bugs that were only present in Preview.
1. For the Preview release, you can retrieve the list of changes by running this command (make sure you have at least `Node 18` installed):

   ```
   GITHUB_ACCESS_TOKEN=your_access_token script/get-preview-channel-changes
   ```

1. The script will list all the merged pull requests and you can use it as a reference to write the release notes. If there were protocol changes, it will also emit a warning.
1. Once CI creates the draft releases, add each release's notes and save the drafts.
1. If there have been server-side changes since the last release, you'll need to re-deploy the `collab` server. See below.
1. Before publishing, download the Zed.dmg and smoke test it to ensure everything looks good.

### Publishing a Patch Release

1. Announce your intent to publish a new patch version in Discord.
1. Open your terminal and `cd` into your local copy of Zed. Check out the branch corresponding to the release channel where the fix is needed. For example, if the fix is for a bug in Stable, and the current stable version is `0.63.0`, then checkout the branch `v0.63.x`. Run `git pull` to ensure your branch is up-to-date.
1. Find the merge commit where your bug-fix landed on `main`. You can browse the merged pull requests on main by running `git log main --grep Merge`.
1. Cherry-pick those commits onto the current release branch:

   ```
   git cherry-pick -m1 <THE-COMMIT-SHA>
   ```

1. Run the following command, which will bump the version of Zed and create a new tag:

   ```
   script/bump-zed-patch-version
   ```

1. The script will make local changes only, and print out a shell command that you can use to push all the branch and tag.
1. Pushing the new tag will trigger a CI build that, when finished, will create a draft release containing a `Zed.dmg` file.
1. Once the draft release is created, fill in the release notes based on the bugfixes that you cherry-picked.
1. If any of the bug-fixes require server-side changes, you'll need to re-deploy the `collab` server. See below.
1. Before publishing, download the Zed.dmg and smoke test it to ensure everything looks good.
1. Clicking publish on the release will cause old Zed instances to auto-update and the Zed.dev releases page to re-build and display the new release.

### Deploying the Server

1. Deploying the server is a two-step process that begins with pushing a tag. 1. Check out the commit you'd like to deploy. Often it will be the head of `main`, but could be on any branch.
1. Run the following script, which will bump the version of the `collab` crate and create a new tag. The script takes an argument `minor` or `patch`, to indicate how to increment the version. If you're releasing new features, use `minor`. If it's just a bugfix, use `patch`

    ```
    script/bump-collab-version patch
    ```

1. This script will make local changes only, and print out a shell command that you can use to push the branch and tag.
1. Pushing the new tag will trigger a CI build that, when finished will upload a new versioned docker image to the DigitalOcean docker registry.
1. Once that CI job completes, you will be able to run the following command to deploy that docker image. The script takes two arguments: an environment (`production`, `preview`, or `staging`), and a version number (e.g. `0.10.1`).

   ```
   script/deploy preview 0.10.1
   ```

1. This command should complete quickly, updating the given environment to use the given version number of the `collab` server.
