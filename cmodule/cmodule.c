#include "cmodule.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <errno.h>

void cmodule_pass_const_char_ptr(const char *ptr)
{
    if (!ptr) {
        printf("%s: Pointer is null\n", __func__);
        return;
    }
    printf("%s: Pointer points to: '%s'\n", __func__, ptr);
}

const char *cmodule_return_const_char_ptr_static(void)
{
    return "Static string from C";
}

const char *cmodule_return_string_no_null(void)
{
    char *ptr = calloc(4, sizeof(char));
    ptr[0] = 'a';
    ptr[1] = 'b';
    ptr[2] = 'c';
    ptr[3] = 'd';   // Overwrites the null terminator!
    return ptr;
}

const char *cmodule_return_const_char_ptr_alloc(void)
{
    const char *ptr = strdup("Dynamic string from C");
    if (!ptr) {
        printf("%s: Failed to duplicate string: %d\n", __func__, errno);
    }
    return ptr;
}

void cmodule_const_char_ptr_free(const char *ptr)
{
    if (!ptr) {
        printf("%s: Pointer is null\n", __func__);
        return;
    }
    free((char *)ptr);  // Cast away "const" qualifier
}
