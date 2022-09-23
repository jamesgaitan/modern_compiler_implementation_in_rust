#include "util.h"
#include "stddef.h"

void *checked_malloc(int len)
{
    void *p = malloc(len);
    assert(p);
    return p;
}

string String(char *str)
{
    const size_t str_len = strnlen(str, MAX_STRING_LEN + 1);
    string ret = checked_malloc(str_len);
    strncpy(ret, str, str_len);
}