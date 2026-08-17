/*
 * CUTest -- C/C++ Unit Test facility
 * <http://github.com/mity/cutest>
 *
 * Copyright (c) 2013-2017 Martin Mitas
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 */

#ifndef CUTEST_H__
#define CUTEST_H__

/************************
 *** Public interface ***
 ************************/

/* By default, <cutest.h> provides the main program entry point (function
 * main()). However, if the test suite is composed of multiple source files
 * which include <cutest.h>, then this causes a problem of multiple main()
 * definitions. To avoid this problem, #define macro TEST_NO_MAIN in all
 * compilation units but one.
 */

/* Macro to specify list of unit tests in the suite.
 * The unit test implementation MUST provide list of unit tests it implements
 * with this macro:
 *
 *   TEST_LIST = {
 *       { "test1_name", test1_func_ptr },
 *       { "test2_name", test2_func_ptr },
 *       ...
 *       { 0 }
 *   };
 *
 * The list specifies names of each test (must be unique) and pointer to
 * a function implementing it. The function does not take any arguments
 * and has no return values, i.e. every test function has tp be compatible
 * with this prototype:
 *
 *   void test_func(void);
 */
#define TEST_LIST const struct test__ test_list__[]

/* Macros for testing whether an unit test succeeds or fails. These macros
 * can be used arbitrarily in functions implementing the unit tests.
 *
 * If any condition fails throughout execution of a test, the test fails.
 *
 * TEST_CHECK takes only one argument (the condition), TEST_CHECK_ allows
 * also to specify an error message to print out if the condition fails.
 * (It expects printf-like format string and its parameters). The macros
 * return non-zero (condition passes) or 0 (condition fails).
 *
 * That can be useful when more conditions should be checked only if some
 * preceding condition passes, as illustrated in this code snippet:
 *
 *   SomeStruct* ptr = allocate_some_struct();
 *   if(TEST_CHECK(ptr != NULL)) {
 *       TEST_CHECK(ptr->member1 < 100);
 *       TEST_CHECK(ptr->member2 > 200);
 *   }
 */
#define TEST_CHECK_(cond, ...)                                                 \
    test_check__((cond), __FILE__, __LINE__, __VA_ARGS__)
#define TEST_CHECK(cond) test_check__((cond), __FILE__, __LINE__, "%s", #cond)

/**********************
 *** Implementation ***
 **********************/

/* The unit test files should not rely on anything below. */

#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#if defined(unix) || defined(__unix__) || defined(__unix) || defined(__APPLE__)
#define CUTEST_UNIX__ 1
#include <errno.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <signal.h>
#endif

#if defined(_WIN32) || defined(__WIN32__) || defined(__WINDOWS__)
#define CUTEST_WIN__ 1
#include <windows.h>
#include <io.h>
#endif

#ifdef __cplusplus
#include <exception>
#endif

/* Note our global private identifiers end with '__' to mitigate risk of clash
 * with the unit tests implementation. */

#ifdef __cplusplus
extern "C" {
#endif

struct test__ {
    const char *name;
    void (*func)(void);
};

extern const struct test__ test_list__[];

int test_check__(int cond, const char *file, int line, const char *fmt, ...);

#ifndef TEST_NO_MAIN

static char *test_argv0__ = NULL;
static int test_count__ = 0;
static int test_no_exec__ = 0;
static int test_no_summary__ = 0;
static int test_skip_mode__ = 0;

static int test_stat_failed_units__ = 0;
static int test_stat_run_units__ = 0;

static const struct test__ *test_current_unit__ = NULL;
static int test_current_already_logged__ = 0;
static int test_verbose_level__ = 2;
static int test_current_failures__ = 0;
static int test_colorize__ = 0;

#define CUTEST_COLOR_DEFAULT__ 0
#define CUTEST_COLOR_GREEN__ 1
#define CUTEST_COLOR_RED__ 2
#define CUTEST_COLOR_DEFAULT_INTENSIVE__ 3
#define CUTEST_COLOR_GREEN_INTENSIVE__ 4
#define CUTEST_COLOR_RED_INTENSIVE__ 5

static size_t test_print_in_color__(int color, const char *fmt, ...)
{
    va_list args;
    char buffer[256];
    size_t n;

    va_start(args, fmt);
    vsnprintf(buffer, sizeof(buffer), fmt, args);
    va_end(args);
    buffer[sizeof(buffer) - 1] = '\0';

    if (!test_colorize__) {
        return printf("%s", buffer);
    }

#if defined CUTEST_UNIX__
    {
        const char *col_str;
        switch (color) {
        case CUTEST_COLOR_GREEN__:
            col_str = "\033[0;32m";
            break;
        case CUTEST_COLOR_RED__:
            col_str = "\033[0;31m";
            break;
        case CUTEST_COLOR_GREEN_INTENSIVE__:
            col_str = "\033[1;32m";
            break;
        case CUTEST_COLOR_RED_INTENSIVE__:
            col_str = "\033[1;30m";
            break;
        case CUTEST_COLOR_DEFAULT_INTENSIVE__:
            col_str = "\033[1m";
            break;
        default:
            col_str = "\033[0m";
            break;
        }
        printf("%s", col_str);
        n = printf("%s", buffer);
        printf("\033[0m");
        return n;
    }
#elif defined CUTEST_WIN__
    {
        HANDLE h;
        CONSOLE_SCREEN_BUFFER_INFO info;
        WORD attr;

        h = GetStdHandle(STD_OUTPUT_HANDLE);
        GetConsoleScreenBufferInfo(h, &info);

        switch (color) {
        case CUTEST_COLOR_GREEN__:
            attr = FOREGROUND_GREEN;
            break;
        case CUTEST_COLOR_RED__:
            attr = FOREGROUND_RED;
            break;
        case CUTEST_COLOR_GREEN_INTENSIVE__:
            attr = FOREGROUND_GREEN | FOREGROUND_INTENSITY;
            break;
        case CUTEST_COLOR_RED_INTENSIVE__:
            attr = FOREGROUND_RED | FOREGROUND_INTENSITY;
            break;
        case CUTEST_COLOR_DEFAULT_INTENSIVE__:
            attr = FOREGROUND_BLUE | FOREGROUND_GREEN | FOREGROUND_RED |
                   FOREGROUND_INTENSITY;
            break;
        default:
            attr = 0;
            break;
        }
        if (attr != 0)
            SetConsoleTextAttribute(h, attr);
        n = printf("%s", buffer);
        SetConsoleTextAttribute(h, info.wAttributes);
        return n;
    }
#else
    n = printf("%s", buffer);
    return n;
#endif
}

int test_check__(int cond, const char *file, int line, const char *fmt, ...)
{
    const char *result_str;
    int result_color;
    int verbose_level;

    if (cond) {
        result_str = "ok";
        result_color = CUTEST_COLOR_GREEN__;
        verbose_level = 3;
    } else {
        if (!test_current_already_logged__ && test_current_unit__ != NULL) {
            printf("[ ");
            test_print_in_color__(CUTEST_COLOR_RED_INTENSIVE__, "FAILED");
            printf(" ]\n");
        }
        result_str = "failed";
        result_color = CUTEST_COLOR_RED__;
        verbose_level = 2;
        test_current_failures__++;
        test_current_already_logged__++;
    }

    if (test_verbose_level__ >= verbose_level) {
        va_list args;

        printf("  ");

        if (file != NULL)
            printf("%s:%d: Check ", file, line);

        va_start(args, fmt);
        vprintf(fmt, args);
        va_end(args);

        printf("... ");
        test_print_in_color__(result_color, result_str);
        printf("\n");
        test_current_already_logged__++;
    }

    return (cond != 0);
}

static void test_list_names__(void)
{
    const struct test__ *test;

    printf("Unit tests:\n");
    for (test = &test_list__[0]; test->func != NULL; test++)
        printf("  %s\n", test->name);
}

static const struct test__ *test_by_name__(const char *name)
{
    const struct test__ *test;

    for (test = &test_list__[0]; test->func != NULL; test++) {
        if (strcmp(test->name, name) == 0)
            return test;
    }

    return NULL;
}

/* Call directly the given test unit function. */
static int test_do_run__(const struct test__ *test)
{
    test_current_unit__ = test;
    test_current_failures__ = 0;
    test_current_already_logged__ = 0;

    if (test_verbose_level__ >= 3) {
        test_print_in_color__(CUTEST_COLOR_DEFAULT_INTENSIVE__, "Test %s:\n",
                              test->name);
        test_current_already_logged__++;
    } else if (test_verbose_level__ >= 1) {
        size_t n;
        char spaces[32];

        n = test_print_in_color__(CUTEST_COLOR_DEFAULT_INTENSIVE__,
                                  "Test %s... ", test->name);
        memset(spaces, ' ', sizeof(spaces));
        if (n < sizeof(spaces))
            printf("%.*s", (int)(sizeof(spaces) - n), spaces);
    } else {
        test_current_already_logged__ = 1;
    }

#ifdef __cplusplus
    try {
#endif

        /* This is good to do for case the test unit e.g. crashes. */
        fflush(stdout);
        fflush(stderr);

        test->func();

#ifdef __cplusplus
    } catch (std::exception &e) {
        const char *what = e.what();
        if (what != NULL)
            test_check__(0, NULL, 0, "Threw std::exception: %s", what);
        else
            test_check__(0, NULL, 0, "Threw std::exception");
    } catch (...) {
        test_check__(0, NULL, 0, "Threw an exception");
    }
#endif

    if (test_verbose_level__ >= 3) {
        switch (test_current_failures__) {
        case 0:
            test_print_in_color__(CUTEST_COLOR_GREEN_INTENSIVE__,
                                  "  All conditions have passed.\n\n");
            break;
        case 1:
            test_print_in_color__(CUTEST_COLOR_RED_INTENSIVE__,
                                  "  One condition has FAILED.\n\n");
            break;
        default:
            test_print_in_color__(CUTEST_COLOR_RED_INTENSIVE__,
                                  "  %d conditions have FAILED.\n\n",
                                  test_current_failures__);
            break;
        }
    } else if (test_verbose_level__ >= 1 && test_current_failures__ == 0) {
        printf("[   ");
        test_print_in_color__(CUTEST_COLOR_GREEN_INTENSIVE__, "OK");
        printf("   ]\n");
    }

    test_current_unit__ = NULL;
    return (test_current_failures__ == 0) ? 0 : -1;
}

#if defined(CUTEST_UNIX__) || defined(CUTEST_WIN__)
/* Called if anything goes bad in cutest, or if the unit test ends in other
 * way then by normal returning from its function (e.g. exception or some
 * abnormal child process termination). */
static void test_error__(const char *fmt, ...)
{
    va_list args;

    if (test_verbose_level__ == 0)
        return;

    if (test_verbose_level__ <= 2 && !test_current_already_logged__ &&
        test_current_unit__ != NULL) {
        printf("[ ");
        test_print_in_color__(CUTEST_COLOR_RED_INTENSIVE__, "FAILED");
        printf(" ]\n");
    }

    if (test_verbose_level__ >= 2) {
        test_print_in_color__(CUTEST_COLOR_RED_INTENSIVE__, "  Error: ");
        va_start(args, fmt);
        vprintf(fmt, args);
        va_end(args);
        printf("\n");
    }
}
#endif

/* Trigger the unit test. If possible (and not suppressed) it starts a child
 * process who calls test_do_run__(), otherwise it calls test_do_run__()
 * directly. */
static void test_run__(const struct test__ *test)
{
    int failed = 1;

    test_current_unit__ = test;
    test_current_already_logged__ = 0;

    if (!test_no_exec__) {
#if defined(CUTEST_UNIX__)

        pid_t pid;
        int exit_code;

        pid = fork();
        if (pid == (pid_t)-1) {
            test_error__("Cannot fork. %s [%d]", strerror(errno), errno);
            failed = 1;
        } else if (pid == 0) {
            /* Child: Do the test. */
            failed = (test_do_run__(test) != 0);
            exit(failed ? 1 : 0);
        } else {
            /* Parent: Wait until child terminates and analyze its exit code. */
            waitpid(pid, &exit_code, 0);
            if (WIFEXITED(exit_code)) {
                switch (WEXITSTATUS(exit_code)) {
                case 0:
                    failed = 0;
                    break; /* test has passed. */
                case 1:    /* noop */
                    break; /* "normal" failure. */
                default:
                    test_error__("Unexpected exit code [%d]",
                                 WEXITSTATUS(exit_code));
                }
            } else if (WIFSIGNALED(exit_code)) {
                char tmp[32];
                const char *signame;
                switch (WTERMSIG(exit_code)) {
                case SIGINT:
                    signame = "SIGINT";
                    break;
                case SIGHUP:
                    signame = "SIGHUP";
                    break;
                case SIGQUIT:
                    signame = "SIGQUIT";
                    break;
                case SIGABRT:
                    signame = "SIGABRT";
                    break;
                case SIGKILL:
                    signame = "SIGKILL";
                    break;
                case SIGSEGV:
                    signame = "SIGSEGV";
                    break;
                case SIGILL:
                    signame = "SIGILL";
                    break;
                case SIGTERM:
                    signame = "SIGTERM";
                    break;
                default:
                    snprintf(tmp, sizeof(tmp), "signal %d",
                             WTERMSIG(exit_code));
                    signame = tmp;
                    break;
                }
                test_error__("Test interrupted by %s", signame);
            } else {
                test_error__("Test ended in an unexpected way [%d]", exit_code);
            }
        }

#elif defined(CUTEST_WIN__)

        char buffer[512] = { 0 };
        STARTUPINFOA startupInfo = { 0 };
        PROCESS_INFORMATION processInfo;
        DWORD exitCode;

        /* Windows has no fork(). So we propagate all info into the child
         * through a command line arguments. */
        _snprintf(buffer, sizeof(buffer) - 1,
                  "%s --no-exec --no-summary --verbose=%d --color=%s -- \"%s\"",
                  test_argv0__, test_verbose_level__,
                  test_colorize__ ? "always" : "never", test->name);
        startupInfo.cb = sizeof(STARTUPINFO);
        if (CreateProcessA(NULL, buffer, NULL, NULL, FALSE, 0, NULL, NULL,
                           &startupInfo, &processInfo)) {
            WaitForSingleObject(processInfo.hProcess, INFINITE);
            GetExitCodeProcess(processInfo.hProcess, &exitCode);
            CloseHandle(processInfo.hThread);
            CloseHandle(processInfo.hProcess);
            failed = (exitCode != 0);
        } else {
            test_error__("Cannot create unit test subprocess [%ld].",
                         GetLastError());
            failed = 1;
        }

#else

        /* A platform where we don't know how to run child process. */
        failed = (test_do_run__(test) != 0);

#endif

    } else {
        /* Child processes suppressed through --no-exec. */
        failed = (test_do_run__(test) != 0);
    }

    test_current_unit__ = NULL;

    test_stat_run_units__++;
    if (failed)
        test_stat_failed_units__++;
}

#if defined(CUTEST_WIN__)
/* Callback for SEH events. */
static LONG CALLBACK test_exception_filter__(EXCEPTION_POINTERS *ptrs)
{
    test_error__("Unhandled SEH exception %08lx at %p.",
                 ptrs->ExceptionRecord->ExceptionCode,
                 ptrs->ExceptionRecord->ExceptionAddress);
    fflush(stdout);
    fflush(stderr);
    return EXCEPTION_EXECUTE_HANDLER;
}
#endif

static void test_help__(void)
{
    printf("Usage: %s [options] [test...]\n", test_argv0__);
    printf("Run the specified unit tests; or if the option '--skip' is used, "
           "run all\n");
    printf("tests in the suite but those listed.  By default, if no tests are "
           "specified\n");
    printf("on the command line, all unit tests in the suite are run.\n");
    printf("\n");
    printf("Options:\n");
    printf(
        "  -s, --skip            Execute all unit tests but the listed ones\n");
    printf("      --no-exec         Do not execute unit tests as child "
           "processes\n");
    printf(
        "      --no-summary      Suppress printing of test results summary\n");
    printf("  -l, --list            List unit tests in the suite and exit\n");
    printf("  -v, --verbose         Enable more verbose output\n");
    printf("      --verbose=LEVEL   Set verbose level to LEVEL:\n");
    printf("                          0 ... Be silent\n");
    printf("                          1 ... Output one line per test (and "
           "summary)\n");
    printf("                          2 ... As 1 and failed conditions (this "
           "is default)\n");
    printf("                          3 ... As 1 and all conditions (and "
           "extended summary)\n");
    printf("      --color=WHEN      Enable colorized output (WHEN is one of "
           "'auto', 'always', 'never')\n");
    printf("  -h, --help            Display this help and exit\n");
    printf("\n");
    test_list_names__();
}

int main(int argc, char **argv)
{
    const struct test__ **tests = NULL;
    int i, j, n = 0;
    int seen_double_dash = 0;

    test_argv0__ = argv[0];

#if defined CUTEST_UNIX__
    test_colorize__ = isatty(STDOUT_FILENO);
#elif defined CUTEST_WIN__
    test_colorize__ = _isatty(_fileno(stdout));
#else
    test_colorize__ = 0;
#endif

    /* Parse options */
    for (i = 1; i < argc; i++) {
        if (seen_double_dash || argv[i][0] != '-') {
            tests = (const struct test__ **)realloc(
                (void *)tests, (n + 1) * sizeof(const struct test__ *));
            if (tests == NULL) {
                fprintf(stderr, "Out of memory.\n");
                exit(2);
            }
            tests[n] = test_by_name__(argv[i]);
            if (tests[n] == NULL) {
                fprintf(stderr, "%s: Unrecognized unit test '%s'\n", argv[0],
                        argv[i]);
                fprintf(stderr, "Try '%s --list' for list of unit tests.\n",
                        argv[0]);
                exit(2);
            }
            n++;
        } else if (strcmp(argv[i], "--") == 0) {
            seen_double_dash = 1;
        } else if (strcmp(argv[i], "--help") == 0 ||
                   strcmp(argv[i], "-h") == 0) {
            test_help__();
            exit(0);
        } else if (strcmp(argv[i], "--verbose") == 0 ||
                   strcmp(argv[i], "-v") == 0) {
            test_verbose_level__++;
        } else if (strncmp(argv[i], "--verbose=", 10) == 0) {
            test_verbose_level__ = atoi(argv[i] + 10);
        } else if (strcmp(argv[i], "--color=auto") == 0) {
            /* noop (set from above) */
        } else if (strcmp(argv[i], "--color=always") == 0 ||
                   strcmp(argv[i], "--color") == 0) {
            test_colorize__ = 1;
        } else if (strcmp(argv[i], "--color=never") == 0) {
            test_colorize__ = 0;
        } else if (strcmp(argv[i], "--skip") == 0 ||
                   strcmp(argv[i], "-s") == 0) {
            test_skip_mode__ = 1;
        } else if (strcmp(argv[i], "--no-exec") == 0) {
            test_no_exec__ = 1;
        } else if (strcmp(argv[i], "--no-summary") == 0) {
            test_no_summary__ = 1;
        } else if (strcmp(argv[i], "--list") == 0 ||
                   strcmp(argv[i], "-l") == 0) {
            test_list_names__();
            exit(0);
        } else {
            fprintf(stderr, "%s: Unrecognized option '%s'\n", argv[0], argv[i]);
            fprintf(stderr, "Try '%s --help' for more information.\n", argv[0]);
            exit(2);
        }
    }

#if defined(CUTEST_WIN__)
    SetUnhandledExceptionFilter(test_exception_filter__);
#endif

    /* Count all test units */
    test_count__ = 0;
    for (i = 0; test_list__[i].func != NULL; i++)
        test_count__++;

    /* Run the tests */
    if (n == 0) {
        /* Run all tests */
        for (i = 0; test_list__[i].func != NULL; i++)
            test_run__(&test_list__[i]);
    } else if (!test_skip_mode__) {
        /* Run the listed tests */
        for (i = 0; i < n; i++)
            test_run__(tests[i]);
    } else {
        /* Run all tests except those listed */
        for (i = 0; test_list__[i].func != NULL; i++) {
            int want_skip = 0;
            for (j = 0; j < n; j++) {
                if (tests[j] == &test_list__[i]) {
                    want_skip = 1;
                    break;
                }
            }
            if (!want_skip)
                test_run__(&test_list__[i]);
        }
    }

    /* Write a summary */
    if (!test_no_summary__ && test_verbose_level__ >= 1) {
        test_print_in_color__(CUTEST_COLOR_DEFAULT_INTENSIVE__, "\nSummary:\n");

        if (test_verbose_level__ >= 3) {
            printf("  Count of all unit tests:     %4d\n", test_count__);
            printf("  Count of run unit tests:     %4d\n",
                   test_stat_run_units__);
            printf("  Count of failed unit tests:  %4d\n",
                   test_stat_failed_units__);
            printf("  Count of skipped unit tests: %4d\n",
                   test_count__ - test_stat_run_units__);
        }

        if (test_stat_failed_units__ == 0) {
            test_print_in_color__(CUTEST_COLOR_GREEN_INTENSIVE__,
                                  "  SUCCESS: All unit tests have passed.\n");
        } else {
            test_print_in_color__(
                CUTEST_COLOR_RED_INTENSIVE__,
                "  FAILED: %d of %d unit tests have failed.\n",
                test_stat_failed_units__, test_stat_run_units__);
        }
    }

    if (tests != NULL)
        free((void *)tests);

    return (test_stat_failed_units__ == 0) ? 0 : 1;
}

#endif /* #ifndef TEST_NO_MAIN */

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* #ifndef CUTEST_H__ */
