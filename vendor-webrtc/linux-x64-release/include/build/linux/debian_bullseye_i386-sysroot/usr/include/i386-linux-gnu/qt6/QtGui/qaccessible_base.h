// Copyright (C) 2022 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QACCESSIBLE_BASE_H
#define QACCESSIBLE_BASE_H

#include <QtGui/qtguiglobal.h>
#if QT_CONFIG(accessibility)

#if 0
// QAccessible class is handled in qaccessible.h
#pragma qt_sync_stop_processing
#endif

#include <QtCore/qobjectdefs.h>

#include <cstring> // memset, memcmp

QT_BEGIN_NAMESPACE

class QAccessibleInterface;
class QAccessibleEvent;
class QTextCursor;

class Q_GUI_EXPORT QAccessible
{
    Q_GADGET
public:

    enum Event {
        SoundPlayed          = 0x0001,
        Alert                = 0x0002,
        ForegroundChanged    = 0x0003,
        MenuStart            = 0x0004,
        MenuEnd              = 0x0005,
        PopupMenuStart       = 0x0006,
        PopupMenuEnd         = 0x0007,
        ContextHelpStart     = 0x000C,
        ContextHelpEnd       = 0x000D,
        DragDropStart        = 0x000E,
        DragDropEnd          = 0x000F,
        DialogStart          = 0x0010,
        DialogEnd            = 0x0011,
        ScrollingStart       = 0x0012,
        ScrollingEnd         = 0x0013,

        MenuCommand          = 0x0018,

        // Values from IAccessible2
        ActionChanged                    = 0x0101,
        ActiveDescendantChanged          = 0x0102,
        AttributeChanged                 = 0x0103,
        DocumentContentChanged           = 0x0104,
        DocumentLoadComplete             = 0x0105,
        DocumentLoadStopped              = 0x0106,
        DocumentReload                   = 0x0107,
        HyperlinkEndIndexChanged         = 0x0108,
        HyperlinkNumberOfAnchorsChanged  = 0x0109,
        HyperlinkSelectedLinkChanged     = 0x010A,
        HypertextLinkActivated           = 0x010B,
        HypertextLinkSelected            = 0x010C,
        HyperlinkStartIndexChanged       = 0x010D,
        HypertextChanged                 = 0x010E,
        HypertextNLinksChanged           = 0x010F,
        ObjectAttributeChanged           = 0x0110,
        PageChanged                      = 0x0111,
        SectionChanged                   = 0x0112,
        TableCaptionChanged              = 0x0113,
        TableColumnDescriptionChanged    = 0x0114,
        TableColumnHeaderChanged         = 0x0115,
        TableModelChanged                = 0x0116,
        TableRowDescriptionChanged       = 0x0117,
        TableRowHeaderChanged            = 0x0118,
        TableSummaryChanged              = 0x0119,
        TextAttributeChanged             = 0x011A,
        TextCaretMoved                   = 0x011B,
        // TextChanged = 0x011C, is deprecated in IA2, use TextUpdated
        TextColumnChanged                = 0x011D,
        TextInserted                     = 0x011E,
        TextRemoved                      = 0x011F,
        TextUpdated                      = 0x0120,
        TextSelectionChanged             = 0x0121,
        VisibleDataChanged               = 0x0122,

        ObjectCreated        = 0x8000,
        ObjectDestroyed      = 0x8001,
        ObjectShow           = 0x8002,
        ObjectHide           = 0x8003,
        ObjectReorder        = 0x8004,
        Focus                = 0x8005,
        Selection            = 0x8006,
        SelectionAdd         = 0x8007,
        SelectionRemove      = 0x8008,
        SelectionWithin      = 0x8009,
        StateChanged         = 0x800A,
        LocationChanged      = 0x800B,
        NameChanged          = 0x800C,
        DescriptionChanged   = 0x800D,
        ValueChanged         = 0x800E,
        ParentChanged        = 0x800F,
        HelpChanged          = 0x80A0,
        DefaultActionChanged = 0x80B0,
        AcceleratorChanged   = 0x80C0,

        InvalidEvent
    };
    Q_ENUM(Event)

    // 64 bit enums seem hard on some platforms (windows...)
    // which makes using a bit field a sensible alternative
    struct State {
        // http://msdn.microsoft.com/en-us/library/ms697270.aspx
        quint64 disabled : 1; // used to be Unavailable
        quint64 selected : 1;
        quint64 focusable : 1;
        quint64 focused : 1;
        quint64 pressed : 1;
        quint64 checkable : 1;
        quint64 checked : 1;
        quint64 checkStateMixed : 1; // used to be Mixed
        quint64 readOnly : 1;
        quint64 hotTracked : 1;
        quint64 defaultButton : 1;
        quint64 expanded : 1;
        quint64 collapsed : 1;
        quint64 busy : 1;
        quint64 expandable : 1;
        quint64 marqueed : 1;
        quint64 animated : 1;
        quint64 invisible : 1;
        quint64 offscreen : 1;
        quint64 sizeable : 1;
        quint64 movable : 1;
        quint64 selfVoicing : 1;
        quint64 selectable : 1;
        quint64 linked : 1;
        quint64 traversed : 1;
        quint64 multiSelectable : 1;
        quint64 extSelectable : 1;
        quint64 passwordEdit : 1; // used to be Protected
        quint64 hasPopup : 1;
        quint64 modal : 1;

        // IA2 - we chose to not add some IA2 states for now
        // Below the ones that seem helpful
        quint64 active : 1;
        quint64 invalid : 1; // = defunct
        quint64 editable : 1;
        quint64 multiLine : 1;
        quint64 selectableText : 1;
        quint64 supportsAutoCompletion : 1;

        quint64 searchEdit : 1;

        // quint64 horizontal : 1;
        // quint64 vertical : 1;
        // quint64 invalidEntry : 1;
        // quint64 managesDescendants : 1;
        // quint64 singleLine : 1; // we have multi line, this is redundant.
        // quint64 stale : 1;
        // quint64 transient : 1;
        // quint64 pinned : 1;

        // Apple - see http://mattgemmell.com/2010/12/19/accessibility-for-iphone-and-ipad-apps/
        // quint64 playsSound : 1;
        // quint64 summaryElement : 1;
        // quint64 updatesFrequently : 1;
        // quint64 adjustable : 1;
        // more and not included here: http://developer.apple.com/library/mac/#documentation/UserExperience/Reference/Accessibility_RoleAttribute_Ref/Attributes.html

        // MSAA
        // quint64 alertLow : 1;
        // quint64 alertMedium : 1;
        // quint64 alertHigh : 1;

        State() {
            std::memset(this, 0, sizeof(State));
        }
        friend inline bool operator==(const QAccessible::State &first, const QAccessible::State &second)
        {
            return std::memcmp(&first, &second, sizeof(QAccessible::State)) == 0;
        }
    };





    enum Role {
        NoRole         = 0x00000000,
        TitleBar       = 0x00000001,
        MenuBar        = 0x00000002,
        ScrollBar      = 0x00000003,
        Grip           = 0x00000004,
        Sound          = 0x00000005,
        Cursor         = 0x00000006,
        Caret          = 0x00000007,
        AlertMessage   = 0x00000008,
        Window         = 0x00000009,
        Client         = 0x0000000A,
        PopupMenu      = 0x0000000B,
        MenuItem       = 0x0000000C,
        ToolTip        = 0x0000000D,
        Application    = 0x0000000E,
        Document       = 0x0000000F,
        Pane           = 0x00000010,
        Chart          = 0x00000011,
        Dialog         = 0x00000012,
        Border         = 0x00000013,
        Grouping       = 0x00000014,
        Separator      = 0x00000015,
        ToolBar        = 0x00000016,
        StatusBar      = 0x00000017,
        Table          = 0x00000018,
        ColumnHeader   = 0x00000019,
        RowHeader      = 0x0000001A,
        Column         = 0x0000001B,
        Row            = 0x0000001C,
        Cell           = 0x0000001D,
        Link           = 0x0000001E,
        HelpBalloon    = 0x0000001F,
        Assistant      = 0x00000020,
        List           = 0x00000021,
        ListItem       = 0x00000022,
        Tree           = 0x00000023,
        TreeItem       = 0x00000024,
        PageTab        = 0x00000025,
        PropertyPage   = 0x00000026,
        Indicator      = 0x00000027,
        Graphic        = 0x00000028,
        StaticText     = 0x00000029,
        EditableText   = 0x0000002A,  // Editable, selectable, etc.
        Button         = 0x0000002B,
#ifndef Q_QDOC
        PushButton     = Button, // deprecated
#endif
        CheckBox       = 0x0000002C,
        RadioButton    = 0x0000002D,
        ComboBox       = 0x0000002E,
        // DropList       = 0x0000002F,
        ProgressBar    = 0x00000030,
        Dial           = 0x00000031,
        HotkeyField    = 0x00000032,
        Slider         = 0x00000033,
        SpinBox        = 0x00000034,
        Canvas         = 0x00000035, // MSAA: ROLE_SYSTEM_DIAGRAM - The object represents a graphical image that is used to diagram data.
        Animation      = 0x00000036,
        Equation       = 0x00000037,
        ButtonDropDown = 0x00000038, // The object represents a button that expands a grid.
        ButtonMenu     = 0x00000039,
        ButtonDropGrid = 0x0000003A,
        Whitespace     = 0x0000003B, // The object represents blank space between other objects.
        PageTabList    = 0x0000003C,
        Clock          = 0x0000003D,
        Splitter       = 0x0000003E,
        // Reserved space in case MSAA roles needs to be added

        // Additional Qt roles where enum value does not map directly to MSAA:
        LayeredPane    = 0x00000080,
        Terminal       = 0x00000081,
        Desktop        = 0x00000082,
        Paragraph      = 0x00000083,
        WebDocument    = 0x00000084,
        Section        = 0x00000085,
        Notification   = 0x00000086,

        // IAccessible2 roles
        // IA2_ROLE_CANVAS = 0x401, // An object that can be drawn into and to manage events from the objects drawn into it
        // IA2_ROLE_CAPTION = 0x402,
        // IA2_ROLE_CHECK_MENU_ITEM = 0x403,
        ColorChooser = 0x404,
        // IA2_ROLE_DATE_EDITOR = 0x405,
        // IA2_ROLE_DESKTOP_ICON = 0x406,
        // IA2_ROLE_DESKTOP_PANE = 0x407,
        // IA2_ROLE_DIRECTORY_PANE = 0x408,
        // IA2_ROLE_EDITBAR = 0x409,
        // IA2_ROLE_EMBEDDED_OBJECT = 0x40A,
        // IA2_ROLE_ENDNOTE = 0x40B,
        // IA2_ROLE_FILE_CHOOSER = 0x40C,
        // IA2_ROLE_FONT_CHOOSER = 0x40D,
        Footer      = 0x40E,
        // IA2_ROLE_FOOTNOTE = 0x40F,
        Form        = 0x410,
        // some platforms (windows and at-spi) use Frame for regular windows
        // because window was taken for tool/dock windows by MSAA
        // Frame = 0x411,
        // IA2_ROLE_GLASS_PANE = 0x412,
        // IA2_ROLE_HEADER = 0x413,
        Heading  = 0x414,
        // IA2_ROLE_ICON = 0x415,
        // IA2_ROLE_IMAGE_MAP = 0x416,
        // IA2_ROLE_INPUT_METHOD_WINDOW = 0x417,
        // IA2_ROLE_INTERNAL_FRAME = 0x418,
        // IA2_ROLE_LABEL = 0x419,
        // IA2_ROLE_LAYERED_PANE = 0x41A,
        Note = 0x41B,
        // IA2_ROLE_OPTION_PANE = 0x41C,
        // IA2_ROLE_PAGE = 0x41D,
        // IA2_ROLE_PARAGRAPH = 0x42E,
        // IA2_ROLE_RADIO_MENU_ITEM = 0x41F,
        // IA2_ROLE_REDUNDANT_OBJECT = 0x420,
        // IA2_ROLE_ROOT_PANE = 0x421,
        // IA2_ROLE_RULER = 0x422,
        // IA2_ROLE_SCROLL_PANE = 0x423,
        // IA2_ROLE_SECTION = 0x424,
        // IA2_ROLE_SHAPE = 0x425,
        // IA2_ROLE_SPLIT_PANE = 0x426,
        // IA2_ROLE_TEAR_OFF_MENU = 0x427,
        // IA2_ROLE_TERMINAL = 0x428,
        // IA2_ROLE_TEXT_FRAME = 0x429,
        // IA2_ROLE_TOGGLE_BUTTON = 0x42A,
        // IA2_ROLE_VIEW_PORT = 0x42B,
        ComplementaryContent = 0x42C,

        UserRole       = 0x0000ffff
    };
    Q_ENUM(Role)

    enum Text {
        Name         = 0,
        Description,
        Value,
        Help,
        Accelerator,
        DebugDescription,
        UserText     = 0x0000ffff
    };

    enum RelationFlag {
        Label         = 0x00000001,
        Labelled      = 0x00000002,
        Controller    = 0x00000004,
        Controlled    = 0x00000008,
        AllRelations  = 0xffffffff
    };
    Q_DECLARE_FLAGS(Relation, RelationFlag)

    enum InterfaceType
    {
        TextInterface,
        EditableTextInterface,
        ValueInterface,
        ActionInterface,
        ImageInterface,
        TableInterface,
        TableCellInterface,
        HyperlinkInterface
    };

    enum TextBoundaryType {
        CharBoundary,
        WordBoundary,
        SentenceBoundary,
        ParagraphBoundary,
        LineBoundary,
        NoBoundary
    };

    typedef QAccessibleInterface*(*InterfaceFactory)(const QString &key, QObject*);
    typedef void(*UpdateHandler)(QAccessibleEvent *event);
    typedef void(*RootObjectHandler)(QObject*);

    typedef unsigned Id;

    static void installFactory(InterfaceFactory);
    static void removeFactory(InterfaceFactory);
    static UpdateHandler installUpdateHandler(UpdateHandler);
    static RootObjectHandler installRootObjectHandler(RootObjectHandler);

    class Q_GUI_EXPORT ActivationObserver
    {
    public:
        virtual ~ActivationObserver();
        virtual void accessibilityActiveChanged(bool active) = 0;
    };
    static void installActivationObserver(ActivationObserver *);
    static void removeActivationObserver(ActivationObserver *);

    static QAccessibleInterface *queryAccessibleInterface(QObject *);
    static Id uniqueId(QAccessibleInterface *iface);
    static QAccessibleInterface *accessibleInterface(Id uniqueId);
    static Id registerAccessibleInterface(QAccessibleInterface *iface);
    static void deleteAccessibleInterface(Id uniqueId);

    static void updateAccessibility(QAccessibleEvent *event);

    static bool isActive();
    static void setActive(bool active);
    static void setRootObject(QObject *object);

    static void cleanup();

    static QPair< int, int > qAccessibleTextBoundaryHelper(const QTextCursor &cursor, TextBoundaryType boundaryType);

private:
    static UpdateHandler updateHandler;
    static RootObjectHandler rootObjectHandler;

    QAccessible() {}

    friend class QAccessibleCache;
};

Q_DECLARE_OPERATORS_FOR_FLAGS(QAccessible::Relation)

QT_END_NAMESPACE

#endif // QT_CONFIG(accessibility)
#endif // QACCESSIBLE_BASE_H
