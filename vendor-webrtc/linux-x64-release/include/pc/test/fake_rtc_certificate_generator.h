/*
 *  Copyright 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_TEST_FAKE_RTC_CERTIFICATE_GENERATOR_H_
#define PC_TEST_FAKE_RTC_CERTIFICATE_GENERATOR_H_

#include <cstdint>
#include <optional>
#include <string>
#include <utility>

#include "api/scoped_refptr.h"
#include "api/task_queue/task_queue_base.h"
#include "api/units/time_delta.h"
#include "rtc_base/checks.h"
#include "rtc_base/rtc_certificate.h"
#include "rtc_base/rtc_certificate_generator.h"
#include "rtc_base/ssl_identity.h"

// RSA with mod size 1024, pub exp 0x10001.
static const webrtc::RTCCertificatePEM kRsaPems[] = {
    webrtc::RTCCertificatePEM(
        "-----BEGIN RSA PRI"  // Linebreak to avoid detection of private
        "VATE KEY-----\n"     // keys by linters.
        "MIICdwIBADANBgkqhkiG9w0BAQEFAASCAmEwggJdAgEAAoGBAMYRkbhmI7kVA/rM\n"
        "czsZ+6JDhDvnkF+vn6yCAGuRPV03zuRqZtDy4N4to7PZu9PjqrRl7nDMXrG3YG9y\n"
        "rlIAZ72KjcKKFAJxQyAKLCIdawKRyp8RdK3LEySWEZb0AV58IadqPZDTNHHRX8dz\n"
        "5aTSMsbbkZ+C/OzTnbiMqLL/vg6jAgMBAAECgYAvgOs4FJcgvp+TuREx7YtiYVsH\n"
        "mwQPTum2z/8VzWGwR8BBHBvIpVe1MbD/Y4seyI2aco/7UaisatSgJhsU46/9Y4fq\n"
        "2TwXH9QANf4at4d9n/R6rzwpAJOpgwZgKvdQjkfrKTtgLV+/dawvpxUYkRH4JZM1\n"
        "CVGukMfKNrSVH4Ap4QJBAOJmGV1ASPnB4r4nc99at7JuIJmd7fmuVUwUgYi4XgaR\n"
        "WhScBsgYwZ/JoywdyZJgnbcrTDuVcWG56B3vXbhdpMsCQQDf9zeJrjnPZ3Cqm79y\n"
        "kdqANep0uwZciiNiWxsQrCHztywOvbFhdp8iYVFG9EK8DMY41Y5TxUwsHD+67zao\n"
        "ZNqJAkEA1suLUP/GvL8IwuRneQd2tWDqqRQ/Td3qq03hP7e77XtF/buya3Ghclo5\n"
        "54czUR89QyVfJEC6278nzA7n2h1uVQJAcG6mztNL6ja/dKZjYZye2CY44QjSlLo0\n"
        "MTgTSjdfg/28fFn2Jjtqf9Pi/X+50LWI/RcYMC2no606wRk9kyOuIQJBAK6VSAim\n"
        "1pOEjsYQn0X5KEIrz1G3bfCbB848Ime3U2/FWlCHMr6ch8kCZ5d1WUeJD3LbwMNG\n"
        "UCXiYxSsu20QNVw=\n"
        "-----END RSA PRIVATE KEY-----\n",
        "-----BEGIN CERTIFICATE-----\n"
        "MIIBmTCCAQKgAwIBAgIEbzBSAjANBgkqhkiG9w0BAQsFADARMQ8wDQYDVQQDEwZX\n"
        "ZWJSVEMwHhcNMTQwMTAyMTgyNDQ3WhcNMTQwMjAxMTgyNDQ3WjARMQ8wDQYDVQQD\n"
        "EwZXZWJSVEMwgZ8wDQYJKoZIhvcNAQEBBQADgY0AMIGJAoGBAMYRkbhmI7kVA/rM\n"
        "czsZ+6JDhDvnkF+vn6yCAGuRPV03zuRqZtDy4N4to7PZu9PjqrRl7nDMXrG3YG9y\n"
        "rlIAZ72KjcKKFAJxQyAKLCIdawKRyp8RdK3LEySWEZb0AV58IadqPZDTNHHRX8dz\n"
        "5aTSMsbbkZ+C/OzTnbiMqLL/vg6jAgMBAAEwDQYJKoZIhvcNAQELBQADgYEAUflI\n"
        "VUe5Krqf5RVa5C3u/UTAOAUJBiDS3VANTCLBxjuMsvqOG0WvaYWP3HYPgrz0jXK2\n"
        "LJE/mGw3MyFHEqi81jh95J+ypl6xKW6Rm8jKLR87gUvCaVYn/Z4/P3AqcQTB7wOv\n"
        "UD0A8qfhfDM+LK6rPAnCsVN0NRDY3jvd6rzix9M=\n"
        "-----END CERTIFICATE-----\n"),
    webrtc::RTCCertificatePEM(
        "-----BEGIN RSA PRI"  // Linebreak to avoid detection of private
        "VATE KEY-----\n"     // keys by linters.
        "MIICXQIBAAKBgQDeYqlyJ1wuiMsi905e3X81/WA/G3ym50PIDZBVtSwZi7JVQPgj\n"
        "Bl8CPZMvDh9EwB4Ji9ytA8dZZbQ4WbJWPr73zPpJSCvQqz6sOXSlenBRi72acNaQ\n"
        "sOR/qPvviJx5I6Hqo4qemfnjZhAW85a5BpgrAwKgMLIQTHCTLWwVSyrDrwIDAQAB\n"
        "AoGARni9eY8/hv+SX+I+05EdXt6MQXNUbQ+cSykBNCfVccLzIFEWUQMT2IHqwl6X\n"
        "ShIXcq7/n1QzOAEiuzixauM3YHg4xZ1Um2Ha9a7ig5Xg4v6b43bmMkNE6LkoAtYs\n"
        "qnQdfMh442b1liDud6IMb1Qk0amt3fSrgRMc547TZQVx4QECQQDxUeDm94r3p4ng\n"
        "5rCLLC1K5/6HSTZsh7jatKPlz7GfP/IZlYV7iE5784/n0wRiCjZOS7hQRy/8m2Gp\n"
        "pf4aZq+DAkEA6+np4d36FYikydvUrupLT3FkdRHGn/v83qOll/VmeNh+L1xMZlIP\n"
        "tM26hAXCcQb7O5+J9y3cx2CAQsBS11ZXZQJAfGgTo76WG9p5UEJdXUInD2jOZPwv\n"
        "XIATolxh6kXKcijLLLlSmT7KB0inNYIpzkkpee+7U1d/u6B3FriGaSHq9QJBAM/J\n"
        "ICnDdLCgwNvWVraVQC3BpwSB2pswvCFwq7py94V60XFvbw80Ogc6qIv98qvQxVlX\n"
        "hJIEgA/PjEi+0ng94Q0CQQDm8XSDby35gmjO+6eRmJtAjtB7nguLvrPXM6CPXRmD\n"
        "sRoBocpHw6j9UdzZ6qYG0FkdXZghezXFY58ro2BYYRR3\n"
        "-----END RSA PRIVATE KEY-----\n",
        "-----BEGIN CERTIFICATE-----\n"
        "MIICWDCCAcGgAwIBAgIJALgDjxMbBOhbMA0GCSqGSIb3DQEBCwUAMEUxCzAJBgNV\n"
        "BAYTAkFVMRMwEQYDVQQIDApTb21lLVN0YXRlMSEwHwYDVQQKDBhJbnRlcm5ldCBX\n"
        "aWRnaXRzIFB0eSBMdGQwHhcNMTUxMTEzMjIzMjEzWhcNMTYxMTEyMjIzMjEzWjBF\n"
        "MQswCQYDVQQGEwJBVTETMBEGA1UECAwKU29tZS1TdGF0ZTEhMB8GA1UECgwYSW50\n"
        "ZXJuZXQgV2lkZ2l0cyBQdHkgTHRkMIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKB\n"
        "gQDeYqlyJ1wuiMsi905e3X81/WA/G3ym50PIDZBVtSwZi7JVQPgjBl8CPZMvDh9E\n"
        "wB4Ji9ytA8dZZbQ4WbJWPr73zPpJSCvQqz6sOXSlenBRi72acNaQsOR/qPvviJx5\n"
        "I6Hqo4qemfnjZhAW85a5BpgrAwKgMLIQTHCTLWwVSyrDrwIDAQABo1AwTjAdBgNV\n"
        "HQ4EFgQUx2tbJdlcSTCepn09UdYORXKuSTAwHwYDVR0jBBgwFoAUx2tbJdlcSTCe\n"
        "pn09UdYORXKuSTAwDAYDVR0TBAUwAwEB/zANBgkqhkiG9w0BAQsFAAOBgQAmp9Id\n"
        "E716gHMqeBG4S2FCgVFCr0a0ugkaneQAN/c2L9CbMemEN9W6jvucUIVOtYd90dDW\n"
        "lXuowWmT/JctPe3D2qt4yvYW3puECHk2tVQmrJOZiZiTRtWm6HxkmoUYHYp/DtaS\n"
        "1Xe29gSTnZtI5sQCrGMzk3SGRSSs7ejLKiVDBQ==\n"
        "-----END CERTIFICATE-----\n")};

// ECDSA with EC_NIST_P256.
// These PEM strings were created by generating an identity with
// `SSLIdentity::Create` and invoking `identity->PrivateKeyToPEMString()`,
// `identity->PublicKeyToPEMString()` and
// `identity->certificate().ToPEMString()`.
static const webrtc::RTCCertificatePEM kEcdsaPems[] = {
    webrtc::RTCCertificatePEM(
        "-----BEGIN PRI"   // Linebreak to avoid detection of private
        "VATE KEY-----\n"  // keys by linters.
        "MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQg+qaRsR5uHtqG689M\n"
        "A3PHSJNeVpyi5wUKCft62h0UWy+hRANCAAS5Mjc85q9fVq4ln+zOPlaEC/Rzj5Pb\n"
        "MVZtf1x/8k2KsbmyZoAMDX2yer/atEuXmItMe3yd6/DXnvboU//D3Lyt\n"
        "-----END PRIVATE KEY-----\n",
        "-----BEGIN CERTIFICATE-----\n"
        "MIIBFTCBu6ADAgECAgkA30tGY5XG7oowCgYIKoZIzj0EAwIwEDEOMAwGA1UEAwwF\n"
        "dGVzdDMwHhcNMTYwNTA5MDkxODA4WhcNMTYwNjA5MDkxODA4WjAQMQ4wDAYDVQQD\n"
        "DAV0ZXN0MzBZMBMGByqGSM49AgEGCCqGSM49AwEHA0IABLkyNzzmr19WriWf7M4+\n"
        "VoQL9HOPk9sxVm1/XH/yTYqxubJmgAwNfbJ6v9q0S5eYi0x7fJ3r8Nee9uhT/8Pc\n"
        "vK0wCgYIKoZIzj0EAwIDSQAwRgIhAIIc3+CqfkZ9lLwTj1PvUtt3KhnqF2kD0War\n"
        "cCoTBbCxAiEAyp9Cn4vo2ZBhRIVDKyoxmwak8Z0PAVhJAQaWCgoY2D4=\n"
        "-----END CERTIFICATE-----\n"),
    webrtc::RTCCertificatePEM(
        "-----BEGIN PRI"   // Linebreak to avoid detection of private
        "VATE KEY-----\n"  // keys by linters.
        "MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQghL/G4JRYnuDNbQuh\n"
        "LqkytcE39Alsq6FItDVFgOesfCmhRANCAATd53FjPLyVUcwYguEPbSJM03fP6Rx5\n"
        "GY1dEZ00+ZykjJI83VfDAyvmpRuGahNtBH0hc+7xkDCbeo6TM0tN35xr\n"
        "-----END PRIVATE KEY-----\n",
        "-----BEGIN CERTIFICATE-----\n"
        "MIIBFDCBu6ADAgECAgkArZYdXMyJ5rswCgYIKoZIzj0EAwIwEDEOMAwGA1UEAwwF\n"
        "dGVzdDQwHhcNMTYwNTA5MDkxODA4WhcNMTYwNjA5MDkxODA4WjAQMQ4wDAYDVQQD\n"
        "DAV0ZXN0NDBZMBMGByqGSM49AgEGCCqGSM49AwEHA0IABN3ncWM8vJVRzBiC4Q9t\n"
        "IkzTd8/pHHkZjV0RnTT5nKSMkjzdV8MDK+alG4ZqE20EfSFz7vGQMJt6jpMzS03f\n"
        "nGswCgYIKoZIzj0EAwIDSAAwRQIgb/LBc8OtsC5lEDyjCP6M9xt5mwzUNrQBOFWZ\n"
        "1fE/g68CIQD7uoFfbiq6dTp8ZwzbwQ8jJf08KjriamqA9OW/4268Dw==\n"
        "-----END CERTIFICATE-----\n")};

class FakeRTCCertificateGenerator
    : public webrtc::RTCCertificateGeneratorInterface {
 public:
  FakeRTCCertificateGenerator() : should_fail_(false), should_wait_(false) {}

  void set_should_fail(bool should_fail) { should_fail_ = should_fail; }

  // If set to true, stalls the generation of the fake certificate until it is
  // set to false.
  void set_should_wait(bool should_wait) { should_wait_ = should_wait; }

  void use_original_key() { key_index_ = 0; }
  void use_alternate_key() { key_index_ = 1; }

  int generated_certificates() { return generated_certificates_; }
  int generated_failures() { return generated_failures_; }

  void GenerateCertificateAsync(const webrtc::KeyParams& key_params,
                                const std::optional<uint64_t>& expires_ms,
                                Callback callback) override {
    // The certificates are created from constant PEM strings and use its coded
    // expiration time, we do not support modifying it.
    RTC_DCHECK(!expires_ms);

    // Only supports RSA-1024-0x10001 and ECDSA-P256.
    if (key_params.type() == webrtc::KT_RSA) {
      RTC_DCHECK_EQ(key_params.rsa_params().mod_size, 1024);
      RTC_DCHECK_EQ(key_params.rsa_params().pub_exp, 0x10001);
    } else {
      RTC_DCHECK_EQ(key_params.type(), webrtc::KT_ECDSA);
      RTC_DCHECK_EQ(key_params.ec_curve(), webrtc::EC_NIST_P256);
    }
    webrtc::KeyType key_type = key_params.type();
    webrtc::TaskQueueBase::Current()->PostTask(
        [this, key_type, callback = std::move(callback)]() mutable {
          GenerateCertificate(key_type, std::move(callback));
        });
  }

  static webrtc::scoped_refptr<webrtc::RTCCertificate> GenerateCertificate() {
    switch (webrtc::KT_DEFAULT) {
      case webrtc::KT_RSA:
        return webrtc::RTCCertificate::FromPEM(kRsaPems[0]);
      case webrtc::KT_ECDSA:
        return webrtc::RTCCertificate::FromPEM(kEcdsaPems[0]);
      default:
        RTC_DCHECK_NOTREACHED();
        return nullptr;
    }
  }

 private:
  const webrtc::RTCCertificatePEM& get_pem(
      const webrtc::KeyType& key_type) const {
    switch (key_type) {
      case webrtc::KT_RSA:
        return kRsaPems[key_index_];
      case webrtc::KT_ECDSA:
        return kEcdsaPems[key_index_];
      default:
        RTC_DCHECK_NOTREACHED();
        return kEcdsaPems[key_index_];
    }
  }
  const std::string& get_key(const webrtc::KeyType& key_type) const {
    return get_pem(key_type).private_key();
  }
  const std::string& get_cert(const webrtc::KeyType& key_type) const {
    return get_pem(key_type).certificate();
  }

  void GenerateCertificate(webrtc::KeyType key_type, Callback callback) {
    // If the certificate generation should be stalled, re-post this same
    // message to the queue with a small delay so as to wait in a loop until
    // set_should_wait(false) is called.
    if (should_wait_) {
      webrtc::TaskQueueBase::Current()->PostDelayedTask(
          [this, key_type, callback = std::move(callback)]() mutable {
            GenerateCertificate(key_type, std::move(callback));
          },
          webrtc::TimeDelta::Millis(1));
      return;
    }
    if (should_fail_) {
      ++generated_failures_;
      std::move(callback)(nullptr);
    } else {
      webrtc::scoped_refptr<webrtc::RTCCertificate> certificate =
          webrtc::RTCCertificate::FromPEM(get_pem(key_type));
      RTC_DCHECK(certificate);
      ++generated_certificates_;
      std::move(callback)(std::move(certificate));
    }
  }

  bool should_fail_;
  bool should_wait_;
  int key_index_ = 0;
  int generated_certificates_ = 0;
  int generated_failures_ = 0;
};

#endif  // PC_TEST_FAKE_RTC_CERTIFICATE_GENERATOR_H_
