<!-- omit in TOC -->
# ttrackr

![CI](https://github.com/michaeldvr/ttrackr/workflows/CI/badge.svg)

A terminal app for tracking spent time working on tasks.

1. [About](#about)
2. [Installation](#installation)

## About

`ttrackr` is used to track spent time on any tasks that you are
doing. You provide a list of tasks that will be tracked, then mark it
as _started_ when you begin working on it, and mark it _stopped_
after you stop or finish working.

## Installation

Install using cargo:

```bash
cargo install ttrackr
```

or by cloning this repo:

```bash
git clone https://github.com/michaeldvr/ttrackr.git
cd ttrackr
cargo install --path .
```

If you don't have _sqlite3_ on your system, pass `--all-features`
flag when calling `cargo install` to include bundled SQLite.
