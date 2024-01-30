/* vi:set ts=8 sts=4 sw=4:
 *
 * VIM - Vi IMproved		by Bram Moolenaar
 *				GUI support by Olaf "Rhialto" Seibert
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 *
 * Haiku GUI.
 *
 * Based on "GUI support for the Buzzword Enhanced Operating System for PPC."
 *
 */

/*
 * This file must be acceptable both as C and C++.
 * The BeOS API is defined in terms of C++, but some classes
 * should be somewhat known in the common C code.
 */

// System classes

struct BMenu;
struct BMenuItem;
struct BPictureButton;

// Our own Vim-related classes

struct VimApp;
struct VimFormView;
struct VimTextAreaView;
struct VimWindow;
struct VimScrollBar;

// Locking functions

extern int vim_lock_screen();
extern void vim_unlock_screen();

#ifndef __cplusplus

typedef struct BMenu BMenu;
typedef struct BMenuItem BMenuItem;
typedef struct BPictureButton BPictureButton;
typedef struct VimWindow VimWindow;
typedef struct VimFormView VimFormView;
typedef struct VimTextAreaView VimTextAreaView;
typedef struct VimApp VimApp;
typedef struct VimScrollBar VimScrollBar;

#endif
