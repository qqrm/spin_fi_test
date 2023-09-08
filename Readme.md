# Rust Parallel Computation

This project provides a mechanism for performing parallel computations on a given data set using Rayon. It involves an algorithm to process numbers based on the Collatz conjecture.

## Overview

Given a vector of numbers, the program can process each number in parallel or sequentially based on a specified threshold. The underlying algorithm follows a specific rule to reduce numbers and counts the steps to do so.

## Setup

### Prerequisites

- Rust (latest stable version recommended)
- Cargo (comes with Rust)

### Installation

1. Clone the repository:

```bash
git clone https://github.com/qqrm/spin_fi_test
```

2. Navigate to the project directory:

```bash
cd spin_fi_test
```

3. Build the project:

```bash
cargo build --release
```

## Running Tests

To run all the tests, use:

```bash
cargo test
```

For benchmarking, you can use:

```bash
cargo bench
```

## License

This project is licensed...
