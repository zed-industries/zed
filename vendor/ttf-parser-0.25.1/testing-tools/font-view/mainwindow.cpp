#include <QElapsedTimer>
#include <QSlider>
#include <QTimer>
#include <QMessageBox>
#include <QDebug>

#include "mainwindow.h"
#include "ui_mainwindow.h"

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
    , ui(new Ui::MainWindow)
{
    ui->setupUi(this);

#ifndef WITH_FREETYPE
    ui->chBoxDrawFreeType->hide();
#endif

#ifndef WITH_HARFBUZZ
    ui->chBoxDrawHarfBuzz->hide();
#endif

    if (qApp->arguments().size() == 2) {
        QTimer::singleShot(1, this, [this](){
            loadFont(qApp->arguments().at(1));
        });
    }
}

MainWindow::~MainWindow()
{
    delete ui;
}

void MainWindow::loadFont(const QString &path)
{
    try {
        m_ttfpFont.open(path);

        const auto variations = m_ttfpFont.loadVariations();
        if (!variations.isEmpty()) {
            ui->widgetVariations->show();

            // Clear layout.
            while (ui->layVariations->count()) {
                delete ui->layVariations->takeAt(0);
            }

            m_variationSliders.clear();

            QVector<Variation> newVariations;

            for (const auto &var : variations) {
                auto hlay = new QHBoxLayout();
                hlay->setContentsMargins(0, 0, 0, 0);
                hlay->addWidget(new QLabel(var.name));

                auto slider = new QSlider(Qt::Horizontal);
                slider->setMinimum(var.min);
                slider->setMaximum(var.max);
                slider->setValue(var.def);
                hlay->addWidget(slider);
                ui->layVariations->addLayout(hlay);

                m_variationSliders.append({ slider, var.tag });

                connect(slider, &QSlider::valueChanged, this, &MainWindow::onVariationChanged);

                newVariations.append({ var.tag, var.def });
            }

            m_ttfpFont.setVariations(newVariations);
        } else {
            ui->widgetVariations->hide();
        }

#ifdef WITH_FREETYPE
        m_ftFont.open(path);
#endif

#ifdef WITH_HARFBUZZ
        m_hbFont.open(path);
#endif

        ui->glyphsView->setFontInfo(m_ttfpFont.fontInfo());
        reloadGlyphs();
    } catch (const QString &err) {
        QMessageBox::warning(this, tr("Error"), err);
    }
}

void MainWindow::reloadGlyphs()
{
    const auto fi = m_ttfpFont.fontInfo();
    for (quint16 i = 0; i < fi.numberOfGlyphs; ++i) {
        try {
            ui->glyphsView->setGlyph(i, m_ttfpFont.outline(i));
        } catch (...) {
        }

#ifdef WITH_FREETYPE
        try {
            ui->glyphsView->setFTGlyph(i, m_ftFont.outline(i));
        } catch (...) {
        }
#endif

#ifdef WITH_HARFBUZZ
        try {
            ui->glyphsView->setHBGlyph(i, m_hbFont.outline(i));
        } catch (...) {
        }
#endif
    }

    ui->glyphsView->viewport()->update();
}

void MainWindow::onVariationChanged()
{
    try {
        QVector<Variation> variations;

        for (auto var : m_variationSliders) {
            variations.append({ var.tag, var.slider->value() });
        }

#ifdef WITH_FREETYPE
        m_ftFont.setVariations(variations);
#endif

#ifdef WITH_HARFBUZZ
        m_hbFont.setVariations(variations);
#endif
        m_ttfpFont.setVariations(variations);

        reloadGlyphs();
    } catch (const QString &err) {
        QMessageBox::warning(this, tr("Error"), err);
    }
}

void MainWindow::on_chBoxDrawBboxes_stateChanged(int flag)
{
    ui->glyphsView->setDrawBboxes(flag);
}

void MainWindow::on_chBoxDrawTtfParser_stateChanged(int flag)
{
    ui->glyphsView->setDrawGlyphs(flag);
}

void MainWindow::on_chBoxDrawFreeType_stateChanged(int flag)
{
    ui->glyphsView->setDrawFTGlyphs(flag);
}

void MainWindow::on_chBoxDrawHarfBuzz_stateChanged(int flag)
{
    ui->glyphsView->setDrawHBGlyphs(flag);
}
