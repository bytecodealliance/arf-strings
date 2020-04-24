//! ARF library for converting to and from ARFs.

#include "arf.h"
#include <assert.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

// UTF-8 encoding for U+FEFF, which marks at the beginning of an ARF string.
static const uint8_t utf8_bom[] = { 0xef, 0xbb, 0xbf };

// UTF-8 encoding for U+FFFD, which is used in the lossy portion of an ARF
// string to replace invalid bytes.
static const uint8_t utf8_replacement[] = { 0xef, 0xbf, 0xbd };

// The number of bytes in a byte escape sequence.
static const size_t sizeof_escaped_byte = 2;

// Return a pointer to the first invalid byte, or a pointer to one past the end
// if the entire string is valid UTF-8.
static const uint8_t *find_invalid_utf8(const uint8_t *ptr, size_t len) {
    const uint8_t *end = ptr + len;
    while (ptr != end) {
        if (ptr[0] < 0x80) {
            ptr += 1;
        } else if ((ptr[0] & 0xe0) == 0xc0) {
            if ((end - ptr) < 2 ||
                (ptr[0] & 0xfe) == 0xc0 ||
                (ptr[1] & 0xc0) != 0x80)
            {
                break;
            }
            ptr += 2;
        } else if ((ptr[0] & 0xf0) == 0xe0) {
            if ((end - ptr) < 3 ||
                (ptr[0] == 0xe0 && (ptr[1] & 0xe0) == 0x80) ||
                (ptr[0] == 0xed && (ptr[1] & 0xe0) == 0xa0) ||
                (ptr[0] == 0xef && ptr[1] == 0xbf && (ptr[2] & 0xfe) == 0xbe) ||
                (ptr[1] & 0xc0) != 0x80 ||
                (ptr[2] & 0xc0) != 0x80)
            {
                break;
            }
            ptr += 3;
        } else if ((ptr[0] & 0xf8) == 0xf0) {
            if ((end - ptr) < 4 ||
                ptr[0] > 0xf4 ||
                (ptr[0] == 0xf0 && (ptr[1] & 0xf0) == 0x80) ||
                (ptr[0] == 0xf4 && ptr[1] > 0x8f) ||
                (ptr[1] & 0xc0) != 0x80 ||
                (ptr[2] & 0xc0) != 0x80 ||
                (ptr[3] & 0xc0) != 0x80)
            {
                break;
            }
            ptr += 4;
        } else {
            break;
        }
    }

    return ptr;
}

bool arf_is_valid_cstr(const char *cstr) {
    // Check that the C-string is all valid UTF-8.
    size_t cstr_len = strlen(cstr);
    return find_invalid_utf8((const uint8_t *)cstr, cstr_len) ==
           (const uint8_t *)cstr + cstr_len;
}

bool arf_has_arf_magic(const uint8_t *ptr, size_t len) {
    // ARF strings start with a UTF-8 BOM.
    return len >= sizeof(utf8_bom) &&
           memcmp(ptr, utf8_bom, sizeof(utf8_bom)) == 0;
}

bool arf_is_valid_arf(const uint8_t *ptr, size_t len) {
    // ARF strings begin and end with fixed bytes.
    if (!arf_has_arf_magic(ptr, len)) {
        return false;
    }
    ptr += sizeof(utf8_bom);
    len -= sizeof(utf8_bom);

    // ARF strings are valid UTF-8.
    if (find_invalid_utf8(ptr, len) != ptr + len) {
        return false;
    }

    // ARF strings contain a NUL byte separating the replacement portion from
    // the NUL-escaped portion.
    size_t first_len = strlen((const char *)ptr);
    if (first_len >= len) {
        return false;
    }

    // Check that the lossy portion translates to the NUL-escaped portion.
    bool any_invalid_bytes = false;
    size_t second_begin = first_len + 1;
    size_t i = 0, j = second_begin;
    while (i != first_len) {
        if (ptr[j] == 0) {
            // Check the NUL-escaped encoding.
            if (len - j < sizeof_escaped_byte || (int8_t)ptr[j + 1] < 0) {
                return false;
            }

            // Check that the escaped string contains a replacement character.
            if (first_len - i < sizeof(utf8_replacement) ||
                memcmp(ptr + i, utf8_replacement,
                       sizeof(utf8_replacement)) != 0)
            {
                return false;
            }

            i += sizeof(utf8_replacement);
            j += sizeof_escaped_byte;
            any_invalid_bytes = true;
        } else {
            // Check that the bytes match.
            if (ptr[i] != ptr[j]) {
                return false;
            }

            i += 1;
            j += 1;
        }
    }

    // If there weren't any invalid bytes, we shouldn't have an ARF string.
    if (!any_invalid_bytes) {
        return false;
    }

    // Arf!
    return true;
}

/// Like `arf_sizeof_cstr_arf`, but has `strlen(cstr)` passed in so that it
/// doesn't need to be recomputed.
static size_t arf_sizeof_cstr_arf_impl(const char *cstr, size_t cstr_len) {
    // Start with the length of the fixed-length parts of an ARF string.
    size_t len = sizeof(utf8_bom) + 1;

    // Add the size of both the lossy portion and the NUL-escaped portion.
    for (size_t i = 0; i != cstr_len; ) {
        const uint8_t *found =
            find_invalid_utf8((const uint8_t *)cstr + i, cstr_len - i);

        // Copy in valid UTF-8 bytes.
        size_t valid_len = (size_t)(found - ((const uint8_t *)cstr + i));
        if (__builtin_add_overflow(len, valid_len * 2, &len))
            return SIZE_MAX;

        i += valid_len;
        if (i == cstr_len)
            break;

        // Handle an invalid byte.
        size_t more = sizeof(utf8_replacement) + sizeof_escaped_byte;
        if (__builtin_add_overflow(len, more, &len))
            return SIZE_MAX;

        i += 1;
    }

    return len;
}

bool arf_categorize_cstr(const char *cstr, size_t *restrict len) {
    size_t cstr_len = strlen(cstr);

    if (__builtin_expect(find_invalid_utf8((const uint8_t *)cstr, cstr_len) ==
                         (const uint8_t *)cstr + cstr_len,
                         true))
    {
        *len = cstr_len;
        return true;
    }

    *len = arf_sizeof_cstr_arf_impl(cstr, cstr_len);
    return false;
}

size_t arf_sizeof_cstr_arf(const char *cstr) {
    return arf_sizeof_cstr_arf_impl(cstr, strlen(cstr));
}

void arf_cstr_arf(const char *cstr, uint8_t *ptr) {
    size_t cstr_len = strlen(cstr);

    memcpy(ptr, utf8_bom, sizeof(utf8_bom));
    ptr += sizeof(utf8_bom);

    // Encode the replacement-encoded portion.
    const uint8_t *in = (const uint8_t*)cstr;
    for (size_t len = cstr_len; len != 0; ) {
        const uint8_t *invalid = find_invalid_utf8(in, len);

        // Copy in valid UTF-8 bytes.
        size_t valid_len = (size_t)(invalid - in);
        memcpy(ptr, in, valid_len);
        ptr += valid_len;
        in += valid_len;
        len -= valid_len;

        if (len == 0)
            break;

        // Handle an invalid byte.
        memcpy(ptr, utf8_replacement, sizeof(utf8_replacement));
        ptr += sizeof(utf8_replacement);
        in += 1;
        len -= 1;
    }

    *ptr++ = '\0';

    // Encode the full-encoded portion.
    in = (const uint8_t*)cstr;
    for (size_t len = cstr_len; len != 0; ) {
        const uint8_t *invalid = find_invalid_utf8(in, len);

        // Copy in valid UTF-8 bytes.
        size_t valid_len = (size_t)(invalid - in);
        memcpy(ptr, in, valid_len);
        ptr += valid_len;
        in += valid_len;
        len -= valid_len;

        if (len == 0)
            break;

        // Emit a NUL-escaped byte.
        *ptr++ = '\0';
        *ptr++ = *in & INT8_MAX;
        in += 1;
        len -= 1;
    }
}

size_t arf_sizeof_arf_cstr(const uint8_t *ptr, size_t len) {
    assert(arf_is_valid_arf(ptr, len));

    const uint8_t *end = ptr + len;

    // Examine the NUL-escaped portion, which is the non-lossy portion.
    ptr += sizeof(utf8_bom);
    ptr += strlen((const char *)ptr) + 1;

    size_t cstr_len = 0;
    while (ptr != end) {
        if (*ptr++ == '\0')
            ptr++;
        cstr_len += 1;
    }

    // Add one for the terminating NUL.
    cstr_len += 1;

    return cstr_len;
}

void arf_arf_cstr(const uint8_t *ptr, size_t len, char *__restrict__ cstr) {
    assert(arf_is_valid_arf(ptr, len));

    const uint8_t *end = ptr + len;

    // Examine the NUL-escaped portion, which is the non-lossy portion.
    ptr += sizeof(utf8_bom);
    ptr += strlen((const char *)ptr) + 1;

    // Copy the string data, inverting any escaped bytes.
    while (ptr != end) {
        uint8_t b = *ptr++;
        if (b == '\0')
            b = *ptr++ | (uint8_t)INT8_MIN;
        *cstr++ = (char)b;
    }

    // Append the terminating NUL.
    *cstr = '\0';
}
