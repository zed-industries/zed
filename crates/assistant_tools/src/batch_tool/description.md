Invoke multiple other tool calls either sequentially or concurrently.

This tool is useful when you need to perform several operations at once, improving efficiency by reducing the number of back-and-forth interactions needed to complete complex tasks.

If the tool calls are set to be run sequentially, then each tool call within the batch is executed in the order provided. If it's set to run concurrently, then they may run in a different order. Regardless, all tool calls will have the same permissions and context as if they were called individually.

This tool should never be used to run a total of one tool. Instead, just run that one tool directly. You can run batches within batches if desired, which is a way you can mix concurrent and sequential tool call execution.

When it's possible to run tools in a batch, you should run as many as possible in the batch, up to a maximum of 32. For example, don't run multiple consecutive batches of 10 when you could instead run one batch of 30.
