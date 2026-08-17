/*
 * Copyright (c) 2020 Intel Corporation. All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sub license, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice (including the
 * next paragraph) shall be included in all copies or substantial portions
 * of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT.
 * IN NO EVENT SHALL INTEL AND/OR ITS SUPPLIERS BE LIABLE FOR
 * ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
 * TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
 * SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

/**
 * \file va_prot.h
 * \brief Protected content API.
 *
 * This file contains the \ref api_prot "Protected content API".
 */

#ifndef VA_PROT_H
#define VA_PROT_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \defgroup api_prot Protected content API
 *
 * @{
 * \section prolouge Prolouge
 * Video streaming is ubiquitous and the support for video streaming is widely
 * available across client open systems such as PCs, MACs, Chromebooks etc. and
 * closed systems such as settop box, smart TVs, DVDs etc. By default,
 * video streaming is not considered premium due to various constraints such as
 * resolution, quality, production cost etc. but recently streaming of premium
 * video(1080p+) has become norm. The streaming of premium video in open systems
 * such as PCs, MACs, Chromebooks etc. makes video particularly susceptible to
 * piracy (due to non-video playback usages of such systems) resulting in
 * millions of dollars of loss to content creators.
 *
 * Digital Rights Management(DRM) has been proposed to stop piracy of premium
 * video streams across a wide spectrum. There are some known open/closed DRM
 * standards such as [Widevine by Google](https://www.widevine.com/),
 * [PlayReady by Microsoft](https://www.microsoft.com/playready/),
 * [FairPlay by Apple](https://developer.apple.com/streaming/fps/),
 * [Merlin by Sony](https://www.marlin-community.com/), etc... Each DRM
 * standard has its properties but all DRM standards support a common
 * mechanism. This common mechanism involves cryptographical method for
 * authenticating the client system, delivering bitstream and required
 * cryptographic assets to client system and then cryptographically processing
 * bitstream in client system. The cryptographic methods used in these steps
 * are asymmetric such as RSA, DH etc. and symmetric such as AES CTR, CBC etc.
 * encryption mechanisms. The authentication of client system, delivery of
 * bitstream and cryptographic assets to client system is performed using
 * asymmetric cryptographic mechanism while bitstream is encrypted and processed
 * using symmetric cryptographic. In DRM world, authentication of client system,
 * delivery of bitstream and required cryptographic assets to client system is
 * loosely called provisioning and license acquisition while the processing of
 * cryptographically secure bitstream is divided as video decryption/decoding,
 * audio decryption/playback, video display. Besides DRM standards, Video/Audio
 * bitstream encryption standard such as
 * [Common Encryption Standard(CENC)](https://www.iso.org/standard/76597.html)
 * provides a mechanism to normalize bitstream encryption methods across vendors
 * while providing flexibility.
 *
 * \section DRM Pipeline
 * Most DRM standards execute the following deep pipeline to playback
 * contents on client systems from streaming servers - provisioning uses
 * provisioning servers, licence aquisition uses license servers, video
 * bitstream delivery uses content servers and decryption/decoding, audio
 * bitstream delivery uses content servers and decyption/playback,
 * display/playback. The system level HWDRM sequence diagram is following -
 * ![HWDRM sequence diagram](https://user-images.githubusercontent.com/75039699/102427278-df284e80-3fc5-11eb-9a3e-129b5f6b567a.png)
 * and HWDRM pipeline view is following -
 * ![HWDRM pipeline view](https://user-images.githubusercontent.com/75039699/102427357-04b55800-3fc6-11eb-8b8c-f34fc44ec061.png)
 *
 * \section LibVA Protected Content APIs
 * The LibVA Protected APIs are designed to enable DRM capabilities or
 * facilitate isolated communicaiton with TEE.
 * The VAEntrypointProtectedTEEComm is to define interfaces for Application
 * to TEE direct communication to perform various TEE centric operations
 * such as standalone provisioning of platform at factory or provisioning
 * TEE for other usages, providing TEE capabilities etc.
 * The VAEntrypointProtectedContent is to define interfaces for protected
 * video playback using HWDRM. This entry point co-ordinates assets across
 * TEE/GPU/Display for HWDRM playback.
 *
 * The difference between Protected Content and Protected TEE Communication
 * is that Protected Content Entrypoint does not provide isolated entry
 * point for TEE and invokes TEE only from HWDRM perspective.
 *
 * Protected Content Entrypoint
 * The most of DRM standards execute following deep pipeline to playback
 * contents on client systems from streaming servers - provisioning uses
 * provisioning servers, licence aquisition uses license servers, video
 * bitstream delivery uses content servers and decryption/decoding, audio
 * bitstream delivery uses content servers and decyption/playback,
 * display/playback.
 *
 * The Provisioning and License aquisition implementations are Independent
 * Hardware Vendor (IHV) specific but most IHVs use some form of Trusted
 * Execution Environment (TEE) to prepare client platform or system for DRM
 * content playback. The provisioning operations use provisioning servers (as
 * instructed in DRM standard) and client system TEE. The communication between
 * provisioning servers and client system TEE uses asymmetic cryptographic
 * mechanism. This step provides a way to establish root-of-trust between
 * client system and streaming servers. Once root-of-trust is established then
 * client system requests for license aquisition for a particular streaming
 * title. The license aquisition involves communication between licensing
 * servers and TEE using asymmetic cryptographic mechanism. At end of this step,
 * client system TEE has required assets to decrypt/decode. Although these
 * communication does not direcly involve video aspect of GPU but **facilitate
 * GPU required assets to playback premium contents**.
 *
 * To support DRM standard requirements in playback pipeline, OSes and HWs
 * incorporate various methods to protect full playback pipeline. These
 * methods of protection could be SW based or HW based. The SW based protection
 * mechanism of DRMs is called SWDRM while HW based protection mechanism is
 * called HWDRM. There is no previous support in LibVA to support either DRM
 * mechanism.
 *
 * For DRM capabilities, APIs inolve creation of protected session to
 * communicate with TEE and then using these protected sessions to process
 * video/audio data. The philophashy behind these API is to leverage existing
 * LibVA infrastructure as much as possible.
 *
 * Note: TEE could be any secure HW device such as ME-FW or FPGA Secure
 * Enclave or NPU Secure Enclave. There are 2 concepts here – TEE Type such
 * as ME-FW or FPGA or NPU; TEE Type Client such as for AMT or HDCP or
 * something else etc.
 *
 * \section description Detailed Description
 * The Protected content API provides a general mechanism for opening
 * protected session with TEE and if required then \ref priming GPU/Display.
 * The behavior of protected session API depends on parameterization/
 * configuration of protected session. Just for TEE tasks, protected
 * session is parameterized/configured as TEE Communication while for
 * HWDRM, protected session is parameterized/confgured as Protected
 * Content.
 *
 * TEE Communication Entrypoint
 * With TEE Communication parameterization/configuration, client
 * executes TEE workloads in TEE with TEE Communication protected
 * session.
 *
 * Protected Content Entrypoint
 * With Protected Content parameterization/configuration, client
 * executes HWDRM playback workloads HW accelerating protected video
 * content decryption/decoding with protected content session.
 *
 * Before calling vaCreateProtectedSession, VAConfigID is obtained using
 * existing libva mechanism to determine configuration parameters of
 * protected session. The VAConfigID is determined in this way so that
 * Protected Session implementation aligns with existing libva implementation.
 * After obtaining VAConfigID, Protected Session needs to be created but
 * note this is a session and not a context. Refer VAProtectedSessionID
 * for more details.
 *
 * Note:- Protected session represents session object that has all security
 * information needed for Secure Enclave to operate certain operations.
 *
 * \subsection priming Priming
 * Priming is used to refer various types of initializations. For example,
 * if license acquisition is being performed then priming means that TEE is
 * already provisioned aka TEE has some sort of "cryptographic" whitelist of
 * servers that TEE will use to do license acquisition for video playback. If
 * HWDRM video playback is being performed then priming means that HWDRM
 * eco-system TEE/GPU/Display has proper keys to do proper video playback etc.
 *
 * Protected content API uses the following paradigm for protected content
 * session:
 * - \ref api_pc_caps
 * - \ref api_pc_setup
 * - \ref api_pc_exec
 * - \ref api_pc_attach
 *
 * \subsection api_pc_caps Query for supported cipher mode, block size, mode
 *
 * Checking whether protected content is supported can be performed with
 * vaQueryConfigEntrypoints() and the profile argument set to
 * #VAProfileProtected. If protected content is supported, then the list of
 * returned entry-points will include #VAEntrypointProtectedContent
 *
 * \code
 * VAEntrypoint *entrypoints;
 * int i, num_entrypoints, supportsProtectedContent = 0;
 *
 * num_entrypoints = vaMaxNumEntrypoints();
 * entrypoints = malloc(num_entrypoints * sizeof(entrypoints[0]);
 * vaQueryConfigEntrypoints(va_dpy, VAProfileProtected, entrypoints,
 *     &num_entrypoints);
 *
 * for (i = 0; !supportsProtectedContent && i < num_entrypoints; i++) {
 *     if (entrypoints[i] == VAEntrypointProtectedContent)
 *         supportsProtectedContent = 1;
 * }
 * \endcode
 *
 * Then, the vaGetConfigAttributes() function is used to query the protected
 * session capabilities.
 *
 * \code
 * VAConfigAttrib attribs;
 * attribs[0].type = VAConfigAttribProtectedContentCipherAlgorithm;
 * attribs[1].type = VAConfigAttribProtectedContentCipherBlockSize;
 * attribs[2].type = VAConfigAttribProtectedContentCipherMode;
 * attribs[3].type = VAConfigAttribProtectedContentCipherSampleType;
 * attribs[4].type = VAConfigAttribProtectedContentUsage;
 * vaGetConfigAttributes(va_dpy, VAProfileProtected,
 *     VAEntrypointProtectedContent, attribs, 5);
 * if ((attribs[1].value & VA_PC_CIPHER_AES) == 0) {
 *     // not find desired cipher algorithm
 *     assert(0);
 * }
 * if ((attribs[2].value & VA_PC_BLOCK_SIZE_128) == 0) {
 *     // not find desired block size
 *     assert(0);
 * }
 * if ((attribs[3].value & VA_PC_CIPHER_MODE_CBC) == 0) {
 *     // not find desired counter mode
 *     assert(0);
 * }
 * if ((attribs[4].value & VA_PC_SAMPLE_TYPE_SUBSAMPLE) == 0) {
 *     // not find desired sample type
 *     assert(0);
 * }
 * if ((attribs[5].value & VA_PC_USAGE_WIDEVINE) == 0) {
 *     // not find desired usage
 *     assert(0);
 * }
 * \endcode
 *
 * \subsection api_pc_setup Set up a protected content session
 *
 * TEE Communication Entrypoint
 * The protected content session provides a TEE session that is used to extract
 * TEE information. This information could be used to peform TEE operations.
 *
 * Protected Content Entrypoint
 * The protected content session can be attached to VA decode/encode/vp context
 * to do decryption/protection in the pipeline.
 * Before creating a protected content session, it needs to create a config
 * first via vaCreateConfig(). Then using this config id to create a protected
 * content session via vaCreateProtectedSession().
 *
 * The general control flow is demonstrated by the following pseudo-code:
 * \code
 * // Create config
 * VAConfigID config_id;
 *
 * attribs[0].value = VA_PC_CIPHER_AES;
 * attribs[1].value = VA_PC_BLOCK_SIZE_128;
 * attribs[2].value = VA_PC_CIPHER_MODE_CBC;
 * attribs[3].value = VA_PC_SAMPLE_TYPE_SUBSAMPLE;
 * attribs[4].value = VA_PC_USAGE_WIDEVINE;
 * va_status = vaCreateConfig(va_dpy, VAProfileProtected,
 *     VAEntrypointProtectedContent, attribs, 5, &config_id);
 * CHECK_VASTATUS(va_status, "vaCreateConfig");
 * \endcode
 *
 * Once the config is set up, we can create protected content session via
 vaCreateProtectedSession().
 * \code
 * // Create a protected session
 * VAProtectedSessionID crypto_session;
 *
 * va_status = vaCreateProtectedSession(va_dpy, config_id, &crypto_session);
 * CHECK_VASTATUS(va_status, "vaCreateProtectedSession");
 * \endcode
 *
 * \subsection api_pc_exec TEE communication via vaProtectedSessionExecute()
 *
 * TEE Communication Entrypoint
 * App needs to communicate with TEE to get TEE information or \ref priming
 * "prime" TEE with  information that will be utilized for future TEE
 * operations/tasks.
 *
 * Protected Content Entrypoint
 * Before starting decryption/encryption operation in GPU, app may need to
 * communicate  with TEE to get encrypted assets for \ref priming HWDRM pipeline
 * for decryption. App need to call vaProtectedSessionExecute() to get this
 * asset. The following pseudo-code demonstrates getting session assets via
 * vaProtectedSessionExecute() as an example.
 *
 * In this example, the vaCreateBuffer is called with exec_buffer mainly becasue TEE
 * Communication Entrypoint buffers are CPU bound and buffer size is small enough to
 * have extra copy operation without impacting performance.
 *
 * \code
 * uint32_t app_id = 0xFF;
 * VABufferID buffer;
 * VAProtectedSessionExecuteBuffer exec_buff = {0};
 *
 * exec_buff.function_id = GET_SESSION_ID;
 * exec_buff.input.data = nullptr;
 * exec_buff.input.data_size = 0;
 * exec_buff.output.data = &app_id;
 * exec_buff.output.max_data_size = sizeof(app_id);
 * va_status = vaCreateBuffer(
 *                 va_dpy,
 *                 crypto_session,
 *                 (VABufferType) VAProtectedSessionExecuteBufferType,
 *                 sizeof(exec_buff),
 *                 1,
 *                 &exec_buff,
 *                 &buffer);
 *
 * va_status = vaProtectedSessionExecute(va_dpy, crypto_session, buffer);
 *
 * vaDestroyBuffer(va_dpy, buffer);
 * \endcode
 *
 * \subsection api_pc_attach Attach/Detach protected content session to the VA
 * context which want to enable/disable decryption/protection
 *
 * Protected content session is attached to VA decode/encode/vp context to
 * enable protected decoding/encoding/video processing per frame or entire
 * stream. If protected session attached per frame then application has 2
 * options for decoding/encoding skip processing i.e. accomodating clear
 * frames - 1. Application could do detach after each frame is processed
 * to process clear frame 2. Application could remains attached to decode/
 * encode session but specify enryption byte length to 0.
 * The video processing does not has option #2 mainly because API does
 * not provide skip processing.
 *
 * \code
 * vaAttachProtectedSession(va_dpy, decode_ctx, crypto_session);
 * foreach (iteration) {
 *     vaBeginPicture(va_dpy, decode_ctx, surface);
 *     ...
 *     vaRenderPicture(va_dpy, decode_ctx, &buf_id1, 1);
 *     vaRenderPicture(va_dpy, decode_ctx, &buf_id2, 1);
 *     // Buffer holding encryption parameters, i.e. VAEncryptionParameterBufferType buffer
 *     vaRenderPicture(va_dpy, decode_ctx, &buf_id_enc_param, 1);
 *     ...
 *     vaEndPicture(va_dpy, decode_ctx);
 * }
 * vaDetachProtectedSession(va_dpy, decode_ctx);
 * \endcode
 *
 * or it could be frame-by-frame attaching/detaching as following:
 *
 * \code
 * foreach (iteration) {
 *     if (encrypted)
 *         vaAttachProtectedSession(va_dpy, decode_ctx, crypto_session);

 *     vaBeginPicture(va_dpy, decode_ctx, surface);
 *     ...
 *     vaRenderPicture(va_dpy, decode_ctx, &buf_id1, 1);
 *     vaRenderPicture(va_dpy, decode_ctx, &buf_id2, 1);
 *     // Buffer holding encryption parameters, i.e. VAEncryptionParameterBufferType buffer
 *     vaRenderPicture(va_dpy, decode_ctx, &buf_id_enc_param, 1);
 *     ...
 *     vaEndPicture(va_dpy, decode_ctx);
 *
 *     if (encrypted)
 *         vaDetachProtectedSession(va_dpy, decode_ctx);

 *     // check encrypted variable for next frame
 * }
 * \endcode
 */

/**
 * ProtectedSessions and Contexts
 *
 * According to #VAContextID, Context represents a "virtual" video decode,
 * encode or video processing pipeline. Surfaces are render targets for a given
 * context. The data in the surfaces are not accessible to the client except if
 * derived image is supported and the internal data format of the surface is
 * implementation specific. Application can create a video decode, encode or
 * processing context which represents a "virtualized" hardware device.
 *
 * Since Protected Session does not virtualize any HW device or build any
 * pipeline but rather accessorize existing virtualized HW device or pipeline
 * to operate in protected mode so we decided to create separate function.
 * Beside this, a virtualized HW device or pipeline could own several protected
 * sessions and operate in those protected modes without ever re-creating
 * virtualization of HW device or re-building HW pipeline (an unique protected
 * environment multiplexing capability in Intel HW).
 *
 * The returned protected_session represents a notion of Host and TEE clients
 * while representing protection status in GPU and Display.
 *
 * Both contexts and protected sessions are identified by unique IDs and its
 * implementation specific internals are kept opaque to the clients
 */
typedef VAGenericID VAProtectedSessionID;

/** \brief TEE Execucte Function ID. */
typedef enum _VA_TEE_EXEC_FUNCTION_ID {
    VA_TEE_EXECUTE_FUNCTION_ID_PASS_THROUGH = 0x00000001,
    VA_TEE_EXECUTE_FUNCTION_ID_GET_FIRMWARE_VERSION = 0x00000002,

} VA_TEE_EXECUTE_FUNCTION_ID;

/** \brief Input/Output buffer of VAProtectedSessionExecuteBuffer */
typedef struct _VAProtectedSessionBuffer {
    /*
     * This is used when this buffer refer to output buffer. The maximum size of
     * data that the driver can return in the output buffer. It is not used for
     * input buffer.
     */
    uint32_t max_data_size;
    /*
     * If it is used for input buffer, it is the size of the input data. If it is
     * used for output buffer, it is the returns size of the output data written
     * by the driver.
     */
    uint32_t data_size;
    /*
     * data pointer of this buffer
     */
    void *data;
    uint32_t va_reserved[VA_PADDING_LOW];
} VAProtectedSessionBuffer;

/** \brief Buffer for vaProtectedSessionExecute() */
typedef struct _VAProtectedSessionExecuteBuffer {
    /** \brief Specify the function to execute. It is IHV's implementation
     * specific */
    uint32_t function_id;
    /** \brief Input buffer */
    VAProtectedSessionBuffer input;
    /** \brief Output buffer */
    VAProtectedSessionBuffer output;
    /** \brief Return the result of this function. The status result is IHV's
     * implementation specific */
    uint32_t status;
    uint32_t va_reserved[VA_PADDING_LOW];
} VAProtectedSessionExecuteBuffer;

/**
 * \brief Create a protected session
 *
 * Create a protected session
 *
 * @param[in] dpy                   the VA display
 * @param[in] config_id             configuration for the protected session
 * @param[out] protected_session    created protected session id upon return
 */
VAStatus vaCreateProtectedSession(VADisplay dpy, VAConfigID config_id,
                                  VAProtectedSessionID *protected_session);

/**
 * \brief Destroy a protected session
 *
 * Destroy a protected session
 *
 * @param[in] dpy                   the VA display
 * @param[in] protected_session     protected session to be destroyed
 */
VAStatus vaDestroyProtectedSession(VADisplay dpy,
                                   VAProtectedSessionID protected_session);

/**
 * \brief Attach a protected content session to VA context
 *
 * Attach a protected content session to the context to enable
 * decryption/protection
 *
 * @param[in] dpy                   the VA display
 * @param[in] id                    the VA decode/encode/vp context
 * @param[in] protected_session     the protected session to attach
 */
VAStatus vaAttachProtectedSession(VADisplay dpy, VAGenericID id,
                                  VAProtectedSessionID protected_session);

/**
 * \brief  Detach the protected content session from the VA context
 *
 * Detach protected content session of the context to disable
 * decryption/protection
 *
 * @param[in] dpy                   the VA display
 * @param[in] id                    TEE client id to be detached
 */
VAStatus vaDetachProtectedSession(VADisplay dpy, VAGenericID id);

/**
 * \brief Execute provides a general mechanism for TEE client tasks execution.
 *
 * vaProtectedSessionExecute provides a mechanism for TEE clients to execute
 * specific tasks. The implementation may differ between IHVs.
 * This is a synchronous API.
 *
 * @param[in] dpy                   the VA display
 * @param[in] protected_session     the protected session
 * @param[in,out] buf_id            the VA buffer
 */
VAStatus vaProtectedSessionExecute(VADisplay dpy,
                                   VAProtectedSessionID protected_session,
                                   VABufferID buf_id);

/**@}*/

#ifdef __cplusplus
}
#endif

#endif /* VA_PROT_H */
