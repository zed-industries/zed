#include <stdlib.h>

#define SCH_STT_FRZ -1

typedef enum {
  RS_STR,
  RS_INT,
  RS_NULL,
  RS_BOOL,
  RS_FLOAT,
} ResultSchema;

static int8_t adv_sch_stt(int8_t sch_stt, int32_t cur_chr, ResultSchema *rlt_sch) {
  switch (sch_stt) {
    case SCH_STT_FRZ:
      break;
    case 0:
        if (cur_chr == '.') {*rlt_sch = RS_STR; return 6;}
        if (cur_chr == '0') {*rlt_sch = RS_INT; return 37;}
        if (cur_chr == 'F') {*rlt_sch = RS_STR; return 2;}
        if (cur_chr == 'N') {*rlt_sch = RS_STR; return 16;}
        if (cur_chr == 'T') {*rlt_sch = RS_STR; return 13;}
        if (cur_chr == 'f') {*rlt_sch = RS_STR; return 17;}
        if (cur_chr == 'n') {*rlt_sch = RS_STR; return 29;}
        if (cur_chr == 't') {*rlt_sch = RS_STR; return 26;}
        if (cur_chr == '~') {*rlt_sch = RS_NULL; return 35;}
        if (cur_chr == '+') {*rlt_sch = RS_STR; return 1;}
        if (cur_chr == '-') {*rlt_sch = RS_STR; return 1;}
        if (('1' <= cur_chr && cur_chr <= '9')) {*rlt_sch = RS_INT; return 38;}
      break;
    case 1:
      if (cur_chr == '.') {*rlt_sch = RS_STR; return 7;}
      if (('0' <= cur_chr && cur_chr <= '9')) {*rlt_sch = RS_INT; return 38;}
      break;
    case 2:
      if (cur_chr == 'A') {*rlt_sch = RS_STR; return 9;}
      if (cur_chr == 'a') {*rlt_sch = RS_STR; return 22;}
      break;
    case 3:
      if (cur_chr == 'A') {*rlt_sch = RS_STR; return 12;}
      if (cur_chr == 'a') {*rlt_sch = RS_STR; return 12;}
      break;
    case 4:
      if (cur_chr == 'E') {*rlt_sch = RS_BOOL; return 36;}
      break;
    case 5:
      if (cur_chr == 'F') {*rlt_sch = RS_FLOAT; return 41;}
      break;
    case 6:
      if (cur_chr == 'I') {*rlt_sch = RS_STR; return 11;}
      if (cur_chr == 'N') {*rlt_sch = RS_STR; return 3;}
      if (cur_chr == 'i') {*rlt_sch = RS_STR; return 24;}
      if (cur_chr == 'n') {*rlt_sch = RS_STR; return 18;}
      if (('0' <= cur_chr && cur_chr <= '9')) {*rlt_sch = RS_FLOAT; return 42;}
      break;
    case 7:
      if (cur_chr == 'I') {*rlt_sch = RS_STR; return 11;}
      if (cur_chr == 'i') {*rlt_sch = RS_STR; return 24;}
      if (('0' <= cur_chr && cur_chr <= '9')) {*rlt_sch = RS_FLOAT; return 42;}
      break;
    case 8:
      if (cur_chr == 'L') {*rlt_sch = RS_NULL; return 35;}
      break;
    case 9:
      if (cur_chr == 'L') {*rlt_sch = RS_STR; return 14;}
      break;
    case 10:
      if (cur_chr == 'L') {*rlt_sch = RS_STR; return 8;}
      break;
    case 11:
      if (cur_chr == 'N') {*rlt_sch = RS_STR; return 5;}
      if (cur_chr == 'n') {*rlt_sch = RS_STR; return 20;}
      break;
    case 12:
      if (cur_chr == 'N') {*rlt_sch = RS_FLOAT; return 41;}
      break;
    case 13:
      if (cur_chr == 'R') {*rlt_sch = RS_STR; return 15;}
      if (cur_chr == 'r') {*rlt_sch = RS_STR; return 28;}
      break;
    case 14:
      if (cur_chr == 'S') {*rlt_sch = RS_STR; return 4;}
      break;
    case 15:
      if (cur_chr == 'U') {*rlt_sch = RS_STR; return 4;}
      break;
    case 16:
      if (cur_chr == 'U') {*rlt_sch = RS_STR; return 10;}
      if (cur_chr == 'u') {*rlt_sch = RS_STR; return 23;}
      break;
    case 17:
      if (cur_chr == 'a') {*rlt_sch = RS_STR; return 22;}
      break;
    case 18:
      if (cur_chr == 'a') {*rlt_sch = RS_STR; return 25;}
      break;
    case 19:
      if (cur_chr == 'e') {*rlt_sch = RS_BOOL; return 36;}
      break;
    case 20:
      if (cur_chr == 'f') {*rlt_sch = RS_FLOAT; return 41;}
      break;
    case 21:
      if (cur_chr == 'l') {*rlt_sch = RS_NULL; return 35;}
      break;
    case 22:
      if (cur_chr == 'l') {*rlt_sch = RS_STR; return 27;}
      break;
    case 23:
      if (cur_chr == 'l') {*rlt_sch = RS_STR; return 21;}
      break;
    case 24:
      if (cur_chr == 'n') {*rlt_sch = RS_STR; return 20;}
      break;
    case 25:
      if (cur_chr == 'n') {*rlt_sch = RS_FLOAT; return 41;}
      break;
    case 26:
      if (cur_chr == 'r') {*rlt_sch = RS_STR; return 28;}
      break;
    case 27:
      if (cur_chr == 's') {*rlt_sch = RS_STR; return 19;}
      break;
    case 28:
      if (cur_chr == 'u') {*rlt_sch = RS_STR; return 19;}
      break;
    case 29:
      if (cur_chr == 'u') {*rlt_sch = RS_STR; return 23;}
      break;
    case 30:
      if (cur_chr == '+' ||
          cur_chr == '-') {*rlt_sch = RS_STR; return 32;}
      if (('0' <= cur_chr && cur_chr <= '9')) {*rlt_sch = RS_FLOAT; return 43;}
      break;
    case 31:
      if (('0' <= cur_chr && cur_chr <= '7')) {*rlt_sch = RS_INT; return 39;}
      break;
    case 32:
      if (('0' <= cur_chr && cur_chr <= '9')) {*rlt_sch = RS_FLOAT; return 43;}
      break;
    case 33:
      if (('0' <= cur_chr && cur_chr <= '9') ||
          ('A' <= cur_chr && cur_chr <= 'F') ||
          ('a' <= cur_chr && cur_chr <= 'f')) {*rlt_sch = RS_INT; return 40;}
      break;
    case 34:
      abort();
      break;
    case 35:
      *rlt_sch = RS_NULL;
      break;
    case 36:
      *rlt_sch = RS_BOOL;
      break;
    case 37:
      *rlt_sch = RS_INT;
      if (cur_chr == '.') {*rlt_sch = RS_FLOAT; return 42;}
      if (cur_chr == 'o') {*rlt_sch = RS_STR; return 31;}
      if (cur_chr == 'x') {*rlt_sch = RS_STR; return 33;}
      if (cur_chr == 'E' ||
          cur_chr == 'e') {*rlt_sch = RS_STR; return 30;}
      if (('0' <= cur_chr && cur_chr <= '9')) {*rlt_sch = RS_INT; return 38;}
      break;
    case 38:
      *rlt_sch = RS_INT;
      if (cur_chr == '.') {*rlt_sch = RS_FLOAT; return 42;}
      if (cur_chr == 'E' ||
          cur_chr == 'e') {*rlt_sch = RS_STR; return 30;}
      if (('0' <= cur_chr && cur_chr <= '9')) {*rlt_sch = RS_INT; return 38;}
      break;
    case 39:
      *rlt_sch = RS_INT;
      if (('0' <= cur_chr && cur_chr <= '7')) {*rlt_sch = RS_INT; return 39;}
      break;
    case 40:
      *rlt_sch = RS_INT;
      if (('0' <= cur_chr && cur_chr <= '9') ||
          ('A' <= cur_chr && cur_chr <= 'F') ||
          ('a' <= cur_chr && cur_chr <= 'f')) {*rlt_sch = RS_INT; return 40;}
      break;
    case 41:
      *rlt_sch = RS_FLOAT;
      break;
    case 42:
      *rlt_sch = RS_FLOAT;
      if (cur_chr == 'E' ||
          cur_chr == 'e') {*rlt_sch = RS_STR; return 30;}
      if (('0' <= cur_chr && cur_chr <= '9')) {*rlt_sch = RS_FLOAT; return 42;}
      break;
    case 43:
      *rlt_sch = RS_FLOAT;
      if (('0' <= cur_chr && cur_chr <= '9')) {*rlt_sch = RS_FLOAT; return 43;}
      break;
    default:
      *rlt_sch = RS_STR;
      return SCH_STT_FRZ;
  }
  if (cur_chr != '\r' && cur_chr != '\n' && cur_chr != ' ' && cur_chr != 0) *rlt_sch = RS_STR;
  return SCH_STT_FRZ;
}
