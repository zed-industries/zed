# Glossary

### blocking

"blocked" generally refers to conditions that keep a task from doing its work. For example, it might need data to be sent by a client before continuing. When tasks become blocked, usually, other tasks are scheduled.

Sometimes you hear that you should never call "blocking functions" in an async context. What this refers to is functions that block the current thread and do not yield control back. This keeps the executor from using this thread to schedule another task.
