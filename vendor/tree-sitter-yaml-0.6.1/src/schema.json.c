#include <stdlib.h>

#define SCH_STT_FRZ -1

typedef enum {
  RS_STR,
  RS_INT,
  RS_BOOL,
  RS_NULL,
  RS_FLOAT,
} ResultSchema;

static int8_t adv_sch_stt(int8_t sch_stt, int32_t cur_chr, ResultSchema *rlt_sch) {
  switch (sch_stt) {
    case SCH_STT_FRZ:
      break;
    case 0:
      if (cur_chr == '-') {*rlt_sch = RS_STR; return 1;}
      if (cur_chr == '0') {*rlt_sch = RS_INT; return 16;}
      if (cur_chr == 'f') {*rlt_sch = RS_STR; return 2;}
      if (cur_chr == 'n') {*rlt_sch = RS_STR; return 10;}
      if (cur_chr == 't') {*rlt_sch = RS_STR; return 7;}
      if (('1' <= cur_chr && cur_chr <= '9')) {*rlt_sch = RS_INT; return 17;}
      break;
    case 1:
      if (cur_chr == '0') {*rlt_sch = RS_INT; return 16;}
      if (('1' <= cur_chr && cur_chr <= '9')) {*rlt_sch = RS_INT; return 17;}
      break;
    case 2:
      if (cur_chr == 'a') {*rlt_sch = RS_STR; return 4;}
      break;
    case 3:
      if (cur_chr == 'e') {*rlt_sch = RS_BOOL; return 15;}
      break;
    case 4:
      if (cur_chr == 'l') {*rlt_sch = RS_STR; return 8;}
      break;
    case 5:
      if (cur_chr == 'l') {*rlt_sch = RS_NULL; return 14;}
      break;
    case 6:
      if (cur_chr == 'l') {*rlt_sch = RS_STR; return 5;}
      break;
    case 7:
      if (cur_chr == 'r') {*rlt_sch = RS_STR; return 9;}
      break;
    case 8:
      if (cur_chr == 's') {*rlt_sch = RS_STR; return 3;}
      break;
    case 9:
      if (cur_chr == 'u') {*rlt_sch = RS_STR; return 3;}
      break;
    case 10:
      if (cur_chr == 'u') {*rlt_sch = RS_STR; return 6;}
      break;
    case 11:
      if (cur_chr == '+' ||
          cur_chr == '-') {*rlt_sch = RS_STR; return 12;}
      if (('0' <= cur_chr && cur_chr <= '9')) {*rlt_sch = RS_FLOAT; return 19;}
      break;
    case 12:
      if (('0' <= cur_chr && cur_chr <= '9')) {*rlt_sch = RS_FLOAT; return 19;}
      break;
    case 13:
      abort();
      break;
    case 14:
      *rlt_sch = RS_NULL;
      break;
    case 15:
      *rlt_sch = RS_BOOL;
      break;
    case 16:
      *rlt_sch = RS_INT;
      if (cur_chr == '.') {*rlt_sch = RS_FLOAT; return 18;}
      if (cur_chr == 'E' ||
          cur_chr == 'e') {*rlt_sch = RS_STR; return 11;}
      break;
    case 17:
      *rlt_sch = RS_INT;
      if (cur_chr == '.') {*rlt_sch = RS_FLOAT; return 18;}
      if (cur_chr == 'E' ||
          cur_chr == 'e') {*rlt_sch = RS_STR; return 11;}
      if (('0' <= cur_chr && cur_chr <= '9')) {*rlt_sch = RS_INT; return 17;}
      break;
    case 18:
      *rlt_sch = RS_FLOAT;
      if (cur_chr == 'E' ||
          cur_chr == 'e') {*rlt_sch = RS_STR; return 11;}
      if (('0' <= cur_chr && cur_chr <= '9')) {*rlt_sch = RS_FLOAT; return 18;}
      break;
    case 19:
      *rlt_sch = RS_FLOAT;
      if (('0' <= cur_chr && cur_chr <= '9')) {*rlt_sch = RS_FLOAT; return 19;}
      break;
    default:
      *rlt_sch = RS_STR;
      return SCH_STT_FRZ;
  }
  if (cur_chr != '\r' && cur_chr != '\n' && cur_chr != ' ' && cur_chr != 0) *rlt_sch = RS_STR;
  return SCH_STT_FRZ;
}
