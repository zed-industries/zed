# Rodio Development Guide

## Quick Start

1. Clone the repository: `git clone https://github.com/RustAudio/rodio.git`
2. Navigate to the project: `cd rodio`
3. Build the project: `cargo build`

## Project Structure

src/:
- `source/`: Audio source implementations
- `decoder/`: Audio format decoders
- `sink/`: Audio playback and mixing
- `dynamic_mixer/`: Real-time audio mixing
- `spatial/`: Spatial audio capabilities

## Coding Guidelines

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for formatting
- Implement `Source` trait for new audio sources
- Use `Player` for playback management

## Common Tasks

### Adding a New Audio Source or Effect

1. Create a new file in `src/source/`
2. Implement the `Source` trait to define how audio samples are generated or modified
3. Consider implementing sources like oscillators, noise generators, or effects like amplification, filtering, or distortion
4. If your contribution creates sound you should give it a public (factory) function that constructs it. If its an effect then add a method with default implementation for it in the `Source` trait.
5. Begin with a test for your new feature (see the [Testing](#testing)). This approach ensures your PR is ready and simplifies development. Don't worry about optimizing initially; focus on functionality.
6. Once your feature works, celebrate your progress! 🎉 Open a draft PR at this stage - we're here to assist with refactoring and optimization.
7. Refactor your code, add benchmarks, and work on improving performance, especially for real-time processing in effects. Refer to the [Rust Performance Book](https://nnethercote.github.io/perf-book/introduction.html) for optimization techniques.
8. Finally add some documentation and an example. For details see [Documentation]($documentation)
9. If you're unsure about creating tests, implement your feature first, then open a PR with what you have asking for guidance. We're happy to help!

### Implementing a New Decoder

1. Add new module in `src/decoder/`
2. Implement necessary traits (e.g., `Decoder`) to handle specific audio formats
3. Focus on efficiently parsing audio file headers and decoding compressed audio data
4. Update `src/decoder/mod.rs` to integrate the new decoder

### Unit Tests

- Feel free to write temporary unit tests during development if they help you verify functionality
- These tests can be rough and don't need to be comprehensive - they're just development aids
- It's okay to include these temporary unit tests in your pull request
- We'll remove these tests before merging into the main codebase, primarily because:
  - They can make refactoring more difficult as tests may break with code changes
  - Rust's robust type system reduces the need for extensive unit testing compared to dynamically typed languages

### Integration Tests

When possible, add integration tests for your new features. Keep in mind:

- Integration tests do not create sound through your speakers. Instead, you write code that examines the produced *samples*.
- For new audio sources:
  - Verify that samples have changed from their initial state.
  - Check if samples are non-zero where appropriate.
  - Look for expected patterns or characteristics in the audio data.
- Examples of integration tests:
  - The `tests/wav_test.rs` test simply checks if the decoder produces nonzero samples.
  - The test `seek_does_not_break_channel_order` in `tests/seek.rs` uses a beeping sound that alternates between two channels. It seeks to a point where we know only the second channel should make sound. Then we check if the first channel is silent while the second is not.
- Be aware that many aspects of audio processing are challenging to verify automatically.
- It's impossible to write a test that checks if something sounds "good". For features requiring audible verification:
  - Create an example in the `examples/` directory that demonstrates the functionality.
  - These examples can produce sound for manual testing.
  - Document the expected behavior in the example's comments.
- We love integration tests but they can be hard to write. If you have trouble adding one its fine to leave it out. If you do add one create a new file for it in `tests/`.
- Run tests: `cargo test`
- Run examples: `cargo run --example <example_name>`

## Documentation

- Add inline documentation to all public items.
- Look at the [documenting components](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html#documenting-components) section of the rustdoc book.
- Generate docs: `cargo doc --open`
- Add an example. That could be as part of the inline documentation or a more complex scenario in `examples/`. The example should not use `unwrap` or `expect` but return `Box<dyn Error>` and use `?`

## Contribution Workflow

1. Fork the repository on GitHub
2. Clone your fork locally: `git clone https://github.com/YOUR_USERNAME/rodio.git`
3. Create a feature branch: `git checkout -b feature/your-feature-name`
4. Make changes and add tests where applicable (Dont be afraid to ask for help)
5. Commit your changes following these guidelines: (`git commit`)
  - Write clear, concise commit messages
  - Limit the first line to 50 characters
  - Provide a detailed description after a blank line, if necessary
  - Reference relevant issue numbers (e.g., "Fixes #123")
  - Separate logical changes into multiple commits
  - Avoid commits with unrelated changes
  Example:
  ```
  Add spatial audio support for stereo sources

  - Implement SpatialSource struct
  - Add panning and distance attenuation
  - Update documentation for spatial audio usage

  Fixes #456
  ```
6. Push your changes to your fork: `git push origin feature/your-feature-name`
7. Create a pull request on GitHub

## Getting Help / Got a question?

- Open an issue on GitHub
- Ask questions in your pull request
- Open an issue for guidance/questions
- Join the Rust Audio Discord

## Useful Commands

- Format: `cargo fmt` - Automatically formats the code according to Rust style guidelines
- Lint: `cargo clippy` - Runs the Clippy linter to catch common mistakes and improve code quality
- Benchmark: `cargo bench` - Executes performance benchmarks for the project

For more detailed information, refer to the full documentation and source code.

## Useful External Resources

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Rustdoc book](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/introduction.html)
- [Rust Audio Discord](https://discord.com/invite/8qW6q2k)


## Disclaimer

Please note that the guidelines and practices outlined in this document
are not strict rules. They are general recommendations to promote
consistency and quality in contributions.

We understand that every situation is unique and encourage contributors
to use their best judgment. If you have any doubts or questions about
how to approach a particular task or contribution, don't hesitate to
reach out to the maintainers for guidance.

# Guidelines for Maintainers

Guidelines for those with write access to rodio. Adhere to them as long as makes
sense. This is a work in progress, more might follow as we get to know
what works. 

Please feel free to open an issue and discuss these if you have a suggestion.

Do not merge your own code to main, unless of course its a trivial change.
For example spelling/grammar or fixing up a PR by someone else.

## Release Procedure

The project is built automatically by a GitHub action when a new revision is pushed to the master branch.
The crate is published by triggering `.github/workflows/publish.yml` GitHub action.
After the crate is successfully published a new version's git tag is created in the repository.

So to publish a new version
1. Run `cargo outdated -R` to ensure all dependencies are up to date
2. Update `version` field in `Cargo.toml`.
3. Push the changes to the `master` branch.
4. Wait until GitHub build job completes successfully.
5. [On the Actions page](https://github.com/RustAudio/rodio/actions) start `.github/workflows/publish.yml`.
