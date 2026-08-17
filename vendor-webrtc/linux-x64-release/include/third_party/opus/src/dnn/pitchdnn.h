#ifndef PITCHDNN_H
#define PITCHDNN_H


typedef struct PitchDNN PitchDNN;

#include "pitchdnn_data.h"

#define PITCH_MIN_PERIOD 32
#define PITCH_MAX_PERIOD 256

#define NB_XCORR_FEATURES (PITCH_MAX_PERIOD-PITCH_MIN_PERIOD)


typedef struct {
  PitchDNN model;
  float gru_state[GRU_1_STATE_SIZE];
  float xcorr_mem1[(NB_XCORR_FEATURES + 2)*2];
  float xcorr_mem2[(NB_XCORR_FEATURES + 2)*2*8];
  float xcorr_mem3[(NB_XCORR_FEATURES + 2)*2*8];
} PitchDNNState;


void pitchdnn_init(PitchDNNState *st);
int pitchdnn_load_model(PitchDNNState *st, const void *data, int len);

float compute_pitchdnn(
    PitchDNNState *st,
    const float *if_features,
    const float *xcorr_features,
    int arch
    );

#endif
