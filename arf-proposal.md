# Non-Unicode filenames, command-line args, and environment variables

## Status

This document is not yet an official WASI proposal. It may be in the
future, but for now it's just an experiment.

## Background

POSIX-ish platforms don't provide a way to reliably determine the encoding for filenames, command-line arguments, and environment variables. And on Windows, filenames, command-line arguments, and environment variables aren't guaranteed to be well-formed UTF-16.

However, applications often need to know the encoding to correctly display, compare, or serialize these names. POSIX provides a flock of environment variables, but for filenames, that assumes that the program is running with the same locale as the user which created the files, which isn't always true. There are also heuristics which can help guess at the encodings of filenames, however they're not reliable.

There isn't a great reason for imposing the resulting complexity on applications, other than POSIX simply predating Unicode and UTF-8. In practice, many applications end up assuming that the inputs are UTF-8 anyway.

For WASI, rather than forever insisting that such applications are at fault, it's preferable to define APIs so that the things applications want to do are straightforward.

## Proposal

All WASI filenames, command-line arguments, and environment variables are valid UTF-8 strings, using the same definition as the [wasm core spec](https://webassembly.github.io/spec/core/binary/values.html#binary-utf8).

When the host environment has strings, in particular filenames, which aren't a known and well-formed Unicode encoding, implementations may do any of the following:

 - Use the host locale environment variables to determine the encoding and transparently transcode the strings into UTF-8.
 - Return `EILSEQ` on calls which encounter invalid UTF-8 names.
 - Translate between invalid UTF-8 strings and *ARF strings*.

## ARF strings

ARF is the Alternative Representation for Filenames, an encoding for
representing NUL-terminated strings of potentially non-UTF-8 or ill-formed UTF-16 data, as can occur within
filesystem paths, as non-NUL-terminated and valid [UTF-8] strings.

[UTF-8]: https://en.wikipedia.org/wiki/UTF-8

For a description of ARF strings, see [this page](https://github.com/bytecodealliance/arf-strings/blob/main/README.md).
