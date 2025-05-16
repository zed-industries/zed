# Zed Vector Store Context Server

This extension provides a Model Context Server for vector storage and semantic search, for use with the Zed AI assistant.

It adds several commands to the Assistant Panel:

- `/vector-create` - Create a new vector store
- `/vector-add` - Add vectors to a store
- `/vector-search` - Search for similar vectors

## Configuration

To use the extension, you can configure the vector store database path in your Zed `settings.json`:

```json
{
  "context_servers": {
    "vector-store-context-server": {
      "settings": {
        "database_path": "~/.config/zed/vector_stores"
      }
    }
  }
}
```

## Tools

This context server provides the following MCP tools:

### vector_store.create

Creates a new vector store with the specified name and dimensions.

```json
{
  "name": "my_store",
  "dimensions": 384
}
```

### vector_store.add

Adds a vector to a store with optional metadata.

```json
{
  "store_name": "my_store",
  "vector": [0.1, 0.2, 0.3],
  "metadata": {
    "text": "This is some text",
    "source": "document1",
    "tags": ["important", "reference"]
  }
}
```

### vector_store.search

Searches for similar vectors in a store.

```json
{
  "store_name": "my_store",
  "query_vector": [0.1, 0.2, 0.3],
  "limit": 5,
  "threshold": 0.7
}
```

### vector_store.list

Lists all available vector stores.

```json
{
  "filter": "my" // Optional filter string
}
```

## Resources

This context server also provides resources:

- `vector-stores://list` - Lists all available vector stores
- `vector-stores://{name}/info` - Gets information about a specific vector store

## Examples

Here's an example of using the vector store with the AI assistant:

1. Create a new store:
   ```
   I need to create a store for my document embeddings. Use vector_store.create to create a store named "documents" with 384 dimensions.
   ```

2. Add vectors:
   ```
   I've got a text embedding vector for a document. Add it to the "documents" store with this vector: [0.1, 0.2, 0.3, ...] and add metadata with the title "Document 1" and source "user manual".
   ```

3. Search for similar content:
   ```
   Search the "documents" store for content similar to this embedding: [0.15, 0.25, 0.35, ...]
   ```

## License

This extension is part of the Zed editor. 