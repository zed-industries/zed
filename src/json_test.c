/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * json_test.c: Unittests for json.c
 */

#undef NDEBUG
#include <assert.h>

// Must include main.c because it contains much more than just main()
#define NO_VIM_MAIN
#include "main.c"

// This file has to be included because the tested functions are static
#include "json.c"

#if defined(FEAT_EVAL)
/*
 * Test json_find_end() with incomplete items.
 */
    static void
test_decode_find_end(void)
{
    js_read_T reader;

    reader.js_fill = NULL;
    reader.js_used = 0;

    // string and incomplete string
    reader.js_buf = (char_u *)"\"hello\"";
    assert(json_find_end(&reader, 0) == OK);
    reader.js_buf = (char_u *)"  \"hello\" ";
    assert(json_find_end(&reader, 0) == OK);
    reader.js_buf = (char_u *)"\"hello";
    assert(json_find_end(&reader, 0) == MAYBE);

    // number and dash (incomplete number)
    reader.js_buf = (char_u *)"123";
    assert(json_find_end(&reader, 0) == OK);
    reader.js_buf = (char_u *)"-";
    assert(json_find_end(&reader, 0) == MAYBE);

    // false, true and null, also incomplete
    reader.js_buf = (char_u *)"false";
    assert(json_find_end(&reader, 0) == OK);
    reader.js_buf = (char_u *)"f";
    assert(json_find_end(&reader, 0) == MAYBE);
    reader.js_buf = (char_u *)"fa";
    assert(json_find_end(&reader, 0) == MAYBE);
    reader.js_buf = (char_u *)"fal";
    assert(json_find_end(&reader, 0) == MAYBE);
    reader.js_buf = (char_u *)"fals";
    assert(json_find_end(&reader, 0) == MAYBE);

    reader.js_buf = (char_u *)"true";
    assert(json_find_end(&reader, 0) == OK);
    reader.js_buf = (char_u *)"t";
    assert(json_find_end(&reader, 0) == MAYBE);
    reader.js_buf = (char_u *)"tr";
    assert(json_find_end(&reader, 0) == MAYBE);
    reader.js_buf = (char_u *)"tru";
    assert(json_find_end(&reader, 0) == MAYBE);

    reader.js_buf = (char_u *)"null";
    assert(json_find_end(&reader, 0) == OK);
    reader.js_buf = (char_u *)"n";
    assert(json_find_end(&reader, 0) == MAYBE);
    reader.js_buf = (char_u *)"nu";
    assert(json_find_end(&reader, 0) == MAYBE);
    reader.js_buf = (char_u *)"nul";
    assert(json_find_end(&reader, 0) == MAYBE);

    // object without white space
    reader.js_buf = (char_u *)"{\"a\":123}";
    assert(json_find_end(&reader, 0) == OK);
    reader.js_buf = (char_u *)"{\"a\":123";
    assert(json_find_end(&reader, 0) == MAYBE);
    reader.js_buf = (char_u *)"{\"a\":";
    assert(json_find_end(&reader, 0) == MAYBE);
    reader.js_buf = (char_u *)"{\"a\"";
    assert(json_find_end(&reader, 0) == MAYBE);
    reader.js_buf = (char_u *)"{\"a";
    assert(json_find_end(&reader, 0) == MAYBE);
    reader.js_buf = (char_u *)"{\"";
    assert(json_find_end(&reader, 0) == MAYBE);
    reader.js_buf = (char_u *)"{";
    assert(json_find_end(&reader, 0) == MAYBE);

    // object with white space
    reader.js_buf = (char_u *)"  {  \"a\"  :  123  }  ";
    assert(json_find_end(&reader, 0) == OK);
    reader.js_buf = (char_u *)"  {  \"a\"  :  123  ";
    assert(json_find_end(&reader, 0) == MAYBE);
    reader.js_buf = (char_u *)"  {  \"a\"  :  ";
    assert(json_find_end(&reader, 0) == MAYBE);
    reader.js_buf = (char_u *)"  {  \"a\"  ";
    assert(json_find_end(&reader, 0) == MAYBE);
    reader.js_buf = (char_u *)"  {  \"a  ";
    assert(json_find_end(&reader, 0) == MAYBE);
    reader.js_buf = (char_u *)"  {   ";
    assert(json_find_end(&reader, 0) == MAYBE);

    // JS object with white space
    reader.js_buf = (char_u *)"  {  a  :  123  }  ";
    assert(json_find_end(&reader, JSON_JS) == OK);
    reader.js_buf = (char_u *)"  {  a  :   ";
    assert(json_find_end(&reader, JSON_JS) == MAYBE);

    // array without white space
    reader.js_buf = (char_u *)"[\"a\",123]";
    assert(json_find_end(&reader, 0) == OK);
    reader.js_buf = (char_u *)"[\"a\",123";
    assert(json_find_end(&reader, 0) == MAYBE);
    reader.js_buf = (char_u *)"[\"a\",";
    assert(json_find_end(&reader, 0) == MAYBE);
    reader.js_buf = (char_u *)"[\"a\"";
    assert(json_find_end(&reader, 0) == MAYBE);
    reader.js_buf = (char_u *)"[\"a";
    assert(json_find_end(&reader, 0) == MAYBE);
    reader.js_buf = (char_u *)"[\"";
    assert(json_find_end(&reader, 0) == MAYBE);
    reader.js_buf = (char_u *)"[";
    assert(json_find_end(&reader, 0) == MAYBE);

    // array with white space
    reader.js_buf = (char_u *)"  [  \"a\"  ,  123  ]  ";
    assert(json_find_end(&reader, 0) == OK);
    reader.js_buf = (char_u *)"  [  \"a\"  ,  123  ";
    assert(json_find_end(&reader, 0) == MAYBE);
    reader.js_buf = (char_u *)"  [  \"a\"  ,  ";
    assert(json_find_end(&reader, 0) == MAYBE);
    reader.js_buf = (char_u *)"  [  \"a\"  ";
    assert(json_find_end(&reader, 0) == MAYBE);
    reader.js_buf = (char_u *)"  [  \"a  ";
    assert(json_find_end(&reader, 0) == MAYBE);
    reader.js_buf = (char_u *)"  [  ";
    assert(json_find_end(&reader, 0) == MAYBE);
}

    static int
fill_from_cookie(js_read_T *reader)
{
    reader->js_buf = reader->js_cookie;
    return TRUE;
}

/*
 * Test json_find_end with an incomplete array, calling the fill function.
 */
    static void
test_fill_called_on_find_end(void)
{
    js_read_T reader;

    reader.js_fill = fill_from_cookie;
    reader.js_used = 0;
    reader.js_buf = (char_u *)"  [  \"a\"  ,  123  ";
    reader.js_cookie =	      "  [  \"a\"  ,  123  ]  ";
    assert(json_find_end(&reader, 0) == OK);
    reader.js_buf = (char_u *)"  [  \"a\"  ,  ";
    assert(json_find_end(&reader, 0) == OK);
    reader.js_buf = (char_u *)"  [  \"a\"  ";
    assert(json_find_end(&reader, 0) == OK);
    reader.js_buf = (char_u *)"  [  \"a";
    assert(json_find_end(&reader, 0) == OK);
    reader.js_buf = (char_u *)"  [  ";
    assert(json_find_end(&reader, 0) == OK);
}

/*
 * Test json_find_end with an incomplete string, calling the fill function.
 */
    static void
test_fill_called_on_string(void)
{
    js_read_T reader;

    reader.js_fill = fill_from_cookie;
    reader.js_used = 0;
    reader.js_buf = (char_u *)" \"foo";
    reader.js_end = reader.js_buf + STRLEN(reader.js_buf);
    reader.js_cookie =	      " \"foobar\"  ";
    assert(json_decode_string(&reader, NULL, '"') == OK);
}
#endif

    int
main(void)
{
#if defined(FEAT_EVAL)
    test_decode_find_end();
    test_fill_called_on_find_end();
    test_fill_called_on_string();
#endif
    return 0;
}
