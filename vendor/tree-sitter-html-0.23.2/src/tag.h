#include "tree_sitter/array.h"

#include <string.h>

typedef enum {
    AREA,
    BASE,
    BASEFONT,
    BGSOUND,
    BR,
    COL,
    COMMAND,
    EMBED,
    FRAME,
    HR,
    IMAGE,
    IMG,
    INPUT,
    ISINDEX,
    KEYGEN,
    LINK,
    MENUITEM,
    META,
    NEXTID,
    PARAM,
    SOURCE,
    TRACK,
    WBR,
    END_OF_VOID_TAGS,

    A,
    ABBR,
    ADDRESS,
    ARTICLE,
    ASIDE,
    AUDIO,
    B,
    BDI,
    BDO,
    BLOCKQUOTE,
    BODY,
    BUTTON,
    CANVAS,
    CAPTION,
    CITE,
    CODE,
    COLGROUP,
    DATA,
    DATALIST,
    DD,
    DEL,
    DETAILS,
    DFN,
    DIALOG,
    DIV,
    DL,
    DT,
    EM,
    FIELDSET,
    FIGCAPTION,
    FIGURE,
    FOOTER,
    FORM,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    HEAD,
    HEADER,
    HGROUP,
    HTML,
    I,
    IFRAME,
    INS,
    KBD,
    LABEL,
    LEGEND,
    LI,
    MAIN,
    MAP,
    MARK,
    MATH,
    MENU,
    METER,
    NAV,
    NOSCRIPT,
    OBJECT,
    OL,
    OPTGROUP,
    OPTION,
    OUTPUT,
    P,
    PICTURE,
    PRE,
    PROGRESS,
    Q,
    RB,
    RP,
    RT,
    RTC,
    RUBY,
    S,
    SAMP,
    SCRIPT,
    SECTION,
    SELECT,
    SLOT,
    SMALL,
    SPAN,
    STRONG,
    STYLE,
    SUB,
    SUMMARY,
    SUP,
    SVG,
    TABLE,
    TBODY,
    TD,
    TEMPLATE,
    TEXTAREA,
    TFOOT,
    TH,
    THEAD,
    TIME,
    TITLE,
    TR,
    U,
    UL,
    VAR,
    VIDEO,

    CUSTOM,

    END_,
} TagType;

typedef Array(char) String;

typedef struct {
    char tag_name[16];
    TagType tag_type;
} TagMapEntry;

typedef struct {
    TagType type;
    String custom_tag_name;
} Tag;

static const TagMapEntry TAG_TYPES_BY_TAG_NAME[126] = {
    {"AREA",       AREA      },
    {"BASE",       BASE      },
    {"BASEFONT",   BASEFONT  },
    {"BGSOUND",    BGSOUND   },
    {"BR",         BR        },
    {"COL",        COL       },
    {"COMMAND",    COMMAND   },
    {"EMBED",      EMBED     },
    {"FRAME",      FRAME     },
    {"HR",         HR        },
    {"IMAGE",      IMAGE     },
    {"IMG",        IMG       },
    {"INPUT",      INPUT     },
    {"ISINDEX",    ISINDEX   },
    {"KEYGEN",     KEYGEN    },
    {"LINK",       LINK      },
    {"MENUITEM",   MENUITEM  },
    {"META",       META      },
    {"NEXTID",     NEXTID    },
    {"PARAM",      PARAM     },
    {"SOURCE",     SOURCE    },
    {"TRACK",      TRACK     },
    {"WBR",        WBR       },
    {"A",          A         },
    {"ABBR",       ABBR      },
    {"ADDRESS",    ADDRESS   },
    {"ARTICLE",    ARTICLE   },
    {"ASIDE",      ASIDE     },
    {"AUDIO",      AUDIO     },
    {"B",          B         },
    {"BDI",        BDI       },
    {"BDO",        BDO       },
    {"BLOCKQUOTE", BLOCKQUOTE},
    {"BODY",       BODY      },
    {"BUTTON",     BUTTON    },
    {"CANVAS",     CANVAS    },
    {"CAPTION",    CAPTION   },
    {"CITE",       CITE      },
    {"CODE",       CODE      },
    {"COLGROUP",   COLGROUP  },
    {"DATA",       DATA      },
    {"DATALIST",   DATALIST  },
    {"DD",         DD        },
    {"DEL",        DEL       },
    {"DETAILS",    DETAILS   },
    {"DFN",        DFN       },
    {"DIALOG",     DIALOG    },
    {"DIV",        DIV       },
    {"DL",         DL        },
    {"DT",         DT        },
    {"EM",         EM        },
    {"FIELDSET",   FIELDSET  },
    {"FIGCAPTION", FIGCAPTION},
    {"FIGURE",     FIGURE    },
    {"FOOTER",     FOOTER    },
    {"FORM",       FORM      },
    {"H1",         H1        },
    {"H2",         H2        },
    {"H3",         H3        },
    {"H4",         H4        },
    {"H5",         H5        },
    {"H6",         H6        },
    {"HEAD",       HEAD      },
    {"HEADER",     HEADER    },
    {"HGROUP",     HGROUP    },
    {"HTML",       HTML      },
    {"I",          I         },
    {"IFRAME",     IFRAME    },
    {"INS",        INS       },
    {"KBD",        KBD       },
    {"LABEL",      LABEL     },
    {"LEGEND",     LEGEND    },
    {"LI",         LI        },
    {"MAIN",       MAIN      },
    {"MAP",        MAP       },
    {"MARK",       MARK      },
    {"MATH",       MATH      },
    {"MENU",       MENU      },
    {"METER",      METER     },
    {"NAV",        NAV       },
    {"NOSCRIPT",   NOSCRIPT  },
    {"OBJECT",     OBJECT    },
    {"OL",         OL        },
    {"OPTGROUP",   OPTGROUP  },
    {"OPTION",     OPTION    },
    {"OUTPUT",     OUTPUT    },
    {"P",          P         },
    {"PICTURE",    PICTURE   },
    {"PRE",        PRE       },
    {"PROGRESS",   PROGRESS  },
    {"Q",          Q         },
    {"RB",         RB        },
    {"RP",         RP        },
    {"RT",         RT        },
    {"RTC",        RTC       },
    {"RUBY",       RUBY      },
    {"S",          S         },
    {"SAMP",       SAMP      },
    {"SCRIPT",     SCRIPT    },
    {"SECTION",    SECTION   },
    {"SELECT",     SELECT    },
    {"SLOT",       SLOT      },
    {"SMALL",      SMALL     },
    {"SPAN",       SPAN      },
    {"STRONG",     STRONG    },
    {"STYLE",      STYLE     },
    {"SUB",        SUB       },
    {"SUMMARY",    SUMMARY   },
    {"SUP",        SUP       },
    {"SVG",        SVG       },
    {"TABLE",      TABLE     },
    {"TBODY",      TBODY     },
    {"TD",         TD        },
    {"TEMPLATE",   TEMPLATE  },
    {"TEXTAREA",   TEXTAREA  },
    {"TFOOT",      TFOOT     },
    {"TH",         TH        },
    {"THEAD",      THEAD     },
    {"TIME",       TIME      },
    {"TITLE",      TITLE     },
    {"TR",         TR        },
    {"U",          U         },
    {"UL",         UL        },
    {"VAR",        VAR       },
    {"VIDEO",      VIDEO     },
    {"CUSTOM",     CUSTOM    },
};

static const TagType TAG_TYPES_NOT_ALLOWED_IN_PARAGRAPHS[] = {
    ADDRESS,  ARTICLE,    ASIDE,  BLOCKQUOTE, DETAILS, DIV, DL,
    FIELDSET, FIGCAPTION, FIGURE, FOOTER,     FORM,    H1,  H2,
    H3,       H4,         H5,     H6,         HEADER,  HR,  MAIN,
    NAV,      OL,         P,      PRE,        SECTION,
};

static TagType tag_type_for_name(const String *tag_name) {
    for (int i = 0; i < 126; i++) {
        const TagMapEntry *entry = &TAG_TYPES_BY_TAG_NAME[i];
        if (
            strlen(entry->tag_name) == tag_name->size &&
            memcmp(tag_name->contents, entry->tag_name, tag_name->size) == 0
        ) {
            return entry->tag_type;
        }
    }
    return CUSTOM;
}

static inline Tag tag_new() {
    Tag tag;
    tag.type = END_;
    tag.custom_tag_name = (String) array_new();
    return tag;
}

static inline Tag tag_for_name(String name) {
    Tag tag = tag_new();
    tag.type = tag_type_for_name(&name);
    if (tag.type == CUSTOM) {
        tag.custom_tag_name = name;
    } else {
        array_delete(&name);
    }
    return tag;
}

static inline void tag_free(Tag *tag) {
    if (tag->type == CUSTOM) {
        array_delete(&tag->custom_tag_name);
    }
}

static inline bool tag_is_void(const Tag *self) {
    return self->type < END_OF_VOID_TAGS;
}

static inline bool tag_eq(const Tag *self, const Tag *other) {
    if (self->type != other->type) return false;
    if (self->type == CUSTOM) {
        if (self->custom_tag_name.size != other->custom_tag_name.size) {
            return false;
        }
        if (memcmp(
            self->custom_tag_name.contents,
            other->custom_tag_name.contents,
            self->custom_tag_name.size
        ) != 0) {
            return false;
        }
    }
    return true;
}

static bool tag_can_contain(Tag *self, const Tag *other) {
    TagType child = other->type;

    switch (self->type) {
        case LI:
            return child != LI;

        case DT:
        case DD:
            return child != DT && child != DD;

        case P:
            for (int i = 0; i < 26; i++) {
                if (child == TAG_TYPES_NOT_ALLOWED_IN_PARAGRAPHS[i]) {
                    return false;
                }
            }
            return true;

        case COLGROUP:
            return child == COL;

        case RB:
        case RT:
        case RP:
            return child != RB && child != RT && child != RP;

        case OPTGROUP:
            return child != OPTGROUP;

        case TR:
            return child != TR;

        case TD:
        case TH:
            return child != TD && child != TH && child != TR;

        default:
            return true;
    }
}
