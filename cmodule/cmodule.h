#ifndef CMODULE_H_INCLUDED
#define CMODULE_H_INCLUDED

/**
 * @brief Accept a pointer to a string that is owned by Rust.
 *
 * This memory can be read but not modified by C.
 * This function prints the argument to the console.
 *
 * @param[in] ptr   A pointer to a null-terminated string.
 */
void cmodule_pass_const_char_ptr(const char *ptr);

/**
 * @brief Return a pointer to statically-allocated memory that is owned by C.
 *
 * This memory can be read but not modified by Rust.
 * The contents of the string are: "Static string from C"
 *
 * @return A pointer to a statically-allocated string.
 * @warning This memory *must not* be freed!
 */
const char *cmodule_return_const_char_ptr_static(void);

/**
 * @brief Return a pointer to dynamically-allocated memory that is owned by C.
 *
 * This memory can be read but not modified by Rust.
 * The contents of the string are: "Dynamic string from C"
 *
 * @return A pointer to a statically-allocated string, or NULL if the allocation fails.
 * @warning This memory *must* be freed by calling another C function!
 */
const char *cmodule_return_const_char_ptr_alloc(void);

/**
 *
 * @return
 */
const char *cmodule_return_string_no_null(void);

/**
 * @brief Free memory that was returned by cmodule_return_const_char_ptr_alloc().
 *
 * There is some discussion as to whether a const pointer can be/should be freed:
 * should freeing be considered a modification?
 * This API takes an opinionated stance for the sake of illustration:
 * users may want to dynamically allocate a string that should not be modified.
 * This requires that a const char * can be dynamically allocated and then freed.
 *
 * @param[in] ptr   A pointer to dynamically allocated memory.
 */
void cmodule_const_char_ptr_free(const char *ptr);

#endif
