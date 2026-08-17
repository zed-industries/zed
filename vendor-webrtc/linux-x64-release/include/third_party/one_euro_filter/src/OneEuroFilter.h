/* -*- coding: utf-8 -*-
 *
 * OneEuroFilter.h -
 *
 * Authors: 
 * Nicolas Roussel (nicolas.roussel@inria.fr)
 * Géry Casiez https://gery.casiez.net
 *
 * Copyright 2019 Inria
 * 
 * BSD License https://opensource.org/licenses/BSD-3-Clause
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *  1. Redistributions of source code must retain the above copyright notice, this list of conditions
 * and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions
 * and the following disclaimer in the documentation and/or other materials provided with the distribution.
 * 
 * 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or
 * promote products derived from this software without specific prior written permission.

 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, 
 * INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN 
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 */

#include <iostream>
#include <stdexcept>
#include <cmath>
#include <ctime>

// -----------------------------------------------------------------
// Utilities

typedef double TimeStamp ; // in seconds

static const TimeStamp UndefinedTime = -1.0 ;

// -----------------------------------------------------------------

class LowPassFilter {
    
  double y, a, s ;
  bool initialized ;

  void setAlpha(double alpha) ;

public:

  LowPassFilter(double alpha, double initval=0.0) ;

  double filter(double value) ;

  double filterWithAlpha(double value, double alpha) ;

  bool hasLastRawValue(void) ;

  double lastRawValue(void) ;

  double lastFilteredValue(void) ;

} ;

// -----------------------------------------------------------------

class OneEuroFilter {

  double freq ;
  double mincutoff ;
  double beta_ ;
  double dcutoff ;
  LowPassFilter *x ;
  LowPassFilter *dx ;
  TimeStamp lasttime ;

  double alpha(double cutoff) ;

public:

  /**
   * @brief Creates the filter and set its parameters
   * @param freq An estimate of the frequency in Hz of the signal (> 0), if timestamps are not available.
   * @param mincutoff Min cutoff frequency in Hz (> 0). Lower values allow to remove more jitter.
   * @param beta_ Parameter to reduce latency (> 0).
   * @param dcutoff Used to filter the derivates. 1 Hz by default. Change this parameter if you know what you are doing.
   */
  OneEuroFilter(double freq, 
		double mincutoff=1.0, double beta_=0.0, double dcutoff=1.0) ;

  /**
   * @brief Filter the noisy signal
   * @param value Noisy value to filter
   * @param timestamp (optional) timestamp in seconds
   * @return The filtered value
   */
  double filter(double value, TimeStamp timestamp=UndefinedTime) ;

  /**
   * @brief Sets the frequency of the signal
   * @param f An estimate of the frequency in Hz of the signal (> 0), if timestamps are not available.
   */
  void setFrequency(double f) ;

  /**
   * @brief Sets the filter min cutoff frequency
   * @param mc Min cutoff frequency in Hz (> 0). Lower values allow to remove more jitter.
   */ 
  void setMinCutoff(double mc) ;

  /**
   * @brief Sets the Beta parameter
   * @param b Parameter to reduce latency (> 0).
   */ 
  void setBeta(double b) ;

  /**
   * @brief Sets the Cutoff frequency for derivates
   * @param dc Used to filter the derivates. 1 Hz by default. Change this parameter if you know what you are doing.
   */ 
  void setDerivateCutoff(double dc) ;

  ~OneEuroFilter(void) ;

} ;

// -----------------------------------------------------------------

