// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_EDITING_COMMAND_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_EDITING_COMMAND_TYPE_H_

namespace blink {

// Enum values are used in user metrics, do not modify. If you add new commands,
// you should assign new values and update MappedEditingCommands
// in tools/metrics/histograms/histograms.xml, and add the new command to
// EditorCommand.cpp.
enum class EditingCommandType {
  kInvalid = 0,
  kAlignJustified = 1,
  kAlignLeft = 2,
  kAlignRight = 3,
  kBackColor = 4,
  kBackwardDelete = 5,
  kBold = 6,
  kCopy = 7,
  kCreateLink = 8,
  kCut = 9,
  kDefaultParagraphSeparator = 10,
  kDelete = 11,
  kDeleteBackward = 12,
  kDeleteBackwardByDecomposingPreviousCharacter = 13,
  kDeleteForward = 14,
  kDeleteToBeginningOfLine = 15,
  kDeleteToBeginningOfParagraph = 16,
  kDeleteToEndOfLine = 17,
  kDeleteToEndOfParagraph = 18,
  kDeleteToMark = 19,
  kDeleteWordBackward = 20,
  kDeleteWordForward = 21,
  kFindString = 22,
  kFontName = 23,
  kFontSize = 24,
  kFontSizeDelta = 25,
  kForeColor = 26,
  kFormatBlock = 27,
  kForwardDelete = 28,
  kHiliteColor = 29,
  kIgnoreSpelling = 30,
  kIndent = 31,
  kInsertBacktab = 32,
  kInsertHTML = 33,
  kInsertHorizontalRule = 34,
  kInsertImage = 35,
  kInsertLineBreak = 36,
  kInsertNewline = 37,
  kInsertNewlineInQuotedContent = 38,
  kInsertOrderedList = 39,
  kInsertParagraph = 40,
  kInsertTab = 41,
  kInsertText = 42,
  kInsertUnorderedList = 43,
  kItalic = 44,
  kJustifyCenter = 45,
  kJustifyFull = 46,
  kJustifyLeft = 47,
  kJustifyNone = 48,
  kJustifyRight = 49,
  kMakeTextWritingDirectionLeftToRight = 50,
  kMakeTextWritingDirectionNatural = 51,
  kMakeTextWritingDirectionRightToLeft = 52,
  kMoveBackward = 53,
  kMoveBackwardAndModifySelection = 54,
  kMoveDown = 55,
  kMoveDownAndModifySelection = 56,
  kMoveForward = 57,
  kMoveForwardAndModifySelection = 58,
  kMoveLeft = 59,
  kMoveLeftAndModifySelection = 60,
  kMovePageDown = 61,
  kMovePageDownAndModifySelection = 62,
  kMovePageUp = 63,
  kMovePageUpAndModifySelection = 64,
  kMoveParagraphBackward = 65,
  kMoveParagraphBackwardAndModifySelection = 66,
  kMoveParagraphForward = 67,
  kMoveParagraphForwardAndModifySelection = 68,
  kMoveRight = 69,
  kMoveRightAndModifySelection = 70,
  kMoveToBeginningOfDocument = 71,
  kMoveToBeginningOfDocumentAndModifySelection = 72,
  kMoveToBeginningOfLine = 73,
  kMoveToBeginningOfLineAndModifySelection = 74,
  kMoveToBeginningOfParagraph = 75,
  kMoveToBeginningOfParagraphAndModifySelection = 76,
  kMoveToBeginningOfSentence = 77,
  kMoveToBeginningOfSentenceAndModifySelection = 78,
  kMoveToEndOfDocument = 79,
  kMoveToEndOfDocumentAndModifySelection = 80,
  kMoveToEndOfLine = 81,
  kMoveToEndOfLineAndModifySelection = 82,
  kMoveToEndOfParagraph = 83,
  kMoveToEndOfParagraphAndModifySelection = 84,
  kMoveToEndOfSentence = 85,
  kMoveToEndOfSentenceAndModifySelection = 86,
  kMoveToLeftEndOfLine = 87,
  kMoveToLeftEndOfLineAndModifySelection = 88,
  kMoveToRightEndOfLine = 89,
  kMoveToRightEndOfLineAndModifySelection = 90,
  kMoveUp = 91,
  kMoveUpAndModifySelection = 92,
  kMoveWordBackward = 93,
  kMoveWordBackwardAndModifySelection = 94,
  kMoveWordForward = 95,
  kMoveWordForwardAndModifySelection = 96,
  kMoveWordLeft = 97,
  kMoveWordLeftAndModifySelection = 98,
  kMoveWordRight = 99,
  kMoveWordRightAndModifySelection = 100,
  kOutdent = 101,
  kOverWrite = 102,
  kPaste = 103,
  kPasteAndMatchStyle = 104,
  kPasteGlobalSelection = 105,
  kPrint = 106,
  kRedo = 107,
  kRemoveFormat = 108,
  kScrollPageBackward = 109,
  kScrollPageForward = 110,
  kScrollLineUp = 111,
  kScrollLineDown = 112,
  kScrollToBeginningOfDocument = 113,
  kScrollToEndOfDocument = 114,
  kSelectAll = 115,
  kSelectLine = 116,
  kSelectParagraph = 117,
  kSelectSentence = 118,
  kSelectToMark = 119,
  kSelectWord = 120,
  kSetMark = 121,
  kStrikethrough = 122,
  kStyleWithCSS = 123,
  kSubscript = 124,
  kSuperscript = 125,
  kSwapWithMark = 126,
  kToggleBold = 127,
  kToggleItalic = 128,
  kToggleUnderline = 129,
  kTranspose = 130,
  kUnderline = 131,
  kUndo = 132,
  kUnlink = 133,
  kUnscript = 134,
  kUnselect = 135,
  kUseCSS = 136,
  kYank = 137,
  kYankAndSelect = 138,
  kAlignCenter = 139,

  // This command is for internal use only; the current use case is pasting GIF
  // images selected from emoji picker on ChromeOS (the GIF URLs are from
  // tenor.com).
  kPasteFromImageURL = 140,

  // Add new commands immediately above this line.
  kNumberOfCommandTypes,

  // These unsupported commands are listed here since they appear in the
  // Microsoft documentation used as the starting point for our DOM
  // executeCommand support.
  //
  // 2D-Position (not supported)
  // AbsolutePosition (not supported)
  // BlockDirLTR (not supported)
  // BlockDirRTL (not supported)
  // BrowseMode (not supported)
  // ClearAuthenticationCache (not supported)
  // CreateBookmark (not supported)
  // DirLTR (not supported)
  // DirRTL (not supported)
  // EditMode (not supported)
  // InlineDirLTR (not supported)
  // InlineDirRTL (not supported)
  // InsertButton (not supported)
  // InsertFieldSet (not supported)
  // InsertIFrame (not supported)
  // InsertInputButton (not supported)
  // InsertInputCheckbox (not supported)
  // InsertInputFileUpload (not supported)
  // InsertInputHidden (not supported)
  // InsertInputImage (not supported)
  // InsertInputPassword (not supported)
  // InsertInputRadio (not supported)
  // InsertInputReset (not supported)
  // InsertInputSubmit (not supported)
  // InsertInputText (not supported)
  // InsertMarquee (not supported)
  // InsertSelectDropDown (not supported)
  // InsertTextArea (not supported)
  // LiveResize (not supported)
  // MultipleSelection (not supported)
  // Open (not supported)
  // PlayImage (not supported)
  // Refresh (not supported)
  // RemoveParaFormat (not supported)
  // SaveAs (not supported)
  // SizeToControl (not supported)
  // SizeToControlHeight (not supported)
  // SizeToControlWidth (not supported)
  // Stop (not supported)
  // StopImage (not supported)
  // Unbookmark (not supported)
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_EDITING_COMMAND_TYPE_H_
