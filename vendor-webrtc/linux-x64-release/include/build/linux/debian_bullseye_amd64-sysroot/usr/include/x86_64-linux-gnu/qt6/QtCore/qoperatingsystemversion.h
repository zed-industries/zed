// Copyright (C) 2021 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#include <QtCore/qglobal.h>
#include <QtCore/qversionnumber.h>

#ifndef QOPERATINGSYSTEMVERSION_H
#define QOPERATINGSYSTEMVERSION_H

QT_BEGIN_NAMESPACE

class QString;

class QOperatingSystemVersionBase
{
public:
    // ### Qt 7: Keep synchronized with the copy in QOperatingSystemVersion until Qt7,
    // then remove this comment :)
    enum OSType {
        Unknown = 0,
        Windows,
        MacOS,
        IOS,
        TvOS,
        WatchOS,
        Android
    };

    constexpr QOperatingSystemVersionBase(OSType osType,
                                            int vmajor, int vminor = -1, int vmicro = -1)
    : m_os(osType),
        m_major(vmajor),
        m_minor(vminor),
        m_micro(vmicro)
    { }

    static Q_CORE_EXPORT QOperatingSystemVersionBase current();
    static Q_CORE_EXPORT QString name(QOperatingSystemVersionBase osversion);
    static Q_CORE_EXPORT bool isAnyOfType(std::initializer_list<OSType> types, OSType type);

    static constexpr OSType currentType()
    {
#if defined(Q_OS_WIN)
        return Windows;
#elif defined(Q_OS_MACOS)
        return MacOS;
#elif defined(Q_OS_IOS)
        return IOS;
#elif defined(Q_OS_TVOS)
        return TvOS;
#elif defined(Q_OS_WATCHOS)
        return WatchOS;
#elif defined(Q_OS_ANDROID)
        return Android;
#else
        return Unknown;
#endif
    }

    inline QVersionNumber version() const { return QVersionNumber(m_major, m_minor, m_micro); }

    constexpr int majorVersion() const { return m_major; }
    constexpr int minorVersion() const { return m_minor; }
    constexpr int microVersion() const { return m_micro; }

    constexpr int segmentCount() const
    { return m_micro >= 0 ? 3 : m_minor >= 0 ? 2 : m_major >= 0 ? 1 : 0; }

    inline bool isAnyOfType(std::initializer_list<OSType> types) const
    {
        return QOperatingSystemVersionBase::isAnyOfType(types, type());
    }
    constexpr OSType type() const { return m_os; }
    inline QString name() const { return name(*this); }

    friend bool operator>(QOperatingSystemVersionBase lhs, QOperatingSystemVersionBase rhs)
    { return lhs.type() == rhs.type() && QOperatingSystemVersionBase::compare(lhs, rhs) > 0; }

    friend bool operator>=(QOperatingSystemVersionBase lhs, QOperatingSystemVersionBase rhs)
    { return lhs.type() == rhs.type() && QOperatingSystemVersionBase::compare(lhs, rhs) >= 0; }

    friend bool operator<(QOperatingSystemVersionBase lhs, QOperatingSystemVersionBase rhs)
    { return lhs.type() == rhs.type() && QOperatingSystemVersionBase::compare(lhs, rhs) < 0; }

    friend bool operator<=(QOperatingSystemVersionBase lhs, QOperatingSystemVersionBase rhs)
    { return lhs.type() == rhs.type() && QOperatingSystemVersionBase::compare(lhs, rhs) <= 0; }

protected:
    static Q_CORE_EXPORT int compare(QOperatingSystemVersionBase v1,
                                     QOperatingSystemVersionBase v2);

    QOperatingSystemVersionBase() = default;
private:
    static QOperatingSystemVersionBase current_impl();


    OSType m_os;
    int m_major;
    int m_minor;
    int m_micro;
};

// ### Qt 7: Un-export the class, export relevant functions. Remove the enum.
class Q_CORE_EXPORT QOperatingSystemVersion : public QOperatingSystemVersionBase
{
public:
    // ### Qt7: Remove. Keep synchronized with QOperatingSystemVersionBase::OSType until then!
#if QT_VERSION < QT_VERSION_CHECK(7, 0, 0)
    enum OSType {
        Unknown = 0,
        Windows,
        MacOS,
        IOS,
        TvOS,
        WatchOS,
        Android
    };
#endif

    // ### Qt7: remove the branch with static const variables. Then group and sort the inline ones.
    // Since the exported variables emit symbols they cannot be cherry-picked back to patch-releases
    // without breaking our BC promises. They must be fully inline but we cannot make that change
    // until Qt7
    // @note: New entries should be added after the if-def-ery until Qt 7!!
#if QT_VERSION < QT_VERSION_CHECK(7, 0, 0)
    static const QOperatingSystemVersion Windows7;
    static const QOperatingSystemVersion Windows8;
    static const QOperatingSystemVersion Windows8_1;
    static const QOperatingSystemVersion Windows10;

    static const QOperatingSystemVersion OSXMavericks;
    static const QOperatingSystemVersion OSXYosemite;
    static const QOperatingSystemVersion OSXElCapitan;
    static const QOperatingSystemVersion MacOSSierra;
    static const QOperatingSystemVersion MacOSHighSierra;
    static const QOperatingSystemVersion MacOSMojave;
    static const QOperatingSystemVersion MacOSCatalina;
    static const QOperatingSystemVersion MacOSBigSur;
    static const QOperatingSystemVersion MacOSMonterey;

    static const QOperatingSystemVersion AndroidJellyBean;
    static const QOperatingSystemVersion AndroidJellyBean_MR1;
    static const QOperatingSystemVersion AndroidJellyBean_MR2;
    static const QOperatingSystemVersion AndroidKitKat;
    static const QOperatingSystemVersion AndroidLollipop;
    static const QOperatingSystemVersion AndroidLollipop_MR1;
    static const QOperatingSystemVersion AndroidMarshmallow;
    static const QOperatingSystemVersion AndroidNougat;
    static const QOperatingSystemVersion AndroidNougat_MR1;
    static const QOperatingSystemVersion AndroidOreo;
    static const QOperatingSystemVersion AndroidOreo_MR1;
    static const QOperatingSystemVersion AndroidPie;
    static const QOperatingSystemVersion Android10;
    static const QOperatingSystemVersion Android11;
#else
    static constexpr QOperatingSystemVersionBase Windows7 { QOperatingSystemVersionBase::Windows, 6, 1 };
    static constexpr QOperatingSystemVersionBase Windows8 { QOperatingSystemVersionBase::Windows, 6, 2 };
    static constexpr QOperatingSystemVersionBase Windows8_1 { QOperatingSystemVersionBase::Windows, 6, 3 };
    static constexpr QOperatingSystemVersionBase Windows10 { QOperatingSystemVersionBase::Windows, 10 };

    static constexpr QOperatingSystemVersionBase OSXMavericks { QOperatingSystemVersionBase::MacOS, 10, 9 };
    static constexpr QOperatingSystemVersionBase OSXYosemite { QOperatingSystemVersionBase::MacOS, 10, 10 };
    static constexpr QOperatingSystemVersionBase OSXElCapitan { QOperatingSystemVersionBase::MacOS, 10, 11 };
    static constexpr QOperatingSystemVersionBase MacOSSierra { QOperatingSystemVersionBase::MacOS, 10, 12 };
    static constexpr QOperatingSystemVersionBase MacOSHighSierra { QOperatingSystemVersionBase::MacOS, 10, 13 };
    static constexpr QOperatingSystemVersionBase MacOSMojave { QOperatingSystemVersionBase::MacOS, 10, 14 };
    static constexpr QOperatingSystemVersionBase MacOSCatalina { QOperatingSystemVersionBase::MacOS, 10, 15 };
#if !defined(Q_OS_DARWIN) || QT_MACOS_PLATFORM_SDK_EQUAL_OR_ABOVE(__MAC_11_0)
    static constexpr QOperatingSystemVersionBase MacOSBigSur = { QOperatingSystemVersionBase::MacOS, 11, 0 };
    static constexpr QOperatingSystemVersionBase MacOSMonterey = { QOperatingSystemVersionBase::MacOS, 12, 0 };
#else // ### Qt 7: Verify the assumption
#   error Either you are using an outdated SDK or my assumption that Qt7 would require at least 11.0 was wrong
#endif

    static constexpr QOperatingSystemVersionBase AndroidJellyBean { QOperatingSystemVersionBase::Android, 4, 1 };
    static constexpr QOperatingSystemVersionBase AndroidJellyBean_MR1 { QOperatingSystemVersionBase::Android, 4, 2 };
    static constexpr QOperatingSystemVersionBase AndroidJellyBean_MR2 { QOperatingSystemVersionBase::Android, 4, 3 };
    static constexpr QOperatingSystemVersionBase AndroidKitKat { QOperatingSystemVersionBase::Android, 4, 4 };
    static constexpr QOperatingSystemVersionBase AndroidLollipop { QOperatingSystemVersionBase::Android, 5, 0 };
    static constexpr QOperatingSystemVersionBase AndroidLollipop_MR1 { QOperatingSystemVersionBase::Android, 5, 1 };
    static constexpr QOperatingSystemVersionBase AndroidMarshmallow { QOperatingSystemVersionBase::Android, 6, 0 };
    static constexpr QOperatingSystemVersionBase AndroidNougat { QOperatingSystemVersionBase::Android, 7, 0 };
    static constexpr QOperatingSystemVersionBase AndroidNougat_MR1 { QOperatingSystemVersionBase::Android, 7, 1 };
    static constexpr QOperatingSystemVersionBase AndroidOreo { QOperatingSystemVersionBase::Android, 8, 0 };
    static constexpr QOperatingSystemVersionBase AndroidOreo_MR1 { QOperatingSystemVersionBase::Android, 8, 1 };
    static constexpr QOperatingSystemVersionBase AndroidPie { QOperatingSystemVersionBase::Android, 9, 0 };
    static constexpr QOperatingSystemVersionBase Android10 { QOperatingSystemVersionBase::Android, 10, 0 };
    static constexpr QOperatingSystemVersionBase Android11 { QOperatingSystemVersionBase::Android, 11, 0 };
#endif // New (static constexpr) entries go here, only cherry-pick as far back as 6.3 (QTBUG-97808):

    static constexpr QOperatingSystemVersionBase Windows10_1809 { QOperatingSystemVersionBase::Windows, 10, 0, 17763 }; // RS5
    static constexpr QOperatingSystemVersionBase Windows10_1903 { QOperatingSystemVersionBase::Windows, 10, 0, 18362 }; // 19H1
    static constexpr QOperatingSystemVersionBase Windows10_1909 { QOperatingSystemVersionBase::Windows, 10, 0, 18363 }; // 19H2
    static constexpr QOperatingSystemVersionBase Windows10_2004 { QOperatingSystemVersionBase::Windows, 10, 0, 19041 }; // 20H1
    static constexpr QOperatingSystemVersionBase Windows10_20H2 { QOperatingSystemVersionBase::Windows, 10, 0, 19042 };
    static constexpr QOperatingSystemVersionBase Windows10_21H1 { QOperatingSystemVersionBase::Windows, 10, 0, 19043 };
    static constexpr QOperatingSystemVersionBase Windows10_21H2 { QOperatingSystemVersionBase::Windows, 10, 0, 19044 };
    static constexpr QOperatingSystemVersionBase Windows10_22H2 { QOperatingSystemVersionBase::Windows, 10, 0, 19045 };
    static constexpr QOperatingSystemVersionBase Windows11 { QOperatingSystemVersionBase::Windows, 10, 0, 22000 };
    static constexpr QOperatingSystemVersionBase Windows11_21H2 = Windows11;
    static constexpr QOperatingSystemVersionBase Windows11_22H2 { QOperatingSystemVersionBase::Windows, 10, 0, 22621 };

    static constexpr QOperatingSystemVersionBase MacOSVentura { QOperatingSystemVersionBase::MacOS, 13, 0 };

    constexpr QOperatingSystemVersion(const QOperatingSystemVersionBase &osversion)
        : QOperatingSystemVersionBase(osversion) {}

    constexpr QOperatingSystemVersion(OSType osType, int vmajor, int vminor = -1, int vmicro = -1)
        : QOperatingSystemVersionBase(QOperatingSystemVersionBase::OSType(osType), vmajor, vminor,
                                      vmicro)
    {
    }

    static QOperatingSystemVersion current();

    static constexpr OSType currentType()
    {
        return OSType(QOperatingSystemVersionBase::currentType());
    }

    QVersionNumber version() const { return QOperatingSystemVersionBase::version(); }

    constexpr int majorVersion() const { return QOperatingSystemVersionBase::majorVersion(); }
    constexpr int minorVersion() const { return QOperatingSystemVersionBase::minorVersion(); }
    constexpr int microVersion() const { return QOperatingSystemVersionBase::microVersion(); }

    constexpr int segmentCount() const
    { return QOperatingSystemVersionBase::segmentCount(); }

    constexpr OSType type() const { return OSType(QOperatingSystemVersionBase::type()); }
    bool isAnyOfType(std::initializer_list<OSType> types) const;
    QString name() const;

private:
    QOperatingSystemVersion() = default;

#if QT_CORE_REMOVED_SINCE(6, 3)
    // ### Qt 7: Remove. It's only here for backwards compat with previous inline calls.
    [[maybe_unused]] static int compare(const QOperatingSystemVersion &v1,
                                        const QOperatingSystemVersion &v2);
#endif
};
Q_DECLARE_TYPEINFO(QOperatingSystemVersion, Q_PRIMITIVE_TYPE);

#ifndef QT_NO_DEBUG_STREAM
class QDebug;
Q_CORE_EXPORT QDebug operator<<(QDebug debug, const QOperatingSystemVersion &ov);
#endif

QT_END_NAMESPACE

#endif // QOPERATINGSYSTEMVERSION_H
