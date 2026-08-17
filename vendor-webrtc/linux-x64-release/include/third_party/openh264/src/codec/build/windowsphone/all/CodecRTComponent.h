#pragma once

namespace CodecRTComponent {
// public ref class WindowsPhoneRuntimeComponent  sealed
public ref class CodecRunTimeComponent sealed {
 public:
  CodecRunTimeComponent();
  int Encode();
  int Decode();

  //Get encoder info
  float GetEncFPS();
  double GetEncTime();
  int  GetEncodedFrameNum();

  //get decoder info
  float GetDecFPS();
  double GetDecTime();
  int  GetDecodedFrameNum();

};
}