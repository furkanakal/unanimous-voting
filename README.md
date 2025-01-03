# Unanimous Voting

## Overview

This project implements a private voting system using the Fully Homomorphic Encryption (FHE) library **TFHE-rs**. It demonstrates how to securely aggregate encrypted votes and determine if the vote outcome is unanimous, while preserving the privacy of individual votes.

Fully Homomorphic Encryption allows computations to be performed directly on encrypted data without needing decryption, ensuring complete confidentiality throughout the process. This project uses the homomorphic properties of TFHE to compare the number of votes to the sum of encrypted votes, enabling a secure unanimity check.

---

## Features

- **Confidential Voting:** Each vote is encrypted, ensuring individual vote privacy.
- **Homomorphic Aggregation:** The server computes the sum of all encrypted votes without decrypting them.
- **Unanimity Check:** The system checks if all votes are identical (unanimous) without revealing individual votes.
- **Cross-Platform Compatibility:** Designed to run on architectures like `x86_64` and `aarch64`.

---

## How It Works

1. **Key Generation:**
   - Client generates a pair of keys: a private `client_key` for encryption and decryption, and a `server_key` for performing homomorphic computations.

2. **Vote Encryption:**
   - Each vote (1 for yes, 0 for no) is encrypted using the `client_key`.

3. **Homomorphic Aggregation:**
   - The server aggregates all encrypted votes using FHE operations to calculate the total number of "yes" votes.

4. **Unanimity Check:**
   - The server compares the aggregated sum to the total number of voters (also encrypted) to determine if the result is unanimous.

5. **Result Decryption:**
   - The client decrypts the result to find out if the vote was unanimous.

---

## Prerequisites

- **Rust:** Ensure you have the latest Rust toolchain installed. You can download it from [rustup.rs](https://rustup.rs).

---

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-repo/tfhe-voting.git
   cd tfhe-voting
   ```

2. Install dependencies:
   ```bash
   cargo build
   ```

---

## Usage

1. Run the project:
   ```bash
   cargo run
   ```

2. The program will output whether the vote was unanimous:
   ```
   The vote is unanimous: True
   ```

---

## Applications

This project can be extended for:

- Secure electronic voting systems.
- Confidential decision-making processes.
- Privacy-preserving surveys.

---

## License

This project is licensed under the MIT License. See the LICENSE file for details.

---

## Acknowledgments

Special thanks to the [Zama](https://zama.ai) team for developing the **TFHE-rs** library.