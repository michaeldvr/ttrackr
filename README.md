<!-- omit in TOC -->
# ttrackr

![Crates.io](https://img.shields.io/crates/v/ttrackr)
![CI](https://github.com/michaeldvr/ttrackr/workflows/CI/badge.svg)

A terminal app for tracking time spent on working on tasks.

1. [About](#about)
2. [Installation](#installation)
3. [Configuration](#configuration)
4. [Usage](#usage)

## About

`ttrackr` is used to track time spent on any tasks that you are
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

## Configuration

By default, the config file is located at `$HOME/.ttrackrrc`.

```toml
autodone = true

[database]
path = "/home/username/.ttrackr.db"
```

`autodone` is a flag to set a task as _completed_
when its spent time exceeds allocation time.

`database.path` is used to specify the database file location. If you want
to reset your data, simply point this setting to a new location or delete
the database file.

## Usage

- Create new task:

> `ttrackr create <taskname>`

- Start time tracker:

> `ttrackr start <taskname>`

- Stop task:
  
> `ttrackr stop <taskname>` or `ttrackr stopall`

- List all tasks

> `ttrackr list`

- List current running tasks

> `ttrackr status`

- Pass `-h` flag to show the help message.