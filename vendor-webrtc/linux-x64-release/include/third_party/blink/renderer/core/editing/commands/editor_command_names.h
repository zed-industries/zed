// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_EDITOR_COMMAND_NAMES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_EDITOR_COMMAND_NAMES_H_

namespace blink {

// Must be ordered in a case-folding manner for binary search. Covered by unit
// tests in editing_command_test.cc (not able to use static_assert)
#define FOR_EACH_BLINK_EDITING_COMMAND_NAME(V)    \
  V(AlignCenter)                                  \
  V(AlignJustified)                               \
  V(AlignLeft)                                    \
  V(AlignRight)                                   \
  V(BackColor)                                    \
  V(BackwardDelete)                               \
  V(Bold)                                         \
  V(Copy)                                         \
  V(CreateLink)                                   \
  V(Cut)                                          \
  V(DefaultParagraphSeparator)                    \
  V(Delete)                                       \
  V(DeleteBackward)                               \
  V(DeleteBackwardByDecomposingPreviousCharacter) \
  V(DeleteForward)                                \
  V(DeleteToBeginningOfLine)                      \
  V(DeleteToBeginningOfParagraph)                 \
  V(DeleteToEndOfLine)                            \
  V(DeleteToEndOfParagraph)                       \
  V(DeleteToMark)                                 \
  V(DeleteWordBackward)                           \
  V(DeleteWordForward)                            \
  V(FindString)                                   \
  V(FontName)                                     \
  V(FontSize)                                     \
  V(FontSizeDelta)                                \
  V(ForeColor)                                    \
  V(FormatBlock)                                  \
  V(ForwardDelete)                                \
  V(HiliteColor)                                  \
  V(IgnoreSpelling)                               \
  V(Indent)                                       \
  V(InsertBacktab)                                \
  V(InsertHorizontalRule)                         \
  V(InsertHTML)                                   \
  V(InsertImage)                                  \
  V(InsertLineBreak)                              \
  V(InsertNewline)                                \
  V(InsertNewlineInQuotedContent)                 \
  V(InsertOrderedList)                            \
  V(InsertParagraph)                              \
  V(InsertTab)                                    \
  V(InsertText)                                   \
  V(InsertUnorderedList)                          \
  V(Italic)                                       \
  V(JustifyCenter)                                \
  V(JustifyFull)                                  \
  V(JustifyLeft)                                  \
  V(JustifyNone)                                  \
  V(JustifyRight)                                 \
  V(MakeTextWritingDirectionLeftToRight)          \
  V(MakeTextWritingDirectionNatural)              \
  V(MakeTextWritingDirectionRightToLeft)          \
  V(MoveBackward)                                 \
  V(MoveBackwardAndModifySelection)               \
  V(MoveDown)                                     \
  V(MoveDownAndModifySelection)                   \
  V(MoveForward)                                  \
  V(MoveForwardAndModifySelection)                \
  V(MoveLeft)                                     \
  V(MoveLeftAndModifySelection)                   \
  V(MovePageDown)                                 \
  V(MovePageDownAndModifySelection)               \
  V(MovePageUp)                                   \
  V(MovePageUpAndModifySelection)                 \
  V(MoveParagraphBackward)                        \
  V(MoveParagraphBackwardAndModifySelection)      \
  V(MoveParagraphForward)                         \
  V(MoveParagraphForwardAndModifySelection)       \
  V(MoveRight)                                    \
  V(MoveRightAndModifySelection)                  \
  V(MoveToBeginningOfDocument)                    \
  V(MoveToBeginningOfDocumentAndModifySelection)  \
  V(MoveToBeginningOfLine)                        \
  V(MoveToBeginningOfLineAndModifySelection)      \
  V(MoveToBeginningOfParagraph)                   \
  V(MoveToBeginningOfParagraphAndModifySelection) \
  V(MoveToBeginningOfSentence)                    \
  V(MoveToBeginningOfSentenceAndModifySelection)  \
  V(MoveToEndOfDocument)                          \
  V(MoveToEndOfDocumentAndModifySelection)        \
  V(MoveToEndOfLine)                              \
  V(MoveToEndOfLineAndModifySelection)            \
  V(MoveToEndOfParagraph)                         \
  V(MoveToEndOfParagraphAndModifySelection)       \
  V(MoveToEndOfSentence)                          \
  V(MoveToEndOfSentenceAndModifySelection)        \
  V(MoveToLeftEndOfLine)                          \
  V(MoveToLeftEndOfLineAndModifySelection)        \
  V(MoveToRightEndOfLine)                         \
  V(MoveToRightEndOfLineAndModifySelection)       \
  V(MoveUp)                                       \
  V(MoveUpAndModifySelection)                     \
  V(MoveWordBackward)                             \
  V(MoveWordBackwardAndModifySelection)           \
  V(MoveWordForward)                              \
  V(MoveWordForwardAndModifySelection)            \
  V(MoveWordLeft)                                 \
  V(MoveWordLeftAndModifySelection)               \
  V(MoveWordRight)                                \
  V(MoveWordRightAndModifySelection)              \
  V(Outdent)                                      \
  V(OverWrite)                                    \
  V(Paste)                                        \
  V(PasteAndMatchStyle)                           \
  V(PasteFromImageURL)                            \
  V(PasteGlobalSelection)                         \
  V(Print)                                        \
  V(Redo)                                         \
  V(RemoveFormat)                                 \
  V(ScrollLineDown)                               \
  V(ScrollLineUp)                                 \
  V(ScrollPageBackward)                           \
  V(ScrollPageForward)                            \
  V(ScrollToBeginningOfDocument)                  \
  V(ScrollToEndOfDocument)                        \
  V(SelectAll)                                    \
  V(SelectLine)                                   \
  V(SelectParagraph)                              \
  V(SelectSentence)                               \
  V(SelectToMark)                                 \
  V(SelectWord)                                   \
  V(SetMark)                                      \
  V(Strikethrough)                                \
  V(StyleWithCSS)                                 \
  V(Subscript)                                    \
  V(Superscript)                                  \
  V(SwapWithMark)                                 \
  V(ToggleBold)                                   \
  V(ToggleItalic)                                 \
  V(ToggleUnderline)                              \
  V(Transpose)                                    \
  V(Underline)                                    \
  V(Undo)                                         \
  V(Unlink)                                       \
  V(Unscript)                                     \
  V(Unselect)                                     \
  V(UseCSS)                                       \
  V(Yank)                                         \
  V(YankAndSelect)

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_EDITOR_COMMAND_NAMES_H_
