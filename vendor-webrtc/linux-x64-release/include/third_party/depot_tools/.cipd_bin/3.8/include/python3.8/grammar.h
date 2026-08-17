
/* Grammar interface */

#ifndef Py_GRAMMAR_H
#define Py_GRAMMAR_H
#ifdef __cplusplus
extern "C" {
#endif

#include "bitset.h" /* Sigh... */

/* A label of an arc */

typedef struct {
    int          lb_type;
    const char  *lb_str;
} label;

#define EMPTY 0         /* Label number 0 is by definition the empty label */

/* A list of labels */

typedef struct {
    int          ll_nlabels;
    const label *ll_label;
} labellist;

/* An arc from one state to another */

typedef struct {
    short       a_lbl;          /* Label of this arc */
    short       a_arrow;        /* State where this arc goes to */
} arc;

/* A state in a DFA */

typedef struct {
    int          s_narcs;
    const arc   *s_arc;         /* Array of arcs */

    /* Optional accelerators */
    int          s_lower;       /* Lowest label index */
    int          s_upper;       /* Highest label index */
    int         *s_accel;       /* Accelerator */
    int          s_accept;      /* Nonzero for accepting state */
} state;

/* A DFA */

typedef struct {
    int          d_type;        /* Non-terminal this represents */
    char        *d_name;        /* For printing */
    int          d_nstates;
    state       *d_state;       /* Array of states */
    bitset       d_first;
} dfa;

/* A grammar */

typedef struct {
    int          g_ndfas;
    const dfa   *g_dfa;         /* Array of DFAs */
    const labellist g_ll;
    int          g_start;       /* Start symbol of the grammar */
    int          g_accel;       /* Set if accelerators present */
} grammar;

/* FUNCTIONS */
const dfa *PyGrammar_FindDFA(grammar *g, int type);
const char *PyGrammar_LabelRepr(label *lb);
void PyGrammar_AddAccelerators(grammar *g);
void PyGrammar_RemoveAccelerators(grammar *);

#ifdef __cplusplus
}
#endif
#endif /* !Py_GRAMMAR_H */
