# RustCrypt

---

## Overview

RustCrypt is a lightweight cryptographic utility for generating secure, entropy-based keys and passwords.
Built with Rust for safety, speed, and cryptographic security.

---

## Features

- High-entropy random generation
- Secure cryptographic hashing pipeline
- Minimal CLI interface
- Zero unnecessary dependencies

---

## Philosophy

Safe. Fast. Cryptographic.

---

## Usage

Generate a cryptographic output from high-entropy randomness.

```bash
# 32-byte output
rustcrypt generate 32

# 64-byte output
rustcrypt generate 64

# version info
rustcrypt version
