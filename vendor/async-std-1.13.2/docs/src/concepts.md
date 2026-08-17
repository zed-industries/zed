# Async concepts using async-std

[Rust Futures][futures] have the reputation of being hard. We don't think this is the case. They are, in our opinion, one of the easiest concurrency concepts around and have an intuitive explanation.

However, there are good reasons for that perception. Futures have three concepts at their base that seem to be a constant source of confusion: deferred computation, asynchronicity and independence of execution strategy.

These concepts are not hard, but something many people are not used to. This base confusion is amplified by many implementations oriented on details. Most explanations of these implementations also target advanced users, and can be hard for beginners. We try to provide both easy-to-understand primitives and approachable overviews of the concepts.

Futures are a concept that abstracts over how code is run. By themselves, they do nothing. This is a weird concept in an imperative language, where usually one thing happens after the other - right now.

So how do Futures run? You decide! Futures do nothing without the piece of code _executing_ them. This part is called an _executor_. An _executor_ decides _when_ and _how_ to execute your futures. The `async-std::task` module provides you with an interface to such an executor.

Let's start with a little bit of motivation, though.

[futures]: https://en.wikipedia.org/wiki/Futures_and_promises
