# Collaboration V1

### Sharing UI

* For each worktree that I edit in Zed, there is a *Share* button that I can click to turn *sharing*
  on or off for that worktree.
* For each worktree that I share, Zed shows me a URL that I can give to others to let them
  collaboratively edit that worktree.
* __Question__ - Does the sharing on/off state of each worktree persist across application restart?
  When I close Zed while sharing a worktree, should I resume sharing when I reopen Zed?
    Pros:
    * This would remove friction from teams collaborating continuously.
    Cons:
    * I might have added something secret to the worktree since I last opened Zed. Could we detect
      changes that have occured outside of Zed, and avoid auto-sharing on startup when that has
      happened?

### Sharing Semantics

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