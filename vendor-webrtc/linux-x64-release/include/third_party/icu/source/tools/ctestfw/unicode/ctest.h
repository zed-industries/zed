// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
 ********************************************************************************
 *
 *   Copyright (C) 1996-2013, International Business Machines
 *   Corporation and others.  All Rights Reserved.
 *
 ********************************************************************************
 */

#ifndef CTEST_H
#define CTEST_H

#include "unicode/testtype.h"
#include "unicode/utrace.h"


/* prototypes *********************************/

U_CDECL_BEGIN
typedef void (U_CALLCONV *TestFunctionPtr)(void);
typedef int (U_CALLCONV *ArgHandlerPtr)(int arg, int argc, const char* const argv[], void *context);
typedef struct TestNode TestNode;
U_CDECL_END

/**
 * This is use to set or get the option value for REPEAT_TESTS.
 * Use with set/getTestOption().
 *
 * @internal
 */
#define REPEAT_TESTS_OPTION 1

/**
 * This is use to set or get the option value for VERBOSITY.
 * When option is set to zero to disable log_verbose() messages.
 * Otherwise nonzero to see log_verbose() messages.
 * Use with set/getTestOption().
 *
 * @internal
 */
#define VERBOSITY_OPTION 2

/**
 * This is use to set or get the option value for ERR_MSG.
 * Use with set/getTestOption().
 *
 * @internal
 */
#define ERR_MSG_OPTION 3

/**
 * This is use to set or get the option value for QUICK.
 * When option is zero, disable some of the slower tests.
 * Otherwise nonzero to run the slower tests.
 * Use with set/getTestOption().
 *
 * @internal
 */
#define QUICK_OPTION 4

/**
 * This is use to set or get the option value for WARN_ON_MISSING_DATA.
 * When option is nonzero, warn on missing data.
 * Otherwise, errors are propagated when data is not available.
 * Affects the behavior of log_dataerr.
 * Use with set/getTestOption().
 *
 * @see log_data_err
 * @internal
 */
#define WARN_ON_MISSING_DATA_OPTION 5

/**
 * This is use to set or get the option value for ICU_TRACE.
 * ICU tracing level, is set by command line option.
 * Use with set/getTestOption().
 *
 * @internal
 */
#define ICU_TRACE_OPTION 6

/**
 * This is used to set or get the option value for WRITE_GOLDEN_DATA.
 * Set to 1 to overwrite golden data files, such as those in testdata/ucptrie.
 * Use with set/getTestOption().
 */
#define WRITE_GOLDEN_DATA_OPTION 7

/**
 * Maximum amount of memory uprv_malloc should allocate before returning NULL.
 *
 * @internal
 */
extern T_CTEST_EXPORT_API size_t MAX_MEMORY_ALLOCATION;

/**
 * If memory tracing was enabled, contains the number of unfreed allocations.
 *
 * @internal
 */
extern T_CTEST_EXPORT_API int32_t ALLOCATION_COUNT;

/**
 * Pass to setTestOption to decrement the test option value.
 *
 * @internal
 */
#define DECREMENT_OPTION_VALUE -99

/**
 * Gets the test option set on commandline.
 *
 * @param testOption macro definition for the individual test option
 * @return value of test option, zero if option is not set or off
 * @internal Internal APIs for testing purpose only
 */
T_CTEST_API int32_t T_CTEST_EXPORT2
getTestOption ( int32_t testOption );

/**
 * Sets the test option with value given on commandline.
 *
 * @param testOption macro definition for the individual test option
 * @param value to set the test option to
 * @internal Internal APIs for testing purpose only
 */
T_CTEST_API void T_CTEST_EXPORT2
setTestOption ( int32_t testOption, int32_t value);

/**
 * Show the names of all nodes.
 *
 * @param root Subtree of tests.
 * @internal Internal APIs for testing purpose only
 */
T_CTEST_API void T_CTEST_EXPORT2
showTests ( const TestNode *root);

/**
 * Run a subtree of tests.
 *
 * @param root Subtree of tests.
 * @internal Internal APIs for testing purpose only
 */
T_CTEST_API void T_CTEST_EXPORT2
runTests ( const TestNode* root);

/**
 * Add a test to the subtree.
 * Example usage:
 * <PRE>
 *     TestNode* root=NULL;
 *     addTest(&root, &mytest, "/a/b/mytest" );
 * </PRE>
 * @param root Pointer to the root pointer.
 * @param test Pointer to 'void function(void)' for actual test.
 * @param path Path from root under which test will be placed. Ex. '/a/b/mytest'
 * @internal Internal APIs for testing purpose only
 */
T_CTEST_API void T_CTEST_EXPORT2
addTest(TestNode** root,
        TestFunctionPtr test,
        const char *path);

/**
 * Clean up any allocated memory.
 * Conditions for calling this function are the same as u_cleanup().
 * @see u_cleanup
 * @internal Internal APIs for testing purpose only
 */
T_CTEST_API void T_CTEST_EXPORT2
cleanUpTestTree(TestNode *tn);

/**
 * Retrieve a specific subtest. (subtree).
 *
 * @param root Pointer to the root.
 * @param path Path relative to the root, Ex. '/a/b'
 * @return The subtest, or NULL on failure.
 * @internal Internal APIs for testing purpose only
 */
T_CTEST_API const TestNode* T_CTEST_EXPORT2
getTest(const TestNode* root,
        const char *path);


/**
 * Log an error message. (printf style)
 * @param pattern printf-style format string
 * @internal Internal APIs for testing purpose only
 */
T_CTEST_API void T_CTEST_EXPORT2
log_err(const char* pattern, ...);

T_CTEST_API void T_CTEST_EXPORT2
log_err_status(UErrorCode status, const char* pattern, ...);
/**
 * Log an informational message. (printf style)
 * @param pattern printf-style format string
 * @internal Internal APIs for testing purpose only
 */
T_CTEST_API void T_CTEST_EXPORT2
log_info(const char* pattern, ...);

/**
 * Log an informational message. (vprintf style)
 * @param prefix a string that is output before the pattern and without formatting
 * @param pattern printf-style format string
 * @param ap variable-arguments list
 * @internal Internal APIs for testing purpose only
 */
T_CTEST_API void T_CTEST_EXPORT2
vlog_info(const char *prefix, const char *pattern, va_list ap);

/**
 * Log a verbose informational message. (printf style)
 * This message will only appear if the global VERBOSITY is nonzero
 * @param pattern printf-style format string
 * @internal Internal APIs for testing purpose only
 */
T_CTEST_API void T_CTEST_EXPORT2
log_verbose(const char* pattern, ...);

/**
 * Log an error message concerning missing data. (printf style)
 * If WARN_ON_MISSING_DATA is nonzero, this will case a log_info (warning) to be
 * printed, but if it is zero this will produce an error (log_err).
 * @param pattern printf-style format string
 * @internal Internal APIs for testing purpose only
 */
T_CTEST_API void T_CTEST_EXPORT2
log_data_err(const char *pattern, ...);

/**
 * Log a known issue.
 * @param ticket ticket number such as "ICU-12345" for ICU tickets or "CLDR-6636" for CLDR tickets.
 * @param fmt ...  sprintf-style format, optional message. can be NULL.
 * @return true if known issue test should be skipped, false if it should be run
 */
T_CTEST_API UBool
T_CTEST_EXPORT2
log_knownIssue(const char *ticket, const char *fmt, ...);

/**
 * Initialize the variables above. This allows the test to set up accordingly
 * before running the tests.
 * This must be called before runTests.
 */
T_CTEST_API int T_CTEST_EXPORT2 
initArgs( int argc, const char* const argv[], ArgHandlerPtr argHandler, void *context);

/**
 * Processes the command line arguments.
 * This is a sample implementation
 * <PRE>Usage: %s [ -l ] [ -v ] [ -? ] [ /path/to/test ]
 *        -l List only, do not run\
 *        -v turn OFF verbosity
 *        -? print this message</PRE>
 * @param root Testnode root with tests already attached to it
 * @param argv argument list from main (stdio.h)
 * @param argc argument list count from main (stdio.h)
 * @return positive for error count, 0 for success, negative for illegal argument
 * @internal Internal APIs for testing purpose only
 */
T_CTEST_API int T_CTEST_EXPORT2 
runTestRequest(const TestNode* root,
            int argc,
            const char* const argv[]);


T_CTEST_API const char* T_CTEST_EXPORT2
getTestName(void);

/**
 * Append a time delta to str if it is significant (>5 ms) otherwise no change
 * @param delta a delta in millis
 * @param str a string to append to.
 */
T_CTEST_API void T_CTEST_EXPORT2
str_timeDelta(char *str, UDate delta);


/* ======== XML (JUnit output) ========= */

/**
 * Set the filename for the XML output. 
 * @param fileName file name. Caller must retain storage.
 * @return 0 on success, 1 on failure.
 */
T_CTEST_API int32_t T_CTEST_EXPORT2
ctest_xml_setFileName(const char *fileName);


/**
 * Init XML subsystem. Call ctest_xml_setFileName first
 * @param rootName the test root name to be written
 * @return 0 on success, 1 on failure.
 */
T_CTEST_API int32_t T_CTEST_EXPORT2
ctest_xml_init(const char *rootName);


/**
 * Set the filename for the XML output. Caller must retain storage.
 * @return 0 on success, 1 on failure.
 */
T_CTEST_API int32_t T_CTEST_EXPORT2
ctest_xml_fini(void);


/**
 * report a test case
 * @return 0 on success, 1 on failure.
 */
T_CTEST_API int32_t
T_CTEST_EXPORT2
ctest_xml_testcase(const char *classname, const char *name, const char *time, const char *failMsg);

#endif
