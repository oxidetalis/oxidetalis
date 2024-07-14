# Contributing to oxidetalis

Thank you for your interest in contributing to oxidetalis! We welcome
contributions from the community to help improve the project.

## Reporting Issues

If you encounter any issues or bugs while using oxidetalis, please open a new
issue on the Forgejo repository. When reporting an issue, please provide as much
detail as possible, including steps to reproduce the issue and any relevant
error messages.

## Feature Requests

If you have a feature request or an idea for improving oxidetalis, we encourage
you to open a new issue on the Forgejo repository. Please describe the feature
or improvement in detail and provide any relevant context or examples.

## Developer Certificate of Origin (DCO)
Please note that all contributions to oxidetalis must be made under the terms of
the Developer Certificate of Origin (DCO). This is a legal statement that
certifies that you have the right to contribute the code and that you agree to
license it under the project's licenses. Please read the DCO carefully before
making a contribution, you can find the DCO here [DCO](./DCO).

To indicate that you agree to the terms of the DCO, you can add a
`Signed-off-by` line to your commit messages. This can be done by adding the
`-s` flag to the `git commit` command, for example:

```bash
git commit -s -m "feat: Add new feature"
```

> **Note**
>
> We will not accept contributions that do not include the `Signed-off-by` line
> in the commit message. We may ask you to re-submit your contribution with the
> `Signed-off-by` line if it is missing.

## Writing Code
Before you start writing code, please open a new issue first to discuss the
proposed changes. This will help ensure that your contribution is aligned with
the project's goals and that you are not duplicating work that is already in
progress or has been completed by someone else.

### Rust Version
In the oxidetalis project, we always try to stay on the lowest MSRV possible for
compatibility, but the development process relies on the nightly release to get
the latest rust-analyzer and rustfmt features.

You can check the nightly version used in the project in the `rust-toolchain`
file. And the MSRV in the `Cargo.toml` file.

### PR title
Your PR will squash and merge, and your PR title will be used as the commit
message. Please make sure your PR title is clear and concise.

The title must follow [Conventional Commits] format. This means that the title
should be in the following format:

```
<type>(<scope>): <description>
```

- The `<scope>` is optional, and the `<description>` should be a clear and
  concise summary of the changes.
- You should use the imperative, present tense (e.g., "Add feature" instead of
  "Added feature").
- The `<type>` should be one of the following:
  - `feat`: A new feature
  - `fix`: A bug fix
  - `docs`: Documentation changes
  - `refactor`: Refactoring code without changing its behavior
  - `change`: Changes that affect the meaning of the code
  - `deprecate`: Changes that deprecate a part of the code
  - `remove`: Changes that remove a deprecated part of the code
  - `security`: Changes that affect the security of the code
  - `perf`: A code change that improves performance
  - `test`: Adding missing tests or correcting existing tests
  - `chore`: Changes to the build process or auxiliary tools and libraries such
    as documentation generation

#### Example
```
- feat: something
- chore(ci): update MSRV
```

### PR description
Your PR description should provide a clear and concise summary of the changes
you have made. It should also include any relevant context or background
information that will help the project maintainers understand the purpose of the
changes. Make sure to reference the issue that your PR is addressing, and note
any breaking changes that your PR introduces.

Make sure to explain why you made the changes not just what changes you made.

### Code Style

Please follow the existing code style and conventions used in the oxidetalis
project. This includes:

- Using Rust's official formatting tool, `rustfmt`, to format your code.
- Writing clear and concise code with meaningful variable and function names.
- Adding comments to explain complex logic or algorithms.

### CI
Run the CI before submitting your code. You can run the CI with the following
command:

```bash
just ci
```

This will run the tests and check the code formatting. If the CI fail, please
fix the issues before submitting your code.

## Code Review

All contributions to oxidetalis will go through a code review process. This
ensures that the code meets the project's standards and maintains its quality.
Please be open to feedback and suggestions from the project maintainers during
the code review process.

## Maintainers
As a maintainer, after approving a PR, you need to pull the PR locally and then merge it manually.

Why? Because Forgejo add some trailers to the commit message, so you need to merge it manually.

### Pull the PR
```bash
# Replace {{pr-number}} with the PR number
git fetch origin +refs/pull/{{pr-number}}/head:refs/heads/pr-{{pr-number}}
```

### Merge the PR

There is important trailers you must add it to the commit message, which is:
- `Signed-off-by: Name <email>`: The name and email of you, the maintainer.
- `Reviewed-on: pr-url`: The URL of the PR on Forgejo.
- `Reviewed-by: Name <email>`: The name and email of the reviewer, can be you and other reviewers (each reviewer in a new line).
- `Co-committed-by: Name <email>`: The name and email of the PR author.
- `Reported-by: Name <email>`: The name and email of the person who reported the issue (if any bug).
- `Suggested-by: Name <email>`: The name and email of the person who suggested the feature (if any feature).
- `Fixes: issue-url`: The URL of the issue that the PR fixes (if any).

Things to note:
- You can add more than one `Reviewed-by` trailer.
- You need to make sure to not commit the changes as author, you need to commit it as the author of the PR. you are the committer, not the author.
- The commit subject must be in the [Conventional Commits] format. To generate the changelog correctly.

```bash
# Replace {{pr-number}} with the PR number
git merge --squash pr-{{pr-number}}
git commit -s --author "Name <email>" -m "feat: Add new feature

Reviewed-on: https://git.4rs.nl/OxideTalis/oxidetalis/pulls/{{pr-number}}
Reviewed-by: Name <email>
Co-committed-by: Name <email>
Reported-by: Name <email>
"
```

### Push the changes
```bash
git push origin master
```

### Mark the PR as merged manually
Copy the full commit hash and mark the PR as merged manually on Forgejo.

That's it! You have merged the PR manually. Thank you for your contribution!

## License

By contributing to oxidetalis, you agree that your contributions will be
licensed under the project's licenses. Each crate in the project has its own
license, so please make sure you are aware of the license terms before making a
contribution.

Happy contributing!

[Conventional Commits]: https://www.conventionalcommits.org/en/v1.0.0/
