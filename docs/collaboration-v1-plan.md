# Collaboration V1

## Logging In

Zed needs to know the identities of the people who are collaborating on a worktree. So the first
time that I share a worktree (or try to join someone else's worktree), Zed will prompt me to
log in to `zed.dev`.

* For simplicity, I can begin sharing by clicking `File > Share` in the application menu.
* To initiate the share, Zed needs a user id and auth token that identifies me.
* Zed checks if it has stored credentials in the file `~/Library/Application Support/Zed/auth.toml`.
  If there is *no* stored credentials, then the user needs to log in. For now, we'll do this through
  the `zed.dev` website, for two reasons:
  - To avoid building complex login UI in Zed (for now)
  - So that Zed users can take advantage of being logged into GitHub in their browser.
* Zed needs a way to track that the user has logged in using their web browser. To do this,
  it makes an API request to the `zed.dev` server for a new "login token" (`POST zed.dev/api/login-tokens`).
* The server generates a unique 40-character `login_token` and stores it in its database.
* The server responds with this `login_token`, and Zed stores it in memory.
* Zed opens a new tab in my browser. The URL is `zed.dev/login`, and the `login_token` is included as a URL
  query parameter. Meanwhile, in the application, Zed displays a modal dialog that says "Please log in".
* For now, `zed.dev` only supports login via GitHub. So this web page will redirect immediately to the first
  step of GitHub's [Web-application flow](https://docs.github.com/en/developers/apps/building-oauth-apps/authorizing-oauth-apps#web-application-flow).
* When I complete the GitHub authorization process, GitHub redirects my browser to a `zed.dev` URL that
  includes the same `login_token` from before, providing a secret `code`. The Zed server completes the Oauth flow, exchanging this `code` for a GitHub `access_token`. It updates its database:
    * Creating or updating a user record for me with the given GitHub data and GitHub `access_token`
    * Marking the `login_token` as complete, and associating it with my user record.
* In Zed, I dismiss the "Please log in" dialog.
* Zed asks the server what happened with the login (`GET zed.dev/api/login-tokens/<the-token>`)
* The server responds with my user credentials
* Zed stores these credentials in `~/Library/Application Support/Zed/auth.toml`

Once Zed has my credentials, I can begin collaborating.

## Sharing

I may or may not have shared this worktree before. If I have shared it before, Zed will have saved a `worktree_id` for this
worktree in `~/Library/Application\ Support/Zed/worktrees.toml` (or something like that).

## Sharing UI

* For each worktree that I edit in Zed, there is a *Share* button that I can click to turn *sharing*
  on or off for that worktree.
* For each worktree that I share, Zed shows me a URL that I can give to others to let them
  collaboratively edit that worktree.
* __Question__ - Does the sharing on/off state of each worktree persist across application restart?
  When I close Zed while sharing a worktree, should I resume sharing when I reopen Zed?
    * **Pro** - This would remove friction from teams collaborating continuously.
    * **Cons** - I might have added something secret to the worktree since I last opened Zed. Could we detect
      changes that have occured outside of Zed, and avoid auto-sharing on startup when that has
      happened?

## Sharing Semantics

* While sharing, the entire state of my worktree is replicated and stored forever on the Zed server.
  Other collaborators can freely read the last state of my worktree, even after I've quit Zed.
* __Potential Scope Cut__ - For now, we won't store the history locally, as this isn't needed for  
  collaboration. Later, we may explore keeping a partial history locally as well, to support using
  the history while offline. A local history would allow:
    * Undo after re-opening a buffer.
    * Avoiding redundant uploads when re-opening a buffer while sharing.

* When I begin sharing:
    * Immediately, I upload a list of all the paths in my worktree, along a digest of each path
    * The server responds with a list of paths that needs
    * First, I upload the contents of all of my open buffers.
    * At this point, sharing has begun. I am shown a URL.
    * Asynchronously, I upload the contents of all other files in my worktree that the server needs.
* While I'm sharing:
    * Buffer operations are streamed to the Zed server, and to any peers that I'm collaborating with.
    * When FS changes are detected to files that I *don't* have open:
        * I again upload to the server a list of the paths that changed and their new digests.
        * The server responds with a list of paths that it needs
        * Asynchronously, I upload the new contents of these paths.
    * If a peer requests to open one of my files that I haven't yet asynchronously uploaded, then
      the server tells me to upload the contents of that file immediately.
* When I stop sharing:
    * I immediately stop uploading anything to the Zed server.

* __Question__  - If, while sharing, I undo an operation that I performed while *not* sharing, what
 information do I need to send to the server?
    * Can we transmit the operation as a new `Edit`, instead of as an `Undo`, so that the server can see
      the details of the operation? Will this be guaranteed to converge, since there can't have been any
      operations concurrent with the undone operation?

### Further Improvements

* When we add a local persisten history of our worktree, we will be able to
  avoid uploading entire snapshots of files that have changes since our last sharing session.
  Instead, the server can report that last version vector that it has seen for a file,
  and we can use that to construct a diff based on our history.

## RPC implementation details

Every client will have a single TCP connection to `zed.dev`.

The API will consist of resources named with URL-like paths, for example: `/worktrees/1`.

You'll be able to communicate with any resource in the following ways:

* `send`: A "fire-and-forget" message with no reply. (We may not need this)
* `request`: A message that expects a reply message that is tagged with the same sequence number as the request.
* `request_stream`: A message that expects a series of reply messages that are tagged with the same sequence number as the request. Unsure if this is needed beyond `subscribe`.
* `subscribe`: Returns a stream that allows the resource to emit messages at any time in the future. When the stream is dropped, we unsubscribe automatically.

Any resource you can subscribe to is considered a *channel*, and all of its processing needs to occur on a single machine. We'll recognize channels based on their URL pattern and handle them specially in our frontend servers. For any channel, the frontend will perform a lookup for the machine on which that channel exists. If no machine exists, we'll select one. Maybe it's always the frontend itself?. If a channel already exists on another server, we'll proxy the connection through the frontend and relay and broadcasts from this channel to the client.

The client will interact with the server via a `api::Client` object. Model objects with remote behavior will interact directly with this client to communicate with the server. For example, `Worktree` will be changed to an enum type with `Local` and `Remote` variants. The local variant will have an optional `client` in order to stream local changes to the server when sharing. The remote variant will always have a client and implement all worktree operations in terms of it.

```rs
enum Worktree {
    Local {
        remote: Option<Client>,
    }
    Remote {
        remote: Client,
    }
}

impl Worktree {
    async fn remote(client, id, cx) -> anyhow::Result<Self> {
        // Subscribe to the stream of all worktree events going forward
        let events = client.subscribe::<WorktreeEvent>(format!("/worktrees/{}", worktree_id)).await?;
        // Stream the entries of the worktree
        let entry_chunks = client.request_stream()

        // In the background, populate all worktree entries in the initial stream and process any change events.
        // This is similar to what we do 
        let _handle = thread::spawn(smol::block_on(async move {
            for chunk in entry_chunks {
                // Grab the lock and fill in the new entries
            }

            while let Some() = events.recv_next() {
                // Update the tree
            }
        }))

        // The _handle depicted here won't actually work, but we need to terminate the thread and drop the subscription
        // when the Worktree is dropped... maybe we use a similar approach to how we handle local worktrees.

        Self::Remote {
            _handle,
            client,
        }
    }
}
```