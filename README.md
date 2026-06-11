# rust-cryptopals
# rust-cryptopals

Cryptopals cryptography challenges implemented in Rust — no external crypto libraries.

## Overview

This repository contains solutions to the [Cryptopals Crypto Challenges](https://cryptopals.com/), implemented from scratch in Rust. The goal is to build real cryptographic primitives and then break them, developing both implementation and attack intuition along the way.

---

## Set 1 — Basics

Foundational encoding and classical cipher attacks.

- **Challenge 1** — Hex to Base64
- **Challenge 2** — Fixed XOR
- **Challenge 3** — Single-byte XOR cipher: break by frequency analysis
- **Challenge 4** — Detect single-character XOR across a file
- **Challenge 5** — Implement repeating-key XOR
- **Challenge 6** — Break repeating-key XOR (key: *Terminator X: Bring the noise*)

---

## Set 2 — Block Crypto

AES-128 implemented from scratch (SubBytes, ShiftRows, MixColumns over GF(2⁸), AddRoundKey, key schedule), then attacked.

- **Challenge 11** — ECB/CBC detection oracle
- **Challenge 12** — Byte-at-a-time ECB decryption (simple)
- **Challenge 13** — ECB cut-and-paste attack

### Challenge 13 — ECB Cut-and-Paste

The oracle encrypts user profiles encoded as `email=...&uid=10&role=user` under a fixed AES-128-ECB key.

**Attack:**
1. Craft an email of 13 bytes so that `email=AAAAAAAAAAAAA&uid=10&role=` aligns exactly to the first two 16-byte blocks.
2. Craft a second email of 10 bytes followed by `admin` padded to a full block with PKCS#7 (`\x0b` × 11), so the padded `admin` block lands at bytes 16–31.
3. Take the first two blocks from ciphertext (1) and append block 1 from ciphertext (2).
4. The forged ciphertext decrypts to a valid profile with `role=admin`.

---

## Structure

```
src/
├── main.rs
├── aes_alg.rs      # AES-128 ECB encrypt/decrypt from scratch
└── set2/
    └── challenge13.rs
```

---

## Running

```bash
cargo test
```

---

## Notes

- AES-128 is implemented without any external crates — including GF(2⁸) multiplication for MixColumns.
- Challenges are validated against NIST test vectors where applicable.