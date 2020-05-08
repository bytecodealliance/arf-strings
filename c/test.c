#undef NDEBUG
#include "arf.h"
#include <assert.h>
#include <string.h>
#include <stdint.h>

#define UTF8_BOM "\xef\xbb\xbf"
#define UTF8_REPLACEMENT "\xef\xbf\xbd"

#define ptr_len(s) ((const uint8_t *)(s)), (sizeof(s) - 1)

int main(void) {
    assert(arf_is_valid_cstr(""));
    assert(arf_is_valid_cstr("foo"));
    assert(arf_is_valid_cstr("%"));
    assert(arf_is_valid_cstr("%ff"));
    assert(arf_is_valid_cstr("🐰�"));
    assert(arf_is_valid_cstr("\2/yes/\3"));
    assert(arf_is_valid_cstr("\xc2\x80"));
    assert(arf_is_valid_cstr(UTF8_BOM));
    assert(arf_is_valid_cstr(UTF8_REPLACEMENT));
    assert(arf_is_valid_cstr("\xd0\x80"));
    assert(!arf_is_valid_cstr("\xc0"));
    assert(!arf_is_valid_cstr("\xf5"));
    assert(!arf_is_valid_cstr("\xf0\xd0\x80\x80"));
    assert(!arf_is_valid_cstr("\xf0\x80\xd0\x80"));
    assert(!arf_is_valid_cstr("\xf0\x80\x80\xd0"));
    assert(!arf_is_valid_cstr("\xef\xbf\xbe"));
    assert(!arf_is_valid_cstr("\xf0\x80"));
    assert(!arf_is_valid_cstr("\xf0\xf0"));
    assert(!arf_is_valid_cstr("\xf3\x90"));
    assert(!arf_is_valid_cstr("\xf4\xf4"));
    assert(!arf_is_valid_cstr("\xf4\x90"));
    assert(!arf_is_valid_cstr("\xf5\x90"));
    assert(!arf_is_valid_cstr("\x80"));
    assert(!arf_is_valid_cstr("\x8f"));
    assert(!arf_is_valid_cstr("\x90"));
    assert(!arf_is_valid_cstr("\x9f"));
    assert(!arf_is_valid_cstr("\xa0"));
    assert(!arf_is_valid_cstr("\xbf"));
    assert(!arf_is_valid_cstr("\xc0\x80"));
    assert(!arf_is_valid_cstr("\xc0\xbf"));
    assert(!arf_is_valid_cstr("\xc1\x80"));
    assert(!arf_is_valid_cstr("\xc1\xbf"));
    assert(!arf_is_valid_cstr("\xc2"));
    assert(!arf_is_valid_cstr("\xc2\x2e"));
    assert(!arf_is_valid_cstr("\xc2\x7f"));
    assert(!arf_is_valid_cstr("\xc2\x80\x80"));
    assert(!arf_is_valid_cstr("\xc2\xc0"));
    assert(!arf_is_valid_cstr("\xc2\xfd"));
    assert(!arf_is_valid_cstr("\xdf"));
    assert(!arf_is_valid_cstr("\xdf\x7f"));
    assert(!arf_is_valid_cstr("\xdf\xc0"));
    assert(!arf_is_valid_cstr("\xdf\xfd"));
    assert(!arf_is_valid_cstr("\xe0"));
    assert(!arf_is_valid_cstr("\xe0\x7f\xa0"));
    assert(!arf_is_valid_cstr("\xe0\x80\x80"));
    assert(!arf_is_valid_cstr("\xe0\x80\xa0"));
    assert(!arf_is_valid_cstr("\xe0\x9f\xa0"));
    assert(!arf_is_valid_cstr("\xe0\x9f\xbf"));
    assert(!arf_is_valid_cstr("\xe0\xa0"));
    assert(!arf_is_valid_cstr("\xe0\xa0\x7f"));
    assert(!arf_is_valid_cstr("\xe0\xa0\xc0"));
    assert(!arf_is_valid_cstr("\xe0\xa0\xfd"));
    assert(!arf_is_valid_cstr("\xe0\xc0\xa0"));
    assert(!arf_is_valid_cstr("\xe0\xfd\xa0"));
    assert(!arf_is_valid_cstr("\xe1"));
    assert(!arf_is_valid_cstr("\xe1\x2e"));
    assert(!arf_is_valid_cstr("\xe1\x7f\x80"));
    assert(!arf_is_valid_cstr("\xe1\x80"));
    assert(!arf_is_valid_cstr("\xe1\x80\x2e"));
    assert(!arf_is_valid_cstr("\xe1\x80\x7f"));
    assert(!arf_is_valid_cstr("\xe1\x80\x80\x80"));
    assert(!arf_is_valid_cstr("\xe1\x80\xc0"));
    assert(!arf_is_valid_cstr("\xe1\x80\xfd"));
    assert(!arf_is_valid_cstr("\xe1\xc0\x80"));
    assert(!arf_is_valid_cstr("\xe1\xfd\x80"));
    assert(!arf_is_valid_cstr("\xec"));
    assert(!arf_is_valid_cstr("\xec\x7f\x80"));
    assert(!arf_is_valid_cstr("\xec\x80"));
    assert(!arf_is_valid_cstr("\xec\x80\x7f"));
    assert(!arf_is_valid_cstr("\xec\x80\xc0"));
    assert(!arf_is_valid_cstr("\xec\x80\xfd"));
    assert(!arf_is_valid_cstr("\xec\xc0\x80"));
    assert(!arf_is_valid_cstr("\xec\xfd\x80"));
    assert(!arf_is_valid_cstr("\xed"));
    assert(!arf_is_valid_cstr("\xed\x7f\x80"));
    assert(!arf_is_valid_cstr("\xed\x80"));
    assert(!arf_is_valid_cstr("\xed\x80\x7f"));
    assert(!arf_is_valid_cstr("\xed\x80\xc0"));
    assert(!arf_is_valid_cstr("\xed\x80\xfd"));
    assert(!arf_is_valid_cstr("\xed\xa0\x80"));
    assert(!arf_is_valid_cstr("\xed\xa0\xbf"));
    assert(!arf_is_valid_cstr("\xed\xbf\x80"));
    assert(!arf_is_valid_cstr("\xed\xbf\xbf"));
    assert(!arf_is_valid_cstr("\xed\xc0\x80"));
    assert(!arf_is_valid_cstr("\xed\xfd\x80"));
    assert(!arf_is_valid_cstr("\xee"));
    assert(!arf_is_valid_cstr("\xee\x7f\x80"));
    assert(!arf_is_valid_cstr("\xee\x80"));
    assert(!arf_is_valid_cstr("\xee\x80\x7f"));
    assert(!arf_is_valid_cstr("\xee\x80\xc0"));
    assert(!arf_is_valid_cstr("\xee\x80\xfd"));
    assert(!arf_is_valid_cstr("\xee\xc0\x80"));
    assert(!arf_is_valid_cstr("\xee\xfd\x80"));
    assert(!arf_is_valid_cstr("\xef"));
    assert(!arf_is_valid_cstr("\xef\x7f\x80"));
    assert(!arf_is_valid_cstr("\xef\x80"));
    assert(!arf_is_valid_cstr("\xef\x80\x7f"));
    assert(!arf_is_valid_cstr("\xef\x80\xc0"));
    assert(!arf_is_valid_cstr("\xef\x80\xfd"));
    assert(!arf_is_valid_cstr("\xef\xc0\x80"));
    assert(!arf_is_valid_cstr("\xef\xfd\x80"));
    assert(!arf_is_valid_cstr("\xf0"));
    assert(!arf_is_valid_cstr("\xf0\x7f\x90\x90"));
    assert(!arf_is_valid_cstr("\xf0\x80\x80\x80"));
    assert(!arf_is_valid_cstr("\xf0\x80\x90\x90"));
    assert(!arf_is_valid_cstr("\xf0\x8f\x90\x90"));
    assert(!arf_is_valid_cstr("\xf0\x8f\xbf\xbf"));
    assert(!arf_is_valid_cstr("\xf0\x90"));
    assert(!arf_is_valid_cstr("\xf0\x90\x7f\x90"));
    assert(!arf_is_valid_cstr("\xf0\x90\x90"));
    assert(!arf_is_valid_cstr("\xf0\x90\x90\x7f"));
    assert(!arf_is_valid_cstr("\xf0\x90\x90\xc0"));
    assert(!arf_is_valid_cstr("\xf0\x90\x90\xfd"));
    assert(!arf_is_valid_cstr("\xf0\x90\xc0\x90"));
    assert(!arf_is_valid_cstr("\xf0\x90\xfd\x90"));
    assert(!arf_is_valid_cstr("\xf0\xc0\x90\x90"));
    assert(!arf_is_valid_cstr("\xf0\xfd\x90\x90"));
    assert(!arf_is_valid_cstr("\xf1"));
    assert(!arf_is_valid_cstr("\xf1\x23"));
    assert(!arf_is_valid_cstr("\xf1\x7f\x80\x80"));
    assert(!arf_is_valid_cstr("\xf1\x80"));
    assert(!arf_is_valid_cstr("\xf1\x80\x23"));
    assert(!arf_is_valid_cstr("\xf1\x80\x7f\x80"));
    assert(!arf_is_valid_cstr("\xf1\x80\x80"));
    assert(!arf_is_valid_cstr("\xf1\x80\x80\x23"));
    assert(!arf_is_valid_cstr("\xf1\x80\x80\x7f"));
    assert(!arf_is_valid_cstr("\xf1\x80\x80\x80\x80"));
    assert(!arf_is_valid_cstr("\xf1\x80\x80\xc0"));
    assert(!arf_is_valid_cstr("\xf1\x80\x80\xfd"));
    assert(!arf_is_valid_cstr("\xf1\x80\xc0\x80"));
    assert(!arf_is_valid_cstr("\xf1\x80\xfd\x80"));
    assert(!arf_is_valid_cstr("\xf1\xc0\x80\x80"));
    assert(!arf_is_valid_cstr("\xf1\xfd\x80\x80"));
    assert(!arf_is_valid_cstr("\xf3"));
    assert(!arf_is_valid_cstr("\xf3\x7f\x80\x80"));
    assert(!arf_is_valid_cstr("\xf3\x80"));
    assert(!arf_is_valid_cstr("\xf3\x80\x7f\x80"));
    assert(!arf_is_valid_cstr("\xf3\x80\x80"));
    assert(!arf_is_valid_cstr("\xf3\x80\x80\x7f"));
    assert(!arf_is_valid_cstr("\xf3\x80\x80\xc0"));
    assert(!arf_is_valid_cstr("\xf3\x80\x80\xfd"));
    assert(!arf_is_valid_cstr("\xf3\x80\xc0\x80"));
    assert(!arf_is_valid_cstr("\xf3\x80\xfd\x80"));
    assert(!arf_is_valid_cstr("\xf3\xc0\x80\x80"));
    assert(!arf_is_valid_cstr("\xf3\xfd\x80\x80"));
    assert(!arf_is_valid_cstr("\xf4"));
    assert(!arf_is_valid_cstr("\xf4\x7f\x80\x80"));
    assert(!arf_is_valid_cstr("\xf4\x80"));
    assert(!arf_is_valid_cstr("\xf4\x80\x7f\x80"));
    assert(!arf_is_valid_cstr("\xf4\x80\x80"));
    assert(!arf_is_valid_cstr("\xf4\x80\x80\x7f"));
    assert(!arf_is_valid_cstr("\xf4\x80\x80\xc0"));
    assert(!arf_is_valid_cstr("\xf4\x80\x80\xfd"));
    assert(!arf_is_valid_cstr("\xf4\x80\xc0\x80"));
    assert(!arf_is_valid_cstr("\xf4\x80\xfd\x80"));
    assert(!arf_is_valid_cstr("\xf4\x90\x80\x80"));
    assert(!arf_is_valid_cstr("\xf4\xbf\x80\x80"));
    assert(!arf_is_valid_cstr("\xf4\xc0\x80\x80"));
    assert(!arf_is_valid_cstr("\xf4\xfd\x80\x80"));
    assert(!arf_is_valid_cstr("\xf5\x80\x80\x80"));
    assert(!arf_is_valid_cstr("\xf7\x80\x80\x80"));
    assert(!arf_is_valid_cstr("\xf7\xbf\xbf\xbf"));
    assert(!arf_is_valid_cstr("\xf8\x23"));
    assert(!arf_is_valid_cstr("\xf8\x80\x23"));
    assert(!arf_is_valid_cstr("\xf8\x80\x80\x23"));
    assert(!arf_is_valid_cstr("\xf8\x80\x80\x80\x23"));
    assert(!arf_is_valid_cstr("\xf8\x80\x80\x80\x80\x80"));
    assert(!arf_is_valid_cstr("\xf8\x80\x80\x80\x80"));
    assert(!arf_is_valid_cstr("\xf8\x80\x80\x80"));
    assert(!arf_is_valid_cstr("\xf8\x80\x80"));
    assert(!arf_is_valid_cstr("\xf8\x80"));
    assert(!arf_is_valid_cstr("\xf8"));
    assert(!arf_is_valid_cstr("\xfb\xbf\xbf\xbf\xbf"));
    assert(!arf_is_valid_cstr("\xfc\x23"));
    assert(!arf_is_valid_cstr("\xfc\x80\x23"));
    assert(!arf_is_valid_cstr("\xfc\x80\x80\x23"));
    assert(!arf_is_valid_cstr("\xfc\x80\x80\x80\x23"));
    assert(!arf_is_valid_cstr("\xfc\x80\x80\x80\x80\x23"));
    assert(!arf_is_valid_cstr("\xfc\x80\x80\x80\x80\x80\x80"));
    assert(!arf_is_valid_cstr("\xfc\x80\x80\x80\x80\x80"));
    assert(!arf_is_valid_cstr("\xfc\x80\x80\x80\x80"));
    assert(!arf_is_valid_cstr("\xfc\x80\x80\x80"));
    assert(!arf_is_valid_cstr("\xfc\x80\x80"));
    assert(!arf_is_valid_cstr("\xfc\x80"));
    assert(!arf_is_valid_cstr("\xfc"));
    assert(!arf_is_valid_cstr("\xfd\xbf\xbf\xbf\xbf\xbf"));
    assert(!arf_is_valid_cstr("\xfe"));
    assert(!arf_is_valid_cstr("\xfe\xff"));
    assert(!arf_is_valid_cstr("\xff"));
    assert(!arf_is_valid_cstr("\xff\xfe"));

    assert(!arf_has_arf_magic(ptr_len("")));
    assert(!arf_has_arf_magic(ptr_len("f")));
    assert(!arf_has_arf_magic(ptr_len("foo")));
    assert(!arf_has_arf_magic(ptr_len(UTF8_REPLACEMENT)));
    assert(!arf_has_arf_magic(ptr_len(UTF8_REPLACEMENT "foo")));
    assert(arf_has_arf_magic(ptr_len(UTF8_BOM "foo")));

    assert(!arf_is_valid_arf(ptr_len("")));
    assert(!arf_is_valid_arf(ptr_len("foo")));
    assert(!arf_is_valid_arf(ptr_len(UTF8_BOM "\xc0" "\0" "\xc0")));
    assert(!arf_is_valid_arf(ptr_len(UTF8_BOM "\xc0" "\0" "\0\xc0")));
    assert(!arf_is_valid_arf(ptr_len(UTF8_BOM "\xc0" "\0" "\0\x40")));
    assert(!arf_is_valid_arf(ptr_len(UTF8_BOM "foo")));
    assert(!arf_is_valid_arf(ptr_len(UTF8_BOM "foo" "\0" "foo")));
    assert(!arf_is_valid_arf(ptr_len(UTF8_BOM "foox" "\0" "foo%FF")));
    assert(!arf_is_valid_arf(ptr_len(UTF8_BOM "foo\xef\xbb\xbf" "\0" "foo%FF")));
    assert(!arf_is_valid_arf(ptr_len(UTF8_BOM "foo�" "\0" "foo%ff")));
    assert(!arf_is_valid_arf(ptr_len(UTF8_BOM "foo�" "\0" "goo\0\x7F")));
    assert(!arf_is_valid_arf(ptr_len(UTF8_BOM "foo�" "\0" "foo\0")));
    assert(!arf_is_valid_arf(ptr_len(UTF8_BOM "foo�" "\0" "foo\0\x80")));
    assert(!arf_is_valid_arf(ptr_len(UTF8_BOM "foo\xef\xbf\xbc" "\0" "foo\0\x7F")));
    assert(!arf_is_valid_arf(ptr_len(UTF8_REPLACEMENT "foo�" "\0" UTF8_REPLACEMENT "foo\0\x7F")));
    assert(!arf_is_valid_arf(ptr_len(UTF8_REPLACEMENT UTF8_BOM "foo�" "\0" UTF8_REPLACEMENT "foo\0\x7F")));
    assert(arf_is_valid_arf(ptr_len(UTF8_BOM "foo\xef\xbf\xbd" "\0" "foo\0\x7F")));
    assert(arf_is_valid_arf(ptr_len(UTF8_BOM "foo�" "\0" "foo\0\x7F")));
    assert(arf_is_valid_arf(ptr_len(UTF8_BOM "�foo���bar�" "\0" "\0\x7F" "foo\0\x7F\0\x7E\0\x7D" "bar\0\x7C")));
    assert(arf_is_valid_arf(ptr_len(UTF8_BOM UTF8_BOM "foo�" "\0" UTF8_BOM "foo\0\x7F")));
    assert(arf_is_valid_arf(ptr_len(UTF8_BOM UTF8_REPLACEMENT "foo�" "\0" UTF8_REPLACEMENT "foo\0\x7F")));

    size_t len;

    assert(arf_categorize_cstr("", &len));
    assert(len == 0);

    assert(arf_categorize_cstr("foo", &len));
    assert(len == 3);

    assert(!arf_categorize_cstr("foo\xff", &len));
    assert(len == 15);

    assert(arf_categorize_cstr(UTF8_BOM, &len));
    assert(len == sizeof(UTF8_BOM) - 1);

    assert(arf_categorize_cstr(UTF8_REPLACEMENT, &len));
    assert(len == sizeof(UTF8_REPLACEMENT) - 1);

    assert(arf_sizeof_cstr_arf("\xff") == 9);
    assert(arf_sizeof_cstr_arf("foo\xff") == 15);
    assert(arf_sizeof_cstr_arf("foo\xff" "bar\xfe") == 26);
    assert(arf_sizeof_cstr_arf(UTF8_BOM) == 10);
    assert(arf_sizeof_cstr_arf(UTF8_REPLACEMENT) == 10);

    uint8_t buffer[1024];

    arf_cstr_arf("\xff", buffer);
    assert(memcmp(buffer, ptr_len(UTF8_BOM "�\0" "\0\x7F")) == 0);

    arf_cstr_arf("foo\xff", buffer);
    assert(memcmp(buffer, ptr_len(UTF8_BOM "foo�\0" "foo\0\x7F")) == 0);

    arf_cstr_arf("foo\xff" "bar\xfe", buffer);
    assert(memcmp(buffer, ptr_len(UTF8_BOM "foo�bar�\0" "foo\0\x7F" "bar\0\x7E")) == 0);

    arf_cstr_arf("foo\xff" "bar\xfe" "end", buffer);
    assert(memcmp(buffer, ptr_len(UTF8_BOM "foo�bar�end\0" "foo\0\x7F" "bar\0\x7E" "end")) == 0);

    arf_cstr_arf(UTF8_BOM "foo\xff" "bar\xfe" "end", buffer);
    assert(memcmp(buffer, ptr_len(UTF8_BOM UTF8_BOM "foo�bar�end\0" UTF8_BOM "foo\0\x7F" "bar\0\x7E" "end")) == 0);

    arf_cstr_arf(UTF8_REPLACEMENT "foo\xff" "bar\xfe" "end", buffer);
    assert(memcmp(buffer, ptr_len(UTF8_BOM UTF8_REPLACEMENT "foo�bar�end\0" UTF8_REPLACEMENT "foo\0\x7F" "bar\0\x7E" "end")) == 0);

    arf_cstr_arf("\xe6\x96", buffer);
    assert(memcmp(buffer, ptr_len(UTF8_BOM "��\0" "\0f" "\0\x16")) == 0);

    assert(arf_sizeof_arf_cstr(ptr_len(UTF8_BOM "�\0" "\0\x7F")) == 2);
    assert(arf_sizeof_arf_cstr(ptr_len(UTF8_BOM "foo�\0" "foo\0\x7F")) == 5);
    assert(arf_sizeof_arf_cstr(ptr_len(UTF8_BOM "foo�bar�\0" "foo\0\x7F" "bar\0\x7E")) == 9);
    assert(arf_sizeof_arf_cstr(ptr_len(UTF8_BOM "foo�bar�end\0" "foo\0\x7F" "bar\0\x7E" "end")) == 12);

    arf_arf_cstr(ptr_len(UTF8_BOM "�\0" "\0\x7F"), (char *)buffer);
    assert(strcmp((const char *)buffer, "\xff") == 0);

    arf_arf_cstr(ptr_len(UTF8_BOM "foo�\0" "foo\0\x7F"), (char *)buffer);
    assert(strcmp((const char *)buffer, "foo\xff") == 0);

    arf_arf_cstr(ptr_len(UTF8_BOM "foo�bar�\0" "foo\0\x7F" "bar\0\x7E"), (char *)buffer);
    assert(strcmp((const char *)buffer, "foo\xff" "bar\xfe") == 0);

    arf_arf_cstr(ptr_len(UTF8_BOM "foo�bar�end\0" "foo\0\x7F" "bar\0\x7E" "end"), (char *)buffer);
    assert(strcmp((const char *)buffer, "foo\xff" "bar\xfe" "end") == 0);

    return 0;
}
