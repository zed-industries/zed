---
title: Frequently Asked Questions
description: "Answers to common questions around publishing and maintaining extensions."
---

# Frequently Asked Questions {#faq}

Questions come up before, during, and after publishing an extension. Here are the ones we hear most often.

## How long will the review of my submission take? {#review-duration}

We do our best to get back to you in a reasonable time frame. However, we are very aware that this is currently not always the case - we sincerely apologize for that! We are continuously iterating on the process to provide every submission with feedback more quickly and with an overall better contribution experience.

That said, most submissions get their first feedback within a few weeks; for some extensions it might take us longer - up to one or two months. We know this is neither optimal nor a pleasant experience, and we are working on it (in fact, this very document is part of that effort to provide you with more context on our side of things).

Please note, though, that we currently cannot make any promises, and reviews might sadly take even longer for various reasons - the biggest one being the large backlog we are working through right now.

## Why was my PR closed? {#pr-closed}

If your PR was closed without much feedback, it severely violated the [publishing prerequisites](./prerequisites.md). Due to the high volume of submissions, we do not provide more context in such cases. Please make sure to read through the prerequisites again.

Furthermore, as stated in the [pull request rules](./publishing-guide.md#pull-request-rules), we consider submissions stale after **3 weeks of no response to maintainer feedback** and close them.

A PR closed due to staleness is not the end of the road: you may open a fresh PR at any time and we will take another look.

## Why is a response timeframe enforced on me, but not on you? {#response-timeframe}

We know this seems unfair, especially given our own response times - we are not proud of this either. But we do this in the interest of everybody: closing stale submissions keeps the review queue in a manageable state, which in turn helps us get to every submission sooner, including your next one.

## Why was I asked to open a new PR instead of continuing my closed one? {#fresh-prs}

Due to the high backlog, we prefer fresh PRs in some cases, as they keep the queue shorter. Experience has shown that fresh PRs tend to be in a more reviewable state, whereas updated PRs often claim to have addressed all feedback while actually being in a broken and unmergeable state. That in turn costs both reviewers and contributors valuable time.

A fresh PR is, in that sense, quicker to work with and more often than not an easier and faster merge.

## Why do you enforce so many strict prerequisites? {#why-prerequisites}

Our goal with the extension prerequisites is to strike a balance between an open extension ecosystem and a certain level of standard and quality: when you install an extension, you should be able to rest assured that it works well to a certain degree - without having to vet it yourself first.

The prerequisites also help us bundle effort. Multiple people maintaining multiple near-identical extensions, with users spread across all of them, creates a lot of churn for everyone involved. Guiding that effort into one place benefits owners, contributors, and users alike - and with only one extension per use case, you don't have to guess which one is the right one.

Of course, we know we will not catch everything with these - but they set a baseline everyone can rely on. And should an extension end up not working or unmaintained regardless, that is exactly what the policies further down this page are for.

## Do the prerequisites also apply to already published extensions? {#prerequisites-for-existing-extensions}

Yes. Existing extensions are not exempt from the prerequisites, and updates are held to the same standards as new submissions.

The only exception are the ID restrictions: since an extension's ID cannot change once published, existing IDs stay as they are.

## Do I have to maintain my extension? {#do-i-have-to-maintain-my-extension}

No. After your extension has been published, there are no further requirements from our side. Thank you for enriching the Zed extension collection, we really appreciate it!

That said, no extension is perfect on day one - bugs may surface and improvement requests may come in. While maintenance is never required, it helps everyone if you respond to those reports within a reasonable timeframe.

## I found a bug in an extension or want to improve it. What should I do? {#reporting-issues-and-improvements}

Report the issue or propose your improvement in the original extension repository first, rather than publishing a competing extension. Most owners appreciate reports and contributions, and keeping the effort in one place spares users from having to pick between near-identical extensions - as well as maintainers from having to review them.

If the owner does not respond for an extended period of time, see [What happens when an extension owner stops responding?](#unresponsive-owner)

## I no longer want to maintain my extension. What now? {#stepping-back}

Completely fine - priorities change, and stepping back from an extension is nothing to feel bad about. As the current owner, you can:

- transfer ownership of the repository to a new owner, or
- open an issue or pull request against [`zed-industries/extensions`](https://github.com/zed-industries/extensions) asking for the removal of your extension.

Note that you may also just keep your extension around - many extensions have received few to no updates and yet have many happy users.

## What happens when an extension owner stops responding to reported issues? {#unresponsive-owner}

We don't want published extensions to go stale, since that leaves their users with a poor experience. So if an extension owner can no longer be reached:

- a contributor may fork the extension and propose their fork as a replacement for the current extension, or
- Zed staff can fork the extension into the [`zed-extensions`](https://github.com/zed-extensions) organization, where maintenance continues as a joint effort of the community and Zed staff.

We will only take either action if one of the following requirements is met:

- the current owner has given written permission, or
- there is **written proof** of attempts to establish contact, and the owner has been unresponsive to them for **at least 6 weeks**.

Without one of these, the extension stays with its current owner as is.

Such a switch also does not have to be permanent: should the original owner become responsive again, the extension may be switched back to the original repository.

## Why can't my language reuse builtin grammars or a grammar from another extension? {#grammar-reuse}

If your language depended on a grammar it does not own, updates to that grammar could change the nodes produced by Tree-sitter parsing and silently break your language. To avoid this, every language must use a grammar defined in its own extension's `extension.toml`.

## I have questions about these policies or disagree with them. Where can I raise that? {#policy-discussion}

First of all, thank you for reading through these!

While all of these policies were added with good reason, not all of them are entirely set in stone - we value open discussion around all of this.

If anything here seems off to you or you have questions, feel free to open a discussion in the [`zed-industries/zed`](https://github.com/zed-industries/zed) repository and ping @MrSubidubi there - we would much rather talk it through than leave you frustrated.
