#include "cmodule.h"
#include <stdio.h>

void cmodule_pass_const_char_ptr(const char *ptr)
{
    if (!ptr) {
        printf("%s: Pointer is null\n", __func__);
        return;
    }
    printf("%s: Pointer points to: '%s'\n", __func__, ptr);
}
