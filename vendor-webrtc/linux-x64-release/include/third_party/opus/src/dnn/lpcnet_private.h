#ifndef LPCNET_PRIVATE_H
#define LPCNET_PRIVATE_H

#include <stdio.h>
#include "freq.h"
#include "lpcnet.h"
#include "plc_data.h"
#include "pitchdnn.h"
#include "fargan.h"


#define PITCH_FRAME_SIZE 320
#define PITCH_BUF_SIZE (PITCH_MAX_PERIOD+PITCH_FRAME_SIZE)

#define PLC_MAX_FEC 100
#define MAX_FEATURE_BUFFER_SIZE 4

#define PITCH_IF_MAX_FREQ 30
#define PITCH_IF_FEATURES (3*PITCH_IF_MAX_FREQ - 2)

#define CONT_VECTORS 5

#define FEATURES_DELAY 1

struct LPCNetEncState{
  PitchDNNState pitchdnn;
  float analysis_mem[OVERLAP_SIZE];
  float mem_preemph;
  kiss_fft_cpx prev_if[PITCH_IF_MAX_FREQ];
  float if_features[PITCH_IF_FEATURES];
  float xcorr_features[PITCH_MAX_PERIOD - PITCH_MIN_PERIOD];
  float dnn_pitch;
  float pitch_mem[LPC_ORDER];
  float pitch_filt;
  float exc_buf[PITCH_BUF_SIZE];
  float lp_buf[PITCH_BUF_SIZE];
  float lp_mem[4];
  float lpc[LPC_ORDER];
  float features[NB_TOTAL_FEATURES];
  float sig_mem[LPC_ORDER];
  float burg_cepstrum[2*NB_BANDS];
};

typedef struct {
  float gru1_state[PLC_GRU1_STATE_SIZE];
  float gru2_state[PLC_GRU2_STATE_SIZE];
} PLCNetState;

#define PLC_BUF_SIZE ((CONT_VECTORS+10)*FRAME_SIZE)
struct LPCNetPLCState {
  PLCModel model;
  FARGANState fargan;
  LPCNetEncState enc;
  int loaded;
  int arch;

#define LPCNET_PLC_RESET_START fec
  float fec[PLC_MAX_FEC][NB_FEATURES];
  int analysis_gap;
  int fec_read_pos;
  int fec_fill_pos;
  int fec_skip;
  int analysis_pos;
  int predict_pos;
  float pcm[PLC_BUF_SIZE];
  int blend;
  float features[NB_TOTAL_FEATURES];
  float cont_features[CONT_VECTORS*NB_FEATURES];
  int loss_count;
  PLCNetState plc_net;
  PLCNetState plc_bak[2];
};

void preemphasis(float *y, float *mem, const float *x, float coef, int N);

void compute_frame_features(LPCNetEncState *st, const float *in, int arch);

void lpcnet_reset_signal(LPCNetState *lpcnet);
void run_frame_network(LPCNetState *lpcnet, float *gru_a_condition, float *gru_b_condition, float *lpc, const float *features);
void run_frame_network_deferred(LPCNetState *lpcnet, const float *features);
void run_frame_network_flush(LPCNetState *lpcnet);


void lpcnet_synthesize_tail_impl(LPCNetState *lpcnet, opus_int16 *output, int N, int preload);
void lpcnet_synthesize_impl(LPCNetState *lpcnet, const float *features, opus_int16 *output, int N, int preload);
void lpcnet_synthesize_blend_impl(LPCNetState *lpcnet, const opus_int16 *pcm_in, opus_int16 *output, int N);

void run_frame_network(LPCNetState *lpcnet, float *gru_a_condition, float *gru_b_condition, float *lpc, const float *features);

#endif
