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

RustCrypt provides a simple CLI for generating cryptographic outputs based on high-entropy randomness.

---

### Generate output

You can generate a 32-character or 64-character cryptographic hash.

#### version info
```bash
rustcrypt version

Example Output:
rustcrypt 0.1.0
Safe, Fast, Cryptographic.
```

---

#### 32-character output
```bash
rustcrypt 32

Example Output:
5ee56aa15401224d21694ae4740e1e8e
```

---

#### 64-character output
```bash
rustcrypt 64

Example Output:
b655bd72b3ab161417ea672f833c5eb53245f471033b9adee3e0e1934702a0e6
```

---

#### JSON output
```bash
rustcrypt 32 json
{
  "output": "5ee56aa15401224d21694ae4740e1e8e",
  "length": 32
}
```

or

```bash
rustcrypt 64 json
{
  "output": "b655bd72b3ab161417ea672f833c5eb53245f471033b9adee3e0e1934702a0e6",
  "length": 64
}
```
