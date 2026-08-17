/*
 * Delay Locked Loop based time filter prototypes and declarations
 * Copyright (c) 2009 Samalyse
 * Copyright (c) 2009 Michael Niedermayer
 * Author: Olivier Guilyardi <olivier samalyse com>
 *         Michael Niedermayer <michaelni gmx at>
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef AVDEVICE_TIMEFILTER_H
#define AVDEVICE_TIMEFILTER_H

/**
 * Opaque type representing a time filter state
 *
 * The purpose of this filter is to provide a way to compute accurate time
 * stamps that can be compared to wall clock time, especially when dealing
 * with two clocks: the system clock and a hardware device clock, such as
 * a soundcard.
 */
typedef struct TimeFilter TimeFilter;


/**
 * Create a new Delay Locked Loop time filter
 *
 * Where bandwidth is up to you to choose. Smaller values will filter out more
 * of the jitter, but also take a longer time for the loop to settle. A good
 * starting point is something between 0.3 and 3 Hz.
 *
 * @param time_base   period of the hardware clock in seconds
 *                    (for example 1.0/44100)
 * @param period      expected update interval, in input units
 * @param brandwidth  filtering bandwidth, in Hz
 *
 * @return a pointer to a TimeFilter struct, or NULL on error
 */
TimeFilter * ff_timefilter_new(double time_base, double period, double bandwidth);

/**
 * Update the filter
 *
 * This function must be called in real time, at each process cycle.
 *
 * @param period the device cycle duration in clock_periods. For example, at
 * 44.1kHz and a buffer size of 512 frames, period = 512 when clock_period
 * was 1.0/44100, or 512/44100 if clock_period was 1.
 *
 * system_time, in seconds, should be the value of the system clock time,
 * at (or as close as possible to) the moment the device hardware interrupt
 * occurred (or any other event the device clock raises at the beginning of a
 * cycle).
 *
 * @return the filtered time, in seconds
 */
double ff_timefilter_update(TimeFilter *self, double system_time, double period);

/**
 * Evaluate the filter at a specified time
 *
 * @param delta  difference between the requested time and the current time
 *               (last call to ff_timefilter_update).
 * @return  the filtered time
 */
double ff_timefilter_eval(TimeFilter *self, double delta);

/**
 * Reset the filter
 *
 * This function should mainly be called in case of XRUN.
 *
 * Warning: after calling this, the filter is in an undetermined state until
 * the next call to ff_timefilter_update()
 */
void ff_timefilter_reset(TimeFilter *);

/**
 * Free all resources associated with the filter
 */
void ff_timefilter_destroy(TimeFilter *);

#endif /* AVDEVICE_TIMEFILTER_H */
