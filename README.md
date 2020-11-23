# ARF strings

ARF is the Alternative Representation for Filenames, an encoding for
representing NUL-terminated non-UTF-8 strings as valid (and non-NUL-terminated)
[UTF-8] strings. It's intended for use in environments that need a way to
represent [POSIX-compatible] and Windows-compatible path names within UTF-8
string types.

This is an experiment, and the Windows encoding scheme is particularly
experimental.

[UTF-8]: https://en.wikipedia.org/wiki/UTF-8
[POSIX-compatible]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap03.html#tag_03_271

## Description

ARF strings have the following form:

```
arf-string ::= U+FEFF lossy-portion U+0000 NUL-escaped-portion
```

`U+FEFF` is the Byte Order Mark (BOM) code point.

The `lossy-portion` consists of the original string data (excluding the
terminating NUL) with any unencodable bytes replaced by `U+FFFD`, the Unicode
replacement character.

`U+0000` is the NULL (NUL) code point.

The `NUL-escaped-portion` of an ARF string consists of the original string
data (again, excluding the terminating NUL) with any unencodable bytes replaced
by `U+0000` followed by:
 - On POSIX-ish platforms, the invalid byte with the most significant bit set to 0.
 - On Windows, a Unicode scalar value between `U+0` and `U+7FF`, representing
   the offset in the surrogate codepoint space (`U+D800` through `U+DFFF`).

## Example

The ARF encoding of `"foo\xffbar"` on POSIX-ish platforms is `"\xef\xbb\xbffoo\xef\xbf\xbdbar\x00foo\x00\x7fbar"`:
 - `"\xef\xbb\xbf"` is the UTF-8 encoding of `U+FEFF`.
 - `"foo\xef\xbf\xbdbar"`is the string with the unencodable byte replaced by the UTF-8 encoding for `U+FFFD`.
 - `"\x00"` is the UTF-8 encoding for `U+0000`.
 - `"foo\0\x7fbar"` is the string with the unencodable byte replaced by a `NUL` followed by the invalid byte with the most significant bit set to 0.

## Rationale

Unencodable pathnames are very rare in practice, so this design doesn't attempt to
make them efficient. In the worst cases, ARF strings may be several times the size
of the corresponding input strings (though they're still O(n)). The redundancy is
used to protect against accidental misuse by code not aware of ARF strings.

C and POSIX code represent paths as NUL-terminated strings. When given an ARF string,
such code will only see the BOM and the lossy portion containing replacement characters.
In most cases, attempts to open such a pathname will produce `ENOENT` errors, since the
leading UTF-8 BOM and UTF-8 replacement byte sequences are unlikely to appear in
non-UTF-8 filenames. Typical application error messages will include the pathname,
where the replacement characters will serve as a hint as to the nature of the problem.

Consequently, by default, ARF-unaware C and POSIX code will not be able to open
unencodable pathnames. For many applications, this limitation is worth the advantage
of being able to assume that all pathnames are UTF-8. Applications that wish to
work with unencodable pathnames can opt in by being explicitly aware of ARF strings,
optionally with the help of the Rust and C libraries in this repository.

Another tricky case is code which modifies paths. ARF-unaware code may modify ARF
strings without being aware of the ARF encoding. Such code won't know to update the
NUL-escaped portion of the ARF string, and the resulting ARF string will subsequently
be detected as invalid, leading to errors rather than surprising behavior.
