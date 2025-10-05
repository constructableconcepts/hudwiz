# Contributing to hudwiz

Thank you for your interest in contributing to hudwiz! We welcome all contributions, from bug reports to new features.

## Getting Started

1.  **Fork the repository** on GitHub.
2.  **Clone your fork** to your local machine.
3.  **Create a new branch** for your changes: `git checkout -b feat/my-new-feature`.

## Development Process

-   **Frontend:** The frontend is a Yew application written in Rust. You will need to have the `wasm-pack` tool installed.
-   **Backend:** The backend server is also written in Rust.
-   **Building:** The entire application can be built and tested by running the `scripts/run_system_test.sh` script. This script handles all the necessary build steps and runs a verification test.

## Submitting a Pull Request

1.  **Push your changes** to your fork on GitHub.
2.  **Create a new Pull Request** from your fork to the main `hudwiz` repository.
3.  **Provide a clear description** of your changes in the Pull Request.
4.  **Ensure all tests are passing.** The `run_system_test.sh` script is your source of truth.

We look forward to your contributions!