// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
//
// Defines the names and types of messages that are logged with the SYSLOG
// macro.
// TODO(pastarmovj): Subdivide into more categories if needed.
//
//  Values are 32 bit values laid out as follows:
//
//   3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1
//   1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
//  +---+-+-+-----------------------+-------------------------------+
//  |Sev|C|R|     Facility          |               Code            |
//  +---+-+-+-----------------------+-------------------------------+
//
//  where
//
//      Sev - is the severity code
//
//          00 - Success
//          01 - Informational
//          10 - Warning
//          11 - Error
//
//      C - is the Customer code flag
//
//      R - is a reserved bit
//
//      Facility - is the facility code
//
//      Code - is the facility's status code
//
//
// Define the facility codes
//
#define FACILITY_SYSTEM                  0x0
#define FACILITY_ELEVATION_SERVICE       0x1
#define FACILITY_TRACING_SERVICE         0x2


//
// Define the severity codes
//
#define STATUS_SEVERITY_INFORMATIONAL    0x0
#define STATUS_SEVERITY_WARNING          0x1
#define STATUS_SEVERITY_ERROR            0x2
#define STATUS_SEVERITY_FATAL            0x3


//
// MessageId: BROWSER_CATEGORY
//
// MessageText:
//
// Browser Events
//
#define BROWSER_CATEGORY                 ((WORD)0x00000001L)

//
// MessageId: ELEVATION_SERVICE_CATEGORY
//
// MessageText:
//
// Elevation Service Events
//
#define ELEVATION_SERVICE_CATEGORY       ((WORD)0x00000002L)

//
// MessageId: TRACING_SERVICE_CATEGORY
//
// MessageText:
//
// ETW Service Events
//
#define TRACING_SERVICE_CATEGORY         ((WORD)0x00000003L)

//
// MessageId: MSG_LOG_MESSAGE
//
// MessageText:
//
// %1!S!
//
#define MSG_LOG_MESSAGE                  ((DWORD)0x80000100L)

//
// MessageId: MSG_ELEVATION_SERVICE_LOG_MESSAGE
//
// MessageText:
//
// %1!S!
//
#define MSG_ELEVATION_SERVICE_LOG_MESSAGE ((DWORD)0x80010101L)

//
// MessageId: MSG_TRACING_SERVICE_LOG_MESSAGE
//
// MessageText:
//
// %1!S!
//
#define MSG_TRACING_SERVICE_LOG_MESSAGE  ((DWORD)0x80020102L)

