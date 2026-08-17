/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Copyright (C) 2018 Intel Corporation.
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

#ifndef QPLUGIN_H
#define QPLUGIN_H

#include <QtCore/qobject.h>
#include <QtCore/qpointer.h>
#include <QtCore/qjsonobject.h>

QT_BEGIN_NAMESPACE


#ifndef Q_EXTERN_C
#  ifdef __cplusplus
#    define Q_EXTERN_C extern "C"
#  else
#    define Q_EXTERN_C extern
#  endif
#endif

inline constexpr unsigned char qPluginArchRequirements()
{
    return 0
#ifndef QT_NO_DEBUG
            | 1
#endif
#ifdef __AVX2__
            | 2
#  ifdef __AVX512F__
            | 4
#  endif
#endif
    ;
}

typedef QObject *(*QtPluginInstanceFunction)();
#if QT_VERSION < QT_VERSION_CHECK(6, 0, 0)
typedef const char *(*QtPluginMetaDataFunction)();
#else
struct QPluginMetaData
{
    const uchar *data;
    size_t size;
};
typedef QPluginMetaData (*QtPluginMetaDataFunction)();
#endif


struct Q_CORE_EXPORT QStaticPlugin
{
#if QT_VERSION >= QT_VERSION_CHECK(6, 0, 0)
public:
    constexpr QStaticPlugin(QtPluginInstanceFunction i, QtPluginMetaDataFunction m)
        : instance(i), rawMetaData(m().data), rawMetaDataSize(m().size)
    {}
    QtPluginInstanceFunction instance;
private:
    // ### Qt 6: revise, as this is not standard-layout
    const void *rawMetaData;
    qsizetype rawMetaDataSize;
public:
#elif !defined(Q_QDOC)
    // Note: This struct is initialized using an initializer list.
    // As such, it cannot have any new constructors or variables.
    QtPluginInstanceFunction instance;
    QtPluginMetaDataFunction rawMetaData;
#else
    // Since qdoc gets confused by the use of function
    // pointers, we add these dummes for it to parse instead:
    QObject *instance();
    const char *rawMetaData();
#endif
    QJsonObject metaData() const;
};
Q_DECLARE_TYPEINFO(QStaticPlugin, Q_PRIMITIVE_TYPE);

void Q_CORE_EXPORT qRegisterStaticPluginFunction(QStaticPlugin staticPlugin);

#if (defined(Q_OF_ELF) || defined(Q_OS_WIN)) && (defined (Q_CC_GNU) || defined(Q_CC_CLANG))
#  define QT_PLUGIN_METADATA_SECTION \
    __attribute__ ((section (".qtmetadata"))) __attribute__((used))
#elif defined(Q_OS_MAC)
// TODO: Implement section parsing on Mac
#  define QT_PLUGIN_METADATA_SECTION \
    __attribute__ ((section ("__TEXT,qtmetadata"))) __attribute__((used))
#elif defined(Q_CC_MSVC)
// TODO: Implement section parsing for MSVC
#pragma section(".qtmetadata",read,shared)
#  define QT_PLUGIN_METADATA_SECTION \
    __declspec(allocate(".qtmetadata"))
#else
#  define QT_PLUGIN_METADATA_SECTION
#endif


#define Q_IMPORT_PLUGIN(PLUGIN) \
        extern const QT_PREPEND_NAMESPACE(QStaticPlugin) qt_static_plugin_##PLUGIN(); \
        class Static##PLUGIN##PluginInstance{ \
        public: \
                Static##PLUGIN##PluginInstance() { \
                    qRegisterStaticPluginFunction(qt_static_plugin_##PLUGIN()); \
                } \
        }; \
       static Static##PLUGIN##PluginInstance static##PLUGIN##Instance;

#if defined(QT_PLUGIN_RESOURCE_INIT_FUNCTION)
#  define QT_PLUGIN_RESOURCE_INIT \
          extern void QT_PLUGIN_RESOURCE_INIT_FUNCTION(); \
          QT_PLUGIN_RESOURCE_INIT_FUNCTION();
#else
#  define QT_PLUGIN_RESOURCE_INIT
#endif

#define Q_PLUGIN_INSTANCE(IMPLEMENTATION) \
        { \
            static QT_PREPEND_NAMESPACE(QPointer)<QT_PREPEND_NAMESPACE(QObject)> _instance; \
            if (!_instance) {    \
                QT_PLUGIN_RESOURCE_INIT \
                _instance = new IMPLEMENTATION; \
            } \
            return _instance; \
        }

#if defined(QT_STATICPLUGIN)

#if QT_VERSION >= QT_VERSION_CHECK(6, 0, 0)
#  define QT_MOC_EXPORT_PLUGIN(PLUGINCLASS, PLUGINCLASSNAME) \
    static QT_PREPEND_NAMESPACE(QObject) *qt_plugin_instance_##PLUGINCLASSNAME() \
    Q_PLUGIN_INSTANCE(PLUGINCLASS) \
    static QPluginMetaData qt_plugin_query_metadata_##PLUGINCLASSNAME() \
        { return { qt_pluginMetaData, sizeof qt_pluginMetaData }; } \
    const QT_PREPEND_NAMESPACE(QStaticPlugin) qt_static_plugin_##PLUGINCLASSNAME() { \
        return { qt_plugin_instance_##PLUGINCLASSNAME, qt_plugin_query_metadata_##PLUGINCLASSNAME}; \
    }
#else
#  define QT_MOC_EXPORT_PLUGIN(PLUGINCLASS, PLUGINCLASSNAME) \
    static QT_PREPEND_NAMESPACE(QObject) *qt_plugin_instance_##PLUGINCLASSNAME() \
    Q_PLUGIN_INSTANCE(PLUGINCLASS) \
    static const char *qt_plugin_query_metadata_##PLUGINCLASSNAME() { return reinterpret_cast<const char *>(qt_pluginMetaData); } \
    const QT_PREPEND_NAMESPACE(QStaticPlugin) qt_static_plugin_##PLUGINCLASSNAME() { \
        QT_PREPEND_NAMESPACE(QStaticPlugin) plugin = { qt_plugin_instance_##PLUGINCLASSNAME, qt_plugin_query_metadata_##PLUGINCLASSNAME}; \
        return plugin; \
    }
#endif

#else
#if QT_VERSION >= QT_VERSION_CHECK(6, 0, 0)

#  define QT_MOC_EXPORT_PLUGIN(PLUGINCLASS, PLUGINCLASSNAME)      \
            Q_EXTERN_C Q_DECL_EXPORT \
            QPluginMetaData qt_plugin_query_metadata() \
            { return { qt_pluginMetaData, sizeof qt_pluginMetaData }; } \
            Q_EXTERN_C Q_DECL_EXPORT QT_PREPEND_NAMESPACE(QObject) *qt_plugin_instance() \
            Q_PLUGIN_INSTANCE(PLUGINCLASS)

#else

#  define QT_MOC_EXPORT_PLUGIN(PLUGINCLASS, PLUGINCLASSNAME)      \
            Q_EXTERN_C Q_DECL_EXPORT \
            const char *qt_plugin_query_metadata() \
            { return reinterpret_cast<const char *>(qt_pluginMetaData); } \
            Q_EXTERN_C Q_DECL_EXPORT QT_PREPEND_NAMESPACE(QObject) *qt_plugin_instance() \
            Q_PLUGIN_INSTANCE(PLUGINCLASS)

#endif

#endif

#define Q_EXPORT_PLUGIN(PLUGIN) \
            Q_EXPORT_PLUGIN2(PLUGIN, PLUGIN)
#  define Q_EXPORT_PLUGIN2(PLUGIN, PLUGINCLASS)      \
    Q_STATIC_ASSERT_X(false, "Old plugin system used")

#  define Q_EXPORT_STATIC_PLUGIN2(PLUGIN, PLUGINCLASS) \
    Q_STATIC_ASSERT_X(false, "Old plugin system used")


QT_END_NAMESPACE

#endif // Q_PLUGIN_H
