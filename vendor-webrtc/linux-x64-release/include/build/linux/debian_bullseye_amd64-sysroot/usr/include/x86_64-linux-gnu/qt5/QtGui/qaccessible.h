/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtGui module of the Qt Toolkit.
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

#include <QtGui/qtguiglobal.h>

#ifndef QT_NO_ACCESSIBILITY
#ifndef QACCESSIBLE_H
#define QACCESSIBLE_H

#include <QtCore/qcoreapplication.h>
#include <QtCore/qdebug.h>
#include <QtCore/qglobal.h>
#include <QtCore/qobject.h>
#include <QtCore/qrect.h>
#include <QtCore/qset.h>
#include <QtCore/qvector.h>
#include <QtCore/qvariant.h>
#include <QtGui/qcolor.h>
#include <QtGui/qevent.h>

#include <stdlib.h>

QT_BEGIN_NAMESPACE

class QAccessibleInterface;
class QAccessibleEvent;
class QWindow;
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
            memset(this, 0, sizeof(State));
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
        TableCellInterface
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


#if QT_DEPRECATED_SINCE(5, 0)
    QT_DEPRECATED static inline void updateAccessibility(QObject *object, int child, Event reason);
#endif
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

Q_GUI_EXPORT bool operator==(const QAccessible::State &first, const QAccessible::State &second);

Q_DECLARE_OPERATORS_FOR_FLAGS(QAccessible::Relation)

class QAccessible2Interface;
class QAccessibleTextInterface;
class QAccessibleEditableTextInterface;
class QAccessibleValueInterface;
class QAccessibleActionInterface;
class QAccessibleImageInterface;
class QAccessibleTableInterface;
class QAccessibleTableCellInterface;
class QAccessibleTableModelChangeEvent;

class Q_GUI_EXPORT QAccessibleInterface
{
protected:
    virtual ~QAccessibleInterface();

public:
    // check for valid pointers
    virtual bool isValid() const = 0;
    virtual QObject *object() const = 0;
    virtual QWindow *window() const;

    // relations
    virtual QVector<QPair<QAccessibleInterface*, QAccessible::Relation> > relations(QAccessible::Relation match = QAccessible::AllRelations) const;
    virtual QAccessibleInterface *focusChild() const;

    virtual QAccessibleInterface *childAt(int x, int y) const = 0;

    // navigation, hierarchy
    virtual QAccessibleInterface *parent() const = 0;
    virtual QAccessibleInterface *child(int index) const = 0;
    virtual int childCount() const = 0;
    virtual int indexOfChild(const QAccessibleInterface *) const = 0;

    // properties and state
    virtual QString text(QAccessible::Text t) const = 0;
    virtual void setText(QAccessible::Text t, const QString &text) = 0;
    virtual QRect rect() const = 0;
    virtual QAccessible::Role role() const = 0;
    virtual QAccessible::State state() const = 0;

    virtual QColor foregroundColor() const;
    virtual QColor backgroundColor() const;

    inline QAccessibleTextInterface *textInterface()
    { return reinterpret_cast<QAccessibleTextInterface *>(interface_cast(QAccessible::TextInterface)); }

    inline QAccessibleEditableTextInterface *editableTextInterface()
    { return reinterpret_cast<QAccessibleEditableTextInterface *>(interface_cast(QAccessible::EditableTextInterface)); }

    inline QAccessibleValueInterface *valueInterface()
    { return reinterpret_cast<QAccessibleValueInterface *>(interface_cast(QAccessible::ValueInterface)); }

    inline QAccessibleActionInterface *actionInterface()
    { return reinterpret_cast<QAccessibleActionInterface *>(interface_cast(QAccessible::ActionInterface)); }

    inline QAccessibleImageInterface *imageInterface()
    { return reinterpret_cast<QAccessibleImageInterface *>(interface_cast(QAccessible::ImageInterface)); }

    inline QAccessibleTableInterface *tableInterface()
    { return reinterpret_cast<QAccessibleTableInterface *>(interface_cast(QAccessible::TableInterface)); }

    inline QAccessibleTableCellInterface *tableCellInterface()
    { return reinterpret_cast<QAccessibleTableCellInterface *>(interface_cast(QAccessible::TableCellInterface)); }

    virtual void virtual_hook(int id, void *data);

    virtual void *interface_cast(QAccessible::InterfaceType)
    { return nullptr; }

protected:
    friend class QAccessibleCache;
};

class Q_GUI_EXPORT QAccessibleTextInterface
{
public:
    virtual ~QAccessibleTextInterface();
    // selection
    virtual void selection(int selectionIndex, int *startOffset, int *endOffset) const = 0;
    virtual int selectionCount() const = 0;
    virtual void addSelection(int startOffset, int endOffset) = 0;
    virtual void removeSelection(int selectionIndex) = 0;
    virtual void setSelection(int selectionIndex, int startOffset, int endOffset) = 0;

    // cursor
    virtual int cursorPosition() const = 0;
    virtual void setCursorPosition(int position) = 0;

    // text
    virtual QString text(int startOffset, int endOffset) const = 0;
    virtual QString textBeforeOffset(int offset, QAccessible::TextBoundaryType boundaryType,
                                     int *startOffset, int *endOffset) const;
    virtual QString textAfterOffset(int offset, QAccessible::TextBoundaryType boundaryType,
                                    int *startOffset, int *endOffset) const;
    virtual QString textAtOffset(int offset, QAccessible::TextBoundaryType boundaryType,
                                 int *startOffset, int *endOffset) const;
    virtual int characterCount() const = 0;

    // character <-> geometry
    virtual QRect characterRect(int offset) const = 0;
    virtual int offsetAtPoint(const QPoint &point) const = 0;

    virtual void scrollToSubstring(int startIndex, int endIndex) = 0;
    virtual QString attributes(int offset, int *startOffset, int *endOffset) const = 0;
};

class Q_GUI_EXPORT QAccessibleEditableTextInterface
{
public:
    virtual ~QAccessibleEditableTextInterface();

    virtual void deleteText(int startOffset, int endOffset) = 0;
    virtual void insertText(int offset, const QString &text) = 0;
    virtual void replaceText(int startOffset, int endOffset, const QString &text) = 0;
};

class Q_GUI_EXPORT QAccessibleValueInterface
{
public:
    virtual ~QAccessibleValueInterface();

    virtual QVariant currentValue() const = 0;
    virtual void setCurrentValue(const QVariant &value) = 0;
    virtual QVariant maximumValue() const = 0;
    virtual QVariant minimumValue() const = 0;
    virtual QVariant minimumStepSize() const = 0;
};

class Q_GUI_EXPORT QAccessibleTableCellInterface
{
public:
    virtual ~QAccessibleTableCellInterface();

    virtual bool isSelected() const = 0;

    virtual QList<QAccessibleInterface*> columnHeaderCells() const = 0;
    virtual QList<QAccessibleInterface*> rowHeaderCells() const = 0;
    virtual int columnIndex() const = 0;
    virtual int rowIndex() const = 0;
    virtual int columnExtent() const = 0;
    virtual int rowExtent() const = 0;

    virtual QAccessibleInterface* table() const = 0;
};

class Q_GUI_EXPORT QAccessibleTableInterface
{
public:
    virtual ~QAccessibleTableInterface();

    virtual QAccessibleInterface *caption() const = 0;
    virtual QAccessibleInterface *summary() const = 0;

    virtual QAccessibleInterface *cellAt (int row, int column) const = 0;
    virtual int selectedCellCount() const = 0;
    virtual QList<QAccessibleInterface*> selectedCells() const = 0;

    virtual QString columnDescription(int column) const = 0;
    virtual QString rowDescription(int row) const = 0;
    virtual int selectedColumnCount() const = 0;
    virtual int selectedRowCount() const = 0;
    virtual int columnCount() const = 0;
    virtual int rowCount() const = 0;
    virtual QList<int> selectedColumns() const = 0;
    virtual QList<int> selectedRows() const = 0;
    virtual bool isColumnSelected(int column) const = 0;
    virtual bool isRowSelected(int row) const = 0;
    virtual bool selectRow(int row) = 0;
    virtual bool selectColumn(int column) = 0;
    virtual bool unselectRow(int row) = 0;
    virtual bool unselectColumn(int column) = 0;

    virtual void modelChange(QAccessibleTableModelChangeEvent *event) = 0;

protected:
friend class QAbstractItemView;
friend class QAbstractItemViewPrivate;
};

class Q_GUI_EXPORT QAccessibleActionInterface
{
    Q_DECLARE_TR_FUNCTIONS(QAccessibleActionInterface)
public:
    virtual ~QAccessibleActionInterface();

    virtual QStringList actionNames() const = 0;
    virtual QString localizedActionName(const QString &name) const;
    virtual QString localizedActionDescription(const QString &name) const;
    virtual void doAction(const QString &actionName) = 0;
    virtual QStringList keyBindingsForAction(const QString &actionName) const = 0;

    static const QString &pressAction();
    static const QString &increaseAction();
    static const QString &decreaseAction();
    static const QString &showMenuAction();
    static const QString &setFocusAction();
    static const QString &toggleAction();
    static QString scrollLeftAction();
    static QString scrollRightAction();
    static QString scrollUpAction();
    static QString scrollDownAction();
    static QString nextPageAction();
    static QString previousPageAction();
};

class Q_GUI_EXPORT QAccessibleImageInterface
{
public:
    virtual ~QAccessibleImageInterface();

    virtual QString imageDescription() const = 0;
    virtual QSize imageSize() const = 0;
    virtual QPoint imagePosition() const = 0;
};


class Q_GUI_EXPORT QAccessibleEvent
{
    Q_DISABLE_COPY(QAccessibleEvent)
public:

    inline QAccessibleEvent(QObject *obj, QAccessible::Event typ)
        : m_type(typ), m_object(obj), m_child(-1)
    {
        Q_ASSERT(obj);
        // All events below have a subclass of QAccessibleEvent.
        // Use the subclass, since it's expected that it's possible to cast to that.
        Q_ASSERT(m_type != QAccessible::ValueChanged);
        Q_ASSERT(m_type != QAccessible::StateChanged);
        Q_ASSERT(m_type != QAccessible::TextCaretMoved);
        Q_ASSERT(m_type != QAccessible::TextSelectionChanged);
        Q_ASSERT(m_type != QAccessible::TextInserted);
        Q_ASSERT(m_type != QAccessible::TextRemoved);
        Q_ASSERT(m_type != QAccessible::TextUpdated);
        Q_ASSERT(m_type != QAccessible::TableModelChanged);
    }

    inline QAccessibleEvent(QAccessibleInterface *iface, QAccessible::Event typ)
        : m_type(typ), m_object(nullptr)
    {
        Q_ASSERT(iface);
        Q_ASSERT(m_type != QAccessible::ValueChanged);
        Q_ASSERT(m_type != QAccessible::StateChanged);
        Q_ASSERT(m_type != QAccessible::TextCaretMoved);
        Q_ASSERT(m_type != QAccessible::TextSelectionChanged);
        Q_ASSERT(m_type != QAccessible::TextInserted);
        Q_ASSERT(m_type != QAccessible::TextRemoved);
        Q_ASSERT(m_type != QAccessible::TextUpdated);
        Q_ASSERT(m_type != QAccessible::TableModelChanged);
        m_uniqueId = QAccessible::uniqueId(iface);
    }

    virtual ~QAccessibleEvent();

    QAccessible::Event type() const { return m_type; }
    QObject *object() const { return m_object; }
    QAccessible::Id uniqueId() const;

    void setChild(int chld) { m_child = chld; }
    int child() const { return m_child; }

    virtual QAccessibleInterface *accessibleInterface() const;

protected:
    QAccessible::Event m_type;
    QObject *m_object;
    union {
        int m_child;
        QAccessible::Id m_uniqueId;
    };

};

class Q_GUI_EXPORT QAccessibleStateChangeEvent :public QAccessibleEvent
{
public:
    inline QAccessibleStateChangeEvent(QObject *obj, QAccessible::State state)
        : QAccessibleEvent(obj, QAccessible::InvalidEvent), m_changedStates(state)
    {
        m_type = QAccessible::StateChanged;
    }
    inline QAccessibleStateChangeEvent(QAccessibleInterface *iface, QAccessible::State state)
        : QAccessibleEvent(iface, QAccessible::InvalidEvent), m_changedStates(state)
    {
        m_type = QAccessible::StateChanged;
    }
    ~QAccessibleStateChangeEvent();

    QAccessible::State changedStates() const {
        return m_changedStates;
    }

protected:
    QAccessible::State m_changedStates;
};

// Update the cursor and optionally the selection.
class Q_GUI_EXPORT QAccessibleTextCursorEvent : public QAccessibleEvent
{
public:
    inline QAccessibleTextCursorEvent(QObject *obj, int cursorPos)
        : QAccessibleEvent(obj, QAccessible::InvalidEvent)
      , m_cursorPosition(cursorPos)
    {
        m_type = QAccessible::TextCaretMoved;
    }
    inline QAccessibleTextCursorEvent(QAccessibleInterface *iface, int cursorPos)
        : QAccessibleEvent(iface, QAccessible::InvalidEvent)
      , m_cursorPosition(cursorPos)
    {
        m_type = QAccessible::TextCaretMoved;
    }

    ~QAccessibleTextCursorEvent();

    void setCursorPosition(int position) { m_cursorPosition = position; }
    int cursorPosition() const { return m_cursorPosition; }

protected:
    int m_cursorPosition;
};

// Updates the cursor position simultaneously. By default the cursor is set to the end of the selection.
class Q_GUI_EXPORT QAccessibleTextSelectionEvent : public QAccessibleTextCursorEvent
{
public:
    inline QAccessibleTextSelectionEvent(QObject *obj, int start, int end)
        : QAccessibleTextCursorEvent(obj, (start == -1) ? 0 : end)
        , m_selectionStart(start), m_selectionEnd(end)
    {
        m_type = QAccessible::TextSelectionChanged;
    }
    inline QAccessibleTextSelectionEvent(QAccessibleInterface *iface, int start, int end)
        : QAccessibleTextCursorEvent(iface, (start == -1) ? 0 : end)
        , m_selectionStart(start), m_selectionEnd(end)
    {
        m_type = QAccessible::TextSelectionChanged;
    }

    ~QAccessibleTextSelectionEvent();

    void setSelection(int start, int end) {
        m_selectionStart = start;
        m_selectionEnd = end;
    }

    int selectionStart() const { return m_selectionStart; }
    int selectionEnd() const { return m_selectionEnd; }

protected:
        int m_selectionStart;
        int m_selectionEnd;
};

class Q_GUI_EXPORT QAccessibleTextInsertEvent : public QAccessibleTextCursorEvent
{
public:
    inline QAccessibleTextInsertEvent(QObject *obj, int position, const QString &text)
        : QAccessibleTextCursorEvent(obj, position + text.length())
        , m_position(position), m_text(text)
    {
        m_type = QAccessible::TextInserted;
    }
    inline QAccessibleTextInsertEvent(QAccessibleInterface *iface, int position, const QString &text)
        : QAccessibleTextCursorEvent(iface, position + text.length())
        , m_position(position), m_text(text)
    {
        m_type = QAccessible::TextInserted;
    }

    ~QAccessibleTextInsertEvent();

    QString textInserted() const {
        return m_text;
    }
    int changePosition() const {
        return m_position;
    }

protected:
    int m_position;
    QString m_text;
};

class Q_GUI_EXPORT QAccessibleTextRemoveEvent : public QAccessibleTextCursorEvent
{
public:
    inline QAccessibleTextRemoveEvent(QObject *obj, int position, const QString &text)
        : QAccessibleTextCursorEvent(obj, position)
        , m_position(position), m_text(text)
    {
        m_type = QAccessible::TextRemoved;
    }
    inline QAccessibleTextRemoveEvent(QAccessibleInterface *iface, int position, const QString &text)
        : QAccessibleTextCursorEvent(iface, position)
        , m_position(position), m_text(text)
    {
        m_type = QAccessible::TextRemoved;
    }

    ~QAccessibleTextRemoveEvent();

    QString textRemoved() const {
        return m_text;
    }
    int changePosition() const {
        return m_position;
    }

protected:
    int m_position;
    QString m_text;
};

class Q_GUI_EXPORT QAccessibleTextUpdateEvent : public QAccessibleTextCursorEvent
{
public:
    inline QAccessibleTextUpdateEvent(QObject *obj, int position, const QString &oldText, const QString &text)
        : QAccessibleTextCursorEvent(obj, position + text.length())
        , m_position(position), m_oldText(oldText), m_text(text)
    {
        m_type = QAccessible::TextUpdated;
    }
    inline QAccessibleTextUpdateEvent(QAccessibleInterface *iface, int position, const QString &oldText, const QString &text)
        : QAccessibleTextCursorEvent(iface, position + text.length())
        , m_position(position), m_oldText(oldText), m_text(text)
    {
        m_type = QAccessible::TextUpdated;
    }

    ~QAccessibleTextUpdateEvent();

    QString textRemoved() const {
        return m_oldText;
    }
    QString textInserted() const {
        return m_text;
    }
    int changePosition() const {
        return m_position;
    }

protected:
    int m_position;
    QString m_oldText;
    QString m_text;
};

class Q_GUI_EXPORT QAccessibleValueChangeEvent : public QAccessibleEvent
{
public:
    inline QAccessibleValueChangeEvent(QObject *obj, const QVariant &val)
        : QAccessibleEvent(obj, QAccessible::InvalidEvent)
      , m_value(val)
    {
        m_type = QAccessible::ValueChanged;
    }
    inline QAccessibleValueChangeEvent(QAccessibleInterface *iface, const QVariant &val)
        : QAccessibleEvent(iface, QAccessible::InvalidEvent)
      , m_value(val)
    {
        m_type = QAccessible::ValueChanged;
    }

    ~QAccessibleValueChangeEvent();

    void setValue(const QVariant & val) { m_value= val; }
    QVariant value() const { return m_value; }

protected:
    QVariant m_value;
};

class Q_GUI_EXPORT QAccessibleTableModelChangeEvent : public QAccessibleEvent
{
public:
    enum ModelChangeType {
        ModelReset,
        DataChanged,
        RowsInserted,
        ColumnsInserted,
        RowsRemoved,
        ColumnsRemoved
    };

    inline QAccessibleTableModelChangeEvent(QObject *obj, ModelChangeType changeType)
        : QAccessibleEvent(obj, QAccessible::InvalidEvent)
        , m_modelChangeType(changeType)
        , m_firstRow(-1), m_firstColumn(-1), m_lastRow(-1), m_lastColumn(-1)
    {
        m_type = QAccessible::TableModelChanged;
    }
    inline QAccessibleTableModelChangeEvent(QAccessibleInterface *iface, ModelChangeType changeType)
        : QAccessibleEvent(iface, QAccessible::InvalidEvent)
        , m_modelChangeType(changeType)
        , m_firstRow(-1), m_firstColumn(-1), m_lastRow(-1), m_lastColumn(-1)
    {
        m_type = QAccessible::TableModelChanged;
    }

    ~QAccessibleTableModelChangeEvent();

    void setModelChangeType(ModelChangeType changeType) { m_modelChangeType = changeType; }
    ModelChangeType modelChangeType() const { return m_modelChangeType; }

    void setFirstRow(int row) { m_firstRow = row; }
    void setFirstColumn(int col) { m_firstColumn = col; }
    void setLastRow(int row) { m_lastRow = row; }
    void setLastColumn(int col) { m_lastColumn = col; }
    int firstRow() const { return m_firstRow; }
    int firstColumn() const { return m_firstColumn; }
    int lastRow() const { return m_lastRow; }
    int lastColumn() const { return m_lastColumn; }

protected:
    ModelChangeType m_modelChangeType;
    int m_firstRow;
    int m_firstColumn;
    int m_lastRow;
    int m_lastColumn;
};

#ifndef Q_CLANG_QDOC
#define QAccessibleInterface_iid "org.qt-project.Qt.QAccessibleInterface"
Q_DECLARE_INTERFACE(QAccessibleInterface, QAccessibleInterface_iid)
#endif

Q_GUI_EXPORT const char *qAccessibleRoleString(QAccessible::Role role);
Q_GUI_EXPORT const char *qAccessibleEventString(QAccessible::Event event);
Q_GUI_EXPORT QString qAccessibleLocalizedActionDescription(const QString &actionName);

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug d, const QAccessibleInterface *iface);
Q_GUI_EXPORT QDebug operator<<(QDebug d, const QAccessibleEvent &ev);
#endif

#if QT_DEPRECATED_SINCE(5, 0)
inline void QAccessible::updateAccessibility(QObject *object, int child, Event reason)
{
    Q_ASSERT(object);

    QAccessibleEvent ev(object, reason);
    ev.setChild(child);
    updateAccessibility(&ev);
}
#endif

QT_END_NAMESPACE

#endif // QACCESSIBLE_H
#endif //!QT_NO_ACCESSIBILITY
