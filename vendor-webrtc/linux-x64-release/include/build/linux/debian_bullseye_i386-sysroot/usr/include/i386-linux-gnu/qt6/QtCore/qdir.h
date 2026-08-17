// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QDIR_H
#define QDIR_H

#include <QtCore/qstring.h>
#include <QtCore/qfile.h>
#include <QtCore/qfileinfo.h>
#include <QtCore/qstringlist.h>
#include <QtCore/qshareddata.h>

QT_BEGIN_NAMESPACE

class QDirIterator;
class QDirPrivate;

class Q_CORE_EXPORT QDir
{
public:
    enum Filter { Dirs        = 0x001,
                  Files       = 0x002,
                  Drives      = 0x004,
                  NoSymLinks  = 0x008,
                  AllEntries  = Dirs | Files | Drives,
                  TypeMask    = 0x00f,

                  Readable    = 0x010,
                  Writable    = 0x020,
                  Executable  = 0x040,
                  PermissionMask    = 0x070,

                  Modified    = 0x080,
                  Hidden      = 0x100,
                  System      = 0x200,

                  AccessMask  = 0x3F0,

                  AllDirs       = 0x400,
                  CaseSensitive = 0x800,
                  NoDot         = 0x2000,
                  NoDotDot      = 0x4000,
                  NoDotAndDotDot = NoDot | NoDotDot,

                  NoFilter = -1
    };
    Q_DECLARE_FLAGS(Filters, Filter)

    enum SortFlag { Name        = 0x00,
                    Time        = 0x01,
                    Size        = 0x02,
                    Unsorted    = 0x03,
                    SortByMask  = 0x03,

                    DirsFirst   = 0x04,
                    Reversed    = 0x08,
                    IgnoreCase  = 0x10,
                    DirsLast    = 0x20,
                    LocaleAware = 0x40,
                    Type        = 0x80,
                    NoSort = -1
    };
    Q_DECLARE_FLAGS(SortFlags, SortFlag)

    QDir(const QDir &);
    QDir(const QString &path = QString());
    QDir(const QString &path, const QString &nameFilter,
         SortFlags sort = SortFlags(Name | IgnoreCase), Filters filter = AllEntries);
#ifdef Q_CLANG_QDOC
    QDir(const std::filesystem::path &path);
    QDir(const std::filesystem::path &path, const QString &nameFilter,
         SortFlags sort = SortFlags(Name | IgnoreCase), Filters filter = AllEntries);
#elif QT_CONFIG(cxx17_filesystem)
    template<typename T, QtPrivate::ForceFilesystemPath<T> = 0>
    QDir(const T &path) : QDir(QtPrivate::fromFilesystemPath(path))
    {
    }
    template<typename T, QtPrivate::ForceFilesystemPath<T> = 0>
    QDir(const T &path, const QString &nameFilter,
         SortFlags sort = SortFlags(Name | IgnoreCase), Filters filter = AllEntries)
         : QDir(QtPrivate::fromFilesystemPath(path), nameFilter, sort, filter)
    {
    }
#endif // QT_CONFIG(cxx17_filesystem)
    ~QDir();

    QDir &operator=(const QDir &);
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_PURE_SWAP(QDir)

    void swap(QDir &other) noexcept
    { d_ptr.swap(other.d_ptr); }

    void setPath(const QString &path);
#ifdef Q_CLANG_QDOC
    void setPath(const std::filesystem::path &path);
#elif QT_CONFIG(cxx17_filesystem)
    template<typename T, QtPrivate::ForceFilesystemPath<T> = 0>
    void setPath(const T &path)
    {
        setPath(QtPrivate::fromFilesystemPath(path));
    }
#endif // QT_CONFIG(cxx17_filesystem)
    QString path() const;
    QString absolutePath() const;
    QString canonicalPath() const;
#if QT_CONFIG(cxx17_filesystem) || defined(Q_CLANG_QDOC)
    std::filesystem::path filesystemPath() const
    { return QtPrivate::toFilesystemPath(path()); }
    std::filesystem::path filesystemAbsolutePath() const
    { return QtPrivate::toFilesystemPath(absolutePath()); }
    std::filesystem::path filesystemCanonicalPath() const
    { return QtPrivate::toFilesystemPath(canonicalPath()); }
#endif // QT_CONFIG(cxx17_filesystem)

    static void setSearchPaths(const QString &prefix, const QStringList &searchPaths);
    static void addSearchPath(const QString &prefix, const QString &path);
#ifdef Q_CLANG_QDOC
    static void addSearchPath(const QString &prefix, const std::filesystem::path &path);
#elif QT_CONFIG(cxx17_filesystem)
    template<typename T, QtPrivate::ForceFilesystemPath<T> = 0>
    static void addSearchPath(const QString &prefix, const T &path)
    {
        addSearchPath(prefix, QtPrivate::fromFilesystemPath(path));
    }
#endif // QT_CONFIG(cxx17_filesystem)
    static QStringList searchPaths(const QString &prefix);

    QString dirName() const;
    QString filePath(const QString &fileName) const;
    QString absoluteFilePath(const QString &fileName) const;
    QString relativeFilePath(const QString &fileName) const;

    static QString toNativeSeparators(const QString &pathName);
    static QString fromNativeSeparators(const QString &pathName);

    bool cd(const QString &dirName);
    bool cdUp();

    QStringList nameFilters() const;
    void setNameFilters(const QStringList &nameFilters);

    Filters filter() const;
    void setFilter(Filters filter);
    SortFlags sorting() const;
    void setSorting(SortFlags sort);

    uint count() const;
    bool isEmpty(Filters filters = Filters(AllEntries | NoDotAndDotDot)) const;

    QString operator[](int) const;

    static QStringList nameFiltersFromString(const QString &nameFilter);

    QStringList entryList(Filters filters = NoFilter, SortFlags sort = NoSort) const;
    QStringList entryList(const QStringList &nameFilters, Filters filters = NoFilter,
                          SortFlags sort = NoSort) const;

    QFileInfoList entryInfoList(Filters filters = NoFilter, SortFlags sort = NoSort) const;
    QFileInfoList entryInfoList(const QStringList &nameFilters, Filters filters = NoFilter,
                                SortFlags sort = NoSort) const;

    bool mkdir(const QString &dirName) const;
    bool mkdir(const QString &dirName, QFile::Permissions permissions) const;
    bool rmdir(const QString &dirName) const;
    bool mkpath(const QString &dirPath) const;
    bool rmpath(const QString &dirPath) const;

    bool removeRecursively();

    bool isReadable() const;
    bool exists() const;
    bool isRoot() const;

    static bool isRelativePath(const QString &path);
    inline static bool isAbsolutePath(const QString &path) { return !isRelativePath(path); }
    bool isRelative() const;
    inline bool isAbsolute() const { return !isRelative(); }
    bool makeAbsolute();

    bool operator==(const QDir &dir) const;
    inline bool operator!=(const QDir &dir) const { return !operator==(dir); }

    bool remove(const QString &fileName);
    bool rename(const QString &oldName, const QString &newName);
    bool exists(const QString &name) const;

    static QFileInfoList drives();

    constexpr static inline QChar listSeparator() noexcept
    {
#if defined(Q_OS_WIN)
        return u';';
#else
        return u':';
#endif
    }

    static QChar separator()
    {
#if defined(Q_OS_WIN)
        return u'\\';
#else
        return u'/';
#endif
    }

    static bool setCurrent(const QString &path);
    static inline QDir current() { return QDir(currentPath()); }
    static QString currentPath();

    static inline QDir home() { return QDir(homePath()); }
    static QString homePath();
    static inline QDir root() { return QDir(rootPath()); }
    static QString rootPath();
    static inline QDir temp() { return QDir(tempPath()); }
    static QString tempPath();

#if QT_CONFIG(regularexpression)
    static bool match(const QStringList &filters, const QString &fileName);
    static bool match(const QString &filter, const QString &fileName);
#endif

    static QString cleanPath(const QString &path);
    void refresh() const;

protected:
    explicit QDir(QDirPrivate &d);

    QSharedDataPointer<QDirPrivate> d_ptr;

private:
    friend class QDirIterator;
    // Q_DECLARE_PRIVATE equivalent for shared data pointers
    QDirPrivate *d_func();
    const QDirPrivate *d_func() const { return d_ptr.constData(); }
};

Q_DECLARE_SHARED(QDir)
Q_DECLARE_OPERATORS_FOR_FLAGS(QDir::Filters)
Q_DECLARE_OPERATORS_FOR_FLAGS(QDir::SortFlags)

#ifndef QT_NO_DEBUG_STREAM
class QDebug;
Q_CORE_EXPORT QDebug operator<<(QDebug debug, QDir::Filters filters);
Q_CORE_EXPORT QDebug operator<<(QDebug debug, const QDir &dir);
#endif

QT_END_NAMESPACE

#endif // QDIR_H
