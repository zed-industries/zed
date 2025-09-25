# Zed Releases

Read about Zed's [release channels here](https://zed.dev/faq#what-are-the-release-channels).

## Wednesday Release Process

You will need write access to the Zed repository to do this.

Credentials for various services used in this process can be found in 1Password.

---

1. Checkout `main` and ensure your working copy is clean.

1. Run `git fetch && git pull` to ensure you have the latest commits locally.

1. Run `git fetch --tags --force` to forcibly ensure your local tags are in sync with the remote.

1. Run `./script/get-stable-channel-release-notes`.

   - Follow the instructions at the end of the script and aggregate the release notes into one structure.

1. Run `./script/bump-zed-minor-versions`.

   - Push the tags and branches as instructed.

1. Run `./script/get-preview-channel-changes`.

   - Take the script's output and build release notes by organizing each release note line into a category.
   - Use a prior release for the initial outline.
   - Make sure to append the `Credit` line, if present, to the end of the release note line.

1. Once release drafts are up on [GitHub Releases](https://github.com/zed-industries/zed/releases), paste both preview and stable release notes into each and **save**.

   - **Do not publish the drafts, yet.**

1. Check the release assets.

   - Ensure the stable and preview release jobs have finished without error.
   - Ensure each draft has the proper number of assets—releases currently have 10 assets each.
   - Download the artifacts for each release draft and test that you can run them locally.

1. Publish stable / preview drafts, one at a time.

   - Use [Vercel](https://vercel.com/zed-industries/zed-dev) to check the progress of the website rebuild.
     The release will be public once the rebuild has completed.

1. Publish the release email that has been sent to [Kit](https://kit.com).

   - Make sure to double-check that the email is correct before publishing.
   - We sometimes correct things here and there that didn't translate from GitHub's renderer to Kit's.

1. Build social media posts based on the popular items in stable.

   - You can use the [prior week's post chain](https://zed.dev/channel/tweets-23331) as your outline.
   - Stage the copy and assets using [Buffer](https://buffer.com), for both X and BlueSky.
   - Publish both, one at a time, ensuring both are posted to each respective platform.

1. Store any media created in the `Feature Media` directory in our Google Drive.
   Other team members may want to reference or use these later on.

## Patch Release Process

If your PR fixes a panic or a crash, you should cherry-pick it to the current stable and preview branches.
If your PR fixes a regression in recently released code, you should cherry-pick it to preview.

You will need write access to the Zed repository to do this:

---

1. Send a PR containing your change to `main` as normal.

1. Once it is merged, cherry-pick the commit locally to either of the release branches (`v0.XXX.x`).

   - In some cases, you may have to handle a merge conflict.
     More often than not, this will happen when cherry-picking to stable, as the stable branch is more "stale" than the preview branch.

1. After the commit is cherry-picked, run `./script/trigger-release {preview|stable}`.
   This will bump the version numbers, create a new release tag, and kick off a release build.

   - This can also be run from the [GitHub Actions UI](https://github.com/zed-industries/zed/actions/workflows/bump_patch_version.yml):
     ![](https://github.com/zed-industries/zed/assets/1486634/9e31ae95-09e1-4c7f-9591-944f4f5b63ea)

1. Once release drafts are up on [GitHub Releases](https://github.com/zed-industries/zed/releases), proofread and edit the release notes as needed and **save**.

   - **Do not publish the drafts, yet.**

1. Check the release assets.

   - Ensure the stable / preview release jobs have finished without error.
   - Ensure each draft has the proper number of assets—releases currently have 10 assets each.
   - Download the artifacts for each release draft and test that you can run them locally.

1. Publish stable / preview drafts, one at a time.
   - Use [Vercel](https://vercel.com/zed-industries/zed-dev) to check the progress of the website rebuild.
     The release will be public once the rebuild has completed.

## Nightly release process

In addition to the public releases, we also have a nightly build that we encourage employees to use.
Nightly is released by cron once a day, and can be shipped as often as you'd like.
There are no release notes or announcements, so you can just merge your changes to main and run `./script/trigger-release nightly`.
