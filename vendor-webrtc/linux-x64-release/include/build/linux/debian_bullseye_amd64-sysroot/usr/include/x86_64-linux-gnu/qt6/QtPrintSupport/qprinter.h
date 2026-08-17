// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QPRINTER_H
#define QPRINTER_H

#include <QtPrintSupport/qtprintsupportglobal.h>
#include <QtCore/qstring.h>
#include <QtCore/qscopedpointer.h>
#include <QtGui/qpagedpaintdevice.h>

QT_BEGIN_NAMESPACE


#ifndef QT_NO_PRINTER

#if defined(B0)
#undef B0 // Terminal hang-up.  We assume that you do not want that.
#endif

class QPrinterPrivate;
class QPaintEngine;
class QPrintEngine;
class QPrinterInfo;
class QPageSize;

class Q_PRINTSUPPORT_EXPORT QPrinter : public QPagedPaintDevice
{
    Q_DECLARE_PRIVATE(QPrinter)
public:
    enum PrinterMode { ScreenResolution, PrinterResolution, HighResolution };

    explicit QPrinter(PrinterMode mode = ScreenResolution);
    explicit QPrinter(const QPrinterInfo& printer, PrinterMode mode = ScreenResolution);
    ~QPrinter();

    int devType() const override;

    enum PageOrder   { FirstPageFirst,
                       LastPageFirst };

    enum ColorMode   { GrayScale,
                       Color };

    enum PaperSource { OnlyOne,
                       Lower,
                       Middle,
                       Manual,
                       Envelope,
                       EnvelopeManual,
                       Auto,
                       Tractor,
                       SmallFormat,
                       LargeFormat,
                       LargeCapacity,
                       Cassette,
                       FormSource,
                       MaxPageSource, // Deprecated
                       CustomSource,
                       LastPaperSource = CustomSource,
                       Upper = OnlyOne  // As defined in Windows
    };

    enum PrinterState { Idle,
                        Active,
                        Aborted,
                        Error };

    enum OutputFormat { NativeFormat, PdfFormat };

    // Keep in sync with QAbstractPrintDialog::PrintRange
    enum PrintRange { AllPages, Selection, PageRange, CurrentPage };

    enum Unit {
        Millimeter,
        Point,
        Inch,
        Pica,
        Didot,
        Cicero,
        DevicePixel
    };

    enum DuplexMode {
        DuplexNone = 0,
        DuplexAuto,
        DuplexLongSide,
        DuplexShortSide
    };

    void setOutputFormat(OutputFormat format);
    OutputFormat outputFormat() const;

    void setPdfVersion(PdfVersion version);
    PdfVersion pdfVersion() const;

    void setPrinterName(const QString &);
    QString printerName() const;

    bool isValid() const;

    void setOutputFileName(const QString &);
    QString outputFileName()const;

    void setPrintProgram(const QString &);
    QString printProgram() const;

    void setDocName(const QString &);
    QString docName() const;

    void setCreator(const QString &);
    QString creator() const;

    void setPageOrder(PageOrder);
    PageOrder pageOrder() const;

    void setResolution(int);
    int resolution() const;

    void setColorMode(ColorMode);
    ColorMode colorMode() const;

    void setCollateCopies(bool collate);
    bool collateCopies() const;

    void setFullPage(bool);
    bool fullPage() const;

    void setCopyCount(int);
    int copyCount() const;
    bool supportsMultipleCopies() const;

    void setPaperSource(PaperSource);
    PaperSource paperSource() const;

    void setDuplex(DuplexMode duplex);
    DuplexMode duplex() const;

    QList<int> supportedResolutions() const;

#if defined(Q_OS_WIN) || defined(Q_CLANG_QDOC)
    QList<PaperSource> supportedPaperSources() const;
#endif

    void setFontEmbeddingEnabled(bool enable);
    bool fontEmbeddingEnabled() const;

    QRectF paperRect(Unit) const;
    QRectF pageRect(Unit) const;

    QString printerSelectionOption() const;
    void setPrinterSelectionOption(const QString &);

    bool newPage() override;
    bool abort();

    PrinterState printerState() const;

    QPaintEngine *paintEngine() const override;
    QPrintEngine *printEngine() const;

    void setFromTo(int fromPage, int toPage);
    int fromPage() const;
    int toPage() const;

    void setPrintRange(PrintRange range);
    PrintRange printRange() const;

protected:
    int metric(PaintDeviceMetric) const override;
    void setEngines(QPrintEngine *printEngine, QPaintEngine *paintEngine);

private:
    Q_DISABLE_COPY(QPrinter)

    QScopedPointer<QPrinterPrivate> d_ptr;

    friend class QPrintDialogPrivate;
    friend class QAbstractPrintDialog;
    friend class QAbstractPrintDialogPrivate;
    friend class QPrintPreviewWidgetPrivate;
    friend class QTextDocument;
    friend class QPageSetupWidget;
};

#endif // QT_NO_PRINTER

QT_END_NAMESPACE

#endif // QPRINTER_H
