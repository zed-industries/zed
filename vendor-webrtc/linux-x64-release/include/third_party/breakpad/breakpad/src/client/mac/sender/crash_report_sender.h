// Copyright 2006 Google LLC
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//     * Neither the name of Google LLC nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// This component uses the HTTPMultipartUpload of the breakpad project to send
// the minidump and associated data to the crash reporting servers.
// It will perform throttling based on the parameters passed to it and will
// prompt the user to send the minidump.

#import <Cocoa/Cocoa.h>

#include "client/mac/sender/uploader.h"
#import "GTMDefines.h"

// We're sublcassing NSTextField in order to override a particular
// method (see the implementation) that lets us reject changes if they
// are longer than a particular length.  Bindings would normally solve
// this problem, but when we implemented a validation method, and
// returned NO for strings that were too long, the UI was not updated
// right away, which was a poor user experience.  The UI would be
// updated as soon as the text field lost first responder status,
// which isn't soon enough.  It is a known bug that the UI KVO didn't
// work in the middle of a validation.
@interface LengthLimitingTextField : NSTextField {
  @private
   NSUInteger maximumLength_;
}

- (void)setMaximumLength:(NSUInteger)maxLength;
@end

@interface Reporter : NSObject {
 @public
  IBOutlet NSWindow *alertWindow_;        // The alert window

  // Grouping boxes used for resizing.
  IBOutlet NSBox *headerBox_;
  IBOutlet NSBox *preEmailBox_;
  IBOutlet NSBox *emailSectionBox_;
  // Localized elements (or things that need to be moved during localization).
  IBOutlet NSTextField                *dialogTitle_;
  IBOutlet NSTextField                *commentMessage_;
  IBOutlet NSTextField                *emailMessage_;
  IBOutlet NSTextField                *emailLabel_;
  IBOutlet NSTextField                *privacyLinkLabel_;
  IBOutlet NSButton                   *sendButton_;
  IBOutlet NSButton                   *cancelButton_;
  IBOutlet LengthLimitingTextField    *emailEntryField_;
  IBOutlet LengthLimitingTextField    *commentsEntryField_;
  IBOutlet NSTextField                *countdownLabel_;
  IBOutlet NSView                     *privacyLinkArrow_;

  // Text field bindings, for user input.
  NSString *commentsValue_;                // Comments from the user
  NSString *emailValue_;                   // Email from the user
  NSString *countdownMessage_;             // Message indicating time
                                           // left for input.
 @private
  NSTimeInterval remainingDialogTime_;     // Keeps track of how long
                                           // we have until we cancel
                                           // the dialog
  NSTimer *messageTimer_;                  // Timer we use to update
                                           // the dialog
  Uploader* uploader_;                     // Uploader we use to send the data.
}

// Stops the modal panel with an NSAlertDefaultReturn value. This is the action
// invoked by the "Send Report" button.
- (IBAction)sendReport:(id)sender;
// Stops the modal panel with an NSAlertAlternateReturn value. This is the
// action invoked by the "Cancel" button.
- (IBAction)cancel:(id)sender;
// Opens the Privacy Policy url in the default web browser.
- (IBAction)showPrivacyPolicy:(id)sender;

// Delegate methods for the NSTextField for comments. We want to capture the
// Return key and use it to send the message when no text has been entered.
// Otherwise, we want Return to add a carriage return to the comments field.
- (BOOL)control:(NSControl *)control textView:(NSTextView *)textView
                          doCommandBySelector:(SEL)commandSelector;

// Accessors to make bindings work
- (NSString *)commentsValue;
- (void)setCommentsValue:(NSString *)value;

- (NSString *)emailValue;
- (void)setEmailValue:(NSString *)value;

- (NSString *)countdownMessage;
- (void)setCountdownMessage:(NSString *)value;

@end
