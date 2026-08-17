// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
************************************************************************
* Copyright (c) 1997-2012, International Business Machines
* Corporation and others.  All Rights Reserved.
************************************************************************
*/

#ifndef _UTIMER_H
#define _UTIMER_H

#include "unicode/utypes.h"

#if U_PLATFORM_USES_ONLY_WIN32_API
#   define VC_EXTRALEAN
#   define WIN32_LEAN_AND_MEAN
#   include <windows.h>
#else
#   if U_PLATFORM == U_PF_OS390 && !defined(__UU)
#     define __UU  /* Universal Unix - for struct timeval */
#   endif
#   include <time.h>
#   include <sys/time.h> 
#   include <unistd.h> 
#endif

/**
 * This API provides functions for performing performance measurement
 * There are 3 main usage scenarios.
 * i) Loop until a threshold time is reached:
 *    Example:
 *    <code>
 *      typedef Params Params;
 *      struct Params{
 *          char16_t* target;
 *          int32_t targetLen;
 *          const char16_t* source;
 *          int32_t sourceLen;
 *          UNormalizationMode mode;
 *      }
 *      void NormFn( void* param){
 *          Params* parameters = ( Params*) param;
 *          UErrorCode error = U_ZERO_ERROR;
 *          unorm_normalize(parameters->source, parameters->sourceLen, parameters->mode, 0, parameters->target, parameters->targetLen, &error);
 *          if(U_FAILURE(error)){
 *              printf("Normalization failed\n");
 *          }
 *      }
 *  
 *      int main(){
 *          // time the normalization function 
 *          double timeTaken = 0;
 *          Params param;
 *          param.source  // set up the source buffer 
 *          param.target   // set up the target buffer
 *          .... so on ...
 *          UTimer timer;
 *          // time the loop for 10 seconds at least and find out the loop count and time taken
 *          timeTaken = utimer_loopUntilDone((double)10,(void*) param, NormFn, &loopCount);
 *      }
 *     </code>
 *
 * ii) Measure the time taken
 *     Example:
 *     <code>
 *      double perfNormalization(NormFn fn,const char* mode,Line* fileLines,int32_t loopCount){
 *          int  line;
 *          int  loops;
 *          UErrorCode error = U_ZERO_ERROR;
 *          char16_t* dest=nullptr;
 *          int32_t destCapacity=0;
 *          int len =-1;
 *          double elapsedTime = 0; 
 *          int retVal=0;
 *
 *          char16_t arr[5000];
 *          dest=arr;
 *          destCapacity = 5000;
 *          UTimer start;
 *
 *          // Initialize cache and ensure the data is loaded.
 *          // This loop checks for errors in Normalization. Once we pass the initialization
 *          // without errors we can safelly assume that there are no errors while timing the 
 *          // function
 *          for (loops=0; loops<10; loops++) {
 *              for (line=0; line < gNumFileLines; line++) {
 *                  if (opt_uselen) {
 *                      len = fileLines[line].len;
 *                  }
 *
 *                  retVal= fn(fileLines[line].name,len,dest,destCapacity,&error);
 *      #if U_PLATFORM_HAS_WIN32_API
 *                  if(retVal==0 ){
 *                      fprintf(stderr,"Normalization of string in Windows API failed for mode %s. ErrorNo: %i at line number %i\n",mode,GetLastError(),line);
 *                      return 0;
 *                  }
 *      #endif
 *                  if(U_FAILURE(error)){
 *                      fprintf(stderr,"Normalization of string in ICU API failed for mode %s. Error: %s at line number %i\n",mode,u_errorName(error),line);
 *                      return 0;
 *                  }
 *        
 *              }
 *          }
 *
 *          //compute the time
 *
 *          utimer_getTime(&start);
 *          for (loops=0; loops<loopCount; loops++) {
 *              for (line=0; line < gNumFileLines; line++) {
 *                  if (opt_uselen) {
 *                      len = fileLines[line].len;
 *                  }
 *
 *                  retVal= fn(fileLines[line].name,len,dest,destCapacity,&error);
 *       
 *              }
 *          }
 *
 *          return utimer_getElapsedSeconds(&start);
 *      }
 *      </code>
 *
 * iii) Let a higher level function do the calculation of confidence levels etc.
 *     Example:
 *     <code>
 *       void perf(UTimer* timer, char16_t* source, int32_t sourceLen, char16_t* target, int32_t targetLen, int32_t loopCount,UNormalizationMode mode, UErrorCode* error){
 *              int32_t loops;
 *              for (loops=0; loops<loopCount; loops++) {
 *                  unorm_normalize(source,sourceLen,target, targetLen,mode,error);
 *              }
 *              utimer_getTime(timer);
 *       }
 *       void main(const char* argsc, int argv){
 *          // read the file and setup the data
 *          // set up options
 *          UTimer start,timer1, timer2, timer3, timer4;
 *          double NFDTimeTaken, NFCTimeTaken, FCDTimeTaken;
 *          switch(opt){
 *              case 0:
 *                  utimer_getTime(start);
 *                  perf(timer1, source,sourceLen, target, targetLen,loopCount,UNORM_NFD,&error);
 *                  NFDTimeTaken = utimer_getDeltaSeconds(start,timer1);
 *              case 1:
 *                  timer_getTime(start);
 *                  perf(timer2,source,sourceLen,target,targetLen,loopCount,UNORM_NFC,&error);
 *                  NFCTimeTaken = utimer_getDeltaSeconds(start,timer2);
 *                  perf(timer3, source, sourceLen, target,targetLen, loopCount, UNORM_FCD,&error);
 *              // ........so on .............          
 *           }
 *          // calculate confidence levels etc and print            
 *
 *       }
 *           
 *     </code>
 *      
 */

typedef struct UTimer UTimer;

typedef void FunctionToBeTimed(void* param);


#if U_PLATFORM_USES_ONLY_WIN32_API

    struct UTimer{
        LARGE_INTEGER start;
        LARGE_INTEGER placeHolder;
    };      
        
static    int uprv_initFrequency(UTimer* timer)
    {
        return QueryPerformanceFrequency(&timer->placeHolder);
    }
static    void uprv_start(UTimer* timer)
    {
        QueryPerformanceCounter(&timer->start);
    }
static    double uprv_delta(UTimer* timer1, UTimer* timer2){
        return ((double)(timer2->start.QuadPart - timer1->start.QuadPart))/((double)timer1->placeHolder.QuadPart);
    }
static    UBool uprv_compareFrequency(UTimer* timer1, UTimer* timer2){
        return (timer1->placeHolder.QuadPart == timer2->placeHolder.QuadPart);
    }

#else

    struct UTimer{
        struct timeval start;
        struct timeval placeHolder;
    };
    
static    int32_t uprv_initFrequency(UTimer* /*timer*/)
    {
        return 0;
    }
static    void uprv_start(UTimer* timer)
    {
        gettimeofday(&timer->start, 0);
    }
static    double uprv_delta(UTimer* timer1, UTimer* timer2){
        double t1, t2;

        t1 =  (double)timer1->start.tv_sec + (double)timer1->start.tv_usec/(1000*1000);
        t2 =  (double)timer2->start.tv_sec + (double)timer2->start.tv_usec/(1000*1000);
        return (t2-t1);
    }
static    UBool uprv_compareFrequency(UTimer* /*timer1*/, UTimer* /*timer2*/){
        return true;
    }

#endif
/**
 * Initializes the timer with the current time
 *
 * @param timer A pointer to UTimer struct to receive the current time
 */
static inline void U_EXPORT2
utimer_getTime(UTimer* timer){
    uprv_initFrequency(timer);
    uprv_start(timer);
}

/**
 * Returns the difference in times between timer1 and timer2 by subtracting
 * timer1's time from timer2's time
 *
 * @param timer1 A pointer to UTimer struct to be used as starting time
 * @param timer2 A pointer to UTimer struct to be used as end time
 * @return Time in seconds
 */
static inline double U_EXPORT2
utimer_getDeltaSeconds(UTimer* timer1, UTimer* timer2){
    if(uprv_compareFrequency(timer1,timer2)){
        return uprv_delta(timer1,timer2);
    }
    /* got error return -1 */
    return -1;
}

/**
 * Returns the time elapsed from the starting time represented by the 
 * UTimer struct pointer passed
 * @param timer A pointer to UTimer struct to be used as starting time
 * @return Time elapsed in seconds
 */
static inline double U_EXPORT2
utimer_getElapsedSeconds(UTimer* timer){
    UTimer temp;
    utimer_getTime(&temp);
    return uprv_delta(timer,&temp);
}

/**
 * Executes the function pointed to for a given time and returns exact time
 * taken and number of iterations of the loop
 * @param thresholTimeVal 
 * @param loopCount output param to receive the number of iterations
 * @param fn    The function to be executed
 * @param param Parameters to be passed to the fn
 * @return the time elapsed in seconds
 */
static inline double U_EXPORT2
utimer_loopUntilDone(double thresholdTimeVal,
                     int32_t* loopCount, 
                     FunctionToBeTimed fn, 
                     void* param){
    UTimer timer;
    double currentVal=0;
    *loopCount = 0;
    utimer_getTime(&timer);
    for(;currentVal<thresholdTimeVal;){
        fn(param);
        currentVal = utimer_getElapsedSeconds(&timer);
        (*loopCount)++;
    }
    return currentVal;
}

#endif

