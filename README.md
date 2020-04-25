# ARF strings

ARF is the Alternative Representation for Filenames, an encoding for
representing NUL-terminated strings of potentially non-UTF-8 data, as can occur within
filesystem paths, as non-NUL-terminated valid [UTF-8] strings.

[UTF-8]: https://en.wikipedia.org/wiki/UTF-8

## Description

The ARF encoding of a valid UTF-8 string is just the string itself, without the trailing NUL. (This should be the common case.)

For a string which contains invalid bytes, the following form is used:

```
U+FEFF <lossy portion> U+0000 <NUL-escaped portion>
```

`U+FEFF` is the Byte Order Mark (BOM) code point.

The `<lossy portion>` consists of the original string data (excluding the
terminating NUL) with any unencodable bytes replaced by `U+FFFD`.

`U+0000` is the NULL (NUL) code point.

The `<NUL-escaped>` portion of an ARF string consists of the original string
data (again, excluding the terminating NUL) with any unencodable bytes replaced
by `U+0000` followed by the invalid byte with the most significant bit set to 0.

## Rationale

The assumption is that unencodable filenames should be very rare, and they
therefore don't need to be efficient or convenient to work with, but it should
be possible to work with them.

Encoding the string data twice, while somewhat inefficient, makes it possible to
detect accidental modifications by ARF-unaware application code.

Applications which are unaware of ARF string should behave correctly or fail
gracefully.

One tricky case is C code which represents paths as NUL-terminated strings.
Given an ARF string, such code will only see the BOM and the lossy portion. In
most cases, attempts to access the file will produce `ENOENT` errors, since the
leading UTF-8 BOM and UTF-8 replacement byte sequences are unlikely to appear in
non-UTF-8 filenames.

Another tricky case is code which modifies paths, which may lead them to modify
ARF strings in place. Such code won't update the NUL-escaped portion of the ARF
string, and the resulting ARF string can be detected as invalid.
