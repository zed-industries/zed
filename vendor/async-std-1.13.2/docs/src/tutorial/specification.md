# Specification and Getting Started

## Specification

The chat uses a simple text protocol over TCP.
The protocol consists of utf-8 messages, separated by `\n`.

The client connects to the server and sends login as a first line.
After that, the client can send messages to other clients using the following syntax:

```text
login1, login2, ... loginN: message
```

Each of the specified clients then receives a `from login: message` message.

A possible session might look like this

```text
On Alice's computer:   |   On Bob's computer:

> alice                |   > bob
> bob: hello               < from alice: hello
                       |   > alice, bob: hi!
                           < from bob: hi!
< from bob: hi!        |
```

The main challenge for the chat server is keeping track of many concurrent connections.
The main challenge for the chat client is managing concurrent outgoing messages, incoming messages and user's typing.

## Getting Started

Let's create a new Cargo project:

```bash
$ cargo new a-chat
$ cd a-chat
```

Add the following lines to `Cargo.toml`:

```toml
[dependencies]
futures = "0.3.0"
async-std = "1"
```
