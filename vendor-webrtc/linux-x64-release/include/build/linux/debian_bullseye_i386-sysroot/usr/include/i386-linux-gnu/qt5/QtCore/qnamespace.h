/****************************************************************************
**
** Copyright (C) 2020 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtCore module of the Qt Toolkit.
**
** $QT_BEGIN_LICENSE:LGPL$
** Commercial License Usage
** Licensees holding valid commercial Qt licenses may use this file in
** accordance with the commercial license agreement provided with the
** Software or, alternatively, in accordance with the terms contained in
** a written agreement between you and The Qt Company. For licensing terms
** and conditions see https://www.qt.io/terms-conditions. For further
** information use the contact form at https://www.qt.io/contact-us.
**
** GNU Lesser General Public License Usage
** Alternatively, this file may be used under the terms of the GNU Lesser
** General Public License version 3 as published by the Free Software
** Foundation and appearing in the file LICENSE.LGPL3 included in the
** packaging of this file. Please review the following information to
** ensure the GNU Lesser General Public License version 3 requirements
** will be met: https://www.gnu.org/licenses/lgpl-3.0.html.
**
** GNU General Public License Usage
** Alternatively, this file may be used under the terms of the GNU
** General Public License version 2.0 or (at your option) the GNU General
** Public license version 3 or any later version approved by the KDE Free
** Qt Foundation. The licenses are as published by the Free Software
** Foundation and appearing in the file LICENSE.GPL2 and LICENSE.GPL3
** included in the packaging of this file. Please review the following
** information to ensure the GNU General Public License requirements will
** be met: https://www.gnu.org/licenses/gpl-2.0.html and
** https://www.gnu.org/licenses/gpl-3.0.html.
**
** $QT_END_LICENSE$
**
****************************************************************************/

#ifndef QNAMESPACE_H
#define QNAMESPACE_H

#include <QtCore/qglobal.h>

#if defined(__OBJC__) && !defined(__cplusplus)
#  warning "File built in Objective-C mode (.m), but using Qt requires Objective-C++ (.mm)"
#endif

QT_BEGIN_NAMESPACE

#if !defined(Q_QDOC) && !defined(Q_MOC_RUN)
struct QMetaObject;
const QMetaObject *qt_getQtMetaObject() noexcept; // defined in qobject.h (which can't be included here)
#define QT_Q_ENUM(ENUM) \
    inline const QMetaObject *qt_getEnumMetaObject(ENUM) noexcept { return qt_getQtMetaObject(); } \
    inline Q_DECL_CONSTEXPR const char *qt_getEnumName(ENUM) noexcept { return #ENUM; }
#define QT_Q_FLAG(ENUM) QT_Q_ENUM(ENUM)
#else
#define QT_Q_ENUM Q_ENUM
#define QT_Q_FLAG Q_FLAG
#endif

#ifndef Q_MOC_RUN
namespace
#else
class Q_CORE_EXPORT
#endif
Qt {

#if defined(Q_MOC_RUN)
    Q_OBJECT
public:
#endif

    enum GlobalColor {
        color0,
        color1,
        black,
        white,
        darkGray,
        gray,
        lightGray,
        red,
        green,
        blue,
        cyan,
        magenta,
        yellow,
        darkRed,
        darkGreen,
        darkBlue,
        darkCyan,
        darkMagenta,
        darkYellow,
        transparent
    };

    enum KeyboardModifier {
        NoModifier           = 0x00000000,
        ShiftModifier        = 0x02000000,
        ControlModifier      = 0x04000000,
        AltModifier          = 0x08000000,
        MetaModifier         = 0x10000000,
        KeypadModifier       = 0x20000000,
        GroupSwitchModifier  = 0x40000000,
        // Do not extend the mask to include 0x01000000
        KeyboardModifierMask = 0xfe000000
    };
    Q_DECLARE_FLAGS(KeyboardModifiers, KeyboardModifier)
    Q_DECLARE_OPERATORS_FOR_FLAGS(KeyboardModifiers)

    //shorter names for shortcuts
    // The use of all-caps identifiers has the potential for clashing with
    // user-defined or third-party macros. More so when the identifiers are not
    // "namespace"-prefixed. This is considered bad practice and is why
    // KeypadModifier was not added to the Modifier enum.
    enum Modifier {
        META          = Qt::MetaModifier,
        SHIFT         = Qt::ShiftModifier,
        CTRL          = Qt::ControlModifier,
        ALT           = Qt::AltModifier,
        MODIFIER_MASK = KeyboardModifierMask,
        UNICODE_ACCEL = 0x00000000
    };

    enum MouseButton {
        NoButton         = 0x00000000,
        LeftButton       = 0x00000001,
        RightButton      = 0x00000002,
        MiddleButton     = 0x00000004,
#if QT_DEPRECATED_SINCE(5, 15) // commented as such since 4.7.0
        MidButton Q_DECL_ENUMERATOR_DEPRECATED_X("MidButton is deprecated. Use MiddleButton instead") = MiddleButton,
#endif
        BackButton       = 0x00000008,
        XButton1         = BackButton,
        ExtraButton1     = XButton1,
        ForwardButton    = 0x00000010,
        XButton2         = ForwardButton,
        ExtraButton2     = ForwardButton,
        TaskButton       = 0x00000020,
        ExtraButton3     = TaskButton,
        ExtraButton4     = 0x00000040,
        ExtraButton5     = 0x00000080,
        ExtraButton6     = 0x00000100,
        ExtraButton7     = 0x00000200,
        ExtraButton8     = 0x00000400,
        ExtraButton9     = 0x00000800,
        ExtraButton10    = 0x00001000,
        ExtraButton11    = 0x00002000,
        ExtraButton12    = 0x00004000,
        ExtraButton13    = 0x00008000,
        ExtraButton14    = 0x00010000,
        ExtraButton15    = 0x00020000,
        ExtraButton16    = 0x00040000,
        ExtraButton17    = 0x00080000,
        ExtraButton18    = 0x00100000,
        ExtraButton19    = 0x00200000,
        ExtraButton20    = 0x00400000,
        ExtraButton21    = 0x00800000,
        ExtraButton22    = 0x01000000,
        ExtraButton23    = 0x02000000,
        ExtraButton24    = 0x04000000,
        AllButtons       = 0x07ffffff,
        MaxMouseButton   = ExtraButton24,
        // 4 high-order bits remain available for future use (0x08000000 through 0x40000000).
        MouseButtonMask  = 0xffffffff
    };
    Q_DECLARE_FLAGS(MouseButtons, MouseButton)
    Q_DECLARE_OPERATORS_FOR_FLAGS(MouseButtons)

    enum Orientation {
        Horizontal = 0x1,
        Vertical = 0x2
    };

    Q_DECLARE_FLAGS(Orientations, Orientation)
    Q_DECLARE_OPERATORS_FOR_FLAGS(Orientations)

    enum FocusPolicy {
        NoFocus = 0,
        TabFocus = 0x1,
        ClickFocus = 0x2,
        StrongFocus = TabFocus | ClickFocus | 0x8,
        WheelFocus = StrongFocus | 0x4
    };

    enum TabFocusBehavior {
        NoTabFocus           = 0x00,
        TabFocusTextControls = 0x01,
        TabFocusListControls = 0x02,
        TabFocusAllControls  = 0xff
    };

    enum SortOrder {
        AscendingOrder,
        DescendingOrder
    };

    enum SplitBehaviorFlags {
        KeepEmptyParts = 0,
        SkipEmptyParts = 0x1,
    };
    Q_DECLARE_FLAGS(SplitBehavior, SplitBehaviorFlags)
    Q_DECLARE_OPERATORS_FOR_FLAGS(SplitBehavior)

    enum TileRule {
        StretchTile,
        RepeatTile,
        RoundTile
    };

    // Text formatting flags for QPainter::drawText and QLabel.
    // The following two enums can be combined to one integer which
    // is passed as 'flags' to QPainter::drawText, QFontMetrics::boundingRect and qt_format_text.

    enum AlignmentFlag {
        AlignLeft = 0x0001,
        AlignLeading = AlignLeft,
        AlignRight = 0x0002,
        AlignTrailing = AlignRight,
        AlignHCenter = 0x0004,
        AlignJustify = 0x0008,
        AlignAbsolute = 0x0010,
        AlignHorizontal_Mask = AlignLeft | AlignRight | AlignHCenter | AlignJustify | AlignAbsolute,

        AlignTop = 0x0020,
        AlignBottom = 0x0040,
        AlignVCenter = 0x0080,
        AlignBaseline = 0x0100,
        // Note that 0x100 will clash with Qt::TextSingleLine = 0x100 due to what the comment above
        // this enum declaration states. However, since Qt::AlignBaseline is only used by layouts,
        // it doesn't make sense to pass Qt::AlignBaseline to QPainter::drawText(), so there
        // shouldn't really be any ambiguity between the two overlapping enum values.
        AlignVertical_Mask = AlignTop | AlignBottom | AlignVCenter | AlignBaseline,

        AlignCenter = AlignVCenter | AlignHCenter
    };

    Q_DECLARE_FLAGS(Alignment, AlignmentFlag)
    Q_DECLARE_OPERATORS_FOR_FLAGS(Alignment)

    enum TextFlag {
        TextSingleLine = 0x0100,
        TextDontClip = 0x0200,
        TextExpandTabs = 0x0400,
        TextShowMnemonic = 0x0800,
        TextWordWrap = 0x1000,
        TextWrapAnywhere = 0x2000,
        TextDontPrint = 0x4000,
        TextIncludeTrailingSpaces = 0x08000000,
        TextHideMnemonic = 0x8000,
        TextJustificationForced = 0x10000,
        TextForceLeftToRight = 0x20000,
        TextForceRightToLeft = 0x40000,
        // Ensures that the longest variant is always used when computing the
        // size of a multi-variant string.
        TextLongestVariant = 0x80000

#if QT_DEPRECATED_SINCE(5, 11) && QT_VERSION < QT_VERSION_CHECK(6, 0, 0)
        , TextBypassShaping = 0x100000
#endif
    };

    enum TextElideMode {
        ElideLeft,
        ElideRight,
        ElideMiddle,
        ElideNone
    };

    enum WhiteSpaceMode {
        WhiteSpaceNormal,
        WhiteSpacePre,
        WhiteSpaceNoWrap,
        WhiteSpaceModeUndefined = -1
    };

    enum HitTestAccuracy { ExactHit, FuzzyHit };

    enum WindowType {
        Widget = 0x00000000,
        Window = 0x00000001,
        Dialog = 0x00000002 | Window,
        Sheet = 0x00000004 | Window,
        Drawer = Sheet | Dialog,
        Popup = 0x00000008 | Window,
        Tool = Popup | Dialog,
        ToolTip = Popup | Sheet,
        SplashScreen = ToolTip | Dialog,
        Desktop = 0x00000010 | Window,
        SubWindow = 0x00000012,
        ForeignWindow = 0x00000020 | Window,
        CoverWindow = 0x00000040 | Window,

        WindowType_Mask = 0x000000ff,
        MSWindowsFixedSizeDialogHint = 0x00000100,
        MSWindowsOwnDC = 0x00000200,
        BypassWindowManagerHint = 0x00000400,
        X11BypassWindowManagerHint = BypassWindowManagerHint,
        FramelessWindowHint = 0x00000800,
        WindowTitleHint = 0x00001000,
        WindowSystemMenuHint = 0x00002000,
        WindowMinimizeButtonHint = 0x00004000,
        WindowMaximizeButtonHint = 0x00008000,
        WindowMinMaxButtonsHint = WindowMinimizeButtonHint | WindowMaximizeButtonHint,
        WindowContextHelpButtonHint = 0x00010000,
        WindowShadeButtonHint = 0x00020000,
        WindowStaysOnTopHint = 0x00040000,
        WindowTransparentForInput = 0x00080000,
        WindowOverridesSystemGestures = 0x00100000,
        WindowDoesNotAcceptFocus = 0x00200000,
        MaximizeUsingFullscreenGeometryHint = 0x00400000,

        CustomizeWindowHint = 0x02000000,
        WindowStaysOnBottomHint = 0x04000000,
        WindowCloseButtonHint = 0x08000000,
        MacWindowToolBarButtonHint = 0x10000000,
        BypassGraphicsProxyWidget = 0x20000000,
        NoDropShadowWindowHint = 0x40000000,
        WindowFullscreenButtonHint = 0x80000000
    };

    Q_DECLARE_FLAGS(WindowFlags, WindowType)
    Q_DECLARE_OPERATORS_FOR_FLAGS(WindowFlags)

    enum WindowState {
        WindowNoState    = 0x00000000,
        WindowMinimized  = 0x00000001,
        WindowMaximized  = 0x00000002,
        WindowFullScreen = 0x00000004,
        WindowActive     = 0x00000008
    };

    Q_DECLARE_FLAGS(WindowStates, WindowState)
    Q_DECLARE_OPERATORS_FOR_FLAGS(WindowStates)

    enum ApplicationState {
        ApplicationSuspended    = 0x00000000,
        ApplicationHidden       = 0x00000001,
        ApplicationInactive     = 0x00000002,
        ApplicationActive       = 0x00000004
    };

    Q_DECLARE_FLAGS(ApplicationStates, ApplicationState)

    enum ScreenOrientation {
        PrimaryOrientation           = 0x00000000,
        PortraitOrientation          = 0x00000001,
        LandscapeOrientation         = 0x00000002,
        InvertedPortraitOrientation  = 0x00000004,
        InvertedLandscapeOrientation = 0x00000008
    };

    Q_DECLARE_FLAGS(ScreenOrientations, ScreenOrientation)
    Q_DECLARE_OPERATORS_FOR_FLAGS(ScreenOrientations)

    enum WidgetAttribute {
        WA_Disabled = 0,
        WA_UnderMouse = 1,
        WA_MouseTracking = 2,
#if QT_DEPRECATED_SINCE(5, 15) // commented as such since 4.5.1
        WA_ContentsPropagated Q_DECL_ENUMERATOR_DEPRECATED = 3,
#endif
        WA_OpaquePaintEvent = 4,
#if QT_DEPRECATED_SINCE(5, 14)
        WA_NoBackground Q_DECL_ENUMERATOR_DEPRECATED = WA_OpaquePaintEvent,
#endif
        WA_StaticContents = 5,
        WA_LaidOut = 7,
        WA_PaintOnScreen = 8,
        WA_NoSystemBackground = 9,
        WA_UpdatesDisabled = 10,
        WA_Mapped = 11,
#if QT_DEPRECATED_SINCE(5, 14)
        WA_MacNoClickThrough Q_DECL_ENUMERATOR_DEPRECATED = 12,
#endif
        WA_InputMethodEnabled = 14,
        WA_WState_Visible = 15,
        WA_WState_Hidden = 16,

        WA_ForceDisabled = 32,
        WA_KeyCompression = 33,
        WA_PendingMoveEvent = 34,
        WA_PendingResizeEvent = 35,
        WA_SetPalette = 36,
        WA_SetFont = 37,
        WA_SetCursor = 38,
        WA_NoChildEventsFromChildren = 39,
        WA_WindowModified = 41,
        WA_Resized = 42,
        WA_Moved = 43,
        WA_PendingUpdate = 44,
        WA_InvalidSize = 45,
#if QT_DEPRECATED_SINCE(5, 14)
        WA_MacBrushedMetal Q_DECL_ENUMERATOR_DEPRECATED = 46,
        WA_MacMetalStyle Q_DECL_ENUMERATOR_DEPRECATED = 46,
#endif
        WA_CustomWhatsThis = 47,
        WA_LayoutOnEntireRect = 48,
        WA_OutsideWSRange = 49,
        WA_GrabbedShortcut = 50,
        WA_TransparentForMouseEvents = 51,
        WA_PaintUnclipped = 52,
        WA_SetWindowIcon = 53,
        WA_NoMouseReplay = 54,
        WA_DeleteOnClose = 55,
        WA_RightToLeft = 56,
        WA_SetLayoutDirection = 57,
        WA_NoChildEventsForParent = 58,
        WA_ForceUpdatesDisabled = 59,

        WA_WState_Created = 60,
        WA_WState_CompressKeys = 61,
        WA_WState_InPaintEvent = 62,
        WA_WState_Reparented = 63,
        WA_WState_ConfigPending = 64,
        WA_WState_Polished = 66,
#if QT_DEPRECATED_SINCE(5, 15) // commented as such in 4.5.1
        WA_WState_DND Q_DECL_ENUMERATOR_DEPRECATED = 67,
#endif
        WA_WState_OwnSizePolicy = 68,
        WA_WState_ExplicitShowHide = 69,

        WA_ShowModal = 70, // ## deprecated since since 4.5.1 but still in use :-(
        WA_MouseNoMask = 71,
        WA_GroupLeader = 72, // ## deprecated since since 4.5.1 but still in use :-(
        WA_NoMousePropagation = 73, // for now, might go away.
        WA_Hover = 74,
        WA_InputMethodTransparent = 75, // Don't reset IM when user clicks on this (for virtual keyboards on embedded)
        WA_QuitOnClose = 76,

        WA_KeyboardFocusChange = 77,

        WA_AcceptDrops = 78,
        WA_DropSiteRegistered = 79, // internal
#if QT_DEPRECATED_SINCE(5, 15) // commented as such since 4.5.1
        WA_ForceAcceptDrops Q_DECL_ENUMERATOR_DEPRECATED_X("WA_ForceAcceptDrops is deprecated. Use WA_DropSiteRegistered instead") = WA_DropSiteRegistered,
#endif

        WA_WindowPropagation = 80,

        WA_NoX11EventCompression = 81,
        WA_TintedBackground = 82,
        WA_X11OpenGLOverlay = 83,
        WA_AlwaysShowToolTips = 84,
        WA_MacOpaqueSizeGrip = 85,
        WA_SetStyle = 86,

        WA_SetLocale = 87,
        WA_MacShowFocusRect = 88,

        WA_MacNormalSize = 89,  // Mac only
        WA_MacSmallSize = 90,   // Mac only
        WA_MacMiniSize = 91,    // Mac only

        WA_LayoutUsesWidgetRect = 92,
        WA_StyledBackground = 93, // internal
#if QT_DEPRECATED_SINCE(5, 14)
        WA_MSWindowsUseDirect3D Q_DECL_ENUMERATOR_DEPRECATED = 94,
#endif
        WA_CanHostQMdiSubWindowTitleBar = 95, // Internal

        WA_MacAlwaysShowToolWindow = 96, // Mac only

        WA_StyleSheet = 97, // internal

        WA_ShowWithoutActivating = 98,

        WA_X11BypassTransientForHint = 99,

        WA_NativeWindow = 100,
        WA_DontCreateNativeAncestors = 101,

        WA_MacVariableSize = 102,    // Mac only

        WA_DontShowOnScreen = 103,

        // window types from http://standards.freedesktop.org/wm-spec/
        WA_X11NetWmWindowTypeDesktop = 104,
        WA_X11NetWmWindowTypeDock = 105,
        WA_X11NetWmWindowTypeToolBar = 106,
        WA_X11NetWmWindowTypeMenu = 107,
        WA_X11NetWmWindowTypeUtility = 108,
        WA_X11NetWmWindowTypeSplash = 109,
        WA_X11NetWmWindowTypeDialog = 110,
        WA_X11NetWmWindowTypeDropDownMenu = 111,
        WA_X11NetWmWindowTypePopupMenu = 112,
        WA_X11NetWmWindowTypeToolTip = 113,
        WA_X11NetWmWindowTypeNotification = 114,
        WA_X11NetWmWindowTypeCombo = 115,
        WA_X11NetWmWindowTypeDND = 116,
#if QT_DEPRECATED_SINCE(5, 14)
        WA_MacFrameworkScaled Q_DECL_ENUMERATOR_DEPRECATED = 117,
#endif
        WA_SetWindowModality = 118,
        WA_WState_WindowOpacitySet = 119, // internal
        WA_TranslucentBackground = 120,

        WA_AcceptTouchEvents = 121,
        WA_WState_AcceptedTouchBeginEvent = 122,
        WA_TouchPadAcceptSingleTouchEvents = 123,

        WA_X11DoNotAcceptFocus = 126,
        WA_MacNoShadow = 127,

        WA_AlwaysStackOnTop = 128,

        WA_TabletTracking = 129,

        WA_ContentsMarginsRespectsSafeArea = 130,

        WA_StyleSheetTarget = 131,

        // Add new attributes before this line
        WA_AttributeCount
    };

    enum ApplicationAttribute
    {
        AA_ImmediateWidgetCreation = 0,
#if QT_DEPRECATED_SINCE(5, 14)
        AA_MSWindowsUseDirect3DByDefault Q_DECL_ENUMERATOR_DEPRECATED = 1,
#endif
        AA_DontShowIconsInMenus = 2,
        AA_NativeWindows = 3,
        AA_DontCreateNativeWidgetSiblings = 4,
        AA_PluginApplication = 5,
#if QT_DEPRECATED_SINCE(5, 13) // ### Qt 6: remove me
        AA_MacPluginApplication Q_DECL_ENUMERATOR_DEPRECATED = AA_PluginApplication,
#endif
        AA_DontUseNativeMenuBar = 6,
        AA_MacDontSwapCtrlAndMeta = 7,
        AA_Use96Dpi = 8,
        AA_DisableNativeVirtualKeyboard = 9,
#if QT_DEPRECATED_SINCE(5, 14)
        AA_X11InitThreads Q_DECL_ENUMERATOR_DEPRECATED = 10,
#endif
        AA_SynthesizeTouchForUnhandledMouseEvents = 11,
        AA_SynthesizeMouseForUnhandledTouchEvents = 12,
        AA_UseHighDpiPixmaps = 13,
        AA_ForceRasterWidgets = 14,
        AA_UseDesktopOpenGL = 15,
        AA_UseOpenGLES = 16,
        AA_UseSoftwareOpenGL = 17,
        AA_ShareOpenGLContexts = 18,
        AA_SetPalette = 19,
        AA_EnableHighDpiScaling = 20,
        AA_DisableHighDpiScaling = 21,
        AA_UseStyleSheetPropagationInWidgetStyles = 22,
        AA_DontUseNativeDialogs = 23,
        AA_SynthesizeMouseForUnhandledTabletEvents = 24,
        AA_CompressHighFrequencyEvents = 25,
        AA_DontCheckOpenGLContextThreadAffinity = 26,
        AA_DisableShaderDiskCache = 27,
        AA_DontShowShortcutsInContextMenus = 28,
        AA_CompressTabletEvents = 29,
        AA_DisableWindowContextHelpButton = 30, // ### Qt 6: remove me
        AA_DisableSessionManager = 31,

        // Add new attributes before this line
        AA_AttributeCount
    };


    // Image conversion flags.  The unusual ordering is caused by
    // compatibility and default requirements.

    enum ImageConversionFlag {
        ColorMode_Mask          = 0x00000003,
        AutoColor               = 0x00000000,
        ColorOnly               = 0x00000003,
        MonoOnly                = 0x00000002,
        // Reserved             = 0x00000001,

        AlphaDither_Mask        = 0x0000000c,
        ThresholdAlphaDither    = 0x00000000,
        OrderedAlphaDither      = 0x00000004,
        DiffuseAlphaDither      = 0x00000008,
        NoAlpha                 = 0x0000000c, // Not supported

        Dither_Mask             = 0x00000030,
        DiffuseDither           = 0x00000000,
        OrderedDither           = 0x00000010,
        ThresholdDither         = 0x00000020,
        // ReservedDither       = 0x00000030,

        DitherMode_Mask         = 0x000000c0,
        AutoDither              = 0x00000000,
        PreferDither            = 0x00000040,
        AvoidDither             = 0x00000080,

        NoOpaqueDetection       = 0x00000100,
        NoFormatConversion      = 0x00000200
    };
    Q_DECLARE_FLAGS(ImageConversionFlags, ImageConversionFlag)
    Q_DECLARE_OPERATORS_FOR_FLAGS(ImageConversionFlags)

    enum BGMode {
        TransparentMode,
        OpaqueMode
    };

    enum Key {
        Key_Escape = 0x01000000,                // misc keys
        Key_Tab = 0x01000001,
        Key_Backtab = 0x01000002,
        Key_Backspace = 0x01000003,
        Key_Return = 0x01000004,
        Key_Enter = 0x01000005,
        Key_Insert = 0x01000006,
        Key_Delete = 0x01000007,
        Key_Pause = 0x01000008,
        Key_Print = 0x01000009,               // print screen
        Key_SysReq = 0x0100000a,
        Key_Clear = 0x0100000b,
        Key_Home = 0x01000010,                // cursor movement
        Key_End = 0x01000011,
        Key_Left = 0x01000012,
        Key_Up = 0x01000013,
        Key_Right = 0x01000014,
        Key_Down = 0x01000015,
        Key_PageUp = 0x01000016,
        Key_PageDown = 0x01000017,
        Key_Shift = 0x01000020,                // modifiers
        Key_Control = 0x01000021,
        Key_Meta = 0x01000022,
        Key_Alt = 0x01000023,
        Key_CapsLock = 0x01000024,
        Key_NumLock = 0x01000025,
        Key_ScrollLock = 0x01000026,
        Key_F1 = 0x01000030,                // function keys
        Key_F2 = 0x01000031,
        Key_F3 = 0x01000032,
        Key_F4 = 0x01000033,
        Key_F5 = 0x01000034,
        Key_F6 = 0x01000035,
        Key_F7 = 0x01000036,
        Key_F8 = 0x01000037,
        Key_F9 = 0x01000038,
        Key_F10 = 0x01000039,
        Key_F11 = 0x0100003a,
        Key_F12 = 0x0100003b,
        Key_F13 = 0x0100003c,
        Key_F14 = 0x0100003d,
        Key_F15 = 0x0100003e,
        Key_F16 = 0x0100003f,
        Key_F17 = 0x01000040,
        Key_F18 = 0x01000041,
        Key_F19 = 0x01000042,
        Key_F20 = 0x01000043,
        Key_F21 = 0x01000044,
        Key_F22 = 0x01000045,
        Key_F23 = 0x01000046,
        Key_F24 = 0x01000047,
        Key_F25 = 0x01000048,                // F25 .. F35 only on X11
        Key_F26 = 0x01000049,
        Key_F27 = 0x0100004a,
        Key_F28 = 0x0100004b,
        Key_F29 = 0x0100004c,
        Key_F30 = 0x0100004d,
        Key_F31 = 0x0100004e,
        Key_F32 = 0x0100004f,
        Key_F33 = 0x01000050,
        Key_F34 = 0x01000051,
        Key_F35 = 0x01000052,
        Key_Super_L = 0x01000053,                 // extra keys
        Key_Super_R = 0x01000054,
        Key_Menu = 0x01000055,
        Key_Hyper_L = 0x01000056,
        Key_Hyper_R = 0x01000057,
        Key_Help = 0x01000058,
        Key_Direction_L = 0x01000059,
        Key_Direction_R = 0x01000060,
        Key_Space = 0x20,                // 7 bit printable ASCII
        Key_Any = Key_Space,
        Key_Exclam = 0x21,
        Key_QuoteDbl = 0x22,
        Key_NumberSign = 0x23,
        Key_Dollar = 0x24,
        Key_Percent = 0x25,
        Key_Ampersand = 0x26,
        Key_Apostrophe = 0x27,
        Key_ParenLeft = 0x28,
        Key_ParenRight = 0x29,
        Key_Asterisk = 0x2a,
        Key_Plus = 0x2b,
        Key_Comma = 0x2c,
        Key_Minus = 0x2d,
        Key_Period = 0x2e,
        Key_Slash = 0x2f,
        Key_0 = 0x30,
        Key_1 = 0x31,
        Key_2 = 0x32,
        Key_3 = 0x33,
        Key_4 = 0x34,
        Key_5 = 0x35,
        Key_6 = 0x36,
        Key_7 = 0x37,
        Key_8 = 0x38,
        Key_9 = 0x39,
        Key_Colon = 0x3a,
        Key_Semicolon = 0x3b,
        Key_Less = 0x3c,
        Key_Equal = 0x3d,
        Key_Greater = 0x3e,
        Key_Question = 0x3f,
        Key_At = 0x40,
        Key_A = 0x41,
        Key_B = 0x42,
        Key_C = 0x43,
        Key_D = 0x44,
        Key_E = 0x45,
        Key_F = 0x46,
        Key_G = 0x47,
        Key_H = 0x48,
        Key_I = 0x49,
        Key_J = 0x4a,
        Key_K = 0x4b,
        Key_L = 0x4c,
        Key_M = 0x4d,
        Key_N = 0x4e,
        Key_O = 0x4f,
        Key_P = 0x50,
        Key_Q = 0x51,
        Key_R = 0x52,
        Key_S = 0x53,
        Key_T = 0x54,
        Key_U = 0x55,
        Key_V = 0x56,
        Key_W = 0x57,
        Key_X = 0x58,
        Key_Y = 0x59,
        Key_Z = 0x5a,
        Key_BracketLeft = 0x5b,
        Key_Backslash = 0x5c,
        Key_BracketRight = 0x5d,
        Key_AsciiCircum = 0x5e,
        Key_Underscore = 0x5f,
        Key_QuoteLeft = 0x60,
        Key_BraceLeft = 0x7b,
        Key_Bar = 0x7c,
        Key_BraceRight = 0x7d,
        Key_AsciiTilde = 0x7e,

        Key_nobreakspace = 0x0a0,
        Key_exclamdown = 0x0a1,
        Key_cent = 0x0a2,
        Key_sterling = 0x0a3,
        Key_currency = 0x0a4,
        Key_yen = 0x0a5,
        Key_brokenbar = 0x0a6,
        Key_section = 0x0a7,
        Key_diaeresis = 0x0a8,
        Key_copyright = 0x0a9,
        Key_ordfeminine = 0x0aa,
        Key_guillemotleft = 0x0ab,        // left angle quotation mark
        Key_notsign = 0x0ac,
        Key_hyphen = 0x0ad,
        Key_registered = 0x0ae,
        Key_macron = 0x0af,
        Key_degree = 0x0b0,
        Key_plusminus = 0x0b1,
        Key_twosuperior = 0x0b2,
        Key_threesuperior = 0x0b3,
        Key_acute = 0x0b4,
        Key_mu = 0x0b5,
        Key_paragraph = 0x0b6,
        Key_periodcentered = 0x0b7,
        Key_cedilla = 0x0b8,
        Key_onesuperior = 0x0b9,
        Key_masculine = 0x0ba,
        Key_guillemotright = 0x0bb,        // right angle quotation mark
        Key_onequarter = 0x0bc,
        Key_onehalf = 0x0bd,
        Key_threequarters = 0x0be,
        Key_questiondown = 0x0bf,
        Key_Agrave = 0x0c0,
        Key_Aacute = 0x0c1,
        Key_Acircumflex = 0x0c2,
        Key_Atilde = 0x0c3,
        Key_Adiaeresis = 0x0c4,
        Key_Aring = 0x0c5,
        Key_AE = 0x0c6,
        Key_Ccedilla = 0x0c7,
        Key_Egrave = 0x0c8,
        Key_Eacute = 0x0c9,
        Key_Ecircumflex = 0x0ca,
        Key_Ediaeresis = 0x0cb,
        Key_Igrave = 0x0cc,
        Key_Iacute = 0x0cd,
        Key_Icircumflex = 0x0ce,
        Key_Idiaeresis = 0x0cf,
        Key_ETH = 0x0d0,
        Key_Ntilde = 0x0d1,
        Key_Ograve = 0x0d2,
        Key_Oacute = 0x0d3,
        Key_Ocircumflex = 0x0d4,
        Key_Otilde = 0x0d5,
        Key_Odiaeresis = 0x0d6,
        Key_multiply = 0x0d7,
        Key_Ooblique = 0x0d8,
        Key_Ugrave = 0x0d9,
        Key_Uacute = 0x0da,
        Key_Ucircumflex = 0x0db,
        Key_Udiaeresis = 0x0dc,
        Key_Yacute = 0x0dd,
        Key_THORN = 0x0de,
        Key_ssharp = 0x0df,
        Key_division = 0x0f7,
        Key_ydiaeresis = 0x0ff,

        // International input method support (X keycode - 0xEE00, the
        // definition follows Qt/Embedded 2.3.7) Only interesting if
        // you are writing your own input method

        // International & multi-key character composition
        Key_AltGr               = 0x01001103,
        Key_Multi_key           = 0x01001120,  // Multi-key character compose
        Key_Codeinput           = 0x01001137,
        Key_SingleCandidate     = 0x0100113c,
        Key_MultipleCandidate   = 0x0100113d,
        Key_PreviousCandidate   = 0x0100113e,

        // Misc Functions
        Key_Mode_switch         = 0x0100117e,  // Character set switch
        //Key_script_switch       = 0x0100117e,  // Alias for mode_switch

        // Japanese keyboard support
        Key_Kanji               = 0x01001121,  // Kanji, Kanji convert
        Key_Muhenkan            = 0x01001122,  // Cancel Conversion
        //Key_Henkan_Mode         = 0x01001123,  // Start/Stop Conversion
        Key_Henkan              = 0x01001123,  // Alias for Henkan_Mode
        Key_Romaji              = 0x01001124,  // to Romaji
        Key_Hiragana            = 0x01001125,  // to Hiragana
        Key_Katakana            = 0x01001126,  // to Katakana
        Key_Hiragana_Katakana   = 0x01001127,  // Hiragana/Katakana toggle
        Key_Zenkaku             = 0x01001128,  // to Zenkaku
        Key_Hankaku             = 0x01001129,  // to Hankaku
        Key_Zenkaku_Hankaku     = 0x0100112a,  // Zenkaku/Hankaku toggle
        Key_Touroku             = 0x0100112b,  // Add to Dictionary
        Key_Massyo              = 0x0100112c,  // Delete from Dictionary
        Key_Kana_Lock           = 0x0100112d,  // Kana Lock
        Key_Kana_Shift          = 0x0100112e,  // Kana Shift
        Key_Eisu_Shift          = 0x0100112f,  // Alphanumeric Shift
        Key_Eisu_toggle         = 0x01001130,  // Alphanumeric toggle
        //Key_Kanji_Bangou        = 0x01001137,  // Codeinput
        //Key_Zen_Koho            = 0x0100113d,  // Multiple/All Candidate(s)
        //Key_Mae_Koho            = 0x0100113e,  // Previous Candidate

        // Korean keyboard support
        //
        // In fact, many Korean users need only 2 keys, Key_Hangul and
        // Key_Hangul_Hanja. But rest of the keys are good for future.

        Key_Hangul              = 0x01001131,  // Hangul start/stop(toggle)
        Key_Hangul_Start        = 0x01001132,  // Hangul start
        Key_Hangul_End          = 0x01001133,  // Hangul end, English start
        Key_Hangul_Hanja        = 0x01001134,  // Start Hangul->Hanja Conversion
        Key_Hangul_Jamo         = 0x01001135,  // Hangul Jamo mode
        Key_Hangul_Romaja       = 0x01001136,  // Hangul Romaja mode
        //Key_Hangul_Codeinput    = 0x01001137,  // Hangul code input mode
        Key_Hangul_Jeonja       = 0x01001138,  // Jeonja mode
        Key_Hangul_Banja        = 0x01001139,  // Banja mode
        Key_Hangul_PreHanja     = 0x0100113a,  // Pre Hanja conversion
        Key_Hangul_PostHanja    = 0x0100113b,  // Post Hanja conversion
        //Key_Hangul_SingleCandidate   = 0x0100113c,  // Single candidate
        //Key_Hangul_MultipleCandidate = 0x0100113d,  // Multiple candidate
        //Key_Hangul_PreviousCandidate = 0x0100113e,  // Previous candidate
        Key_Hangul_Special      = 0x0100113f,  // Special symbols
        //Key_Hangul_switch       = 0x0100117e,  // Alias for mode_switch

        // dead keys (X keycode - 0xED00 to avoid the conflict)
        Key_Dead_Grave          = 0x01001250,
        Key_Dead_Acute          = 0x01001251,
        Key_Dead_Circumflex     = 0x01001252,
        Key_Dead_Tilde          = 0x01001253,
        Key_Dead_Macron         = 0x01001254,
        Key_Dead_Breve          = 0x01001255,
        Key_Dead_Abovedot       = 0x01001256,
        Key_Dead_Diaeresis      = 0x01001257,
        Key_Dead_Abovering      = 0x01001258,
        Key_Dead_Doubleacute    = 0x01001259,
        Key_Dead_Caron          = 0x0100125a,
        Key_Dead_Cedilla        = 0x0100125b,
        Key_Dead_Ogonek         = 0x0100125c,
        Key_Dead_Iota           = 0x0100125d,
        Key_Dead_Voiced_Sound   = 0x0100125e,
        Key_Dead_Semivoiced_Sound = 0x0100125f,
        Key_Dead_Belowdot       = 0x01001260,
        Key_Dead_Hook           = 0x01001261,
        Key_Dead_Horn           = 0x01001262,
        Key_Dead_Stroke         = 0x01001263,
        Key_Dead_Abovecomma     = 0x01001264,
        Key_Dead_Abovereversedcomma = 0x01001265,
        Key_Dead_Doublegrave    = 0x01001266,
        Key_Dead_Belowring      = 0x01001267,
        Key_Dead_Belowmacron    = 0x01001268,
        Key_Dead_Belowcircumflex = 0x01001269,
        Key_Dead_Belowtilde     = 0x0100126a,
        Key_Dead_Belowbreve     = 0x0100126b,
        Key_Dead_Belowdiaeresis = 0x0100126c,
        Key_Dead_Invertedbreve  = 0x0100126d,
        Key_Dead_Belowcomma     = 0x0100126e,
        Key_Dead_Currency       = 0x0100126f,
        Key_Dead_a              = 0x01001280,
        Key_Dead_A              = 0x01001281,
        Key_Dead_e              = 0x01001282,
        Key_Dead_E              = 0x01001283,
        Key_Dead_i              = 0x01001284,
        Key_Dead_I              = 0x01001285,
        Key_Dead_o              = 0x01001286,
        Key_Dead_O              = 0x01001287,
        Key_Dead_u              = 0x01001288,
        Key_Dead_U              = 0x01001289,
        Key_Dead_Small_Schwa    = 0x0100128a,
        Key_Dead_Capital_Schwa  = 0x0100128b,
        Key_Dead_Greek          = 0x0100128c,
        Key_Dead_Lowline        = 0x01001290,
        Key_Dead_Aboveverticalline = 0x01001291,
        Key_Dead_Belowverticalline = 0x01001292,
        Key_Dead_Longsolidusoverlay = 0x01001293,

        // multimedia/internet keys - ignored by default - see QKeyEvent c'tor
        Key_Back  = 0x01000061,
        Key_Forward  = 0x01000062,
        Key_Stop  = 0x01000063,
        Key_Refresh  = 0x01000064,
        Key_VolumeDown = 0x01000070,
        Key_VolumeMute  = 0x01000071,
        Key_VolumeUp = 0x01000072,
        Key_BassBoost = 0x01000073,
        Key_BassUp = 0x01000074,
        Key_BassDown = 0x01000075,
        Key_TrebleUp = 0x01000076,
        Key_TrebleDown = 0x01000077,
        Key_MediaPlay  = 0x01000080,
        Key_MediaStop  = 0x01000081,
        Key_MediaPrevious  = 0x01000082,
        Key_MediaNext  = 0x01000083,
        Key_MediaRecord = 0x01000084,
        Key_MediaPause = 0x1000085,
        Key_MediaTogglePlayPause = 0x1000086,
        Key_HomePage  = 0x01000090,
        Key_Favorites  = 0x01000091,
        Key_Search  = 0x01000092,
        Key_Standby = 0x01000093,
        Key_OpenUrl = 0x01000094,
        Key_LaunchMail  = 0x010000a0,
        Key_LaunchMedia = 0x010000a1,
        Key_Launch0  = 0x010000a2,
        Key_Launch1  = 0x010000a3,
        Key_Launch2  = 0x010000a4,
        Key_Launch3  = 0x010000a5,
        Key_Launch4  = 0x010000a6,
        Key_Launch5  = 0x010000a7,
        Key_Launch6  = 0x010000a8,
        Key_Launch7  = 0x010000a9,
        Key_Launch8  = 0x010000aa,
        Key_Launch9  = 0x010000ab,
        Key_LaunchA  = 0x010000ac,
        Key_LaunchB  = 0x010000ad,
        Key_LaunchC  = 0x010000ae,
        Key_LaunchD  = 0x010000af,
        Key_LaunchE  = 0x010000b0,
        Key_LaunchF  = 0x010000b1,
        Key_MonBrightnessUp = 0x010000b2,
        Key_MonBrightnessDown = 0x010000b3,
        Key_KeyboardLightOnOff = 0x010000b4,
        Key_KeyboardBrightnessUp = 0x010000b5,
        Key_KeyboardBrightnessDown = 0x010000b6,
        Key_PowerOff = 0x010000b7,
        Key_WakeUp = 0x010000b8,
        Key_Eject = 0x010000b9,
        Key_ScreenSaver = 0x010000ba,
        Key_WWW = 0x010000bb,
        Key_Memo = 0x010000bc,
        Key_LightBulb = 0x010000bd,
        Key_Shop = 0x010000be,
        Key_History = 0x010000bf,
        Key_AddFavorite = 0x010000c0,
        Key_HotLinks = 0x010000c1,
        Key_BrightnessAdjust = 0x010000c2,
        Key_Finance = 0x010000c3,
        Key_Community = 0x010000c4,
        Key_AudioRewind = 0x010000c5, // Media rewind
        Key_BackForward = 0x010000c6,
        Key_ApplicationLeft = 0x010000c7,
        Key_ApplicationRight = 0x010000c8,
        Key_Book = 0x010000c9,
        Key_CD = 0x010000ca,
        Key_Calculator = 0x010000cb,
        Key_ToDoList = 0x010000cc,
        Key_ClearGrab = 0x010000cd,
        Key_Close = 0x010000ce,
        Key_Copy = 0x010000cf,
        Key_Cut = 0x010000d0,
        Key_Display = 0x010000d1, // Output switch key
        Key_DOS = 0x010000d2,
        Key_Documents = 0x010000d3,
        Key_Excel = 0x010000d4,
        Key_Explorer = 0x010000d5,
        Key_Game = 0x010000d6,
        Key_Go = 0x010000d7,
        Key_iTouch = 0x010000d8,
        Key_LogOff = 0x010000d9,
        Key_Market = 0x010000da,
        Key_Meeting = 0x010000db,
        Key_MenuKB = 0x010000dc,
        Key_MenuPB = 0x010000dd,
        Key_MySites = 0x010000de,
        Key_News = 0x010000df,
        Key_OfficeHome = 0x010000e0,
        Key_Option = 0x010000e1,
        Key_Paste = 0x010000e2,
        Key_Phone = 0x010000e3,
        Key_Calendar = 0x010000e4,
        Key_Reply = 0x010000e5,
        Key_Reload = 0x010000e6,
        Key_RotateWindows = 0x010000e7,
        Key_RotationPB = 0x010000e8,
        Key_RotationKB = 0x010000e9,
        Key_Save = 0x010000ea,
        Key_Send = 0x010000eb,
        Key_Spell = 0x010000ec,
        Key_SplitScreen = 0x010000ed,
        Key_Support = 0x010000ee,
        Key_TaskPane = 0x010000ef,
        Key_Terminal = 0x010000f0,
        Key_Tools = 0x010000f1,
        Key_Travel = 0x010000f2,
        Key_Video = 0x010000f3,
        Key_Word = 0x010000f4,
        Key_Xfer = 0x010000f5,
        Key_ZoomIn = 0x010000f6,
        Key_ZoomOut = 0x010000f7,
        Key_Away = 0x010000f8,
        Key_Messenger = 0x010000f9,
        Key_WebCam = 0x010000fa,
        Key_MailForward = 0x010000fb,
        Key_Pictures = 0x010000fc,
        Key_Music = 0x010000fd,
        Key_Battery = 0x010000fe,
        Key_Bluetooth = 0x010000ff,
        Key_WLAN = 0x01000100,
        Key_UWB = 0x01000101,
        Key_AudioForward = 0x01000102, // Media fast-forward
        Key_AudioRepeat = 0x01000103, // Toggle repeat mode
        Key_AudioRandomPlay = 0x01000104, // Toggle shuffle mode
        Key_Subtitle = 0x01000105,
        Key_AudioCycleTrack = 0x01000106,
        Key_Time = 0x01000107,
        Key_Hibernate = 0x01000108,
        Key_View = 0x01000109,
        Key_TopMenu = 0x0100010a,
        Key_PowerDown = 0x0100010b,
        Key_Suspend = 0x0100010c,
        Key_ContrastAdjust = 0x0100010d,

        Key_LaunchG  = 0x0100010e,
        Key_LaunchH  = 0x0100010f,

        Key_TouchpadToggle = 0x01000110,
        Key_TouchpadOn = 0x01000111,
        Key_TouchpadOff = 0x01000112,

        Key_MicMute = 0x01000113,

        Key_Red = 0x01000114,
        Key_Green = 0x01000115,
        Key_Yellow = 0x01000116,
        Key_Blue = 0x01000117,

        Key_ChannelUp = 0x01000118,
        Key_ChannelDown = 0x01000119,

        Key_Guide    = 0x0100011a,
        Key_Info     = 0x0100011b,
        Key_Settings = 0x0100011c,

        Key_MicVolumeUp   = 0x0100011d,
        Key_MicVolumeDown = 0x0100011e,

        Key_New      = 0x01000120,
        Key_Open     = 0x01000121,
        Key_Find     = 0x01000122,
        Key_Undo     = 0x01000123,
        Key_Redo     = 0x01000124,

        Key_MediaLast = 0x0100ffff,

        // Keypad navigation keys
        Key_Select = 0x01010000,
        Key_Yes = 0x01010001,
        Key_No = 0x01010002,

        // Newer misc keys
        Key_Cancel  = 0x01020001,
        Key_Printer = 0x01020002,
        Key_Execute = 0x01020003,
        Key_Sleep   = 0x01020004,
        Key_Play    = 0x01020005, // Not the same as Key_MediaPlay
        Key_Zoom    = 0x01020006,
        //Key_Jisho   = 0x01020007, // IME: Dictionary key
        //Key_Oyayubi_Left = 0x01020008, // IME: Left Oyayubi key
        //Key_Oyayubi_Right = 0x01020009, // IME: Right Oyayubi key
        Key_Exit    = 0x0102000a,

        // Device keys
        Key_Context1 = 0x01100000,
        Key_Context2 = 0x01100001,
        Key_Context3 = 0x01100002,
        Key_Context4 = 0x01100003,
        Key_Call = 0x01100004,      // set absolute state to in a call (do not toggle state)
        Key_Hangup = 0x01100005,    // set absolute state to hang up (do not toggle state)
        Key_Flip = 0x01100006,
        Key_ToggleCallHangup = 0x01100007, // a toggle key for answering, or hanging up, based on current call state
        Key_VoiceDial = 0x01100008,
        Key_LastNumberRedial = 0x01100009,

        Key_Camera = 0x01100020,
        Key_CameraFocus = 0x01100021,

        Key_unknown = 0x01ffffff
    };

    enum ArrowType {
        NoArrow,
        UpArrow,
        DownArrow,
        LeftArrow,
        RightArrow
    };

    enum PenStyle { // pen style
        NoPen,
        SolidLine,
        DashLine,
        DotLine,
        DashDotLine,
        DashDotDotLine,
        CustomDashLine
#ifndef Q_MOC_RUN
        , MPenStyle = 0x0f
#endif
    };

    enum PenCapStyle { // line endcap style
        FlatCap = 0x00,
        SquareCap = 0x10,
        RoundCap = 0x20,
        MPenCapStyle = 0x30
    };

    enum PenJoinStyle { // line join style
        MiterJoin = 0x00,
        BevelJoin = 0x40,
        RoundJoin = 0x80,
        SvgMiterJoin = 0x100,
        MPenJoinStyle = 0x1c0
    };

    enum BrushStyle { // brush style
        NoBrush,
        SolidPattern,
        Dense1Pattern,
        Dense2Pattern,
        Dense3Pattern,
        Dense4Pattern,
        Dense5Pattern,
        Dense6Pattern,
        Dense7Pattern,
        HorPattern,
        VerPattern,
        CrossPattern,
        BDiagPattern,
        FDiagPattern,
        DiagCrossPattern,
        LinearGradientPattern,
        RadialGradientPattern,
        ConicalGradientPattern,
        TexturePattern = 24
    };

    enum SizeMode {
        AbsoluteSize,
        RelativeSize
    };

    enum UIEffect {
        UI_General,
        UI_AnimateMenu,
        UI_FadeMenu,
        UI_AnimateCombo,
        UI_AnimateTooltip,
        UI_FadeTooltip,
        UI_AnimateToolBox
    };

    enum CursorShape {
        ArrowCursor,
        UpArrowCursor,
        CrossCursor,
        WaitCursor,
        IBeamCursor,
        SizeVerCursor,
        SizeHorCursor,
        SizeBDiagCursor,
        SizeFDiagCursor,
        SizeAllCursor,
        BlankCursor,
        SplitVCursor,
        SplitHCursor,
        PointingHandCursor,
        ForbiddenCursor,
        WhatsThisCursor,
        BusyCursor,
        OpenHandCursor,
        ClosedHandCursor,
        DragCopyCursor,
        DragMoveCursor,
        DragLinkCursor,
        LastCursor = DragLinkCursor,
        BitmapCursor = 24,
        CustomCursor = 25
    };

    enum TextFormat {
        PlainText,
        RichText,
        AutoText,
        MarkdownText
    };

    enum AspectRatioMode {
        IgnoreAspectRatio,
        KeepAspectRatio,
        KeepAspectRatioByExpanding
    };

    enum DockWidgetArea {
        LeftDockWidgetArea = 0x1,
        RightDockWidgetArea = 0x2,
        TopDockWidgetArea = 0x4,
        BottomDockWidgetArea = 0x8,

        DockWidgetArea_Mask = 0xf,
        AllDockWidgetAreas = DockWidgetArea_Mask,
        NoDockWidgetArea = 0
    };
    enum DockWidgetAreaSizes {
        NDockWidgetAreas = 4
    };

    Q_DECLARE_FLAGS(DockWidgetAreas, DockWidgetArea)
    Q_DECLARE_OPERATORS_FOR_FLAGS(DockWidgetAreas)

    enum ToolBarArea {
        LeftToolBarArea = 0x1,
        RightToolBarArea = 0x2,
        TopToolBarArea = 0x4,
        BottomToolBarArea = 0x8,

        ToolBarArea_Mask = 0xf,
        AllToolBarAreas = ToolBarArea_Mask,
        NoToolBarArea = 0
    };

    enum ToolBarAreaSizes {
        NToolBarAreas = 4
    };

    Q_DECLARE_FLAGS(ToolBarAreas, ToolBarArea)
    Q_DECLARE_OPERATORS_FOR_FLAGS(ToolBarAreas)

    enum DateFormat {
        TextDate,      // default Qt
        ISODate,       // ISO 8601
#if QT_DEPRECATED_SINCE(5, 15)
        SystemLocaleDate Q_DECL_ENUMERATOR_DEPRECATED_X("Use QLocale"),
        LocalDate Q_DECL_ENUMERATOR_DEPRECATED_X("Use QLocale") = 2, // i.e. SystemLocaleDate
        LocaleDate Q_DECL_ENUMERATOR_DEPRECATED_X("Use QLocale"),
        SystemLocaleShortDate Q_DECL_ENUMERATOR_DEPRECATED_X("Use QLocale"),
        SystemLocaleLongDate Q_DECL_ENUMERATOR_DEPRECATED_X("Use QLocale"),
        DefaultLocaleShortDate Q_DECL_ENUMERATOR_DEPRECATED_X("Use QLocale"),
        DefaultLocaleLongDate Q_DECL_ENUMERATOR_DEPRECATED_X("Use QLocale"),
#endif
        RFC2822Date = 8, // RFC 2822 (+ 850 and 1036 during parsing)
        ISODateWithMs
    };

    enum TimeSpec {
        LocalTime,
        UTC,
        OffsetFromUTC,
        TimeZone
    };

    enum DayOfWeek {
        Monday = 1,
        Tuesday = 2,
        Wednesday = 3,
        Thursday = 4,
        Friday = 5,
        Saturday = 6,
        Sunday = 7
    };

    enum ScrollBarPolicy {
        ScrollBarAsNeeded,
        ScrollBarAlwaysOff,
        ScrollBarAlwaysOn
    };

    enum CaseSensitivity {
        CaseInsensitive,
        CaseSensitive
    };

    enum Corner {
        TopLeftCorner = 0x00000,
        TopRightCorner = 0x00001,
        BottomLeftCorner = 0x00002,
        BottomRightCorner = 0x00003
    };

    enum Edge {
        TopEdge = 0x00001,
        LeftEdge = 0x00002,
        RightEdge = 0x00004,
        BottomEdge = 0x00008
    };

    Q_DECLARE_FLAGS(Edges, Edge)
    Q_DECLARE_OPERATORS_FOR_FLAGS(Edges)

    enum ConnectionType {
        AutoConnection,
        DirectConnection,
        QueuedConnection,
        BlockingQueuedConnection,
        UniqueConnection =  0x80
    };

    enum ShortcutContext {
        WidgetShortcut,
        WindowShortcut,
        ApplicationShortcut,
        WidgetWithChildrenShortcut
    };

    enum FillRule {
        OddEvenFill,
        WindingFill
    };

    enum MaskMode {
        MaskInColor,
        MaskOutColor
    };

    enum ClipOperation {
        NoClip,
        ReplaceClip,
        IntersectClip
    };

    // Shape = 0x1, BoundingRect = 0x2
    enum ItemSelectionMode {
        ContainsItemShape = 0x0,
        IntersectsItemShape = 0x1,
        ContainsItemBoundingRect = 0x2,
        IntersectsItemBoundingRect = 0x3
    };

    enum ItemSelectionOperation {
        ReplaceSelection,
        AddToSelection
    };

    enum TransformationMode {
        FastTransformation,
        SmoothTransformation
    };

    enum Axis {
        XAxis,
        YAxis,
        ZAxis
    };

    enum FocusReason {
        MouseFocusReason,
        TabFocusReason,
        BacktabFocusReason,
        ActiveWindowFocusReason,
        PopupFocusReason,
        ShortcutFocusReason,
        MenuBarFocusReason,
        OtherFocusReason,
        NoFocusReason
    };

    enum ContextMenuPolicy {
        NoContextMenu,
        DefaultContextMenu,
        ActionsContextMenu,
        CustomContextMenu,
        PreventContextMenu
    };

    enum InputMethodQuery {
        ImEnabled = 0x1,
        ImCursorRectangle = 0x2,
#if QT_DEPRECATED_SINCE(5, 14)
        ImMicroFocus Q_DECL_ENUMERATOR_DEPRECATED = 0x2,
#endif
        ImFont = 0x4,
        ImCursorPosition = 0x8,
        ImSurroundingText = 0x10,
        ImCurrentSelection = 0x20,
        ImMaximumTextLength = 0x40,
        ImAnchorPosition = 0x80,
        ImHints = 0x100,
        ImPreferredLanguage = 0x200,

        ImAbsolutePosition = 0x400,
        ImTextBeforeCursor = 0x800,
        ImTextAfterCursor = 0x1000,
        ImEnterKeyType = 0x2000,
        ImAnchorRectangle = 0x4000,
        ImInputItemClipRectangle = 0x8000,

        ImPlatformData = 0x80000000,
        ImQueryInput = ImCursorRectangle | ImCursorPosition | ImSurroundingText |
                       ImCurrentSelection | ImAnchorRectangle | ImAnchorPosition,
        ImQueryAll = 0xffffffff
    };
    Q_DECLARE_FLAGS(InputMethodQueries, InputMethodQuery)
    Q_DECLARE_OPERATORS_FOR_FLAGS(InputMethodQueries)

    enum InputMethodHint {
        ImhNone = 0x0,

        ImhHiddenText = 0x1,
        ImhSensitiveData = 0x2,
        ImhNoAutoUppercase = 0x4,
        ImhPreferNumbers = 0x8,
        ImhPreferUppercase = 0x10,
        ImhPreferLowercase = 0x20,
        ImhNoPredictiveText = 0x40,

        ImhDate = 0x80,
        ImhTime = 0x100,

        ImhPreferLatin = 0x200,

        ImhMultiLine = 0x400,

        ImhNoEditMenu = 0x800,
        ImhNoTextHandles = 0x1000,

        ImhDigitsOnly = 0x10000,
        ImhFormattedNumbersOnly = 0x20000,
        ImhUppercaseOnly = 0x40000,
        ImhLowercaseOnly = 0x80000,
        ImhDialableCharactersOnly = 0x100000,
        ImhEmailCharactersOnly = 0x200000,
        ImhUrlCharactersOnly = 0x400000,
        ImhLatinOnly = 0x800000,

        ImhExclusiveInputMask = 0xffff0000
    };
    Q_DECLARE_FLAGS(InputMethodHints, InputMethodHint)
    Q_DECLARE_OPERATORS_FOR_FLAGS(InputMethodHints)

    enum EnterKeyType {
        EnterKeyDefault,
        EnterKeyReturn,
        EnterKeyDone,
        EnterKeyGo,
        EnterKeySend,
        EnterKeySearch,
        EnterKeyNext,
        EnterKeyPrevious
    };

    enum ToolButtonStyle {
        ToolButtonIconOnly,
        ToolButtonTextOnly,
        ToolButtonTextBesideIcon,
        ToolButtonTextUnderIcon,
        ToolButtonFollowStyle
    };

    enum LayoutDirection {
        LeftToRight,
        RightToLeft,
        LayoutDirectionAuto
    };

    enum AnchorPoint {
        AnchorLeft = 0,
        AnchorHorizontalCenter,
        AnchorRight,
        AnchorTop,
        AnchorVerticalCenter,
        AnchorBottom
    };

    enum FindChildOption {
        FindDirectChildrenOnly = 0x0,
        FindChildrenRecursively = 0x1
    };
    Q_DECLARE_FLAGS(FindChildOptions, FindChildOption)

    enum DropAction {
        CopyAction = 0x1,
        MoveAction = 0x2,
        LinkAction = 0x4,
        ActionMask = 0xff,
        TargetMoveAction = 0x8002,
        IgnoreAction = 0x0
    };
    Q_DECLARE_FLAGS(DropActions, DropAction)
    Q_DECLARE_OPERATORS_FOR_FLAGS(DropActions)

    enum CheckState {
        Unchecked,
        PartiallyChecked,
        Checked
    };

    enum ItemDataRole {
        DisplayRole = 0,
        DecorationRole = 1,
        EditRole = 2,
        ToolTipRole = 3,
        StatusTipRole = 4,
        WhatsThisRole = 5,
        // Metadata
        FontRole = 6,
        TextAlignmentRole = 7,
        BackgroundRole = 8,
        ForegroundRole = 9,
#if QT_DEPRECATED_SINCE(5, 13) // ### Qt 6: remove me
        BackgroundColorRole Q_DECL_ENUMERATOR_DEPRECATED = BackgroundRole,
        TextColorRole Q_DECL_ENUMERATOR_DEPRECATED = ForegroundRole,
#endif
        CheckStateRole = 10,
        // Accessibility
        AccessibleTextRole = 11,
        AccessibleDescriptionRole = 12,
        // More general purpose
        SizeHintRole = 13,
        InitialSortOrderRole = 14,
        // Internal UiLib roles. Start worrying when public roles go that high.
        DisplayPropertyRole = 27,
        DecorationPropertyRole = 28,
        ToolTipPropertyRole = 29,
        StatusTipPropertyRole = 30,
        WhatsThisPropertyRole = 31,
        // Reserved
        UserRole = 0x0100
    };

    enum ItemFlag {
        NoItemFlags = 0,
        ItemIsSelectable = 1,
        ItemIsEditable = 2,
        ItemIsDragEnabled = 4,
        ItemIsDropEnabled = 8,
        ItemIsUserCheckable = 16,
        ItemIsEnabled = 32,
        ItemIsAutoTristate = 64,
#if QT_DEPRECATED_SINCE(5, 6)
        ItemIsTristate = ItemIsAutoTristate,
#endif
        ItemNeverHasChildren = 128,
        ItemIsUserTristate = 256
    };
    Q_DECLARE_FLAGS(ItemFlags, ItemFlag)
    Q_DECLARE_OPERATORS_FOR_FLAGS(ItemFlags)

    enum MatchFlag {
        MatchExactly = 0,
        MatchContains = 1,
        MatchStartsWith = 2,
        MatchEndsWith = 3,
#if QT_DEPRECATED_SINCE(5, 15)
        MatchRegExp Q_DECL_ENUMERATOR_DEPRECATED_X("MatchRegExp is deprecated. Use MatchRegularExpression instead") = 4,
#endif
        MatchWildcard = 5,
        MatchFixedString = 8,
        MatchRegularExpression = 9,
        MatchCaseSensitive = 16,
        MatchWrap = 32,
        MatchRecursive = 64
    };
    Q_DECLARE_FLAGS(MatchFlags, MatchFlag)
    Q_DECLARE_OPERATORS_FOR_FLAGS(MatchFlags)

    typedef void * HANDLE;
#if QT_DEPRECATED_SINCE(5, 0)
    typedef WindowFlags WFlags;
#endif

    enum WindowModality {
        NonModal,
        WindowModal,
        ApplicationModal
    };

    enum TextInteractionFlag {
        NoTextInteraction         = 0,
        TextSelectableByMouse     = 1,
        TextSelectableByKeyboard  = 2,
        LinksAccessibleByMouse    = 4,
        LinksAccessibleByKeyboard = 8,
        TextEditable              = 16,

        TextEditorInteraction     = TextSelectableByMouse | TextSelectableByKeyboard | TextEditable,
        TextBrowserInteraction    = TextSelectableByMouse | LinksAccessibleByMouse | LinksAccessibleByKeyboard
    };
    Q_DECLARE_FLAGS(TextInteractionFlags, TextInteractionFlag)
    Q_DECLARE_OPERATORS_FOR_FLAGS(TextInteractionFlags)

    enum EventPriority {
        HighEventPriority = 1,
        NormalEventPriority = 0,
        LowEventPriority = -1
    };

    enum SizeHint {
        MinimumSize,
        PreferredSize,
        MaximumSize,
        MinimumDescent,
        NSizeHints
    };

    enum WindowFrameSection {
        NoSection,
        LeftSection,           // For resize
        TopLeftSection,
        TopSection,
        TopRightSection,
        RightSection,
        BottomRightSection,
        BottomSection,
        BottomLeftSection,
        TitleBarArea    // For move
    };

#if defined(Q_COMPILER_CONSTEXPR)
    enum class Initialization {
        Uninitialized
    };
    static constexpr Q_DECL_UNUSED Initialization Uninitialized = Initialization::Uninitialized;
#else
    enum Initialization {
        Uninitialized
    };
#endif

    enum CoordinateSystem {
        DeviceCoordinates,
        LogicalCoordinates
    };

    enum TouchPointState {
        TouchPointPressed    = 0x01,
        TouchPointMoved      = 0x02,
        TouchPointStationary = 0x04,
        TouchPointReleased   = 0x08
    };
    Q_DECLARE_FLAGS(TouchPointStates, TouchPointState)
    Q_DECLARE_OPERATORS_FOR_FLAGS(TouchPointStates)

#ifndef QT_NO_GESTURES
    enum GestureState
    {
        NoGesture,
        GestureStarted  = 1,
        GestureUpdated  = 2,
        GestureFinished = 3,
        GestureCanceled = 4
    };

    enum GestureType
    {
        TapGesture        = 1,
        TapAndHoldGesture = 2,
        PanGesture        = 3,
        PinchGesture      = 4,
        SwipeGesture      = 5,

        CustomGesture     = 0x0100,

        LastGestureType   = ~0u
    };

    enum GestureFlag
    {
        DontStartGestureOnChildren = 0x01,
        ReceivePartialGestures     = 0x02,
        IgnoredGesturesPropagateToParent = 0x04
    };
    Q_DECLARE_FLAGS(GestureFlags, GestureFlag)
    Q_DECLARE_OPERATORS_FOR_FLAGS(GestureFlags)

    enum NativeGestureType
    {
        BeginNativeGesture,
        EndNativeGesture,
        PanNativeGesture,
        ZoomNativeGesture,
        SmartZoomNativeGesture,
        RotateNativeGesture,
        SwipeNativeGesture
    };

#endif // QT_NO_GESTURES

    enum NavigationMode
    {
        NavigationModeNone,
        NavigationModeKeypadTabOrder,
        NavigationModeKeypadDirectional,
        NavigationModeCursorAuto,
        NavigationModeCursorForceVisible
    };

    enum CursorMoveStyle {
        LogicalMoveStyle,
        VisualMoveStyle
    };

    enum TimerType {
        PreciseTimer,
        CoarseTimer,
        VeryCoarseTimer
    };

    enum ScrollPhase {
        NoScrollPhase = 0,
        ScrollBegin,
        ScrollUpdate,
        ScrollEnd,
        ScrollMomentum
    };

    enum MouseEventSource {
        MouseEventNotSynthesized,
        MouseEventSynthesizedBySystem,
        MouseEventSynthesizedByQt,
        MouseEventSynthesizedByApplication
    };

    enum MouseEventFlag {
        MouseEventCreatedDoubleClick = 0x01,
        MouseEventFlagMask = 0xFF
    };
    Q_DECLARE_FLAGS(MouseEventFlags, MouseEventFlag)
    Q_DECLARE_OPERATORS_FOR_FLAGS(MouseEventFlags)

    enum ChecksumType {
        ChecksumIso3309,
        ChecksumItuV41
    };

    enum class HighDpiScaleFactorRoundingPolicy {
        Unset,
        Round,
        Ceil,
        Floor,
        RoundPreferFloor,
        PassThrough
    };

    // QTBUG-48701
    enum ReturnByValueConstant { ReturnByValue }; // ### Qt 7: Remove me

#ifndef Q_QDOC
    // NOTE: Generally, do not add QT_Q_ENUM if a corresponding Q_Q_FLAG exists.
    QT_Q_ENUM(ScrollBarPolicy)
    QT_Q_ENUM(FocusPolicy)
    QT_Q_ENUM(ContextMenuPolicy)
    QT_Q_ENUM(ArrowType)
    QT_Q_ENUM(ToolButtonStyle)
    QT_Q_ENUM(PenStyle)
    QT_Q_ENUM(PenCapStyle)
    QT_Q_ENUM(PenJoinStyle)
    QT_Q_ENUM(BrushStyle)
    QT_Q_ENUM(FillRule)
    QT_Q_ENUM(MaskMode)
    QT_Q_ENUM(BGMode)
    QT_Q_ENUM(ClipOperation)
    QT_Q_ENUM(SizeMode)
    QT_Q_ENUM(Axis)
    QT_Q_ENUM(Corner)
    QT_Q_ENUM(Edge)
    QT_Q_ENUM(LayoutDirection)
    QT_Q_ENUM(SizeHint)
    QT_Q_ENUM(Orientation)
    QT_Q_ENUM(DropAction)
    QT_Q_FLAG(Alignment)
    QT_Q_ENUM(TextFlag)
    QT_Q_FLAG(Orientations)
    QT_Q_FLAG(SplitBehavior)
    QT_Q_FLAG(DropActions)
    QT_Q_FLAG(Edges)
    QT_Q_FLAG(DockWidgetAreas)
    QT_Q_FLAG(ToolBarAreas)
    QT_Q_ENUM(DockWidgetArea)
    QT_Q_ENUM(ToolBarArea)
    QT_Q_ENUM(TextFormat)
    QT_Q_ENUM(TextElideMode)
    QT_Q_ENUM(DateFormat)
    QT_Q_ENUM(TimeSpec)
    QT_Q_ENUM(DayOfWeek)
    QT_Q_ENUM(CursorShape)
    QT_Q_ENUM(GlobalColor)
    QT_Q_ENUM(AspectRatioMode)
    QT_Q_ENUM(TransformationMode)
    QT_Q_FLAG(ImageConversionFlags)
    QT_Q_ENUM(Key)
    QT_Q_ENUM(ShortcutContext)
    QT_Q_ENUM(TextInteractionFlag)
    QT_Q_FLAG(TextInteractionFlags)
    QT_Q_ENUM(ItemSelectionMode)
    QT_Q_ENUM(ItemSelectionOperation)
    QT_Q_FLAG(ItemFlags)
    QT_Q_ENUM(CheckState)
    QT_Q_ENUM(ItemDataRole)
    QT_Q_ENUM(SortOrder)
    QT_Q_ENUM(CaseSensitivity)
    QT_Q_FLAG(MatchFlags)
    QT_Q_FLAG(KeyboardModifiers)
    QT_Q_FLAG(MouseButtons)
    QT_Q_ENUM(WindowType)
    QT_Q_ENUM(WindowState)
    QT_Q_ENUM(WindowModality)
    QT_Q_ENUM(WidgetAttribute)
    QT_Q_ENUM(ApplicationAttribute)
    QT_Q_FLAG(WindowFlags)
    QT_Q_FLAG(WindowStates)
    QT_Q_ENUM(FocusReason)
    QT_Q_ENUM(InputMethodHint)
    QT_Q_ENUM(InputMethodQuery)
    QT_Q_FLAG(InputMethodHints)
    QT_Q_ENUM(EnterKeyType)
    QT_Q_FLAG(InputMethodQueries)
    QT_Q_FLAG(TouchPointStates)
    QT_Q_ENUM(ScreenOrientation)
    QT_Q_FLAG(ScreenOrientations)
    QT_Q_ENUM(ConnectionType)
    QT_Q_ENUM(ApplicationState)
#ifndef QT_NO_GESTURES
    QT_Q_ENUM(GestureState)
    QT_Q_ENUM(GestureType)
    QT_Q_ENUM(NativeGestureType)
#endif
    QT_Q_ENUM(CursorMoveStyle)
    QT_Q_ENUM(TimerType)
    QT_Q_ENUM(ScrollPhase)
    QT_Q_ENUM(MouseEventSource)
    QT_Q_FLAG(MouseEventFlag)
    QT_Q_ENUM(ChecksumType)
    QT_Q_ENUM(HighDpiScaleFactorRoundingPolicy)
    QT_Q_ENUM(TabFocusBehavior)
#endif // Q_DOC

}
#ifdef Q_MOC_RUN
 ;
#endif

#undef QT_Q_ENUM
#undef QT_Q_FLAG

typedef bool (*qInternalCallback)(void **);

class Q_CORE_EXPORT QInternal {
public:
    enum PaintDeviceFlags {
        UnknownDevice = 0x00,
        Widget        = 0x01,
        Pixmap        = 0x02,
        Image         = 0x03,
        Printer       = 0x04,
        Picture       = 0x05,
        Pbuffer       = 0x06,    // GL pbuffer
        FramebufferObject = 0x07, // GL framebuffer object
        CustomRaster  = 0x08,
        MacQuartz     = 0x09,
        PaintBuffer   = 0x0a,
        OpenGL        = 0x0b
    };
    enum RelayoutType {
        RelayoutNormal,
        RelayoutDragging,
        RelayoutDropped
    };

    enum DockPosition {
        LeftDock,
        RightDock,
        TopDock,
        BottomDock,
        DockCount
    };

    enum Callback {
        EventNotifyCallback,
        LastCallback
    };
    static bool registerCallback(Callback, qInternalCallback);
    static bool unregisterCallback(Callback, qInternalCallback);
    static bool activateCallbacks(Callback, void **);
};

QT_END_NAMESPACE

#endif // QNAMESPACE_H
