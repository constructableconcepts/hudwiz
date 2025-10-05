# Contributing to hudwiz

First off, thank you for considering contributing to `hudwiz`! We welcome any and all contributions, from bug reports to feature requests and code submissions.

This document provides a set of guidelines to help you contribute to the project in a way that is consistent and effective.

## Table of Contents
1.  [Code of Conduct](#1-code-of-conduct)
2.  [How Can I Contribute?](#2-how-can-i-contribute)
    -   [Reporting Bugs](#21-reporting-bugs)
    -   [Suggesting Enhancements](#22-suggesting-enhancements)
    -   [Submitting Pull Requests](#23-submitting-pull-requests)
3.  [Development Setup](#3-development-setup)
4.  [Style Guides](#4-style-guides)
    -   [Git Commit Messages](#41-git-commit-messages)
    -   [Rust Style Guide](#42-rust-style-guide)

---

## 1. Code of Conduct
This project and everyone participating in it is governed by the [hudwiz Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to the project maintainers.

## 2. How Can I Contribute?

### 2.1. Reporting Bugs
If you find a bug, please open an issue on our GitHub repository. When filing a bug report, please include the following:
-   A clear and descriptive title.
-   A detailed description of the problem, including steps to reproduce it.
-   The expected behavior and what actually happened.
-   Any relevant screenshots or error messages.

You can use the following template to help you create your bug report:

<details>
<summary>Bug Report Template</summary>

```markdown
**Describe the bug**
A clear and concise description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Go to '...'
2. Click on '....'
3. Scroll down to '....'
4. See error

**Expected behavior**
A clear and concise description of what you expected to happen.

**Screenshots**
If applicable, add screenshots to help explain your problem.

**Desktop (please complete the following information):**
 - OS: [e.g. iOS]
 - Browser [e.g. chrome, safari]
 - Version [e.g. 22]

**Additional context**
Add any other context about the problem here.
```
</details>

### 2.2. Suggesting Enhancements
If you have an idea for a new feature or an enhancement to an existing one, we'd love to hear it. Please open an issue on GitHub with the following:
-   A clear and descriptive title.
-   A detailed description of the proposed enhancement and why it would be useful.
-   Any mockups or examples that might help illustrate your idea.

You can use the following template to help you create your feature request:

<details>
<summary>Feature Request Template</summary>

```markdown
**Is your feature request related to a problem? Please describe.**
A clear and concise description of what the problem is. Ex. I'm always frustrated when [...]

**Describe the solution you'd like**
A clear and concise description of what you want to happen.

**Describe alternatives you've considered**
A clear and concise description of any alternative solutions or features you've considered.

**Additional context**
Add any other context or screenshots about the feature request here.
```
</details>

### 2.3. Submitting Pull Requests
We welcome pull requests for bug fixes and new features. Before submitting a pull request, please ensure the following:
1.  **Open an Issue:** It's best to open an issue first to discuss the change you want to make.
2.  **Fork the Repository:** Fork the repository and create a new branch for your changes.
3.  **Follow the Style Guides:** Ensure your code adheres to the project's style guides (see below).
4.  **Run the Tests:** Make sure all tests pass before submitting your pull request.
    ```bash
    ./scripts/run_system_test.sh
    ```
5.  **Submit the Pull Request:** Push your changes to your fork and open a pull request against the `main` branch. Provide a clear description of the changes you've made.

## 3. Development Setup
To get started with `hudwiz` development, you will need to have the following tools installed:
-   [Rust](https://www.rust-lang.org/tools/install)
-   [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
-   [Node.js and npm](https://nodejs.org/en/download/)
-   [Docker](https://docs.docker.com/get-docker/)

Once you have these installed, follow these steps to set up your development environment:
1.  Clone the repository.
2.  Start the required services with `docker-compose up -d`.
3.  Build the project with `./scripts/run_system_test.sh`.

## 4. Style Guides

### 4.1. Git Commit Messages
-   Use the present tense ("Add feature" not "Added feature").
-   Use the imperative mood ("Move component to..." not "Moves component to...").
-   Limit the first line to 72 characters or less.
-   Reference issues and pull requests liberally after the first line.

### 4.2. Rust Style Guide
We follow the official Rust style guidelines. Please run `cargo fmt` before submitting your code to ensure it is formatted correctly.

Thank you for your contributions!