 // Copyright 2013 The Chromium Authors
 // Use of this source code is governed by a BSD-style license that can be
 // found in the LICENSE file.
 // The categories of events.
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
#define FACILITY_HOST                    0x0


//
// Define the severity codes
//
#define STATUS_SEVERITY_SUCCESS          0x0
#define STATUS_SEVERITY_INFORMATIONAL    0x1
#define STATUS_SEVERITY_WARNING          0x2
#define STATUS_SEVERITY_ERROR            0x3


//
// MessageId: HOST_CATEGORY
//
// MessageText:
//
//
#define HOST_CATEGORY                    ((WORD)0x00000001L)

 // The message definitions.
//
// MessageId: MSG_HOST_CLIENT_CONNECTED
//
// MessageText:
//
//
#define MSG_HOST_CLIENT_CONNECTED        ((DWORD)0x40000001L)

//
// MessageId: MSG_HOST_CLIENT_DISCONNECTED
//
// MessageText:
//
//
#define MSG_HOST_CLIENT_DISCONNECTED     ((DWORD)0x40000002L)

//
// MessageId: MSG_HOST_CLIENT_ACCESS_DENIED
//
// MessageText:
//
//
#define MSG_HOST_CLIENT_ACCESS_DENIED    ((DWORD)0xC0000003L)

//
// MessageId: MSG_HOST_CLIENT_ROUTING_CHANGED
//
// MessageText:
//
//
#define MSG_HOST_CLIENT_ROUTING_CHANGED  ((DWORD)0x40000004L)

//
// MessageId: MSG_HOST_STARTED
//
// MessageText:
//
//
#define MSG_HOST_STARTED                 ((DWORD)0x40000005L)

//
// MessageId: MSG_HOST_LOG_EVENT
//
// MessageText:
//
// %1
//
#define MSG_HOST_LOG_EVENT               ((DWORD)0x40000006L)

 // This line makes sure that mc.exe does not complain about a single '.' at
 // the end of the file.