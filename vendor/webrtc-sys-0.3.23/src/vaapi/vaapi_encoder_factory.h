

 #ifndef VAAPI_VIDEO_ENCODER_FACTORY_H_
 #define VAAPI_VIDEO_ENCODER_FACTORY_H_
 
 #include <vector>
 
 #include "api/environment/environment.h"
 #include "api/video_codecs/sdp_video_format.h"
 #include "api/video_codecs/video_encoder_factory.h"
 
 namespace webrtc {

 class VAAPIVideoEncoderFactory : public VideoEncoderFactory {
  public:
  VAAPIVideoEncoderFactory();
   ~VAAPIVideoEncoderFactory() override;

   static bool IsSupported();
 
   std::unique_ptr<VideoEncoder> Create(const Environment& env,
                                        const SdpVideoFormat& format) override;
 
   // Returns a list of supported codecs in order of preference.
   std::vector<SdpVideoFormat> GetSupportedFormats() const override;
 
   std::vector<SdpVideoFormat> GetImplementations() const override;
 
   std::unique_ptr<EncoderSelectorInterface> GetEncoderSelector() const override {
    return nullptr;
   }
 
  private:
   std::vector<SdpVideoFormat> supported_formats_;
 };
 
 }  // namespace webrtc
 
 #endif  // VAAPI_VIDEO_ENCODER_FACTORY_H_
 