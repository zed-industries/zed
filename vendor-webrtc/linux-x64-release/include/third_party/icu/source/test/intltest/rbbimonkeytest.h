// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*************************************************************************
 * Copyright (c) 2016, International Business Machines
 * Corporation and others. All Rights Reserved.
 *************************************************************************
*/
#ifndef RBBIMONKEYTEST_H
#define RBBIMONKEYTEST_H

#include "unicode/utypes.h"

#if !UCONFIG_NO_BREAK_ITERATION && !UCONFIG_NO_REGULAR_EXPRESSIONS && !UCONFIG_NO_FORMATTING

#include "intltest.h"

#include "unicode/rbbi.h"
#include "unicode/regex.h"
#include "unicode/uniset.h"
#include "unicode/unistr.h"
#include "unicode/uobject.h"

#include "simplethread.h"
#include "ucbuf.h"
#include "uhash.h"
#include "uvector.h"

// RBBI Monkey Test. Run break iterators against randomly generated strings, compare results with
//                   an independent reference implementation.
//
//         The monkey test can be run with parameters, e.g.
//              intltest rbbi/RBBIMonkeyTest@loop=-1,rules=word.txt
//         will run word break testing in an infinite loop.
//         Summary of options
//               rules=name             Test against the named reference rule file.
//                                     Files are found in source/test/testdata/break_rules
//               loop=nnn              Loop nnn times. -1 for no limit. loop of 1 is useful for debugging.
//               seed=nnnn             Random number generator seed. Allows recreation of a failure.
//                                     Error messages include the necessary seed value.
//               verbose               Display details of a failure. Useful for debugging. Use with loop=1.
//               expansions            Debug option, show expansions of rules and sets.
//
//  TODO:
//     Develop a tailoring format.
//     Hook to old tests that use monkey impl to get expected data.
//     Remove old tests.

class BreakRules;       // Forward declaration
class RBBIMonkeyImpl;

/**
 * Test the RuleBasedBreakIterator class giving different rules
 */
class RBBIMonkeyTest: public IntlTest {
  public:
    RBBIMonkeyTest();
    virtual ~RBBIMonkeyTest();

    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par = nullptr ) override;
    void testMonkey();


  private:
    const char *fParams;                  // Copy of user parameters passed in from IntlTest.


    void testRules(const char *ruleFile);
    static UBool getIntParam(UnicodeString name, UnicodeString &params, int64_t &val, UErrorCode &status);
    static UBool getStringParam(UnicodeString name, UnicodeString &params, CharString &dest, UErrorCode &status);
    static UBool getBoolParam(UnicodeString name, UnicodeString &params, UBool &dest, UErrorCode &status);

};

// The following classes are internal to the RBBI Monkey Test implementation.



//  class CharClass    Represents a single character class from the source break rules.
//                     Inherits from UObject because instances are adopted by UHashtable, which ultimately
//                     deletes them using hash's object deleter function.

class CharClass: public UObject {
  public:
    UnicodeString                fName;
    UnicodeString                fOriginalDef;    // set definition as it appeared in user supplied rules.
    UnicodeString                fExpandedDef;    // set definition with any embedded named sets replaced by their defs, recursively.
    LocalPointer<const UnicodeSet>     fSet;
    CharClass(const UnicodeString &name, const UnicodeString &originalDef, const UnicodeString &expandedDef, const UnicodeSet *set) :
            fName(name), fOriginalDef(originalDef), fExpandedDef(expandedDef), fSet(set) {}
};


// class BreakRule    represents a single rule from a set of break rules.
//                    Each rule has the set definitions expanded, and
//                    is compiled to a regular expression.

class BreakRule: public UObject {
  public:
    BreakRule();
    ~BreakRule();
    UnicodeString    fName;                            // Name of the rule.
    UnicodeString    fRule;                            // Rule expression, excluding the name, as written in user source.
    UnicodeString    fExpandedRule;                    // Rule expression after expanding the set definitions.
    LocalPointer<RegexMatcher>  fRuleMatcher;          // Regular expression that matches the rule.
    bool             fInitialMatchOnly = false;        // True if rule begins with '^', meaning no chaining.
};


// class BreakRules    represents a complete set of break rules, possibly tailored,
//                     compiled from testdata break rules.

class BreakRules: public UObject {
  public:
    BreakRules(RBBIMonkeyImpl *monkeyImpl, UErrorCode &status);
    ~BreakRules();

    void compileRules(UCHARBUF *rules, UErrorCode &status);

    const CharClass *getClassForChar(UChar32 c, int32_t *iter=nullptr) const;


    RBBIMonkeyImpl    *fMonkeyImpl;        // Pointer back to the owning MonkeyImpl instance.
    icu::UVector       fBreakRules;        // Contents are of type (BreakRule *).

    LocalUHashtablePointer fCharClasses;   // Key is set name (UnicodeString).
                                           // Value is (CharClass *)
    LocalPointer<UVector>  fCharClassList; // Char Classes, same contents as fCharClasses values,
                                           //   but in a vector so they can be accessed by index.
    UnicodeSet         fDictionarySet;     // Dictionary set, empty if none is defined.
    Locale             fLocale;
    UBreakIteratorType fType;

    CharClass *addCharClass(const UnicodeString &name, const UnicodeString &def, UErrorCode &status);
    void addRule(const UnicodeString &name, const UnicodeString &def, UErrorCode &status);
    bool setKeywordParameter(const UnicodeString &keyword, const UnicodeString &value, UErrorCode &status);
    RuleBasedBreakIterator *createICUBreakIterator(UErrorCode &status);

    LocalPointer<RegexMatcher> fSetRefsMatcher;
    LocalPointer<RegexMatcher> fCommentsMatcher;
    LocalPointer<RegexMatcher> fClassDefMatcher;
    LocalPointer<RegexMatcher> fRuleDefMatcher;
};


// class MonkeyTestData    represents a randomly synthesized test data string together
//                         with the expected break positions obtained by applying
//                         the test break rules.

class MonkeyTestData: public UObject {
  public:
    MonkeyTestData() {}
    ~MonkeyTestData() {}
    void set(BreakRules *rules, IntlTest::icu_rand &rand, UErrorCode &status);
    void clearActualBreaks();
    void dump(int32_t around = -1) const;

    uint32_t               fRandomSeed;        // The initial seed value from the random number genererator.
    const BreakRules      *fBkRules;           // The break rules used to generate this data.
    UnicodeString          fString;            // The text.
    UnicodeString          fExpectedBreaks;    // Breaks as found by the reference rules.
                                               //     Parallel to fString. Non-zero if break preceding.
    UnicodeString          fActualBreaks;      // Breaks as found by ICU break iterator.
    UnicodeString          fRuleForPosition;   // Index into BreakRules.fBreakRules of rule that applied at each position.
                                               // Also parallel to fString.
    UnicodeString          f2ndRuleForPos;     // As above. A 2nd rule applies when the preceding rule
                                               //   didn't cause a break, and a subsequent rule match starts
                                               //   on the last code point of the preceding match.

};




// class RBBIMonkeyImpl     holds (some indirectly) everything associated with running a monkey
//                          test for one set of break rules.
//
//                          When running RBBIMonkeyTest with multiple threads, there is a 1:1 correspondence
//                          between instances of RBBIMonkeyImpl and threads.
//
class RBBIMonkeyImpl: public UObject {
  public:
    RBBIMonkeyImpl(UErrorCode &status);
    ~RBBIMonkeyImpl();

    void setup(const char *ruleFileName, UErrorCode &status);

    void startTest();
    void runTest();
    void join();

    LocalUCHARBUFPointer                 fRuleCharBuffer;         // source file contents of the reference rules.
    LocalPointer<BreakRules>             fRuleSet;
    LocalPointer<RuleBasedBreakIterator> fBI;
    LocalPointer<MonkeyTestData>         fTestData;
    IntlTest::icu_rand                   fRandomGenerator;
    const char                          *fRuleFileName;
    UBool                                fVerbose;                 // True to do long dump of failing data.
    int32_t                              fLoopCount;

    UBool                                fDumpExpansions;          // Debug flag to output epananded form of rules and sets.

    enum CheckDirection {
        FORWARD = 1,
        REVERSE = 2
    };
    void clearActualBreaks();
    void testForwards(UErrorCode &status);
    void testPrevious(UErrorCode &status);
    void testFollowing(UErrorCode &status);
    void testPreceding(UErrorCode &status);
    void testIsBoundary(UErrorCode &status);
    void testIsBoundaryRandom(UErrorCode &status);
    void checkResults(const char *msg, CheckDirection dir, UErrorCode &status);

    class RBBIMonkeyThread: public SimpleThread {
      private:
        RBBIMonkeyImpl *fMonkeyImpl;
      public:
        RBBIMonkeyThread(RBBIMonkeyImpl *impl) : fMonkeyImpl(impl) {}
        void run() override { fMonkeyImpl->runTest(); }
    };
  private:
    void openBreakRules(const char *fileName, UErrorCode &status);
    RBBIMonkeyThread fThread;

};

#endif /* !UCONFIG_NO_BREAK_ITERATION && !UCONFIG_NO_REGULAR_EXPRESSIONS && !UCONFIG_NO_FORMATTING */

#endif  //  RBBIMONKEYTEST_H
