# Skywalker

⚠️ DO NOT RUN THIS SOFTWARE IF YOU DON'T UNDERSTAND WHAT IT DOES, RUNNING THIS SOFTWARE ON YOUR HARDWARE IS YOUR SOLE RESPONSIBILITY ⚠️

## Introduction

I'm a student in cybersecurity, this project aims to let users test themselves for private keys (ethereum and solana format) readable on your computer before "bad guys" find them.

It's best practice to encrypt your sensitive data, with strong and reliable encryption (see [encfs](https://linux.die.net/man/1/encfs) and [gpg](https://linux.die.net/man/1/gpg))

## Prerequesites

### Rust

Install [rust](https://www.rust-lang.org/) by running the following in a bash terminal

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Running Skywalker

### Clone the repository

```bash
git clone https://github.com/EsyWin/skywalker/
```

### Run

Move in the folder with `cd skywalker` and fire `cargo run`

This may take a while to get the results, as it scans **every single file on your computer**.
