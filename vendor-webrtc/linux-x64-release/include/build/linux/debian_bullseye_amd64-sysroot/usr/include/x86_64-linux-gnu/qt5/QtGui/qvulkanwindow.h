/****************************************************************************
**
** Copyright (C) 2017 The Qt Company Ltd.
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

#ifndef QVULKANWINDOW_H
#define QVULKANWINDOW_H

#include <QtGui/qtguiglobal.h>

#if 0
#pragma qt_no_master_include
#endif

#if QT_CONFIG(vulkan) || defined(Q_CLANG_QDOC)

#include <QtGui/qvulkaninstance.h>
#include <QtGui/qwindow.h>
#include <QtGui/qimage.h>
#include <QtGui/qmatrix4x4.h>
#include <QtCore/qset.h>

#ifdef Q_CLANG_QDOC
typedef void* VkQueue;
typedef void* VkCommandPool;
typedef void* VkRenderPass;
typedef void* VkCommandBuffer;
typedef void* VkFramebuffer;
typedef int VkPhysicalDeviceProperties;
typedef int VkFormat;
typedef int VkQueueFamilyProperties;
typedef int VkDeviceQueueCreateInfo;
typedef int VkFormat;
typedef int VkSampleCountFlagBits;
#endif

QT_BEGIN_NAMESPACE

class QVulkanWindowPrivate;

class Q_GUI_EXPORT QVulkanWindowRenderer
{
public:
    virtual ~QVulkanWindowRenderer();

    virtual void preInitResources();
    virtual void initResources();
    virtual void initSwapChainResources();
    virtual void releaseSwapChainResources();
    virtual void releaseResources();

    virtual void startNextFrame() = 0;

    virtual void physicalDeviceLost();
    virtual void logicalDeviceLost();
};

class Q_GUI_EXPORT QVulkanWindow : public QWindow
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QVulkanWindow)

public:
    enum Flag {
        PersistentResources = 0x01
    };
    Q_DECLARE_FLAGS(Flags, Flag)

    explicit QVulkanWindow(QWindow *parent = nullptr);
    ~QVulkanWindow();

    void setFlags(Flags flags);
    Flags flags() const;

    QVector<VkPhysicalDeviceProperties> availablePhysicalDevices();
    void setPhysicalDeviceIndex(int idx);

    QVulkanInfoVector<QVulkanExtension> supportedDeviceExtensions();
    void setDeviceExtensions(const QByteArrayList &extensions);

    void setPreferredColorFormats(const QVector<VkFormat> &formats);

    QVector<int> supportedSampleCounts();
    void setSampleCount(int sampleCount);

    typedef std::function<void(const VkQueueFamilyProperties *,
                               uint32_t,
                               QVector<VkDeviceQueueCreateInfo> &)> QueueCreateInfoModifier;
    void setQueueCreateInfoModifier(const QueueCreateInfoModifier &modifier);

    bool isValid() const;

    virtual QVulkanWindowRenderer *createRenderer();
    void frameReady();

    VkPhysicalDevice physicalDevice() const;
    const VkPhysicalDeviceProperties *physicalDeviceProperties() const;
    VkDevice device() const;
    VkQueue graphicsQueue() const;
    uint32_t graphicsQueueFamilyIndex() const;
    VkCommandPool graphicsCommandPool() const;
    uint32_t hostVisibleMemoryIndex() const;
    uint32_t deviceLocalMemoryIndex() const;
    VkRenderPass defaultRenderPass() const;

    VkFormat colorFormat() const;
    VkFormat depthStencilFormat() const;
    QSize swapChainImageSize() const;

    VkCommandBuffer currentCommandBuffer() const;
    VkFramebuffer currentFramebuffer() const;
    int currentFrame() const;

    static const int MAX_CONCURRENT_FRAME_COUNT = 3;
    int concurrentFrameCount() const;

    int swapChainImageCount() const;
    int currentSwapChainImageIndex() const;
    VkImage swapChainImage(int idx) const;
    VkImageView swapChainImageView(int idx) const;
    VkImage depthStencilImage() const;
    VkImageView depthStencilImageView() const;

    VkSampleCountFlagBits sampleCountFlagBits() const;
    VkImage msaaColorImage(int idx) const;
    VkImageView msaaColorImageView(int idx) const;

    bool supportsGrab() const;
    QImage grab();

    QMatrix4x4 clipCorrectionMatrix();

Q_SIGNALS:
    void frameGrabbed(const QImage &image);

protected:
    void exposeEvent(QExposeEvent *) override;
    void resizeEvent(QResizeEvent *) override;
    bool event(QEvent *) override;

private:
    Q_DISABLE_COPY(QVulkanWindow)
};

Q_DECLARE_OPERATORS_FOR_FLAGS(QVulkanWindow::Flags)

QT_END_NAMESPACE

#endif // QT_CONFIG(vulkan)

#endif
