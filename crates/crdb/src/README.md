# CRDB: A conflict-free replicated database for code and markdown

Our goal is for this database to contain all the text inserted in Zed.

## Contexts

The database is divided into *contexts*, with each context containing a collection of *documents*.

### Contexts contain documents

These contexts and the documents are really just namespaces in a global table of document *fragments*. Each fragment is a sequence of one or more characters, which may or may not be visible in a given branch.

#### Documents with paths are files

Documents in a context can be associated with metadata. If a document is associated with a relative path, it represents a file. A context that contains files can be synchronized with a directory tree on the file system, much like a Git repository.

#### Conversations are also documents

Contexts can also be associated with conversations, which are special documents that embed other documents that represent messages. Messages are embedded via a mechanism called *transclusion*, which will be discussed further below.

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




https://zed.dev/zed -> The #zed channel.
https://zed.dev/zed/insiders -> The #zed/insiders channel.


## Versions

Our current buffer CRDT represents versions with version vectors. Every collaborator is assigned a unique replica id, and for each collaborator that has ever participated, there is an entry in the version vector mapping their replica id to an operation count.

With CRDB, our goal is to expand from an individual buffer to collections of multiple documents called *contexts*. Each context allows an arbitrary number of branches to be created, and each branch can be edited independently from other branches, with synchronization between branches performed at a time of the user's choosing, much like Git.

This raises some concerns about using vectors to represent versions, as the size of the vectors will grow linearly in the number of branches, even if portions of the vectors
