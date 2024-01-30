#include "vterm.h"
#include "../src/vterm_internal.h" // We pull in some internal bits too

#include <assert.h>
#include <stdio.h>
#include <string.h>

#define streq(a,b) (!strcmp(a,b))
#define strstartswith(a,b) (!strncmp(a,b,strlen(b)))

static size_t inplace_hex2bytes(char *s)
{
  char *inpos = s, *outpos = s;

  while(*inpos) {
    unsigned int ch;
    if(sscanf(inpos, "%2x", &ch) < 1)
      break;
    *outpos = ch;
    outpos += 1; inpos += 2;
  }

  return outpos - s;
}

static VTermModifier strpe_modifiers(char **strp)
{
  VTermModifier state = 0;

  while((*strp)[0]) {
    switch(((*strp)++)[0]) {
      case 'S': state |= VTERM_MOD_SHIFT; break;
      case 'C': state |= VTERM_MOD_CTRL;  break;
      case 'A': state |= VTERM_MOD_ALT;   break;
      default: return state;
    }
  }

  return state;
}

static VTermKey strp_key(char *str)
{
  static struct {
    char *name;
    VTermKey key;
  } keys[] = {
    { "Up",    VTERM_KEY_UP },
    { "Tab",   VTERM_KEY_TAB },
    { "Enter", VTERM_KEY_ENTER },
    { "KP0",   VTERM_KEY_KP_0 },
    { "F1",    VTERM_KEY_FUNCTION(1) },
    { NULL,    VTERM_KEY_NONE },
  };
  int i;

  for(i = 0; keys[i].name; i++) {
    if(streq(str, keys[i].name))
      return keys[i].key;
  }

  return VTERM_KEY_NONE;
}

static void print_color(const VTermColor *col)
{
  if (VTERM_COLOR_IS_RGB(col)) {
    printf("rgb(%d,%d,%d", col->red, col->green, col->blue);
  }
  else if (VTERM_COLOR_IS_INDEXED(col)) {
    printf("idx(%d", col->index);
  }
  else {
    printf("invalid(%d", col->type);
  }
  if (VTERM_COLOR_IS_DEFAULT_FG(col)) {
    printf(",is_default_fg");
  }
  if (VTERM_COLOR_IS_DEFAULT_BG(col)) {
    printf(",is_default_bg");
  }
  printf(")");
}

static VTermColor strpe_color(char **strp)
{
  uint8_t r, g, b, idx;
  int len = 0;
  VTermColor col;

  if(sscanf(*strp, "rgb(%hhu,%hhu,%hhu)%n", &r, &g, &b, &len) == 3 && len > 0) {
    *strp += len;
    vterm_color_rgb(&col, r, g, b);
  }
  else if(sscanf(*strp, "idx(%hhu)%n", &idx, &len) == 1 && len > 0) {
    *strp += len;
    vterm_color_indexed(&col, idx);
  }
  else
    vterm_color_rgb(&col, 127, 127, 127);

  return col;
}

static VTerm *vt;
static VTermState *state;
static VTermScreen *screen;

static VTermEncodingInstance encoding;

static void term_output(const char *s, size_t len, void *user UNUSED)
{
  size_t i;

  printf("output ");
  for(i = 0; i < len; i++)
    printf("%x%s", (unsigned char)s[i], i < len-1 ? "," : "\n");
}

static void printhex(const char *s, size_t len)
{
  while(len--)
    printf("%02x", (uint8_t)(s++)[0]);
}

static int parser_text(const char bytes[], size_t len, void *user UNUSED)
{
  size_t i;

  printf("text ");
  for(i = 0; i < len; i++) {
    unsigned char b = bytes[i];
    if(b < 0x20 || b == 0x7f || (b >= 0x80 && b < 0xa0))
      break;
    printf(i ? ",%x" : "%x", b);
  }
  printf("\n");

  return i;
}

static int parser_control(unsigned char control, void *user UNUSED)
{
  printf("control %02x\n", control);

  return 1;
}

static int parser_escape(const char bytes[], size_t len, void *user UNUSED)
{
  if(bytes[0] >= 0x20 && bytes[0] < 0x30) {
    if(len < 2)
      return -1;
    len = 2;
  }
  else {
    len = 1;
  }

  printf("escape ");
  printhex(bytes, len);
  printf("\n");

  return len;
}

static int parser_csi(const char *leader, const long args[], int argcount, const char *intermed, char command, void *user UNUSED)
{
  int i;
  printf("csi %02x", command);

  if(leader && leader[0]) {
    printf(" L=");
    for(i = 0; leader[i]; i++)
      printf("%02x", leader[i]);
  }

  for(i = 0; i < argcount; i++) {
    char sep = i ? ',' : ' ';

    if(args[i] == CSI_ARG_MISSING)
      printf("%c*", sep);
    else
      printf("%c%ld%s", sep, CSI_ARG(args[i]), CSI_ARG_HAS_MORE(args[i]) ? "+" : "");
  }

  if(intermed && intermed[0]) {
    printf(" I=");
    for(i = 0; intermed[i]; i++)
      printf("%02x", intermed[i]);
  }

  printf("\n");

  return 1;
}

static int parser_osc(int command, VTermStringFragment frag, void *user UNUSED)
{

  printf("osc ");

  if(frag.initial) {
    if(command == -1)
      printf("[");
    else
      printf("[%d;", command);
  }

  printhex(frag.str, frag.len);

  if(frag.final)
    printf("]");

  printf("\n");

  return 1;
}

static int parser_dcs(const char *command, size_t commandlen, VTermStringFragment frag, void *user UNUSED)
{
  printf("dcs ");

  if(frag.initial) {
    size_t i;
    printf("[");
    for(i = 0; i < commandlen; i++)
      printf("%02x", command[i]);
  }

  printhex(frag.str, frag.len);

  if(frag.final)
    printf("]");

  printf("\n");

  return 1;
}

static int parser_apc(VTermStringFragment frag, void *user UNUSED)
{
  printf("apc ");

  if(frag.initial)
    printf("[");

  printhex(frag.str, frag.len);

  if(frag.final)
    printf("]");

  printf("\n");

  return 1;
}

static int parser_pm(VTermStringFragment frag, void *user UNUSED)
{
  printf("pm ");

  if(frag.initial)
    printf("[");

  printhex(frag.str, frag.len);

  if(frag.final)
    printf("]");

  printf("\n");

  return 1;
}

static int parser_sos(VTermStringFragment frag, void *user UNUSED)
{
  printf("sos ");

  if(frag.initial)
    printf("[");

  printhex(frag.str, frag.len);

  if(frag.final)
    printf("]");

  printf("\n");

  return 1;
}

static VTermParserCallbacks parser_cbs = {
  parser_text, // text
  parser_control, // control
  parser_escape, // escape
  parser_csi, // csi
  parser_osc, // osc
  parser_dcs, // dcs
  parser_apc, // apc
  parser_pm, // pm
  parser_sos, // sos
  NULL // resize
};

static VTermStateFallbacks fallbacks = {
  parser_control, // control
  parser_csi, // csi
  parser_osc, // osc
  parser_dcs, // dcs
  parser_apc, // dcs
  parser_pm, // pm
  parser_sos // sos
};

/* These callbacks are shared by State and Screen */

static int want_movecursor = 0;
static VTermPos state_pos;
static int movecursor(VTermPos pos, VTermPos oldpos UNUSED, int visible UNUSED, void *user UNUSED)
{
  state_pos = pos;

  if(want_movecursor)
    printf("movecursor %d,%d\n", pos.row, pos.col);

  return 1;
}

static int want_scrollrect = 0;
static int scrollrect(VTermRect rect, int downward, int rightward, void *user UNUSED)
{
  if(!want_scrollrect)
    return 0;

  printf("scrollrect %d..%d,%d..%d => %+d,%+d\n",
      rect.start_row, rect.end_row, rect.start_col, rect.end_col,
      downward, rightward);

  return 1;
}

static int want_moverect = 0;
static int moverect(VTermRect dest, VTermRect src, void *user UNUSED)
{
  if(!want_moverect)
    return 0;

  printf("moverect %d..%d,%d..%d -> %d..%d,%d..%d\n",
      src.start_row,  src.end_row,  src.start_col,  src.end_col,
      dest.start_row, dest.end_row, dest.start_col, dest.end_col);

  return 1;
}

static int want_settermprop = 0;
static int settermprop(VTermProp prop, VTermValue *val, void *user UNUSED)
{
  VTermValueType type;
  if(!want_settermprop)
    return 1;

  type = vterm_get_prop_type(prop);
  switch(type) {
  case VTERM_VALUETYPE_BOOL:
    printf("settermprop %d %s\n", prop, val->boolean ? "true" : "false");
    return 1;
  case VTERM_VALUETYPE_INT:
    printf("settermprop %d %d\n", prop, val->number);
    return 1;
  case VTERM_VALUETYPE_STRING:
    printf("settermprop %d %s\"%.*s\"%s\n", prop,
        val->string.initial ? "[" : "", val->string.len, val->string.str, val->string.final ? "]" : "");
    return 1;
  case VTERM_VALUETYPE_COLOR:
    printf("settermprop %d ", prop);
    print_color(&val->color);
    printf("\n");
    return 1;

  case VTERM_N_VALUETYPES:
    return 0;
  }

  return 0;
}

// These callbacks are for State

static int want_state_putglyph = 0;
static int state_putglyph(VTermGlyphInfo *info, VTermPos pos, void *user UNUSED)
{
  int i;
  if(!want_state_putglyph)
    return 1;

  printf("putglyph ");
  for(i = 0; i < VTERM_MAX_CHARS_PER_CELL && info->chars[i]; i++)
    printf(i ? ",%x" : "%x", info->chars[i]);
  printf(" %d %d,%d", info->width, pos.row, pos.col);
  if(info->protected_cell)
    printf(" prot");
  if(info->dwl)
    printf(" dwl");
  if(info->dhl)
    printf(" dhl-%s", info->dhl == 1 ? "top" : info->dhl == 2 ? "bottom" : "?" );
  printf("\n");

  return 1;
}

static int want_state_erase = 0;
static int state_erase(VTermRect rect, int selective, void *user UNUSED)
{
  if(!want_state_erase)
    return 1;

  printf("erase %d..%d,%d..%d%s\n",
      rect.start_row, rect.end_row, rect.start_col, rect.end_col,
      selective ? " selective" : "");

  return 1;
}

static struct {
  int bold;
  int underline;
  int italic;
  int blink;
  int reverse;
  int conceal;
  int strike;
  int font;
  int small;
  int baseline;
  VTermColor foreground;
  VTermColor background;
} state_pen;
static int state_setpenattr(VTermAttr attr, VTermValue *val, void *user UNUSED)
{
  switch(attr) {
  case VTERM_ATTR_BOLD:
    state_pen.bold = val->boolean;
    break;
  case VTERM_ATTR_UNDERLINE:
    state_pen.underline = val->number;
    break;
  case VTERM_ATTR_ITALIC:
    state_pen.italic = val->boolean;
    break;
  case VTERM_ATTR_BLINK:
    state_pen.blink = val->boolean;
    break;
  case VTERM_ATTR_REVERSE:
    state_pen.reverse = val->boolean;
    break;
  case VTERM_ATTR_CONCEAL:
    state_pen.conceal = val->boolean;
    break;
  case VTERM_ATTR_STRIKE:
    state_pen.strike = val->boolean;
    break;
  case VTERM_ATTR_FONT:
    state_pen.font = val->number;
    break;
  case VTERM_ATTR_SMALL:
    state_pen.small = val->boolean;
    break;
  case VTERM_ATTR_BASELINE:
    state_pen.baseline = val->number;
    break;
  case VTERM_ATTR_FOREGROUND:
    state_pen.foreground = val->color;
    break;
  case VTERM_ATTR_BACKGROUND:
    state_pen.background = val->color;
    break;

  case VTERM_N_ATTRS:
    return 0;
  }

  return 1;
}

static int state_setlineinfo(int row UNUSED, const VTermLineInfo *newinfo UNUSED, const VTermLineInfo *oldinfo UNUSED, void *user UNUSED)
{
  return 1;
}

static int want_state_scrollback = 0;
static int state_sb_clear(void *user UNUSED) {
  if(!want_state_scrollback)
    return 1;

  printf("sb_clear\n");
  return 0;
}

VTermStateCallbacks state_cbs = {
  state_putglyph, // putglyph
  movecursor, // movecursor
  scrollrect, // scrollrect
  moverect, // moverect
  state_erase, // erase
  NULL, // initpen
  state_setpenattr, // setpenattr
  settermprop, // settermprop
  NULL, // bell
  NULL, // resize
  state_setlineinfo, // setlineinfo
  state_sb_clear, // sb_clear
};

static int selection_set(VTermSelectionMask mask, VTermStringFragment frag, void *user UNUSED)
{
  printf("selection-set mask=%04X ", mask);
  if(frag.initial)
    printf("[");
  printhex(frag.str, frag.len);
  if(frag.final)
    printf("]");
  printf("\n");

  return 1;
}

static int selection_query(VTermSelectionMask mask, void *user UNUSED)
{
  printf("selection-query mask=%04X\n", mask);

  return 1;
}

VTermSelectionCallbacks selection_cbs = {
  .set   = selection_set,
  .query = selection_query,
};

static int want_screen_damage = 0;
static int want_screen_damage_cells = 0;
static int screen_damage(VTermRect rect, void *user UNUSED)
{
  if(!want_screen_damage)
    return 1;

  printf("damage %d..%d,%d..%d",
      rect.start_row, rect.end_row, rect.start_col, rect.end_col);

  if(want_screen_damage_cells) {
    int equals = FALSE;
    int row;
    int col;

    for(row = rect.start_row; row < rect.end_row; row++) {
      int eol = rect.end_col;
      while(eol > rect.start_col) {
        VTermScreenCell cell;
	VTermPos pos;
	pos.row = row;
	pos.col = eol-1;
        vterm_screen_get_cell(screen, pos, &cell);
        if(cell.chars[0])
          break;

        eol--;
      }

      if(eol == rect.start_col)
        break;

      if(!equals)
        printf(" ="), equals = TRUE;

      printf(" %d<", row);
      for(col = rect.start_col; col < eol; col++) {
        VTermScreenCell cell;
	VTermPos pos;
	pos.row = row;
	pos.col = col;
        vterm_screen_get_cell(screen, pos, &cell);
        printf(col == rect.start_col ? "%02X" : " %02X", cell.chars[0]);
      }
      printf(">");
    }
  }

  printf("\n");

  return 1;
}

static int want_screen_scrollback = 0;
static int screen_sb_pushline(int cols, const VTermScreenCell *cells, void *user UNUSED)
{
  int eol;
  int c;

  if(!want_screen_scrollback)
    return 1;

  eol = cols;
  while(eol && !cells[eol-1].chars[0])
    eol--;

  printf("sb_pushline %d =", cols);
  for(c = 0; c < eol; c++)
    printf(" %02X", cells[c].chars[0]);
  printf("\n");

  return 1;
}

static int screen_sb_popline(int cols, VTermScreenCell *cells, void *user UNUSED)
{
  int col;

  if(!want_screen_scrollback)
    return 0;

  // All lines of scrollback contain "ABCDE"
  for(col = 0; col < cols; col++) {
    if(col < 5)
      cells[col].chars[0] = 'A' + col;
    else
      cells[col].chars[0] = 0;

    cells[col].width = 1;
  }

  printf("sb_popline %d\n", cols);
  return 1;
}

static int screen_sb_clear(void *user UNUSED)
{
  if(!want_screen_scrollback)
    return 1;

  printf("sb_clear\n");
  return 0;
}

VTermScreenCallbacks screen_cbs = {
  screen_damage, // damage
  moverect, // moverect
  movecursor, // movecursor
  settermprop, // settermprop
  NULL, // bell
  NULL, // resize
  screen_sb_pushline, // sb_pushline
  screen_sb_popline, // sb_popline
  screen_sb_clear, // sb_clear
};

int main(int argc UNUSED, char **argv UNUSED)
{
  char line[1024] = {0};
  int flag;

  int err;

  setvbuf(stdout, NULL, _IONBF, 0);

  while(fgets(line, sizeof line, stdin)) {
    char *nl;
    size_t outlen;
    err = 0;

    if((nl = strchr(line, '\n')))
      *nl = '\0';

    if(streq(line, "INIT")) {
      if(!vt)
        vt = vterm_new(25, 80);

      // Somehow this makes tests fail
      // vterm_output_set_callback(vt, term_output, NULL);
    }

    else if(streq(line, "WANTPARSER")) {
      assert(vt);
      vterm_parser_set_callbacks(vt, &parser_cbs, NULL);
    }

    else if(strstartswith(line, "WANTSTATE") && (line[9] == '\0' || line[9] == ' ')) {
      int i = 9;
      int sense = 1;
      assert(vt);
      if(!state) {
        state = vterm_obtain_state(vt);
        vterm_state_set_callbacks(state, &state_cbs, NULL);
        /* In some tests we want to check the behaviour of overflowing the
         * buffer, so make it nicely small
         */
        vterm_state_set_selection_callbacks(state, &selection_cbs, NULL, NULL, 16);
        vterm_state_set_bold_highbright(state, 1);
        vterm_state_reset(state, 1);
      }

      while(line[i] == ' ')
        i++;
      for( ; line[i]; i++)
        switch(line[i]) {
        case '+':
          sense = 1;
          break;
        case '-':
          sense = 0;
          break;
        case 'g':
          want_state_putglyph = sense;
          break;
        case 's':
          want_scrollrect = sense;
          break;
        case 'm':
          want_moverect = sense;
          break;
        case 'e':
          want_state_erase = sense;
          break;
        case 'p':
          want_settermprop = sense;
          break;
        case 'f':
          vterm_state_set_unrecognised_fallbacks(state, sense ? &fallbacks : NULL, NULL);
          break;
        case 'b':
          want_state_scrollback = sense;
          break;
        default:
          fprintf(stderr, "Unrecognised WANTSTATE flag '%c'\n", line[i]);
        }
    }

    else if(strstartswith(line, "WANTSCREEN") && (line[10] == '\0' || line[10] == ' ')) {
      int i = 10;
      int sense = 1;
      assert(vt);
      if(!screen)
        screen = vterm_obtain_screen(vt);
      vterm_screen_set_callbacks(screen, &screen_cbs, NULL);

      while(line[i] == ' ')
        i++;
      for( ; line[i]; i++)
        switch(line[i]) {
        case '-':
          sense = 0;
          break;
        case 'a':
          vterm_screen_enable_altscreen(screen, 1);
          break;
        case 'd':
          want_screen_damage = sense;
          break;
        case 'D':
          want_screen_damage = sense;
          want_screen_damage_cells = sense;
          break;
        case 'm':
          want_moverect = sense;
          break;
        case 'c':
          want_movecursor = sense;
          break;
        case 'p':
          want_settermprop = 1;
          break;
        case 'b':
          want_screen_scrollback = sense;
          break;
        case 'r':
          vterm_screen_enable_reflow(screen, sense);
          break;
        default:
          fprintf(stderr, "Unrecognised WANTSCREEN flag '%c'\n", line[i]);
        }
    }

    else if(sscanf(line, "UTF8 %d", &flag)) {
      vterm_set_utf8(vt, flag);
    }

    else if(streq(line, "RESET")) {
      if(state) {
        vterm_state_reset(state, 1);
        vterm_state_get_cursorpos(state, &state_pos);
      }
      if(screen) {
        vterm_screen_reset(screen, 1);
      }
    }

    else if(strstartswith(line, "RESIZE ")) {
      int rows, cols;
      char *linep = line + 7;
      while(linep[0] == ' ')
        linep++;
      sscanf(linep, "%d, %d", &rows, &cols);
      vterm_set_size(vt, rows, cols);
    }

    else if(strstartswith(line, "PUSH ")) {
      char *bytes = line + 5;
      size_t len = inplace_hex2bytes(bytes);
      assert(len);

      size_t written = vterm_input_write(vt, bytes, len);
      if(written < len)
        fprintf(stderr, "! short write\n");
    }

    else if(streq(line, "WANTENCODING")) {
      // This isn't really external API but it's hard to get this out any
      // other way
      encoding.enc = vterm_lookup_encoding(ENC_UTF8, 'u');
      if(encoding.enc->init)
        (*encoding.enc->init)(encoding.enc, encoding.data);
    }

    else if(strstartswith(line, "ENCIN ")) {
      char *bytes = line + 6;
      size_t len = inplace_hex2bytes(bytes);
      assert(len);

      uint32_t cp[1024];
      int cpi = 0;
      size_t pos = 0;

      (*encoding.enc->decode)(encoding.enc, encoding.data,
          cp, &cpi, len, bytes, &pos, len);

      if(cpi > 0) {
	int i;
        printf("encout ");
        for(i = 0; i < cpi; i++) {
          printf(i ? ",%x" : "%x", cp[i]);
        }
        printf("\n");
      }
    }

    else if(strstartswith(line, "INCHAR ")) {
      char *linep = line + 7;
      unsigned int c = 0;
      VTermModifier mod;
      while(linep[0] == ' ')
        linep++;
      mod = strpe_modifiers(&linep);
      sscanf(linep, " %x", &c);

      vterm_keyboard_unichar(vt, c, mod);
    }

    else if(strstartswith(line, "INKEY ")) {
      VTermModifier mod;
      VTermKey key;
      char *linep = line + 6;
      while(linep[0] == ' ')
        linep++;
      mod = strpe_modifiers(&linep);
      while(linep[0] == ' ')
        linep++;
      key = strp_key(linep);

      vterm_keyboard_key(vt, key, mod);
    }

    else if(strstartswith(line, "PASTE ")) {
      char *linep = line + 6;
      if(streq(linep, "START"))
        vterm_keyboard_start_paste(vt);
      else if(streq(linep, "END"))
        vterm_keyboard_end_paste(vt);
      else
        goto abort_line;
    }

    else if(strstartswith(line, "FOCUS ")) {
      assert(state);
      char *linep = line + 6;
      if(streq(linep, "IN"))
        vterm_state_focus_in(state);
      else if(streq(linep, "OUT"))
        vterm_state_focus_out(state);
      else
        goto abort_line;
    }

    else if(strstartswith(line, "MOUSEMOVE ")) {
      char *linep = line + 10;
      int row, col, len;
      VTermModifier mod;
      while(linep[0] == ' ')
        linep++;
      sscanf(linep, "%d,%d%n", &row, &col, &len);
      linep += len;
      while(linep[0] == ' ')
        linep++;
      mod = strpe_modifiers(&linep);
      vterm_mouse_move(vt, row, col, mod);
    }

    else if(strstartswith(line, "MOUSEBTN ")) {
      char *linep = line + 9;
      char press;
      int button, len;
      VTermModifier mod;
      while(linep[0] == ' ')
        linep++;
      sscanf(linep, "%c %d%n", &press, &button, &len);
      linep += len;
      while(linep[0] == ' ')
        linep++;
      mod = strpe_modifiers(&linep);
      vterm_mouse_button(vt, button, (press == 'd' || press == 'D'), mod);
    }

    else if(strstartswith(line, "SELECTION ")) {
      char *linep = line + 10;
      unsigned int mask;
      int len;
      VTermStringFragment frag = { 0 };
      sscanf(linep, "%x%n", &mask, &len);
      linep += len;
      while(linep[0] == ' ')
        linep++;
      if(linep[0] == '[') {
        frag.initial = TRUE;
        linep++;
        while(linep[0] == ' ')
          linep++;
      }
      frag.len = inplace_hex2bytes(linep);
      frag.str = linep;
      assert(frag.len);

      linep += frag.len * 2;
      while(linep[0] == ' ')
        linep++;
      if(linep[0] == ']') {
        frag.final = TRUE;
      }
      vterm_state_send_selection(state, mask, frag);
    }

    else if(strstartswith(line, "DAMAGEMERGE ")) {
      assert(screen);
      char *linep = line + 12;
      while(linep[0] == ' ')
        linep++;
      if(streq(linep, "CELL"))
        vterm_screen_set_damage_merge(screen, VTERM_DAMAGE_CELL);
      else if(streq(linep, "ROW"))
        vterm_screen_set_damage_merge(screen, VTERM_DAMAGE_ROW);
      else if(streq(linep, "SCREEN"))
        vterm_screen_set_damage_merge(screen, VTERM_DAMAGE_SCREEN);
      else if(streq(linep, "SCROLL"))
        vterm_screen_set_damage_merge(screen, VTERM_DAMAGE_SCROLL);
    }

    else if(strstartswith(line, "DAMAGEFLUSH")) {
      assert(screen);
      vterm_screen_flush_damage(screen);
    }

    else if(strstartswith(line, "SETDEFAULTCOL ")) {
      assert(screen);
      char *linep = line + 14;
      while(linep[0] == ' ')
        linep++;
      VTermColor fg = strpe_color(&linep);
      if(linep[0]) {
        while(linep[0] == ' ')
          linep++;
        VTermColor bg = strpe_color(&linep);

        vterm_screen_set_default_colors(screen, &fg, &bg);
      }
      else
        vterm_screen_set_default_colors(screen, &fg, NULL);
    }

    else if(line[0] == '?') {
      if(streq(line, "?cursor")) {
        assert(state);
        VTermPos pos;
        vterm_state_get_cursorpos(state, &pos);
        if(pos.row != state_pos.row)
          printf("! row mismatch: state=%d,%d event=%d,%d\n",
              pos.row, pos.col, state_pos.row, state_pos.col);
        else if(pos.col != state_pos.col)
          printf("! col mismatch: state=%d,%d event=%d,%d\n",
              pos.row, pos.col, state_pos.row, state_pos.col);
        else
          printf("%d,%d\n", state_pos.row, state_pos.col);
      }
      else if(strstartswith(line, "?pen ")) {
        assert(state);
        VTermValue val;
        char *linep = line + 5;
        while(linep[0] == ' ')
          linep++;

#define BOOLSTR(v) ((v) ? "on" : "off")

        if(streq(linep, "bold")) {
          vterm_state_get_penattr(state, VTERM_ATTR_BOLD, &val);
          if(val.boolean != state_pen.bold)
            printf("! pen bold mismatch; state=%s, event=%s\n",
                BOOLSTR(val.boolean), BOOLSTR(state_pen.bold));
          else
            printf("%s\n", BOOLSTR(state_pen.bold));
        }
        else if(streq(linep, "underline")) {
          vterm_state_get_penattr(state, VTERM_ATTR_UNDERLINE, &val);
          if(val.boolean != state_pen.underline)
            printf("! pen underline mismatch; state=%d, event=%d\n",
                val.boolean, state_pen.underline);
          else
            printf("%d\n", state_pen.underline);
        }
        else if(streq(linep, "italic")) {
          vterm_state_get_penattr(state, VTERM_ATTR_ITALIC, &val);
          if(val.boolean != state_pen.italic)
            printf("! pen italic mismatch; state=%s, event=%s\n",
                BOOLSTR(val.boolean), BOOLSTR(state_pen.italic));
          else
            printf("%s\n", BOOLSTR(state_pen.italic));
        }
        else if(streq(linep, "blink")) {
          vterm_state_get_penattr(state, VTERM_ATTR_BLINK, &val);
          if(val.boolean != state_pen.blink)
            printf("! pen blink mismatch; state=%s, event=%s\n",
                BOOLSTR(val.boolean), BOOLSTR(state_pen.blink));
          else
            printf("%s\n", BOOLSTR(state_pen.blink));
        }
        else if(streq(linep, "reverse")) {
          vterm_state_get_penattr(state, VTERM_ATTR_REVERSE, &val);
          if(val.boolean != state_pen.reverse)
            printf("! pen reverse mismatch; state=%s, event=%s\n",
                BOOLSTR(val.boolean), BOOLSTR(state_pen.reverse));
          else
            printf("%s\n", BOOLSTR(state_pen.reverse));
        }
        else if(streq(linep, "font")) {
          vterm_state_get_penattr(state, VTERM_ATTR_FONT, &val);
          if(val.boolean != state_pen.font)
            printf("! pen font mismatch; state=%d, event=%d\n",
                val.boolean, state_pen.font);
          else
            printf("%d\n", state_pen.font);
        }
        else if(streq(linep, "small")) {
          vterm_state_get_penattr(state, VTERM_ATTR_SMALL, &val);
          if(val.boolean != state_pen.small)
            printf("! pen small mismatch; state=%s, event=%s\n",
                BOOLSTR(val.boolean), BOOLSTR(state_pen.small));
          else
            printf("%s\n", BOOLSTR(state_pen.small));
        }
        else if(streq(linep, "baseline")) {
          vterm_state_get_penattr(state, VTERM_ATTR_BASELINE, &val);
          if(val.number != state_pen.baseline)
            printf("! pen baseline mismatch: state=%d, event=%d\n",
                val.number, state_pen.baseline);
          else
            printf("%s\n", state_pen.baseline == VTERM_BASELINE_RAISE ? "raise"
                         : state_pen.baseline == VTERM_BASELINE_LOWER ? "lower"
                         : "normal");
        }
        else if(streq(linep, "foreground")) {
          print_color(&state_pen.foreground);
          printf("\n");
        }
        else if(streq(linep, "background")) {
          print_color(&state_pen.background);
          printf("\n");
        }
        else
          printf("?\n");
      }
      else if(strstartswith(line, "?lineinfo ")) {
        assert(state);
        char *linep = line + 10;
        int row;
        const VTermLineInfo *info;
        while(linep[0] == ' ')
          linep++;
        if(sscanf(linep, "%d", &row) < 1) {
          printf("! lineinfo unrecognised input\n");
          goto abort_line;
        }
        info = vterm_state_get_lineinfo(state, row);
        if(info->doublewidth)
          printf("dwl ");
        if(info->doubleheight)
          printf("dhl ");
        if(info->continuation)
          printf("cont ");
        printf("\n");
      }
      else if(strstartswith(line, "?screen_chars ")) {
        assert(screen);
        char *linep = line + 13;
        VTermRect rect;
        size_t len;
        while(linep[0] == ' ')
          linep++;
        if(sscanf(linep, "%d,%d,%d,%d", &rect.start_row, &rect.start_col, &rect.end_row, &rect.end_col) == 4)
          ; // fine
        else if(sscanf(linep, "%d", &rect.start_row) == 1) {
          rect.end_row = rect.start_row + 1;
          rect.start_col = 0;
          vterm_get_size(vt, NULL, &rect.end_col);
        }
        else {
          printf("! screen_chars unrecognised input\n");
          goto abort_line;
        }
        len = vterm_screen_get_chars(screen, NULL, 0, rect);
        if(len == (size_t)-1)
          printf("! screen_chars error\n");
        else if(len == 0)
          printf("\n");
        else {
          uint32_t *chars = malloc(sizeof(uint32_t) * len);
          size_t i;
          vterm_screen_get_chars(screen, chars, len, rect);
          for(i = 0; i < len; i++) {
            printf("0x%02x%s", chars[i], i < len-1 ? "," : "\n");
          }
          free(chars);
        }
      }
      else if(strstartswith(line, "?screen_text ")) {
        assert(screen);
        char *linep = line + 12;
        VTermRect rect;
        size_t len;
        while(linep[0] == ' ')
          linep++;
        if(sscanf(linep, "%d,%d,%d,%d", &rect.start_row, &rect.start_col, &rect.end_row, &rect.end_col) < 4) {
          printf("! screen_text unrecognised input\n");
          goto abort_line;
        }
        len = vterm_screen_get_text(screen, NULL, 0, rect);
        if(len == (size_t)-1)
          printf("! screen_text error\n");
        else if(len == 0)
          printf("\n");
        else {
          // Put an overwrite guard at both ends of the buffer
          unsigned char *buffer = malloc(len + 4);
          unsigned char *text = buffer + 2;
          text[-2] = 0x55; text[-1] = 0xAA;
          text[len] = 0x55; text[len+1] = 0xAA;

          vterm_screen_get_text(screen, (char *)text, len, rect);

          if(text[-2] != 0x55 || text[-1] != 0xAA)
            printf("! screen_get_text buffer overrun left [%02x,%02x]\n", text[-2], text[-1]);
          else if(text[len] != 0x55 || text[len+1] != 0xAA)
            printf("! screen_get_text buffer overrun right [%02x,%02x]\n", text[len], text[len+1]);
          else
	  {
	    size_t i;
            for(i = 0; i < len; i++) {
              printf("0x%02x%s", text[i], i < len-1 ? "," : "\n");
            }
	  }

          free(buffer);
        }
      }
      else if(strstartswith(line, "?screen_cell ")) {
        assert(screen);
        char *linep = line + 12;
	int i;
        VTermPos pos;
        VTermScreenCell cell;
        while(linep[0] == ' ')
          linep++;
        if(sscanf(linep, "%d,%d\n", &pos.row, &pos.col) < 2) {
          printf("! screen_cell unrecognised input\n");
          goto abort_line;
        }
        if(!vterm_screen_get_cell(screen, pos, &cell))
          goto abort_line;
        printf("{");
        for(i = 0; i < VTERM_MAX_CHARS_PER_CELL && cell.chars[i]; i++) {
          printf("%s0x%x", i ? "," : "", cell.chars[i]);
        }
        printf("} width=%d attrs={", cell.width);
        if(cell.attrs.bold)      printf("B");
        if(cell.attrs.underline) printf("U%d", cell.attrs.underline);
        if(cell.attrs.italic)    printf("I");
        if(cell.attrs.blink)     printf("K");
        if(cell.attrs.reverse)   printf("R");
        if(cell.attrs.font)      printf("F%d", cell.attrs.font);
        if(cell.attrs.small)     printf("S");
        if(cell.attrs.baseline)  printf(
            cell.attrs.baseline == VTERM_BASELINE_RAISE ? "^" :
                                                          "_");
        printf("} ");
        if(cell.attrs.dwl)       printf("dwl ");
        if(cell.attrs.dhl)       printf("dhl-%s ", cell.attrs.dhl == 2 ? "bottom" : "top");
        printf("fg=");
        vterm_screen_convert_color_to_rgb(screen, &cell.fg);
        print_color(&cell.fg);
        printf(" bg=");
        vterm_screen_convert_color_to_rgb(screen, &cell.bg);
        print_color(&cell.bg);
        printf("\n");
      }
      else if(strstartswith(line, "?screen_eol ")) {
        assert(screen);
        VTermPos pos;
        char *linep = line + 12;
        while(linep[0] == ' ')
          linep++;
        if(sscanf(linep, "%d,%d\n", &pos.row, &pos.col) < 2) {
          printf("! screen_eol unrecognised input\n");
          goto abort_line;
        }
        printf("%d\n", vterm_screen_is_eol(screen, pos));
      }
      else if(strstartswith(line, "?screen_attrs_extent ")) {
        assert(screen);
        VTermPos pos;
        VTermRect rect;
        char *linep = line + 21;
        while(linep[0] == ' ')
          linep++;
        if(sscanf(linep, "%d,%d\n", &pos.row, &pos.col) < 2) {
          printf("! screen_attrs_extent unrecognised input\n");
          goto abort_line;
        }
	rect.start_col = 0;
	rect.end_col   = -1;
        if(!vterm_screen_get_attrs_extent(screen, &rect, pos, ~0)) {
          printf("! screen_attrs_extent failed\n");
          goto abort_line;
        }
        printf("%d,%d-%d,%d\n", rect.start_row, rect.start_col, rect.end_row, rect.end_col);
      }
      else
        printf("?\n");

      memset(line, 0, sizeof line);
      continue;
    }

    else
      abort_line: err = 1;

    outlen = vterm_output_get_buffer_current(vt);
    if(outlen > 0) {
      char outbuff[1024];
      vterm_output_read(vt, outbuff, outlen);

      term_output(outbuff, outlen, NULL);
    }

    printf(err ? "?\n" : "DONE\n");
  }

  vterm_free(vt);

  return 0;
}
