# CRDB: A conflict-free replicated database for code and markdown

Our goal is for this database to contain all the text inserted in Zed.

## Contexts

The database is divided into *contexts*, with each context containing a collection of *documents*.

### Contexts contain documents

These contexts and the documents are really just namespaces in a global table of document *fragments*. Each fragment is a sequence of one or more characters, which may or may not be visible in a given branch.

#### Documents with paths are files

Documents in a context can be associated with metadata. If a document is associated with a relative path, it represents a file. A context that contains files can be synchronized with a directory tree on the file system, much like a Git repository.

#### Conversations are also documents

Contexts can also be associated with conversations, which are special documents that embed other documents that represent messages. Messages are embedded via a mechanism called *portals*, which will be discussed further below.

### Contexts occupy a hierarchical namespace

For example, at genesis, zed.dev will contain the following channels:

#zed
    - This is where people get oriented about what Zed is all about. We'll link to it from our landing page.
#zed/staff
    - Here's where we talk about stuff private to the company, and host company-specific files.
#zed/insiders
    - Users we've worked with.
#zed/zed
    - This contains the actual source code for Zed.
    - It also has a conversation where potential contributors can engage with us and each other.
#zed/zed/debugger
    - A subcontext of zed/zed where we talk about and eventually implement a debugger. Associated with a different branch of zed/zed where the debugger is being built, but could also have multiple branches. Branches and contexts are independent.

## Versions

Our goal is for each context to allow an arbitrary number of branches to be created, where each branch can be edited independently from other branches. Synchronization between branches can be deferred to a time of the user's choosing, like Git, or branches can be kept in sync in real time. Every branch is associated with a version.

A version identifies a subset of all operations that have been performed in a context. If we start from an empty context and applied the operations in this subset in any causally valid order, we arrive at the unique state of the context described by the version.

We use versions to query the database. Since we're storing every operation, it isn't feasible to always load every operation in a context into a given replica.

Instead, we can query the database to return a snapshot of how it appears at a specific version. To do that, given the sequence of fragments, we need to efficiently query all fragments within that sequence that are visible at a given version.

* We want to exclude fragments from subsequent and concurrent versions.
* We want to include fragments introduced before the given version, but only those that are still visible.

We maintain a B-tree index for all fragments.

When querying this B-tree, we only want to descend into subtrees that contain at least one fragment whose insertion causally precedes the target version. But based on our hypothesis that hidden fragments will tend to cluster, we want to avoid descending into nodes for which all the fragments in question are invisible at the current version.

To support descending into nodes that contain operations from a current version, we index the minimal set of concurrent versions that causally precede all fragments in each subtree. Put another way: If a node only contains fragments that were concurrent or subsequent to our target version, we can skip it.

How can we skip nodes that only contain fragments that were hidden before our version?

Here's an idea I'm still thinking through:

For each subtree, we maintain the following version sets in its summary:

I'm wondering if the fragment summary can contain a history of versions at which the first fragment in the sequence appears or the last fragment in the sequence is hidden.

Then, when combining fragments, we combine these summaries, producing a new history in which the first fragment is introduced or all fragments are hidden. Assuming we have this summary, we can use it to determine if a node contains any visible fragments in a given version.

But not quite sure how to produce this summary yet.

For one fragment, every time it becomes hidden or visible we would add an entry to this history. How do we combine two histories?
           v0          v1          v2          v3
History A: show first, hide last
History B:                         show first, hide last
Combined:  v0: show first, v1: hide last, v2: show first, v3: hide last
