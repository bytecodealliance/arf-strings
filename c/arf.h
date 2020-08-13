#ifndef ARF_H
#define ARF_H

#include <stddef.h>
#include <stdint.h>
#ifndef __cplusplus
#include <stdbool.h>
#endif

#ifdef __cplusplus
extern "C" {
#endif

/// Test whether the provided C-string is already valid UTF-8.
bool arf_is_valid_c_str(const char *c_str)
    __attribute__((__pure__, __nonnull__(1), __nothrow__));

/// Quickly test whether the provided buffer looks like an ARF string.
bool arf_has_arf_magic(const uint8_t *ptr, size_t len)
    __attribute__((__pure__, __nothrow__));

/// Test whether the provided buffer is a valid ARF string.
bool arf_is_valid_arf(const uint8_t *ptr, size_t len)
    __attribute__((__pure__, __nothrow__));

/// Test whether the provided C-string is already valid UTF-8. If it is, store
/// the length in *len and return true. If it isn't, store the length of the ARF
/// string needed to represent it in *len and return false.
bool arf_categorize_c_str(const char *c_str, size_t *__restrict__ len)
    __attribute__((__nonnull__(1, 2), __nothrow__));

/// Return the length of an ARF string for the given C-string. Returns
/// `SIZE_MAX` on overflow.
size_t arf_sizeof_c_str_arf(const char *c_str)
    __attribute__((__pure__, __nonnull__(1), __nothrow__));

/// Write the ARF string for the given C-string into the provided buffer. Use
/// `arf_sizeof_c_str_arf` to determine the required buffer size.
void arf_c_str_arf(const char *c_str, uint8_t *__restrict__ ptr)
    __attribute__((__nonnull__(1, 2), __nothrow__));

/// Return the length of a C-string for the given ARF.
size_t arf_sizeof_arf_c_str(const uint8_t *ptr, size_t len)
    __attribute__((__pure__, __nonnull__(1), __nothrow__));

/// Write the C-string for the given ARF into the provided buffer. Use
/// `arf_sizeof_arf_c_str` to determine the required buffer size.
void arf_arf_c_str(const uint8_t *ptr, size_t len, char *__restrict__ c_str)
    __attribute__((__nonnull__(1, 3), __nothrow__));

#ifdef __cplusplus
}
#endif

#endif
