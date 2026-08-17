// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only


#if 0
// keep existing syncqt header working after the move of the class
// into qaccessible_base
#pragma qt_class(QAccessible)
#endif

#ifndef QACCESSIBLE_H
#define QACCESSIBLE_H
#include <QtGui/qtguiglobal.h>

#if QT_CONFIG(accessibility)

#include <QtCore/qcoreapplication.h>
#include <QtCore/qdebug.h>
#include <QtCore/qglobal.h>
#include <QtCore/qlist.h>
#include <QtCore/qobject.h>
#include <QtCore/qrect.h>
#include <QtCore/qset.h>
#include <QtCore/qvariant.h>
#include <QtGui/qcolor.h>
#include <QtGui/qevent.h>
#include <QtGui/qaccessible_base.h>

QT_BEGIN_NAMESPACE

class QAccessibleInterface;
class QAccessibleEvent;
class QWindow;
class QTextCursor;

class QAccessible2Interface;
class QAccessibleTextInterface;
class QAccessibleEditableTextInterface;
class QAccessibleValueInterface;
class QAccessibleActionInterface;
class QAccessibleImageInterface;
class QAccessibleTableInterface;
class QAccessibleTableCellInterface;
class QAccessibleHyperlinkInterface;
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
    virtual QList<QPair<QAccessibleInterface *, QAccessible::Relation>>
    relations(QAccessible::Relation match = QAccessible::AllRelations) const;
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

    inline QAccessibleHyperlinkInterface *hyperlinkInterface()
    { return reinterpret_cast<QAccessibleHyperlinkInterface *>(interface_cast(QAccessible::HyperlinkInterface)); }

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

class Q_GUI_EXPORT QAccessibleHyperlinkInterface
{
public:
    virtual ~QAccessibleHyperlinkInterface();

    virtual QString anchor() const = 0;
    virtual QString anchorTarget() const = 0;
    virtual int startIndex() const = 0;
    virtual int endIndex() const = 0;
    virtual bool isValid() const = 0;
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
        : m_type(typ)
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
        m_object = iface->object();
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

    friend class QTestAccessibility;
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
        : QAccessibleTextCursorEvent(obj, position + int(text.size()))
        , m_position(position), m_text(text)
    {
        m_type = QAccessible::TextInserted;
    }
    inline QAccessibleTextInsertEvent(QAccessibleInterface *iface, int position, const QString &text)
        : QAccessibleTextCursorEvent(iface, position + int(text.size()))
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
        : QAccessibleTextCursorEvent(obj, position + int(text.size()))
        , m_position(position), m_oldText(oldText), m_text(text)
    {
        m_type = QAccessible::TextUpdated;
    }
    inline QAccessibleTextUpdateEvent(QAccessibleInterface *iface, int position, const QString &oldText, const QString &text)
        : QAccessibleTextCursorEvent(iface, position + int(text.size()))
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

QT_END_NAMESPACE

#endif // QT_CONFIG(accessibility)
#endif // QACCESSIBLE_H
