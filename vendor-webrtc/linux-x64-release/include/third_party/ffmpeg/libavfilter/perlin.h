/*
 * Perlin noise generator
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

/**
 * @file
 * Perlin Noise generator
 */

#ifndef AVFILTER_PERLIN_H
#define AVFILTER_PERLIN_H

#include <stdint.h>

enum FFPerlinRandomMode {
    FF_PERLIN_RANDOM_MODE_RANDOM,
    FF_PERLIN_RANDOM_MODE_KEN,
    FF_PERLIN_RANDOM_MODE_SEED,
    FF_PERLIN_RANDOM_MODE_NB
};

/**
 * Perlin generator context. This needs to be initialized with the
 * parameters used to generate the Perlin noise.
 */
typedef struct FFPerlin {
    /**
     * spatial repeat period, if negative it is ignored
     */
    double period;

    /**
     * total number of components making up the noise, each one with
     * doubled frequency
     */
    int octaves;

    /**
     * ratio used to compute the amplitude of the next octave
     * component with respect to the previous component
     */
    double persistence;

    /**
     * permutations array used to compute the Perlin noise hash
     */
    uint8_t permutations[512];

    /**
     * define how to compute the permutations array
     */
    enum FFPerlinRandomMode random_mode;

    /**
     * when random_mode is set FF_PERLIN_RANDOM_MODE_RANDOM, set random
     * seed used to compute the permutations array
     */
    unsigned int random_seed;
} FFPerlin;

/**
 * Initialize the Perlin noise generator with parameters.
 *
 * @param perlin Perlin noise generator context
 * @param period spatial repeat period, if negative it is ignored
 * @param octaves total number of components making up the noise, each one with doubled frequency
 * @param persistence define ratio used to compute the amplitude of the next octave
 *                    component with respect to the previous component
 * @param random_mode define how to compute the permutations array
 * @param random_seed when random_mode is set to FF_PERLIN_RANDOM_MODE_RANDOM, set random
 *                    seed used to compute the permutations array
 * @return a negative AVERROR code in case of error, a non negative value otherwise
 */
int ff_perlin_init(FFPerlin *perlin, double period, int octaves, double persistence,
                   enum FFPerlinRandomMode random_mode, unsigned int random_seed);

/**
 * Compute Perlin noise given the x, y, z coordinates.
 *
 * @param perlin Perlin noise generator context
 * @return normalized value for the perlin noise, in the range [0, 1]
 */
double ff_perlin_get(FFPerlin *perlin, double x, double y, double z);

#endif  /* AVFILTER_PERLIN_H */
