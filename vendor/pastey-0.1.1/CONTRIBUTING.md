# Contributing to Pastey

Thank you for your interest in contributing to Pastey! Here are some guidelines to help you get started.

## Project Goals

One of our main goals is to ensure that Pastey is a drop-in replacement for the paste crate. It should
not change the behavior or names of existing modifiers. For new behavior, please create a new modifier.

For this we have a special test crate named [paste-compat](./paste-compat/), it is always appreciated
to add more test cases here that works on paste crate.

## Testing

Run `./test.sh` for testing your code

## How to Contribute

1. **Fork the repository**: Click the "Fork" button at the top right of the repository page.
2. **Clone your fork**:

    ```sh
    git clone https://github.com/your-username/pastey.git
    cd pastey
    ```

3. **Create a new branch**:

    ```sh
    git checkout -b my-feature-branch
    ```

4. **Make your changes**: Implement your feature or bug fix.
5. **Commit your changes**:

    ```sh
    git commit -am 'Add new feature'
    ```

6. **Push to your branch**:

    ```sh
    git push origin my-feature-branch
    ```

7. **Create a Pull Request**: Go to the repository on GitHub and click "New Pull Request".

## Code Style

Please follow the existing code style and conventions. Ensure your code is well-documented and tested.

## Reporting Issues

If you find a bug or have a feature request, please create an issue on GitHub. Provide as much detail as
possible to help us understand and address the issue.

## License

By contributing to Pastey, you agree that your contributions will be licensed under the same license as the project: either the Apache License, Version 2.0 or the MIT license.

Thank you for your contributions!
