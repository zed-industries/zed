/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
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

#ifndef QPROCESS_H
#define QPROCESS_H

#include <QtCore/qiodevice.h>
#include <QtCore/qstringlist.h>
#include <QtCore/qshareddata.h>

#include <functional>

QT_REQUIRE_CONFIG(processenvironment);

#ifdef Q_OS_WIN
typedef struct _PROCESS_INFORMATION *Q_PID;
#endif

#if defined(Q_OS_WIN) || defined(Q_CLANG_QDOC)
typedef struct _SECURITY_ATTRIBUTES Q_SECURITY_ATTRIBUTES;
typedef struct _STARTUPINFOW Q_STARTUPINFO;
#endif

QT_BEGIN_NAMESPACE

class QProcessPrivate;
class QProcessEnvironmentPrivate;

#ifndef Q_OS_WIN
typedef qint64 Q_PID;
#endif

class Q_CORE_EXPORT QProcessEnvironment
{
public:
    QProcessEnvironment();
    QProcessEnvironment(const QProcessEnvironment &other);
    ~QProcessEnvironment();
    QProcessEnvironment &operator=(QProcessEnvironment && other) noexcept { swap(other); return *this; }
    QProcessEnvironment &operator=(const QProcessEnvironment &other);

    void swap(QProcessEnvironment &other) noexcept { qSwap(d, other.d); }

    bool operator==(const QProcessEnvironment &other) const;
    inline bool operator!=(const QProcessEnvironment &other) const
    { return !(*this == other); }

    bool isEmpty() const;
    void clear();

    bool contains(const QString &name) const;
    void insert(const QString &name, const QString &value);
    void remove(const QString &name);
    QString value(const QString &name, const QString &defaultValue = QString()) const;

    QStringList toStringList() const;

    QStringList keys() const;

    void insert(const QProcessEnvironment &e);

    static QProcessEnvironment systemEnvironment();

private:
    friend class QProcessPrivate;
    friend class QProcessEnvironmentPrivate;
    QSharedDataPointer<QProcessEnvironmentPrivate> d;
};

Q_DECLARE_SHARED(QProcessEnvironment)

#if QT_CONFIG(process)

class Q_CORE_EXPORT QProcess : public QIODevice
{
    Q_OBJECT
public:
    enum ProcessError {
        FailedToStart, //### file not found, resource error
        Crashed,
        Timedout,
        ReadError,
        WriteError,
        UnknownError
    };
    Q_ENUM(ProcessError)

    enum ProcessState {
        NotRunning,
        Starting,
        Running
    };
    Q_ENUM(ProcessState)

    enum ProcessChannel {
        StandardOutput,
        StandardError
    };
    Q_ENUM(ProcessChannel)

    enum ProcessChannelMode {
        SeparateChannels,
        MergedChannels,
        ForwardedChannels,
        ForwardedOutputChannel,
        ForwardedErrorChannel
    };
    Q_ENUM(ProcessChannelMode)

    enum InputChannelMode {
        ManagedInputChannel,
        ForwardedInputChannel
    };
    Q_ENUM(InputChannelMode)

    enum ExitStatus {
        NormalExit,
        CrashExit
    };
    Q_ENUM(ExitStatus)

    explicit QProcess(QObject *parent = nullptr);
    virtual ~QProcess();

    void start(const QString &program, const QStringList &arguments, OpenMode mode = ReadWrite);
#if !defined(QT_NO_PROCESS_COMBINED_ARGUMENT_START)
#if QT_DEPRECATED_SINCE(5, 15)
    QT_DEPRECATED_X(
        "Use QProcess::start(const QString &program, const QStringList &arguments,"
        "OpenMode mode = ReadWrite) instead"
    )
    void start(const QString &command, OpenMode mode = ReadWrite);
#endif
#endif
    void start(OpenMode mode = ReadWrite);
    bool startDetached(qint64 *pid = nullptr);
    bool open(OpenMode mode = ReadWrite) override;

    QString program() const;
    void setProgram(const QString &program);

    QStringList arguments() const;
    void setArguments(const QStringList & arguments);

#if QT_DEPRECATED_SINCE(5, 13)
    QT_DEPRECATED_X("Use QProcess::processChannelMode() instead")
    ProcessChannelMode readChannelMode() const;
    QT_DEPRECATED_X("Use QProcess::setProcessChannelMode() instead")
    void setReadChannelMode(ProcessChannelMode mode);
#endif
    ProcessChannelMode processChannelMode() const;
    void setProcessChannelMode(ProcessChannelMode mode);
    InputChannelMode inputChannelMode() const;
    void setInputChannelMode(InputChannelMode mode);

    ProcessChannel readChannel() const;
    void setReadChannel(ProcessChannel channel);

    void closeReadChannel(ProcessChannel channel);
    void closeWriteChannel();

    void setStandardInputFile(const QString &fileName);
    void setStandardOutputFile(const QString &fileName, OpenMode mode = Truncate);
    void setStandardErrorFile(const QString &fileName, OpenMode mode = Truncate);
    void setStandardOutputProcess(QProcess *destination);

#if defined(Q_OS_WIN) || defined(Q_CLANG_QDOC)
    QString nativeArguments() const;
    void setNativeArguments(const QString &arguments);
    struct CreateProcessArguments
    {
        const wchar_t *applicationName;
        wchar_t *arguments;
        Q_SECURITY_ATTRIBUTES *processAttributes;
        Q_SECURITY_ATTRIBUTES *threadAttributes;
        bool inheritHandles;
        unsigned long flags;
        void *environment;
        const wchar_t *currentDirectory;
        Q_STARTUPINFO *startupInfo;
        Q_PID processInformation;
    };
    typedef std::function<void(CreateProcessArguments *)> CreateProcessArgumentModifier;
    CreateProcessArgumentModifier createProcessArgumentsModifier() const;
    void setCreateProcessArgumentsModifier(CreateProcessArgumentModifier modifier);
#endif // Q_OS_WIN || Q_CLANG_QDOC

    QString workingDirectory() const;
    void setWorkingDirectory(const QString &dir);

    void setEnvironment(const QStringList &environment);
    QStringList environment() const;
    void setProcessEnvironment(const QProcessEnvironment &environment);
    QProcessEnvironment processEnvironment() const;

    QProcess::ProcessError error() const;
    QProcess::ProcessState state() const;

#if QT_DEPRECATED_SINCE(5, 15)
    QT_DEPRECATED_VERSION_X_5_15("Use processId() instead")
    Q_PID pid() const;
#endif
    qint64 processId() const;

    bool waitForStarted(int msecs = 30000);
    bool waitForReadyRead(int msecs = 30000) override;
    bool waitForBytesWritten(int msecs = 30000) override;
    bool waitForFinished(int msecs = 30000);

    QByteArray readAllStandardOutput();
    QByteArray readAllStandardError();

    int exitCode() const;
    QProcess::ExitStatus exitStatus() const;

    // QIODevice
    qint64 bytesAvailable() const override; // ### Qt6: remove trivial override
    qint64 bytesToWrite() const override;
    bool isSequential() const override;
    bool canReadLine() const override; // ### Qt6: remove trivial override
    void close() override;
    bool atEnd() const override; // ### Qt6: remove trivial override

    static int execute(const QString &program, const QStringList &arguments);
#if QT_DEPRECATED_SINCE(5, 15)
    QT_DEPRECATED_X(
        "Use QProcess::execute(const QString &program, const QStringList &arguments) instead"
    )
    static int execute(const QString &command);
#endif
    static bool startDetached(const QString &program, const QStringList &arguments,
                              const QString &workingDirectory
#if defined(Q_QDOC)
                              = QString()
#endif
                              , qint64 *pid = nullptr);
#if !defined(Q_QDOC)
    static bool startDetached(const QString &program, const QStringList &arguments); // ### Qt6: merge overloads
#endif
#if QT_DEPRECATED_SINCE(5, 15)
    QT_DEPRECATED_X(
        "Use QProcess::startDetached(const QString &program, const QStringList &arguments) instead"
    )
    static bool startDetached(const QString &command);
#endif

    static QStringList systemEnvironment();

    static QString nullDevice();

    static QStringList splitCommand(QStringView command);

public Q_SLOTS:
    void terminate();
    void kill();

Q_SIGNALS:
    void started(QPrivateSignal);
#if QT_DEPRECATED_SINCE(5, 13)
    QT_DEPRECATED_X("Use QProcess::finished(int, QProcess::ExitStatus) instead")
    void finished(int exitCode); // ### Qt 6: merge the two signals with a default value
#endif
    void finished(int exitCode, QProcess::ExitStatus exitStatus);
#if QT_DEPRECATED_SINCE(5, 6)
    QT_DEPRECATED_X("Use QProcess::errorOccurred(QProcess::ProcessError) instead")
    void error(QProcess::ProcessError error);
#endif
    void errorOccurred(QProcess::ProcessError error);
    void stateChanged(QProcess::ProcessState state, QPrivateSignal);

    void readyReadStandardOutput(QPrivateSignal);
    void readyReadStandardError(QPrivateSignal);

protected:
    void setProcessState(ProcessState state);

    virtual void setupChildProcess();

    // QIODevice
    qint64 readData(char *data, qint64 maxlen) override;
    qint64 writeData(const char *data, qint64 len) override;

private:
    Q_DECLARE_PRIVATE(QProcess)
    Q_DISABLE_COPY(QProcess)

    Q_PRIVATE_SLOT(d_func(), bool _q_canReadStandardOutput())
    Q_PRIVATE_SLOT(d_func(), bool _q_canReadStandardError())
    Q_PRIVATE_SLOT(d_func(), bool _q_canWrite())
    Q_PRIVATE_SLOT(d_func(), bool _q_startupNotification())
    Q_PRIVATE_SLOT(d_func(), bool _q_processDied())
    friend class QProcessManager;
};

#endif // QT_CONFIG(process)

QT_END_NAMESPACE

#endif // QPROCESS_H
