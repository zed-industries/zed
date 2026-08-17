/*
 * Copyright (c) 2018, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_TOOLS_TXFM_ANALYZER_TXFM_GRAPH_H_
#define AOM_TOOLS_TXFM_ANALYZER_TXFM_GRAPH_H_

struct Node {
  Node *inNode[2];
  int inNodeNum;
  int inNodeIdx[2];
  double inWeight[2];
  double value;
  int nodeIdx;
  int stageIdx;
  int visited;
};

#define STAGENUM (10)
#define NODENUM (32)
#define COS_MOD (128)

typedef enum {
  TYPE_DCT = 0,
  TYPE_ADST,
  TYPE_IDCT,
  TYPE_IADST,
  TYPE_LAST
} TYPE_TXFM;

TYPE_TXFM get_inv_type(TYPE_TXFM type);
void get_fun_name(char *str_fun_name, int str_buf_size, const TYPE_TXFM type,
                  const int txfm_size);

void get_txfm_type_name(char *str_fun_name, int str_buf_size,
                        const TYPE_TXFM type, const int txfm_size);
void get_hybrid_2d_type_name(char *buf, int buf_size, const TYPE_TXFM type0,
                             const TYPE_TXFM type1, const int txfm_size0,
                             const int txfm_size1);
unsigned int get_max_bit(unsigned int x);
unsigned int bitwise_reverse(unsigned int x, int max_bit);
int get_idx(int ri, int ci, int cSize);

int get_dct_stage_num(int size);
void reference_dct_1d(double *in, double *out, int size);
void reference_dct_2d(double *in, double *out, int size);
void connect_node(Node *node, int stage_num, int node_num, int stage_idx,
                  int node_idx, int in0, double w0, int in1, double w1);
void propagate(Node *node, int stage_num, int node_num, int stage);
void init_graph(Node *node, int stage_num, int node_num);
void graph_reset_visited(Node *node, int stage_num, int node_num);
void gen_B_graph(Node *node, int stage_num, int node_num, int stage_idx,
                 int node_idx, int N, int star);
void gen_P_graph(Node *node, int stage_num, int node_num, int stage_idx,
                 int node_idx, int N);

void gen_type1_graph(Node *node, int stage_num, int node_num, int stage_idx,
                     int node_idx, int N);
void gen_type2_graph(Node *node, int stage_num, int node_num, int stage_idx,
                     int node_idx, int N);
void gen_type3_graph(Node *node, int stage_num, int node_num, int stage_idx,
                     int node_idx, int idx, int N);
void gen_type4_graph(Node *node, int stage_num, int node_num, int stage_idx,
                     int node_idx, int idx, int N);

void gen_R_graph(Node *node, int stage_num, int node_num, int stage_idx,
                 int node_idx, int N);

void gen_DCT_graph(Node *node, int stage_num, int node_num, int stage_idx,
                   int node_idx, int N);

void gen_DCT_graph_1d(Node *node, int stage_num, int node_num, int stage_idx,
                      int node_idx, int dct_node_num);
void connect_layer_2d(Node *node, int stage_num, int node_num, int stage_idx,
                      int node_idx, int dct_node_num);

void gen_DCT_graph_2d(Node *node, int stage_num, int node_num, int stage_idx,
                      int node_idx, int dct_node_num);

void gen_adst_B_graph(Node *node, int stage_num, int node_num, int stage_idx,
                      int node_idx, int adst_idx);

void gen_adst_U_graph(Node *node, int stage_num, int node_num, int stage_idx,
                      int node_idx, int adst_idx, int adst_node_num);
void gen_adst_T_graph(Node *node, int stage_num, int node_num, int stage_idx,
                      int node_idx, double freq);

void gen_adst_E_graph(Node *node, int stage_num, int node_num, int stage_idx,
                      int node_idx, int adst_idx);

void gen_adst_V_graph(Node *node, int stage_num, int node_num, int stage_idx,
                      int node_idx, int adst_idx, int adst_node_num);

void gen_adst_VJ_graph(Node *node, int stage_num, int node_num, int stage_idx,
                       int node_idx, int adst_node_num);
void gen_adst_Q_graph(Node *node, int stage_num, int node_num, int stage_idx,
                      int node_idx, int adst_node_num);
void gen_adst_Ibar_graph(Node *node, int stage_num, int node_num, int stage_idx,
                         int node_idx, int adst_node_num);

void gen_adst_D_graph(Node *node, int stage_num, int node_num, int stage_idx,
                      int node_idx, int adst_node_num);

int get_hadamard_idx(int x, int adst_node_num);
void gen_adst_Ht_graph(Node *node, int stage_num, int node_num, int stage_idx,
                       int node_idx, int adst_node_num);

int gen_adst_graph(Node *node, int stage_num, int node_num, int stage_idx,
                   int node_idx, int adst_node_num);
int gen_iadst_graph(Node *node, int stage_num, int node_num, int stage_idx,
                    int node_idx, int adst_node_num);
void reference_adst_1d(double *in, double *out, int size);

int get_adst_stage_num(int adst_node_num);
int get_hybrid_stage_num(int type, int hybrid_node_num);
int get_hybrid_2d_stage_num(int type0, int type1, int hybrid_node_num);
int get_hybrid_2d_stage_num_new(int type0, int type1, int hybrid_node_num0,
                                int hybrid_node_num1);
int get_hybrid_amplify_factor(int type, int hybrid_node_num);
void gen_hybrid_graph_1d(Node *node, int stage_num, int node_num, int stage_idx,
                         int node_idx, int hybrid_node_num, int type);
void gen_hybrid_graph_2d(Node *node, int stage_num, int node_num, int stage_idx,
                         int node_idx, int hybrid_node_num, int type0,
                         int type1);
void gen_hybrid_graph_2d_new(Node *node, int stage_num, int node_num,
                             int stage_idx, int node_idx, int hybrid_node_num0,
                             int hybrid_node_num1, int type0, int type1);

void reference_hybrid_2d(double *in, double *out, int size, int type0,
                         int type1);

void reference_hybrid_2d_new(double *in, double *out, int size0, int size1,
                             int type0, int type1);
void reference_adst_dct_2d(double *in, double *out, int size);

void gen_code(Node *node, int stage_num, int node_num, TYPE_TXFM type);

void gen_inv_graph(Node *node, int stage_num, int node_num, Node *invNode,
                   int inv_stage_num, int inv_node_num, int inv_stage_idx,
                   int inv_node_idx);

TYPE_TXFM hybrid_char_to_int(char ctype);

int64_t round_shift(int64_t value, int bit);
void round_shift_array(int32_t *arr, int size, int bit);
void estimate_value(Node *node, int stage_num, int node_num, int stage_idx,
                    int node_idx, int estimate_bit);
void amplify_value(Node *node, int stage_num, int node_num, int stage_idx,
                   int node_idx, int estimate_bit);
void propagate_estimate_amlify(Node *node, int stage_num, int node_num,
                               int stage_idx, int amplify_bit,
                               int estimate_bit);
#endif  // AOM_TOOLS_TXFM_ANALYZER_TXFM_GRAPH_H_
