//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//
//  testfilerunner.h
//  testObjects
//
//  Created by Blaine Garst on 9/24/08.
//

#import <Cocoa/Cocoa.h>

/*
    variations:
        four source types:  C, ObjC, C++, ObjC++,
          and for ObjC or ObjC++ we have
             RR and GC capabilities
        we assume C++ friendly includes for C/ObjC even if C++ isn't used
             
             
        four compilers: C, ObjC, C++, ObjC++
          and for ObjC or ObjC++ we can compile
              RR, RR+GC, GC+RR, GC
          although to test RR+GC we need to build a shell "main" in both modes
          and/or run with GC disabled if possible.
              
    To maximize coverage we mark files with capabilities and then ask them to be
    compiled with each variation of compiler and option.
    If the file doesn't have the capability it politely refuses.
*/

enum options {
    Do64   = (1 << 0),
    DoCPP  = (1 << 1),
    DoOBJC = (1 << 3),
    DoGC   = (1 << 4),
    DoRR   = (1 << 5),
    DoRRGC = (1 << 6),  // -fobjc-gc but main w/o so it runs in RR mode
    DoGCRR = (1 << 7),  // -fobjc-gc & run GC mode

    //DoDashG = (1 << 8),
    DoDashO = (1 << 9),
    DoDashOs = (1 << 10),
    DoDashO2 = (1 << 11),
    
    DoC99 = (1 << 12), // -std=c99
};


@class TestFileExeGenerator;

// this class will actually compile and/or run a target binary
// XXX we don't track which dynamic libraries requested/used nor set them up
@interface TestFileExe : NSObject {
    NSPointerArray *compileLine;
    int options;
    bool shouldFail;
    TestFileExeGenerator *generator;
    __strong char *binaryName;
    __strong char *sourceName;
    __strong char *libraryPath;
    __strong char *frameworkPath;
}
@property int options;
@property(assign) NSPointerArray *compileLine;
@property(assign) TestFileExeGenerator *generator;
@property bool shouldFail;
@property __strong char *binaryName;
@property __strong char *sourceName;
@property __strong char *libraryPath;
@property __strong char *frameworkPath;
- (bool) compileUnlessExists:(bool)skip;
- (bool) run;
@property(readonly) __strong char *radar;
@end

// this class generates an appropriate set of configurations to compile
// we don't track which gcc we use but we should XXX
@interface TestFileExeGenerator : NSObject {
    bool hasObjC;
    bool hasRR;
    bool hasGC;
    bool hasCPlusPlus;
    bool wantsC99;
    bool wants64;
    bool wants32;
    bool supposedToNotCompile;
    bool open;              // this problem is still open - e.g. unresolved
    __strong char *radar; // for things already known to go wrong
    __strong char *filename;
    __strong char *compilerPath;
    __strong char *errorString;
    __strong char *warningString;
    NSPointerArray *extraLibraries;
}
@property bool hasObjC, hasRR, hasGC, hasCPlusPlus, wantsC99, supposedToNotCompile, open, wants32, wants64;
@property(assign) __strong char *radar;
@property __strong char *filename;
@property __strong char *compilerPath;
@property __strong char *errorString;
@property __strong char *warningString;
- (TestFileExe *)lineForOptions:(int)options; // nil if no can do
+ (NSArray *)generatorsFromFILE:(FILE *)fd;
+ (NSArray *)generatorsFromPath:(NSString *)path;
@end


