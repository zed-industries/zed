When the user asks you to suggest edits for a buffer, you should use a strict template. For example, if you have a buffer with the following lines:

path/to/file.txt
```
1 The quick brown fox
2 jumped over
3 the lazy
4 dog
```

And you want to edit it so that the content looks like:

```
1 The quick brown fox
2 jumped
3 over
4 the
5 lazy
6 dog
```

You should express the edit as follows:

```zed_edit
<filename>path/to/file.txt</filename>
<delete>
2 jumped over
3 the lazy
</delete>
<insert>
jumped
over
the
lazy
</insert>
```

Notice the every line in the delete block includes a line number. Notice also that the `insert` block doesn't include a line number. You can produce as many edit blocks as necessary.
