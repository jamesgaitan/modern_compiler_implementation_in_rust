#ifndef UTIL_H
#define UTIL_H

#include <assert.h>
#include <stdbool.h>

#define MAX_STRING_LEN (128)

typedef char *string;
string String(char *);

#define TRUE (true)
#define FALSE (false)

void *checked_malloc(int);

#endif /* UTIL_H */