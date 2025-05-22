This contains the code for Zed's Vim emulation mode.

Vim mode in Zed is supposed to primarily "do what you expect": it mostly tries to copy vim exactly, but will use Zed-specific functionality when available to make things smoother. This means Zed will never be 100% vim compatible, but should be 100% vim familiar!

The backlog is maintained in the `#vim` channel notes.

## Testing against Neovim

If you are making a change to make Zed's behavior more closely match vim/nvim, you can create a test using the `NeovimBackedTestContext`.

For example, the following test checks that Zed and Neovim have the same behavior when running `*` in visual mode:

```rust
#[gpui::test]
async fn test_visual_star_hash(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_state("ˇa.c. abcd a.c. abcd").await;
    cx.simulate_shared_keystrokes(["v", "3", "l", "*"]).await;
    cx.assert_shared_state("a.c. abcd ˇa.c. abcd").await;
}
```

To keep CI runs fast, by default the neovim tests use a cached JSON file that records what neovim did (see crates/vim/test_data),
but while developing this test you'll need to run it with the neovim flag enabled:

```sh
cargo test -p vim --features neovim test_visual_star_hash
```

This will run your keystrokes against a headless neovim and cache the results in the test_data directory. Note that neovim must be installed and reachable on your $PATH in order to run the feature.


## Testing zed-only behavior

Zed does more than vim/neovim in their default modes. The `VimTestContext` can be used instead. This lets you test integration with the language server and other parts of zed's UI that don't have a NeoVim equivalent.
