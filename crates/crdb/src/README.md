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
