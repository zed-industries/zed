// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSTYLE_H
#define QSTYLE_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtCore/qobject.h>
#include <QtCore/qrect.h>
#include <QtCore/qsize.h>
#include <QtGui/qicon.h>
#include <QtGui/qpixmap.h>
#include <QtGui/qpalette.h>
#include <QtWidgets/qsizepolicy.h>

QT_BEGIN_NAMESPACE


class QAction;
class QDebug;
class QTab;
class QFontMetrics;
class QStyleHintReturn;
class QStyleOption;
class QStyleOptionComplex;
class QStylePrivate;

class Q_WIDGETS_EXPORT QStyle : public QObject
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QStyle)

protected:
    QStyle(QStylePrivate &dd);

public:
    QStyle();
    virtual ~QStyle();

    QString name() const;

    virtual void polish(QWidget *widget);
    virtual void unpolish(QWidget *widget);

    virtual void polish(QApplication *application);
    virtual void unpolish(QApplication *application);

    virtual void polish(QPalette &palette);

    virtual QRect itemTextRect(const QFontMetrics &fm, const QRect &r,
                           int flags, bool enabled,
                           const QString &text) const;

    virtual QRect itemPixmapRect(const QRect &r, int flags, const QPixmap &pixmap) const;

    virtual void drawItemText(QPainter *painter, const QRect &rect,
                              int flags, const QPalette &pal, bool enabled,
                              const QString &text, QPalette::ColorRole textRole = QPalette::NoRole) const;

    virtual void drawItemPixmap(QPainter *painter, const QRect &rect,
                                int alignment, const QPixmap &pixmap) const;

    virtual QPalette standardPalette() const;

    enum StateFlag {
        State_None =                0x00000000,
        State_Enabled =             0x00000001,
        State_Raised =              0x00000002,
        State_Sunken =              0x00000004,
        State_Off =                 0x00000008,
        State_NoChange =            0x00000010,
        State_On =                  0x00000020,
        State_DownArrow =           0x00000040,
        State_Horizontal =          0x00000080,
        State_HasFocus =            0x00000100,
        State_Top =                 0x00000200,
        State_Bottom =              0x00000400,
        State_FocusAtBorder =       0x00000800,
        State_AutoRaise =           0x00001000,
        State_MouseOver =           0x00002000,
        State_UpArrow =             0x00004000,
        State_Selected =            0x00008000,
        State_Active =              0x00010000,
        State_Window =              0x00020000,
        State_Open =                0x00040000,
        State_Children =            0x00080000,
        State_Item =                0x00100000,
        State_Sibling =             0x00200000,
        State_Editing =             0x00400000,
        State_KeyboardFocusChange = 0x00800000,
#ifdef QT_KEYPAD_NAVIGATION
        State_HasEditFocus =        0x01000000,
#endif
        State_ReadOnly =            0x02000000,
        State_Small =               0x04000000,
        State_Mini =                0x08000000
    };
    Q_ENUM(StateFlag)
    Q_DECLARE_FLAGS(State, StateFlag)


    enum PrimitiveElement {
        PE_Frame,
        PE_FrameDefaultButton,
        PE_FrameDockWidget,
        PE_FrameFocusRect,
        PE_FrameGroupBox,
        PE_FrameLineEdit,
        PE_FrameMenu,
        PE_FrameStatusBarItem,
        PE_FrameTabWidget,
        PE_FrameWindow,
        PE_FrameButtonBevel,
        PE_FrameButtonTool,
        PE_FrameTabBarBase,

        PE_PanelButtonCommand,
        PE_PanelButtonBevel,
        PE_PanelButtonTool,
        PE_PanelMenuBar,
        PE_PanelToolBar,
        PE_PanelLineEdit,

        PE_IndicatorArrowDown,
        PE_IndicatorArrowLeft,
        PE_IndicatorArrowRight,
        PE_IndicatorArrowUp,
        PE_IndicatorBranch,
        PE_IndicatorButtonDropDown,
        PE_IndicatorItemViewItemCheck,
        PE_IndicatorCheckBox,
        PE_IndicatorDockWidgetResizeHandle,
        PE_IndicatorHeaderArrow,
        PE_IndicatorMenuCheckMark,
        PE_IndicatorProgressChunk,
        PE_IndicatorRadioButton,
        PE_IndicatorSpinDown,
        PE_IndicatorSpinMinus,
        PE_IndicatorSpinPlus,
        PE_IndicatorSpinUp,
        PE_IndicatorToolBarHandle,
        PE_IndicatorToolBarSeparator,
        PE_PanelTipLabel,
        PE_IndicatorTabTear,
        PE_IndicatorTabTearLeft = PE_IndicatorTabTear,
        PE_PanelScrollAreaCorner,

        PE_Widget,

        PE_IndicatorColumnViewArrow,
        PE_IndicatorItemViewItemDrop,

        PE_PanelItemViewItem,
        PE_PanelItemViewRow,

        PE_PanelStatusBar,

        PE_IndicatorTabClose,
        PE_PanelMenu,

        PE_IndicatorTabTearRight,

        // do not add any values below/greater this
        PE_CustomBase = 0xf000000
    };
    Q_ENUM(PrimitiveElement)

    virtual void drawPrimitive(PrimitiveElement pe, const QStyleOption *opt, QPainter *p,
                               const QWidget *w = nullptr) const = 0;
    enum ControlElement {
        CE_PushButton,
        CE_PushButtonBevel,
        CE_PushButtonLabel,

        CE_CheckBox,
        CE_CheckBoxLabel,

        CE_RadioButton,
        CE_RadioButtonLabel,

        CE_TabBarTab,
        CE_TabBarTabShape,
        CE_TabBarTabLabel,

        CE_ProgressBar,
        CE_ProgressBarGroove,
        CE_ProgressBarContents,
        CE_ProgressBarLabel,

        CE_MenuItem,
        CE_MenuScroller,
        CE_MenuVMargin,
        CE_MenuHMargin,
        CE_MenuTearoff,
        CE_MenuEmptyArea,

        CE_MenuBarItem,
        CE_MenuBarEmptyArea,

        CE_ToolButtonLabel,

        CE_Header,
        CE_HeaderSection,
        CE_HeaderLabel,

        CE_ToolBoxTab,
        CE_SizeGrip,
        CE_Splitter,
        CE_RubberBand,
        CE_DockWidgetTitle,

        CE_ScrollBarAddLine,
        CE_ScrollBarSubLine,
        CE_ScrollBarAddPage,
        CE_ScrollBarSubPage,
        CE_ScrollBarSlider,
        CE_ScrollBarFirst,
        CE_ScrollBarLast,

        CE_FocusFrame,
        CE_ComboBoxLabel,

        CE_ToolBar,
        CE_ToolBoxTabShape,
        CE_ToolBoxTabLabel,
        CE_HeaderEmptyArea,

        CE_ColumnViewGrip,

        CE_ItemViewItem,

        CE_ShapedFrame,

        // do not add any values below/greater than this
        CE_CustomBase = 0xf0000000
    };
    Q_ENUM(ControlElement)

    virtual void drawControl(ControlElement element, const QStyleOption *opt, QPainter *p,
                             const QWidget *w = nullptr) const = 0;

    enum SubElement {
        SE_PushButtonContents,
        SE_PushButtonFocusRect,

        SE_CheckBoxIndicator,
        SE_CheckBoxContents,
        SE_CheckBoxFocusRect,
        SE_CheckBoxClickRect,

        SE_RadioButtonIndicator,
        SE_RadioButtonContents,
        SE_RadioButtonFocusRect,
        SE_RadioButtonClickRect,

        SE_ComboBoxFocusRect,

        SE_SliderFocusRect,

        SE_ProgressBarGroove,
        SE_ProgressBarContents,
        SE_ProgressBarLabel,

        SE_ToolBoxTabContents,

        SE_HeaderLabel,
        SE_HeaderArrow,

        SE_TabWidgetTabBar,
        SE_TabWidgetTabPane,
        SE_TabWidgetTabContents,
        SE_TabWidgetLeftCorner,
        SE_TabWidgetRightCorner,

        SE_ItemViewItemCheckIndicator,
        SE_TabBarTearIndicator,
        SE_TabBarTearIndicatorLeft = SE_TabBarTearIndicator,

        SE_TreeViewDisclosureItem,

        SE_LineEditContents,
        SE_FrameContents,

        SE_DockWidgetCloseButton,
        SE_DockWidgetFloatButton,
        SE_DockWidgetTitleBarText,
        SE_DockWidgetIcon,

        SE_CheckBoxLayoutItem,
        SE_ComboBoxLayoutItem,
        SE_DateTimeEditLayoutItem,
        SE_LabelLayoutItem,
        SE_ProgressBarLayoutItem,
        SE_PushButtonLayoutItem,
        SE_RadioButtonLayoutItem,
        SE_SliderLayoutItem,
        SE_SpinBoxLayoutItem,
        SE_ToolButtonLayoutItem,

        SE_FrameLayoutItem,
        SE_GroupBoxLayoutItem,
        SE_TabWidgetLayoutItem,

        SE_ItemViewItemDecoration,
        SE_ItemViewItemText,
        SE_ItemViewItemFocusRect,

        SE_TabBarTabLeftButton,
        SE_TabBarTabRightButton,
        SE_TabBarTabText,

        SE_ShapedFrameContents,

        SE_ToolBarHandle,

        SE_TabBarScrollLeftButton,
        SE_TabBarScrollRightButton,
        SE_TabBarTearIndicatorRight,

        SE_PushButtonBevel,

        // do not add any values below/greater than this
        SE_CustomBase = 0xf0000000
    };
    Q_ENUM(SubElement)

    virtual QRect subElementRect(SubElement subElement, const QStyleOption *option,
                                 const QWidget *widget = nullptr) const = 0;


    enum ComplexControl {
        CC_SpinBox,
        CC_ComboBox,
        CC_ScrollBar,
        CC_Slider,
        CC_ToolButton,
        CC_TitleBar,
        CC_Dial,
        CC_GroupBox,
        CC_MdiControls,

        // do not add any values below/greater than this
        CC_CustomBase = 0xf0000000
    };
    Q_ENUM(ComplexControl)

    enum SubControl {
        SC_None =                  0x00000000,

        SC_ScrollBarAddLine =      0x00000001,
        SC_ScrollBarSubLine =      0x00000002,
        SC_ScrollBarAddPage =      0x00000004,
        SC_ScrollBarSubPage =      0x00000008,
        SC_ScrollBarFirst =        0x00000010,
        SC_ScrollBarLast =         0x00000020,
        SC_ScrollBarSlider =       0x00000040,
        SC_ScrollBarGroove =       0x00000080,

        SC_SpinBoxUp =             0x00000001,
        SC_SpinBoxDown =           0x00000002,
        SC_SpinBoxFrame =          0x00000004,
        SC_SpinBoxEditField =      0x00000008,

        SC_ComboBoxFrame =         0x00000001,
        SC_ComboBoxEditField =     0x00000002,
        SC_ComboBoxArrow =         0x00000004,
        SC_ComboBoxListBoxPopup =  0x00000008,

        SC_SliderGroove =          0x00000001,
        SC_SliderHandle =          0x00000002,
        SC_SliderTickmarks =       0x00000004,

        SC_ToolButton =            0x00000001,
        SC_ToolButtonMenu =        0x00000002,

        SC_TitleBarSysMenu =       0x00000001,
        SC_TitleBarMinButton =     0x00000002,
        SC_TitleBarMaxButton =     0x00000004,
        SC_TitleBarCloseButton =   0x00000008,
        SC_TitleBarNormalButton =  0x00000010,
        SC_TitleBarShadeButton =   0x00000020,
        SC_TitleBarUnshadeButton = 0x00000040,
        SC_TitleBarContextHelpButton = 0x00000080,
        SC_TitleBarLabel =         0x00000100,

        SC_DialGroove =            0x00000001,
        SC_DialHandle =            0x00000002,
        SC_DialTickmarks =         0x00000004,

        SC_GroupBoxCheckBox =      0x00000001,
        SC_GroupBoxLabel =         0x00000002,
        SC_GroupBoxContents =      0x00000004,
        SC_GroupBoxFrame =         0x00000008,

        SC_MdiMinButton     =      0x00000001,
        SC_MdiNormalButton  =      0x00000002,
        SC_MdiCloseButton   =      0x00000004,

        SC_CustomBase =            0xf0000000,
        SC_All =                   0xffffffff
    };
    Q_ENUM(SubControl)
    Q_DECLARE_FLAGS(SubControls, SubControl)


    virtual void drawComplexControl(ComplexControl cc, const QStyleOptionComplex *opt, QPainter *p,
                                    const QWidget *widget = nullptr) const = 0;
    virtual SubControl hitTestComplexControl(ComplexControl cc, const QStyleOptionComplex *opt,
                                             const QPoint &pt, const QWidget *widget = nullptr) const = 0;
    virtual QRect subControlRect(ComplexControl cc, const QStyleOptionComplex *opt,
                                 SubControl sc, const QWidget *widget = nullptr) const = 0;

    enum PixelMetric {
        PM_ButtonMargin,
        PM_ButtonDefaultIndicator,
        PM_MenuButtonIndicator,
        PM_ButtonShiftHorizontal,
        PM_ButtonShiftVertical,

        PM_DefaultFrameWidth,
        PM_SpinBoxFrameWidth,
        PM_ComboBoxFrameWidth,

        PM_MaximumDragDistance,

        PM_ScrollBarExtent,
        PM_ScrollBarSliderMin,

        PM_SliderThickness,             // total slider thickness
        PM_SliderControlThickness,      // thickness of the business part
        PM_SliderLength,                // total length of slider
        PM_SliderTickmarkOffset,        //
        PM_SliderSpaceAvailable,        // available space for slider to move

        PM_DockWidgetSeparatorExtent,
        PM_DockWidgetHandleExtent,
        PM_DockWidgetFrameWidth,

        PM_TabBarTabOverlap,
        PM_TabBarTabHSpace,
        PM_TabBarTabVSpace,
        PM_TabBarBaseHeight,
        PM_TabBarBaseOverlap,

        PM_ProgressBarChunkWidth,

        PM_SplitterWidth,
        PM_TitleBarHeight,

        PM_MenuScrollerHeight,
        PM_MenuHMargin,
        PM_MenuVMargin,
        PM_MenuPanelWidth,
        PM_MenuTearoffHeight,
        PM_MenuDesktopFrameWidth,

        PM_MenuBarPanelWidth,
        PM_MenuBarItemSpacing,
        PM_MenuBarVMargin,
        PM_MenuBarHMargin,

        PM_IndicatorWidth,
        PM_IndicatorHeight,
        PM_ExclusiveIndicatorWidth,
        PM_ExclusiveIndicatorHeight,

        PM_DialogButtonsSeparator,
        PM_DialogButtonsButtonWidth,
        PM_DialogButtonsButtonHeight,

        PM_MdiSubWindowFrameWidth,
        PM_MdiSubWindowMinimizedWidth,

        PM_HeaderMargin,
        PM_HeaderMarkSize,
        PM_HeaderGripMargin,
        PM_TabBarTabShiftHorizontal,
        PM_TabBarTabShiftVertical,
        PM_TabBarScrollButtonWidth,

        PM_ToolBarFrameWidth,
        PM_ToolBarHandleExtent,
        PM_ToolBarItemSpacing,
        PM_ToolBarItemMargin,
        PM_ToolBarSeparatorExtent,
        PM_ToolBarExtensionExtent,

        PM_SpinBoxSliderHeight,

        PM_ToolBarIconSize,
        PM_ListViewIconSize,
        PM_IconViewIconSize,
        PM_SmallIconSize,
        PM_LargeIconSize,

        PM_FocusFrameVMargin,
        PM_FocusFrameHMargin,

        PM_ToolTipLabelFrameWidth,
        PM_CheckBoxLabelSpacing,
        PM_TabBarIconSize,
        PM_SizeGripSize,
        PM_DockWidgetTitleMargin,
        PM_MessageBoxIconSize,
        PM_ButtonIconSize,

        PM_DockWidgetTitleBarButtonMargin,

        PM_RadioButtonLabelSpacing,
        PM_LayoutLeftMargin,
        PM_LayoutTopMargin,
        PM_LayoutRightMargin,
        PM_LayoutBottomMargin,
        PM_LayoutHorizontalSpacing,
        PM_LayoutVerticalSpacing,
        PM_TabBar_ScrollButtonOverlap,

        PM_TextCursorWidth,

        PM_TabCloseIndicatorWidth,
        PM_TabCloseIndicatorHeight,

        PM_ScrollView_ScrollBarSpacing,
        PM_ScrollView_ScrollBarOverlap,
        PM_SubMenuOverlap,
        PM_TreeViewIndentation,

        PM_HeaderDefaultSectionSizeHorizontal,
        PM_HeaderDefaultSectionSizeVertical,

        PM_TitleBarButtonIconSize,
        PM_TitleBarButtonSize,

        PM_LineEditIconSize,
        PM_LineEditIconMargin,
        // do not add any values below/greater than this
        PM_CustomBase = 0xf0000000
    };
    Q_ENUM(PixelMetric)

    virtual int pixelMetric(PixelMetric metric, const QStyleOption *option = nullptr,
                            const QWidget *widget = nullptr) const = 0;

    enum ContentsType {
        CT_PushButton,
        CT_CheckBox,
        CT_RadioButton,
        CT_ToolButton,
        CT_ComboBox,
        CT_Splitter,
        CT_ProgressBar,
        CT_MenuItem,
        CT_MenuBarItem,
        CT_MenuBar,
        CT_Menu,
        CT_TabBarTab,
        CT_Slider,
        CT_ScrollBar,
        CT_LineEdit,
        CT_SpinBox,
        CT_SizeGrip,
        CT_TabWidget,
        CT_DialogButtons,
        CT_HeaderSection,
        CT_GroupBox,
        CT_MdiControls,
        CT_ItemViewItem,
        // do not add any values below/greater than this
        CT_CustomBase = 0xf0000000
    };
    Q_ENUM(ContentsType)

    virtual QSize sizeFromContents(ContentsType ct, const QStyleOption *opt,
                                   const QSize &contentsSize, const QWidget *w = nullptr) const = 0;

    enum RequestSoftwareInputPanel {
        RSIP_OnMouseClickAndAlreadyFocused,
        RSIP_OnMouseClick
    };
    Q_ENUM(RequestSoftwareInputPanel)

    enum StyleHint {
        SH_EtchDisabledText,
        SH_DitherDisabledText,
        SH_ScrollBar_MiddleClickAbsolutePosition,
        SH_ScrollBar_ScrollWhenPointerLeavesControl,
        SH_TabBar_SelectMouseType,
        SH_TabBar_Alignment,
        SH_Header_ArrowAlignment,
        SH_Slider_SnapToValue,
        SH_Slider_SloppyKeyEvents,
        SH_ProgressDialog_CenterCancelButton,
        SH_ProgressDialog_TextLabelAlignment,
        SH_PrintDialog_RightAlignButtons,
        SH_MainWindow_SpaceBelowMenuBar,
        SH_FontDialog_SelectAssociatedText,
        SH_Menu_AllowActiveAndDisabled,
        SH_Menu_SpaceActivatesItem,
        SH_Menu_SubMenuPopupDelay,
        SH_ScrollView_FrameOnlyAroundContents,
        SH_MenuBar_AltKeyNavigation,
        SH_ComboBox_ListMouseTracking,
        SH_Menu_MouseTracking,
        SH_MenuBar_MouseTracking,
        SH_ItemView_ChangeHighlightOnFocus,
        SH_Widget_ShareActivation,
        SH_Workspace_FillSpaceOnMaximize,
        SH_ComboBox_Popup,
        SH_TitleBar_NoBorder,
        SH_Slider_StopMouseOverSlider,
        SH_BlinkCursorWhenTextSelected,
        SH_RichText_FullWidthSelection,
        SH_Menu_Scrollable,
        SH_GroupBox_TextLabelVerticalAlignment,
        SH_GroupBox_TextLabelColor,
        SH_Menu_SloppySubMenus,
        SH_Table_GridLineColor,
        SH_LineEdit_PasswordCharacter,
        SH_DialogButtons_DefaultButton,
        SH_ToolBox_SelectedPageTitleBold,
        SH_TabBar_PreferNoArrows,
        SH_ScrollBar_LeftClickAbsolutePosition,
        SH_ListViewExpand_SelectMouseType,
        SH_UnderlineShortcut,
        SH_SpinBox_AnimateButton,
        SH_SpinBox_KeyPressAutoRepeatRate,
        SH_SpinBox_ClickAutoRepeatRate,
        SH_Menu_FillScreenWithScroll,
        SH_ToolTipLabel_Opacity,
        SH_DrawMenuBarSeparator,
        SH_TitleBar_ModifyNotification,
        SH_Button_FocusPolicy,
        SH_MessageBox_UseBorderForButtonSpacing,
        SH_TitleBar_AutoRaise,
        SH_ToolButton_PopupDelay,
        SH_FocusFrame_Mask,
        SH_RubberBand_Mask,
        SH_WindowFrame_Mask,
        SH_SpinControls_DisableOnBounds,
        SH_Dial_BackgroundRole,
        SH_ComboBox_LayoutDirection,
        SH_ItemView_EllipsisLocation,
        SH_ItemView_ShowDecorationSelected,
        SH_ItemView_ActivateItemOnSingleClick,
        SH_ScrollBar_ContextMenu,
        SH_ScrollBar_RollBetweenButtons,
        SH_Slider_AbsoluteSetButtons,
        SH_Slider_PageSetButtons,
        SH_Menu_KeyboardSearch,
        SH_TabBar_ElideMode,
        SH_DialogButtonLayout,
        SH_ComboBox_PopupFrameStyle,
        SH_MessageBox_TextInteractionFlags,
        SH_DialogButtonBox_ButtonsHaveIcons,
        SH_MessageBox_CenterButtons,
        SH_Menu_SelectionWrap,
        SH_ItemView_MovementWithoutUpdatingSelection,
        SH_ToolTip_Mask,
        SH_FocusFrame_AboveWidget,
        SH_TextControl_FocusIndicatorTextCharFormat,
        SH_WizardStyle,
        SH_ItemView_ArrowKeysNavigateIntoChildren,
        SH_Menu_Mask,
        SH_Menu_FlashTriggeredItem,
        SH_Menu_FadeOutOnHide,
        SH_SpinBox_ClickAutoRepeatThreshold,
        SH_ItemView_PaintAlternatingRowColorsForEmptyArea,
        SH_FormLayoutWrapPolicy,
        SH_TabWidget_DefaultTabPosition,
        SH_ToolBar_Movable,
        SH_FormLayoutFieldGrowthPolicy,
        SH_FormLayoutFormAlignment,
        SH_FormLayoutLabelAlignment,
        SH_ItemView_DrawDelegateFrame,
        SH_TabBar_CloseButtonPosition,
        SH_DockWidget_ButtonsHaveFrame,
        SH_ToolButtonStyle,
        SH_RequestSoftwareInputPanel,
        SH_ScrollBar_Transient,
        SH_Menu_SupportsSections,
        SH_ToolTip_WakeUpDelay,
        SH_ToolTip_FallAsleepDelay,
        SH_Widget_Animate,
        SH_Splitter_OpaqueResize,
        // Whether we should use a native popup.
        // Only supported for non-editable combo boxes on Mac OS X so far.
        SH_ComboBox_UseNativePopup,
        SH_LineEdit_PasswordMaskDelay,
        SH_TabBar_ChangeCurrentDelay,
        SH_Menu_SubMenuUniDirection,
        SH_Menu_SubMenuUniDirectionFailCount,
        SH_Menu_SubMenuSloppySelectOtherActions,
        SH_Menu_SubMenuSloppyCloseTimeout,
        SH_Menu_SubMenuResetWhenReenteringParent,
        SH_Menu_SubMenuDontStartSloppyOnLeave,
        SH_ItemView_ScrollMode,
        SH_TitleBar_ShowToolTipsOnButtons,
        SH_Widget_Animation_Duration,
        SH_ComboBox_AllowWheelScrolling,
        SH_SpinBox_ButtonsInsideFrame,
        SH_SpinBox_StepModifier,
        SH_TabBar_AllowWheelScrolling,
        SH_Table_AlwaysDrawLeftTopGridLines,
        SH_SpinBox_SelectOnStep,
        // Add new style hint values here

        SH_CustomBase = 0xf0000000
    };
    Q_ENUM(StyleHint)

    virtual int styleHint(StyleHint stylehint, const QStyleOption *opt = nullptr,
                          const QWidget *widget = nullptr, QStyleHintReturn* returnData = nullptr) const = 0;

    enum StandardPixmap {
        SP_TitleBarMenuButton,
        SP_TitleBarMinButton,
        SP_TitleBarMaxButton,
        SP_TitleBarCloseButton,
        SP_TitleBarNormalButton,
        SP_TitleBarShadeButton,
        SP_TitleBarUnshadeButton,
        SP_TitleBarContextHelpButton,
        SP_DockWidgetCloseButton,
        SP_MessageBoxInformation,
        SP_MessageBoxWarning,
        SP_MessageBoxCritical,
        SP_MessageBoxQuestion,
        SP_DesktopIcon,
        SP_TrashIcon,
        SP_ComputerIcon,
        SP_DriveFDIcon,
        SP_DriveHDIcon,
        SP_DriveCDIcon,
        SP_DriveDVDIcon,
        SP_DriveNetIcon,
        SP_DirOpenIcon,
        SP_DirClosedIcon,
        SP_DirLinkIcon,
        SP_DirLinkOpenIcon,
        SP_FileIcon,
        SP_FileLinkIcon,
        SP_ToolBarHorizontalExtensionButton,
        SP_ToolBarVerticalExtensionButton,
        SP_FileDialogStart,
        SP_FileDialogEnd,
        SP_FileDialogToParent,
        SP_FileDialogNewFolder,
        SP_FileDialogDetailedView,
        SP_FileDialogInfoView,
        SP_FileDialogContentsView,
        SP_FileDialogListView,
        SP_FileDialogBack,
        SP_DirIcon,
        SP_DialogOkButton,
        SP_DialogCancelButton,
        SP_DialogHelpButton,
        SP_DialogOpenButton,
        SP_DialogSaveButton,
        SP_DialogCloseButton,
        SP_DialogApplyButton,
        SP_DialogResetButton,
        SP_DialogDiscardButton,
        SP_DialogYesButton,
        SP_DialogNoButton,
        SP_ArrowUp,
        SP_ArrowDown,
        SP_ArrowLeft,
        SP_ArrowRight,
        SP_ArrowBack,
        SP_ArrowForward,
        SP_DirHomeIcon,
        SP_CommandLink,
        SP_VistaShield,
        SP_BrowserReload,
        SP_BrowserStop,
        SP_MediaPlay,
        SP_MediaStop,
        SP_MediaPause,
        SP_MediaSkipForward,
        SP_MediaSkipBackward,
        SP_MediaSeekForward,
        SP_MediaSeekBackward,
        SP_MediaVolume,
        SP_MediaVolumeMuted,
        SP_LineEditClearButton,
        SP_DialogYesToAllButton,
        SP_DialogNoToAllButton,
        SP_DialogSaveAllButton,
        SP_DialogAbortButton,
        SP_DialogRetryButton,
        SP_DialogIgnoreButton,
        SP_RestoreDefaultsButton,
        SP_TabCloseButton,
        NStandardPixmap, // assertion value for sync with QPlatformTheme::StandardPixmap
        // do not add any values below/greater than this
        SP_CustomBase = 0xf0000000
    };
    Q_ENUM(StandardPixmap)

    virtual QPixmap standardPixmap(StandardPixmap standardPixmap, const QStyleOption *opt = nullptr,
                                   const QWidget *widget = nullptr) const = 0;

    virtual QIcon standardIcon(StandardPixmap standardIcon, const QStyleOption *option = nullptr,
                               const QWidget *widget = nullptr) const = 0;

    virtual QPixmap generatedIconPixmap(QIcon::Mode iconMode, const QPixmap &pixmap,
                                        const QStyleOption *opt) const = 0;

    static QRect visualRect(Qt::LayoutDirection direction, const QRect &boundingRect,
                            const QRect &logicalRect);
    static QPoint visualPos(Qt::LayoutDirection direction, const QRect &boundingRect,
                            const QPoint &logicalPos);
    static int sliderPositionFromValue(int min, int max, int val, int space,
                                       bool upsideDown = false);
    static int sliderValueFromPosition(int min, int max, int pos, int space,
                                       bool upsideDown = false);
    static Qt::Alignment visualAlignment(Qt::LayoutDirection direction, Qt::Alignment alignment);
    static QRect alignedRect(Qt::LayoutDirection direction, Qt::Alignment alignment,
                             const QSize &size, const QRect &rectangle);

    virtual int layoutSpacing(QSizePolicy::ControlType control1,
                              QSizePolicy::ControlType control2, Qt::Orientation orientation,
                              const QStyleOption *option = nullptr, const QWidget *widget = nullptr) const = 0;
    int combinedLayoutSpacing(QSizePolicy::ControlTypes controls1,
                              QSizePolicy::ControlTypes controls2, Qt::Orientation orientation,
                              QStyleOption *option = nullptr, QWidget *widget = nullptr) const;

    const QStyle * proxy() const;

private:
    void setName(const QString &name);

private:
    Q_DISABLE_COPY(QStyle)
    friend class QWidget;
    friend class QWidgetPrivate;
    friend class QApplication;
    friend class QProxyStyle;
    friend class QProxyStylePrivate;
    friend class QStyleFactory;
    void setProxy(QStyle *style);
};

Q_DECLARE_OPERATORS_FOR_FLAGS(QStyle::State)
Q_DECLARE_OPERATORS_FOR_FLAGS(QStyle::SubControls)

QT_END_NAMESPACE

#endif // QSTYLE_H
