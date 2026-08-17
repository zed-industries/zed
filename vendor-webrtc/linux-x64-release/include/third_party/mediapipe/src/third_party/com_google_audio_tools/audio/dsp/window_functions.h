/*
 * Copyright 2020 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// A few window functions for filter design and spectral analysis.
//
// Cosine, Hamming, Hann, Kaiser, Nuttall, Planck-taper, and quartic polynomial
// windows are implemented as classes with common base class WindowFunction.
// Three use cases are supported:
//
// 1. Generating an N-point symmetric window for filter design is done with
//    GetSymmetricSamples(). Corresponds to Matlab's default 'symmetric' option.
//    Examples, where window is a vector<double>:
//      HannWindow().GetSymmetricSamples(N, &window);
//      KaiserWindow(beta).GetSymmetricSamples(N, &window);
//      PlanckTaperWindow(epsilon).GetSymmetricSamples(N, &window);
//
// 2. Generating an N-point window for spectral analysis, where the right
//    endpoint is excluded (a "periodic" or "DTF-even" window). Corresponds to
//    Matlab's 'periodic' option.
//    Example:
//      HannWindow().GetPeriodicSamples(N, &window);
//
// 3. Evaluating a window as a function of a continuous variable is done with
//    Eval(). "window_function.Eval(x)" evaluates the window function at x,
//    where x = 0 corresponds to the center of the window and the nonzero
//    support is in [-radius, radius]. The radius of the window can be specified
//    as the first argument in construction (default radius is 1.0).
//    Example:
//      KaiserWindow window(radius, beta);
//      double value = window.Eval(x);
//
// An arbitrary window can be represented with a base class WindowFunction
// reference, for example
//   void SpectralAnalysis(const WindowFunction& window, /* other args */);
//   // Call with Nuttall window.
//   SpectralAnalysis(NuttallWindow(), ...);
//
// Plots and more information about these window functions can be found at
// https://en.wikipedia.org/wiki/Window_function

#ifndef AUDIO_DSP_WINDOW_FUNCTIONS_H_
#define AUDIO_DSP_WINDOW_FUNCTIONS_H_

#include <string>
#include <vector>

#include "audio/dsp/porting.h"  // auto-added.


namespace audio_dsp {

// Base class for window functions. Derived classes implement the Eval(),
// zero_at_endpoints(), and name() methods and optionally the
// EvalFourierTransform() method.
class WindowFunction {
 public:
  explicit WindowFunction(double radius = 1.0);
  virtual ~WindowFunction() = default;

  // Sample symmetric window samples, suitable for filter design. The resulting
  // num_samples samples are symmetric. For an odd num_samples, the center
  // sample has the maximum window value of 1.
  //
  // The samples are uniformly spaced over [-radius, radius]. However, if the
  // window is zero at the endpoints, sampling is done as if (num_samples + 2)
  // were requested and then the first and last samples discarded. This
  // procedure obtains num_samples window samples having overall slightly better
  // spectral characteristics.
  //
  // The function is overloaded such that the result may be either vector<float>
  // or vector<double>.
  void GetSymmetricSamples(int num_samples, std::vector<float>* samples) const;
  void GetSymmetricSamples(int num_samples, std::vector<double>* samples) const;

  // Sample periodizing window samples, suitable for spectral analysis. The
  // sampling is done as if (num_samples + 1) samples were uniformly spaced
  // over [-radius, radius] and the then the last sample discarded. This
  // sampling corresponds to Matlab's 'periodic' option, as opposed to
  // 'symmetric'. This is preferred for 50%-overlap spectrograms.
  void GetPeriodicSamples(int num_samples, std::vector<float>* samples) const;
  void GetPeriodicSamples(int num_samples, std::vector<double>* samples) const;

  // The radius parameter scales the nonzero support of the window function; it
  // is nonzero in the interval [-radius, radius]. Note that the radius does not
  // affect the result of GetSymmetricSamples() or GetPeriodicSamples().
  double radius() const { return radius_; }
  void set_radius(double radius) { radius_ = radius; }

  // Evaluate the window function at x, where x = 0 is the center of the window
  // and the nonzero support is in |x| <= radius. All windows satisfy
  // Eval(0) == 1.0.
  virtual double Eval(double x) const = 0;

  // Evaluate the Fourier transform of the window function at frequency f.
  // Assuming radius is in units of seconds, f is in Hz. Since window functions
  // are symmetric, their Fourier transform are real valued.
  virtual double EvalFourierTransform(double f) const;

  // Returns true if the window function is zero at the endpoints |x| = radius
  // (which is the case for cosine, Hann, and Planck-taper windows, but not for
  // the Hamming, Kaiser, and Nuttall windows).
  virtual bool zero_at_endpoints() const = 0;

  // Get the window name as a string.
  virtual std::string name() const = 0;

  struct SpectralProperties {
    // Full width at half maximum of main lobe. Assuming radius is in units of
    // seconds, main_lobe_fwhm is in Hz.
    double main_lobe_fwhm;
    // Concentration of energy in the main lobe, the ratio
    // (main lobe energy) / (total energy).
    double main_lobe_energy_ratio;
    // Height of the highest sidelobe in dB relative to DC.
    double highest_sidelobe_db;
  };
  SpectralProperties ComputeSpectralProperties() const;

 private:
  void MemoizeSamples() const;

  double radius_;
  // Memoized samples used for generic implementation of EvalFourierTransform().
  mutable std::vector<double> memoized_samples_;
};

// RectangularWindow implements the rectangular window (aka Dirichlet window)
//   w(x) = 1 for |x| <= radius and 0 otherwise.
// This is a crude and simple window, but included here for completeness.
//
// Spectral properties:
// Main lobe FWHM = 0.44 / radius
// Main lobe energy ratio = 0.9028
// Highest sidelobe = -13.26dB
class RectangularWindow: public WindowFunction {
 public:
  explicit RectangularWindow(double radius = 1.0): WindowFunction(radius) {}

  double Eval(double x) const override;
  double EvalFourierTransform(double f) const override;
  bool zero_at_endpoints() const override { return false; }
  std::string name() const override { return "rectangular"; }
};

// CosineWindow implements the cosine (aka sine) window
//   w(x) = cos((pi/2) x/radius) for |x| <= radius,
// or equivalently
//   w(y - radius) = sin((pi/2) y/radius) for 0 <= y <= 2 radius.
// The cosine window is a lightly tapered window with a narrow main lobe but
// limited sidelobe attenuation.
//
// Spectral properties:
// Main lobe FWHM = 0.59 / radius
// Main lobe energy ratio = 0.99495
// Highest sidelobe = -23.0dB
class CosineWindow: public WindowFunction {
 public:
  explicit CosineWindow(double radius = 1.0): WindowFunction(radius) {}

  double Eval(double x) const override;
  double EvalFourierTransform(double f) const override;
  bool zero_at_endpoints() const override { return true; }
  std::string name() const override { return "cosine"; }
};

// The cosine window is also the square root of the Hann window,
//   w(x) = sqrt(HannWindow(radius).Eval(x)).
using SqrtHannWindow = CosineWindow;

// HammingWindow implements the Hamming window
//   w(x) = 0.54 + 0.46 cos(pi x/radius) for |x| <= radius,
// or equivalently
//   w(y - radius) = 0.54 - 0.46 cos(pi y/radius) for 0 <= y <= 2 radius.
// The window is nonzero at the endpoints with value 0.08. The 0.54 and 0.46
// coefficients are selected to suppress the first side lobe.
//
// Spectral properties:
// Main lobe FWHM = 0.65 / radius
// Main lobe energy / total energy = 0.99963
// Highest sidelobe = -42.7dB
class HammingWindow: public WindowFunction {
 public:
  explicit HammingWindow(double radius = 1.0): WindowFunction(radius) {}

  double Eval(double x) const override;
  double EvalFourierTransform(double f) const override;
  bool zero_at_endpoints() const override { return false; }
  std::string name() const override { return "Hamming"; }

 private:
  static constexpr double kA[2] = {0.54, 0.46};
};

// HannWindow implements the Hann (aka Hanning or raised cosine) window
//   w(x) = 0.5 + 0.5 cos(pi x/radius) for |x| <= radius,
// or equivalently
//   w(y - radius) = 0.5 - 0.5 cos(pi y/radius) for 0 <= y <= 2 radius.
//
// Spectral properties:
// Main lobe FWHM = 0.72 / radius
// Main lobe energy / total energy = 0.99949
// Highest sidelobe = -31.5dB
class HannWindow: public WindowFunction {
 public:
  explicit HannWindow(double radius = 1.0): WindowFunction(radius) {}

  double Eval(double x) const override;
  double EvalFourierTransform(double f) const override;
  bool zero_at_endpoints() const override { return true; }
  std::string name() const override { return "Hann"; }
};

// KaiserWindow implements the Kaiser (aka Kaiser-Bessel) window
//   w(x) = I0(beta sqrt(1 - (x/radius)^2)) / I0(beta) for |x| <= radius,
// where I0(x) is the zeroth order modified Bessel function of the first kind
// and beta is a parameter [https://en.wikipedia.org/wiki/Kaiser_window]. The
// Kaiser window is a close approximation of the prolate spheroidal window,
// which is designed to maximize the concentration of energy in the main lobe.
// Larger beta implies stronger sidelobe attenuation but a wider main lobe. A
// typical value of beta is 4.0, for which the highest side lobe is -30dB.
//
// Spectral properties with beta = 2.0:
// Main lobe FWHM = 0.50 / radius
// Main lobe energy ratio = 0.97322
// Highest sidelobe = -18.4dB
//
// Spectral properties with beta = 4.0:
// Main lobe FWHM = 0.60 / radius
// Main lobe energy ratio = 0.99858
// Highest sidelobe = -30.0dB
//
// Spectral properties with beta = 8.0:
// Main lobe FWHM = 0.79 / radius
// Main lobe energy ratio = 0.999999
// Highest sidelobe = -58.7dB
class KaiserWindow: public WindowFunction {
 public:
  explicit KaiserWindow(double beta): KaiserWindow(1.0, beta) {}
  KaiserWindow(double radius, double beta);
  double beta() const { return beta_; }

  double Eval(double x) const override;
  double EvalFourierTransform(double f) const override;
  bool zero_at_endpoints() const override { return false; }
  std::string name() const override { return "Kaiser"; }

 private:
  double beta_;
  double denom_;
};

// NuttallWindow implements Nuttall's 4-term symmetric Blackman-Harris window
//   w(x) = sum_{k=0}^3 a[k] cos(k pi x/radius) for |x| <= radius,
// or equivalently
//   w(y - radius) = sum (-1)^k a[k] cos((pi/2) y/radius) for 0 <= y <= 2 radius
// with coefficients
//   a[0] = 0.3635819, a[1] = 0.4891775,  a[2] = 0.1365995, a[3] = 0.0106411.
// The window is strongly tapered and has strong sidelobe attenuation but a wide
// main lobe. The first side lobe is -93dB.
//
// Spectral properties:
// Main lobe FWHM = 0.94 / radius
// Main lobe energy ratio = 0.999999997
// Highest sidelobe = -98.2dB
class NuttallWindow: public WindowFunction {
 public:
  explicit NuttallWindow(double radius = 1.0): WindowFunction(radius) {}

  double Eval(double x) const override;
  double EvalFourierTransform(double f) const override;
  bool zero_at_endpoints() const override { return false; }
  std::string name() const override { return "Nuttall"; }

 private:
  static constexpr double kA[4] = {0.3635819, 0.4891775, 0.1365995, 0.0106411};
};

// PlanckTaperWindow implements the Planck-taper window
// [https://en.wikipedia.org/wiki/Window_function#Planck-taper_window]. The
// window function is exactly equal to 1 for |x| <= radius - epsilon and
// smoothly transitions to zero over (radius - epsilon) < |x| < radius, where
// epsilon is a parameter. The window is designed to be a C^infinity function
// so that its spectrum decays exponentially fast. Note, however, the first
// sidelobe attenuation is poor, around -12dB depending on epsilon.
//
// Spectral properties with epsilon = 0.15:
// Main lobe FWHM = 0.48 / radius
// Main lobe energy ratio = 0.916125
// Highest sidelobe = -13.3dB
class PlanckTaperWindow: public WindowFunction {
 public:
  explicit PlanckTaperWindow(double epsilon): PlanckTaperWindow(1.0, epsilon) {}
  PlanckTaperWindow(double radius, double epsilon);
  double epsilon() const { return epsilon_; }

  double Eval(double x) const override;
  bool zero_at_endpoints() const override { return true; }
  std::string name() const override { return "Planck-taper"; }

 private:
  double epsilon_;
};

// QuarticWindow implements a generic quartic (4th degree) polynomial window
//   w(x) = 1.0 + c_2 (x / radius)^2 + c_4 (x / radius)^4 for |x| <= radius,
// where coefficients c_2 and c_4 are configurable. To ensure a nonnegative
// unimodal window, the coefficients are required to satisfy
//   -2.0 <= c_2 <= 0.0,
//   -(1 + c_2) <= c_4 <= -c_2 / 2.
// The default coefficients are c_2 = -1.2, c_4 = 0.4.
//
// Spectral properties with c_2 = -1.2, c_4 = 0.4:
// Main lobe FWHM = 0.55 / radius
// Main lobe energy ratio = 0.99369
// Highest sidelobe = -24.0dB
class QuarticWindow: public WindowFunction {
 public:
  QuarticWindow(): QuarticWindow(1.0, -1.2, 0.4) {}
  QuarticWindow(double c_2, double c_4): QuarticWindow(1.0, c_2, c_4) {}
  QuarticWindow(double radius, double c_2, double c_4);
  double c_2() const { return c_2_; }
  double c_4() const { return c_4_; }

  double Eval(double x) const override;
  double EvalFourierTransform(double f) const override;
  bool zero_at_endpoints() const override { return false; }
  std::string name() const override { return "quartic"; }

 private:
  double c_2_;
  double c_4_;
};

}  // namespace audio_dsp

#endif  // AUDIO_DSP_WINDOW_FUNCTIONS_H_
