<!-- 
PR AUTHOR INSTRUCTIONS; PLEASE READ.

Give your pull request an accurate and descriptive title. It should mention what component(s) or database driver(s) it touches.
Pull requests with undescriptive or inaccurate titles *may* be closed or have their titles changed before merging.

Fill out the fields below.

All pull requests *must* pass CI to be merged. Check your pull request frequently for build failures until all checks pass.
Address build failures by pushing new commits or amending existing ones. Feel free to ask for help if you get stuck.
If a failure seems spurious (timeout or cache failure), you may push a new commit to re-run it.

After addressing review comments, re-request review to show that you are ready for your PR to be looked at again.

Pull requests which sit for a long time with broken CI or unaddressed review comments will be closed to clear the backlog.
If this happens, you are welcome to open a new pull request, but please be sure to address the feedback you have received previously.

Bug fixes should include a regression test which fails before the fix and passes afterwards. If this is infeasible, please explain why.

New features *should* include unit or integration tests in the appropriate folders. Database specific tests should go in `tests/<database>`.

Note that unsolicited pull requests implementing large or complex changes may not be reviwed right away.
Maintainer time and energy is limited and massive unsolicited pull requests require an outsized effort to review.

To make the best use of your time and ours, search for and participate in existing discussion on the issue tracker before opening a pull request.
The solution you came up with may have already been rejected or postponed due to other work needing to be done first,
or there may be a pending solution going down a different direction that you hadn't considered.

Pull requests that take existing discussion into account are the most likely to be merged.

Delete this block comment before submission to show that you have read and understand these instructions.
-->

### Does your PR solve an issue?
Delete this text and add "fixes #(issue number)".

Do *not* just list issue numbers here as they will not be automatically closed on merging this pull request unless prefixed with "fixes" or "closes".

### Is this a breaking change?
Delete this text and answer yes/no and explain.

If yes, this pull request will need to wait for the next major release (`0.{x + 1}.0`)

Behavior changes _can_ be breaking if significant enough.
Consider [Hyrum's Law](https://www.hyrumslaw.com/):

> With a sufficient number of users of an API,  
> it does not matter what you promise in the contract:  
> all observable behaviors of your system  
> will be depended on by somebody.
