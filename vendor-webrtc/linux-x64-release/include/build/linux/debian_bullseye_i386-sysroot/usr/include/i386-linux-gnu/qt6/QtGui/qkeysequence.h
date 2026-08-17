// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QKEYSEQUENCE_H
#define QKEYSEQUENCE_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qstring.h>
#include <QtCore/qobjectdefs.h>

QT_REQUIRE_CONFIG(shortcut);

QT_BEGIN_NAMESPACE

class QKeySequence;

/*****************************************************************************
  QKeySequence stream functions
 *****************************************************************************/
#if !defined(QT_NO_DATASTREAM) || defined(Q_CLANG_QDOC)
Q_GUI_EXPORT QDataStream &operator<<(QDataStream &in, const QKeySequence &ks);
Q_GUI_EXPORT QDataStream &operator>>(QDataStream &out, QKeySequence &ks);
#endif

#if defined(Q_CLANG_QDOC)
void qt_set_sequence_auto_mnemonic(bool b);
#endif

class QVariant;
class QKeySequencePrivate;

Q_GUI_EXPORT Q_DECL_PURE_FUNCTION size_t qHash(const QKeySequence &key, size_t seed = 0) noexcept;

class Q_GUI_EXPORT QKeySequence
{
    Q_GADGET

public:
    enum StandardKey {
        UnknownKey,
        HelpContents,
        WhatsThis,
        Open,
        Close,
        Save,
        New,
        Delete,
        Cut,
        Copy,
        Paste,
        Undo,
        Redo,
        Back,
        Forward,
        Refresh,
        ZoomIn,
        ZoomOut,
        Print,
        AddTab,
        NextChild,
        PreviousChild,
        Find,
        FindNext,
        FindPrevious,
        Replace,
        SelectAll,
        Bold,
        Italic,
        Underline,
        MoveToNextChar,
        MoveToPreviousChar,
        MoveToNextWord,
        MoveToPreviousWord,
        MoveToNextLine,
        MoveToPreviousLine,
        MoveToNextPage,
        MoveToPreviousPage,
        MoveToStartOfLine,
        MoveToEndOfLine,
        MoveToStartOfBlock,
        MoveToEndOfBlock,
        MoveToStartOfDocument,
        MoveToEndOfDocument,
        SelectNextChar,
        SelectPreviousChar,
        SelectNextWord,
        SelectPreviousWord,
        SelectNextLine,
        SelectPreviousLine,
        SelectNextPage,
        SelectPreviousPage,
        SelectStartOfLine,
        SelectEndOfLine,
        SelectStartOfBlock,
        SelectEndOfBlock,
        SelectStartOfDocument,
        SelectEndOfDocument,
        DeleteStartOfWord,
        DeleteEndOfWord,
        DeleteEndOfLine,
        InsertParagraphSeparator,
        InsertLineSeparator,
        SaveAs,
        Preferences,
        Quit,
        FullScreen,
        Deselect,
        DeleteCompleteLine,
        Backspace,
        Cancel
     };
     Q_ENUM(StandardKey)

    enum SequenceFormat {
        NativeText,
        PortableText
    };

    QKeySequence();
    QKeySequence(const QString &key, SequenceFormat format = NativeText);
    QKeySequence(int k1, int k2 = 0, int k3 = 0, int k4 = 0);
    QKeySequence(QKeyCombination k1,
                 QKeyCombination k2 = QKeyCombination::fromCombined(0),
                 QKeyCombination k3 = QKeyCombination::fromCombined(0),
                 QKeyCombination k4 = QKeyCombination::fromCombined(0));
    QKeySequence(const QKeySequence &ks);
    QKeySequence(StandardKey key);
    ~QKeySequence();

    int count() const;
    bool isEmpty() const;

    enum SequenceMatch {
        NoMatch,
        PartialMatch,
        ExactMatch
    };

    QString toString(SequenceFormat format = PortableText) const;
    static QKeySequence fromString(const QString &str, SequenceFormat format = PortableText);

    static QList<QKeySequence> listFromString(const QString &str, SequenceFormat format = PortableText);
    static QString listToString(const QList<QKeySequence> &list, SequenceFormat format = PortableText);

    SequenceMatch matches(const QKeySequence &seq) const;
    static QKeySequence mnemonic(const QString &text);
    static QList<QKeySequence> keyBindings(StandardKey key);

    operator QVariant() const;
    QKeyCombination operator[](uint i) const;
    QKeySequence &operator=(const QKeySequence &other);
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_PURE_SWAP(QKeySequence)
    void swap(QKeySequence &other) noexcept { qt_ptr_swap(d, other.d); }

    bool operator==(const QKeySequence &other) const;
    inline bool operator!= (const QKeySequence &other) const
    { return !(*this == other); }
    bool operator< (const QKeySequence &ks) const;
    inline bool operator> (const QKeySequence &other) const
    { return other < *this; }
    inline bool operator<= (const QKeySequence &other) const
    { return !(other < *this); }
    inline bool operator>= (const QKeySequence &other) const
    { return !(*this < other); }

    bool isDetached() const;
private:
    static int decodeString(const QString &ks);
    static QString encodeString(int key);
    int assign(const QString &str);
    int assign(const QString &str, SequenceFormat format);
    void setKey(QKeyCombination key, int index);

    QKeySequencePrivate *d;

    friend Q_GUI_EXPORT QDataStream &operator<<(QDataStream &in, const QKeySequence &ks);
    friend Q_GUI_EXPORT QDataStream &operator>>(QDataStream &in, QKeySequence &ks);
    friend Q_GUI_EXPORT size_t qHash(const QKeySequence &key, size_t seed) noexcept;
    friend class QShortcutMap;
    friend class QShortcut;

public:
    typedef QKeySequencePrivate * DataPtr;
    inline DataPtr &data_ptr() { return d; }
};

Q_DECLARE_SHARED(QKeySequence)

#if !defined(QT_NO_DEBUG_STREAM) || defined(Q_CLANG_QDOC)
Q_GUI_EXPORT QDebug operator<<(QDebug, const QKeySequence &);
#endif

QT_END_NAMESPACE

#endif // QKEYSEQUENCE_H
