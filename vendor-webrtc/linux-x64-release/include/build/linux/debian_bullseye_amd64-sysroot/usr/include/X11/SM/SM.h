/*

Copyright 1993, 1998  The Open Group

Permission to use, copy, modify, distribute, and sell this software and its
documentation for any purpose is hereby granted without fee, provided that
the above copyright notice appear in all copies and that both that
copyright notice and this permission notice appear in supporting
documentation.

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL THE
OPEN GROUP BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of The Open Group shall not be
used in advertising or otherwise to promote the sale, use or other dealings
in this Software without prior written authorization from The Open Group.

*/

/*
 * Author: Ralph Mor, X Consortium
 */

#ifndef _SM_H_
#define _SM_H_

/*
 * Protocol Version
 */

#define SmProtoMajor	1
#define SmProtoMinor	0


/*
 * Interact Style
 */

#define SmInteractStyleNone	0
#define SmInteractStyleErrors	1
#define SmInteractStyleAny	2


/*
 * Dialog Type
 */

#define SmDialogError		0
#define SmDialogNormal		1


/*
 * Save Type
 */

#define SmSaveGlobal	0
#define SmSaveLocal	1
#define SmSaveBoth	2


/*
 * Restart Style Hints
 */

#define SmRestartIfRunning	0
#define SmRestartAnyway		1
#define SmRestartImmediately	2
#define SmRestartNever		3


/*
 * Property names
 */

#define SmCloneCommand		"CloneCommand"
#define SmCurrentDirectory	"CurrentDirectory"
#define SmDiscardCommand	"DiscardCommand"
#define SmEnvironment		"Environment"
#define SmProcessID		"ProcessID"
#define SmProgram		"Program"
#define SmRestartCommand	"RestartCommand"
#define SmResignCommand		"ResignCommand"
#define SmRestartStyleHint	"RestartStyleHint"
#define SmShutdownCommand	"ShutdownCommand"
#define SmUserID		"UserID"


/*
 * Property types
 */

#define SmCARD8			"CARD8"
#define SmARRAY8		"ARRAY8"
#define SmLISTofARRAY8		"LISTofARRAY8"


/*
 * SM minor opcodes
 */

#define SM_Error			0
#define SM_RegisterClient 		1
#define SM_RegisterClientReply 		2
#define SM_SaveYourself 		3
#define SM_SaveYourselfRequest 		4
#define SM_InteractRequest 		5
#define SM_Interact 			6
#define SM_InteractDone 		7
#define SM_SaveYourselfDone 		8
#define SM_Die 				9
#define SM_ShutdownCancelled		10
#define SM_CloseConnection 		11
#define SM_SetProperties 		12
#define SM_DeleteProperties 		13
#define SM_GetProperties 		14
#define SM_PropertiesReply 		15
#define SM_SaveYourselfPhase2Request	16
#define SM_SaveYourselfPhase2		17
#define SM_SaveComplete			18

#endif /* _SM_H_ */
