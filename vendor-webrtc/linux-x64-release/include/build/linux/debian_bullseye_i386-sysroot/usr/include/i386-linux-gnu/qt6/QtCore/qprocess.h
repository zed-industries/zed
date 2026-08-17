// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QPROCESS_H
#define QPROCESS_H

#include <QtCore/qiodevice.h>
#include <QtCore/qstringlist.h>
#include <QtCore/qshareddata.h>

#include <functional>

QT_REQUIRE_CONFIG(processenvironment);

#if defined(Q_OS_WIN) || defined(Q_CLANG_QDOC)
struct _PROCESS_INFORMATION;
struct _SECURITY_ATTRIBUTES;
struct _STARTUPINFOW;
using Q_PROCESS_INFORMATION = _PROCESS_INFORMATION;
using Q_SECURITY_ATTRIBUTES = _SECURITY_ATTRIBUTES;
using Q_STARTUPINFO = _STARTUPINFOW;
#endif

QT_BEGIN_NAMESPACE

class QProcessPrivate;
class QProcessEnvironmentPrivate;

class Q_CORE_EXPORT QProcessEnvironment
{
public:
    enum Initialization { InheritFromParent };

    QProcessEnvironment();
    QProcessEnvironment(Initialization) noexcept;
    QProcessEnvironment(const QProcessEnvironment &other);
    ~QProcessEnvironment();
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_PURE_SWAP(QProcessEnvironment)
    QProcessEnvironment &operator=(const QProcessEnvironment &other);

    void swap(QProcessEnvironment &other) noexcept { d.swap(other.d); }

    bool operator==(const QProcessEnvironment &other) const;
    inline bool operator!=(const QProcessEnvironment &other) const
    { return !(*this == other); }

    bool isEmpty() const;
    [[nodiscard]] bool inheritsFromParent() const;
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
        FailedToStart,
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

    void start(const QString &program, const QStringList &arguments = {}, OpenMode mode = ReadWrite);
    void start(OpenMode mode = ReadWrite);
    void startCommand(const QString &command, OpenMode mode = ReadWrite);
    bool startDetached(qint64 *pid = nullptr);
    bool open(OpenMode mode = ReadWrite) override;

    QString program() const;
    void setProgram(const QString &program);

    QStringList arguments() const;
    void setArguments(const QStringList & arguments);

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
        Q_PROCESS_INFORMATION *processInformation;
    };
    typedef std::function<void(CreateProcessArguments *)> CreateProcessArgumentModifier;
    CreateProcessArgumentModifier createProcessArgumentsModifier() const;
    void setCreateProcessArgumentsModifier(CreateProcessArgumentModifier modifier);
#endif // Q_OS_WIN || Q_CLANG_QDOC
#if defined(Q_OS_UNIX) || defined(Q_CLANG_QDOC)
    std::function<void(void)> childProcessModifier() const;
    void setChildProcessModifier(const std::function<void(void)> &modifier);
#endif

    QString workingDirectory() const;
    void setWorkingDirectory(const QString &dir);

    void setEnvironment(const QStringList &environment);
    QStringList environment() const;
    void setProcessEnvironment(const QProcessEnvironment &environment);
    QProcessEnvironment processEnvironment() const;

    QProcess::ProcessError error() const;
    QProcess::ProcessState state() const;

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
    qint64 bytesToWrite() const override;
    bool isSequential() const override;
    void close() override;

    static int execute(const QString &program, const QStringList &arguments = {});
    static bool startDetached(const QString &program, const QStringList &arguments = {},
                              const QString &workingDirectory = QString(), qint64 *pid = nullptr);

    static QStringList systemEnvironment();

    static QString nullDevice();

    static QStringList splitCommand(QStringView command);

public Q_SLOTS:
    void terminate();
    void kill();

Q_SIGNALS:
    void started(QPrivateSignal);
    void finished(int exitCode, QProcess::ExitStatus exitStatus = NormalExit);
    void errorOccurred(QProcess::ProcessError error);
    void stateChanged(QProcess::ProcessState state, QPrivateSignal);

    void readyReadStandardOutput(QPrivateSignal);
    void readyReadStandardError(QPrivateSignal);

protected:
    void setProcessState(ProcessState state);

    // QIODevice
    qint64 readData(char *data, qint64 maxlen) override;
    qint64 writeData(const char *data, qint64 len) override;

private:
    Q_DECLARE_PRIVATE(QProcess)
    Q_DISABLE_COPY(QProcess)

#if QT_VERSION < QT_VERSION_CHECK(7,0,0)
    // ### Qt7: Remove this struct and the virtual function; they're here only
    // to cause build errors in Qt 5 code that wasn't updated to Qt 6's
    // setChildProcessModifier()
    struct Use_setChildProcessModifier_Instead {};
    QT_DEPRECATED_X("Use setChildProcessModifier() instead")
    virtual Use_setChildProcessModifier_Instead setupChildProcess();
#endif

    Q_PRIVATE_SLOT(d_func(), bool _q_canReadStandardOutput())
    Q_PRIVATE_SLOT(d_func(), bool _q_canReadStandardError())
#ifdef Q_OS_UNIX
    Q_PRIVATE_SLOT(d_func(), bool _q_canWrite())
#endif
    Q_PRIVATE_SLOT(d_func(), bool _q_startupNotification())
    Q_PRIVATE_SLOT(d_func(), void _q_processDied())
};

#endif // QT_CONFIG(process)

QT_END_NAMESPACE

#endif // QPROCESS_H
