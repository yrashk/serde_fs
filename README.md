# serde_fs

Serializes/deserializes Rust data types to and from the filesystem.

## Motivation

So, this project is basically about mapping JSON/YAML/TOML kind of structures
onto the filesystem. Why, would you ask? What is so bad about these formats
that it warranted writing this library?

These formats are fine for what they were designed for... more or less. However,
they have some shortcomings, especially when they are used in contexts such
as configuration files. Let's list some of them:

* They require tooling to access data in them. For JSON, that'd be [jq](https://stedolan.github.io/jq/)
  (awesome project, check it out!), for example. So, writing simple shell scripts
  is out of the question as this is no longer quite zero-tech.
* They are not exactly merge-friendly. Re-ordering, changing lines that are close by will
  cause merge conflicts.
* Handling of binary data. Sure, you can refer to binary files and read them separately, but
  there's no support of binary data as a first-class citizen for a good reason (human-editable
  configuration or markup files are text files)

Logically, the easiest way to access values from shell scripts is by reading a file in its
entirety. So, what if we just serialized our data as a set of directories & files? This is how
serde_fs was born.

## Project Status

This is an early version and API *and* the mappings might change before 1.0.

## Type-Filesystem Mapping

| Type                        | Mapping                                                                   |
|-----------------------------|---------------------------------------------------------------------------|
| Option<T>                   | No target file if `None`, target file if `Some`                           |
| (u,i)(8,16,32,64), f(32,64) | String representation of the number                                       |
| String/str                  | String itself                                                             |
| bool                        | "true" or "false" string                                                  |
| char                        | First character of the file                                               |
| ()                          | Empty file                                                                |
| tuple                       | Directory with files called `0`, `1`, ..                                  |
| Map                         | Directory with files/directories called after their keys                  |
| struct                      | Same as map                                                               |
| unit variant                | File with variant name                                                    |
| tuple variant               | Directory with `variant` file containing variant name, the rest as tuple  |
| struct variant              | Directory with `variant` file containing variant name, the rest as struct |

## Known drawbacks

* Copy-pasting examples is not as trivial, but this can be worked around by sharing patches/ed scripts instead.

## Issue Tracking

This project uses [SIT](http://sit-it.org) for managing and tracking issues independently from GitHub. You
can find brief instructions on sending issue updates to this repository in its [README](https://github.com/sit-it/sit#questions-bug-reports-etc).

## License

serde_fs is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.
