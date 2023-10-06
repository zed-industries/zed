use gpui3::serde_json::{self, json};
use ui::Theme;

pub fn rose_pine() -> Theme {
    serde_json::from_value(json! {
        {
          "name": "Rosé Pine",
          "is_light": false,
          "ramps": {},
          "lowest": {
            "base": {
              "default": {
                "background": "#292739",
                "border": "#423f55",
                "foreground": "#e0def4"
              },
              "hovered": {
                "background": "#423f55",
                "border": "#423f55",
                "foreground": "#e0def4"
              },
              "pressed": {
                "background": "#4e4b63",
                "border": "#423f55",
                "foreground": "#e0def4"
              },
              "active": {
                "background": "#47445b",
                "border": "#36334a",
                "foreground": "#e0def4"
              },
              "disabled": {
                "background": "#292739",
                "border": "#353347",
                "foreground": "#2f2b43"
              },
              "inverted": {
                "background": "#e0def4",
                "border": "#191724",
                "foreground": "#4b4860"
              }
            },
            "variant": {
              "default": {
                "background": "#292739",
                "border": "#423f55",
                "foreground": "#75718e"
              },
              "hovered": {
                "background": "#423f55",
                "border": "#423f55",
                "foreground": "#75718e"
              },
              "pressed": {
                "background": "#4e4b63",
                "border": "#423f55",
                "foreground": "#75718e"
              },
              "active": {
                "background": "#47445b",
                "border": "#36334a",
                "foreground": "#e0def4"
              },
              "disabled": {
                "background": "#292739",
                "border": "#353347",
                "foreground": "#2f2b43"
              },
              "inverted": {
                "background": "#e0def4",
                "border": "#191724",
                "foreground": "#4b4860"
              }
            },
            "on": {
              "default": {
                "background": "#1d1b2a",
                "border": "#232132",
                "foreground": "#e0def4"
              },
              "hovered": {
                "background": "#232132",
                "border": "#232132",
                "foreground": "#e0def4"
              },
              "pressed": {
                "background": "#2f2d40",
                "border": "#232132",
                "foreground": "#e0def4"
              },
              "active": {
                "background": "#403e53",
                "border": "#504d65",
                "foreground": "#e0def4"
              },
              "disabled": {
                "background": "#1d1b2a",
                "border": "#1e1c2c",
                "foreground": "#3b384f"
              },
              "inverted": {
                "background": "#e0def4",
                "border": "#191724",
                "foreground": "#3b394e"
              }
            },
            "accent": {
              "default": {
                "background": "#2f3739",
                "border": "#435255",
                "foreground": "#9cced7"
              },
              "hovered": {
                "background": "#435255",
                "border": "#435255",
                "foreground": "#9cced7"
              },
              "pressed": {
                "background": "#4e6164",
                "border": "#435255",
                "foreground": "#9cced7"
              },
              "active": {
                "background": "#5d757a",
                "border": "#6e8f94",
                "foreground": "#fbfdfd"
              },
              "disabled": {
                "background": "#2f3739",
                "border": "#3a4446",
                "foreground": "#85aeb5"
              },
              "inverted": {
                "background": "#fbfdfd",
                "border": "#171717",
                "foreground": "#587074"
              }
            },
            "positive": {
              "default": {
                "background": "#182e23",
                "border": "#254839",
                "foreground": "#5dc2a3"
              },
              "hovered": {
                "background": "#254839",
                "border": "#254839",
                "foreground": "#5dc2a3"
              },
              "pressed": {
                "background": "#2c5645",
                "border": "#254839",
                "foreground": "#5dc2a3"
              },
              "active": {
                "background": "#356b57",
                "border": "#40836c",
                "foreground": "#f9fdfb"
              },
              "disabled": {
                "background": "#182e23",
                "border": "#1e3b2e",
                "foreground": "#4ea287"
              },
              "inverted": {
                "background": "#f9fdfb",
                "border": "#000e00",
                "foreground": "#326552"
              }
            },
            "warning": {
              "default": {
                "background": "#50341a",
                "border": "#6d4d2b",
                "foreground": "#f5c177"
              },
              "hovered": {
                "background": "#6d4d2b",
                "border": "#6d4d2b",
                "foreground": "#f5c177"
              },
              "pressed": {
                "background": "#7e5a34",
                "border": "#6d4d2b",
                "foreground": "#f5c177"
              },
              "active": {
                "background": "#946e41",
                "border": "#b0854f",
                "foreground": "#fffcf9"
              },
              "disabled": {
                "background": "#50341a",
                "border": "#5e4023",
                "foreground": "#d2a263"
              },
              "inverted": {
                "background": "#fffcf9",
                "border": "#2c1600",
                "foreground": "#8e683c"
              }
            },
            "negative": {
              "default": {
                "background": "#431820",
                "border": "#612834",
                "foreground": "#ea6f92"
              },
              "hovered": {
                "background": "#612834",
                "border": "#612834",
                "foreground": "#ea6f92"
              },
              "pressed": {
                "background": "#71303f",
                "border": "#612834",
                "foreground": "#ea6f92"
              },
              "active": {
                "background": "#883c4f",
                "border": "#a44961",
                "foreground": "#fff9fa"
              },
              "disabled": {
                "background": "#431820",
                "border": "#52202a",
                "foreground": "#c75c79"
              },
              "inverted": {
                "background": "#fff9fa",
                "border": "#230000",
                "foreground": "#82384a"
              }
            }
          },
          "middle": {
            "base": {
              "default": {
                "background": "#1d1b2a",
                "border": "#232132",
                "foreground": "#e0def4"
              },
              "hovered": {
                "background": "#232132",
                "border": "#232132",
                "foreground": "#e0def4"
              },
              "pressed": {
                "background": "#2f2d40",
                "border": "#232132",
                "foreground": "#e0def4"
              },
              "active": {
                "background": "#403e53",
                "border": "#504d65",
                "foreground": "#e0def4"
              },
              "disabled": {
                "background": "#1d1b2a",
                "border": "#1e1c2c",
                "foreground": "#3b384f"
              },
              "inverted": {
                "background": "#e0def4",
                "border": "#191724",
                "foreground": "#3b394e"
              }
            },
            "variant": {
              "default": {
                "background": "#1d1b2a",
                "border": "#232132",
                "foreground": "#75718e"
              },
              "hovered": {
                "background": "#232132",
                "border": "#232132",
                "foreground": "#75718e"
              },
              "pressed": {
                "background": "#2f2d40",
                "border": "#232132",
                "foreground": "#75718e"
              },
              "active": {
                "background": "#403e53",
                "border": "#504d65",
                "foreground": "#e0def4"
              },
              "disabled": {
                "background": "#1d1b2a",
                "border": "#1e1c2c",
                "foreground": "#3b384f"
              },
              "inverted": {
                "background": "#e0def4",
                "border": "#191724",
                "foreground": "#3b394e"
              }
            },
            "on": {
              "default": {
                "background": "#191724",
                "border": "#1c1a29",
                "foreground": "#e0def4"
              },
              "hovered": {
                "background": "#1c1a29",
                "border": "#1c1a29",
                "foreground": "#e0def4"
              },
              "pressed": {
                "background": "#1d1b2b",
                "border": "#1c1a29",
                "foreground": "#e0def4"
              },
              "active": {
                "background": "#222031",
                "border": "#353347",
                "foreground": "#e0def4"
              },
              "disabled": {
                "background": "#191724",
                "border": "#1a1826",
                "foreground": "#4e4b63"
              },
              "inverted": {
                "background": "#e0def4",
                "border": "#191724",
                "foreground": "#1f1d2e"
              }
            },
            "accent": {
              "default": {
                "background": "#2f3739",
                "border": "#435255",
                "foreground": "#9cced7"
              },
              "hovered": {
                "background": "#435255",
                "border": "#435255",
                "foreground": "#9cced7"
              },
              "pressed": {
                "background": "#4e6164",
                "border": "#435255",
                "foreground": "#9cced7"
              },
              "active": {
                "background": "#5d757a",
                "border": "#6e8f94",
                "foreground": "#fbfdfd"
              },
              "disabled": {
                "background": "#2f3739",
                "border": "#3a4446",
                "foreground": "#85aeb5"
              },
              "inverted": {
                "background": "#fbfdfd",
                "border": "#171717",
                "foreground": "#587074"
              }
            },
            "positive": {
              "default": {
                "background": "#182e23",
                "border": "#254839",
                "foreground": "#5dc2a3"
              },
              "hovered": {
                "background": "#254839",
                "border": "#254839",
                "foreground": "#5dc2a3"
              },
              "pressed": {
                "background": "#2c5645",
                "border": "#254839",
                "foreground": "#5dc2a3"
              },
              "active": {
                "background": "#356b57",
                "border": "#40836c",
                "foreground": "#f9fdfb"
              },
              "disabled": {
                "background": "#182e23",
                "border": "#1e3b2e",
                "foreground": "#4ea287"
              },
              "inverted": {
                "background": "#f9fdfb",
                "border": "#000e00",
                "foreground": "#326552"
              }
            },
            "warning": {
              "default": {
                "background": "#50341a",
                "border": "#6d4d2b",
                "foreground": "#f5c177"
              },
              "hovered": {
                "background": "#6d4d2b",
                "border": "#6d4d2b",
                "foreground": "#f5c177"
              },
              "pressed": {
                "background": "#7e5a34",
                "border": "#6d4d2b",
                "foreground": "#f5c177"
              },
              "active": {
                "background": "#946e41",
                "border": "#b0854f",
                "foreground": "#fffcf9"
              },
              "disabled": {
                "background": "#50341a",
                "border": "#5e4023",
                "foreground": "#d2a263"
              },
              "inverted": {
                "background": "#fffcf9",
                "border": "#2c1600",
                "foreground": "#8e683c"
              }
            },
            "negative": {
              "default": {
                "background": "#431820",
                "border": "#612834",
                "foreground": "#ea6f92"
              },
              "hovered": {
                "background": "#612834",
                "border": "#612834",
                "foreground": "#ea6f92"
              },
              "pressed": {
                "background": "#71303f",
                "border": "#612834",
                "foreground": "#ea6f92"
              },
              "active": {
                "background": "#883c4f",
                "border": "#a44961",
                "foreground": "#fff9fa"
              },
              "disabled": {
                "background": "#431820",
                "border": "#52202a",
                "foreground": "#c75c79"
              },
              "inverted": {
                "background": "#fff9fa",
                "border": "#230000",
                "foreground": "#82384a"
              }
            }
          },
          "highest": {
            "base": {
              "default": {
                "background": "#191724",
                "border": "#1c1a29",
                "foreground": "#e0def4"
              },
              "hovered": {
                "background": "#1c1a29",
                "border": "#1c1a29",
                "foreground": "#e0def4"
              },
              "pressed": {
                "background": "#1d1b2b",
                "border": "#1c1a29",
                "foreground": "#e0def4"
              },
              "active": {
                "background": "#222031",
                "border": "#353347",
                "foreground": "#e0def4"
              },
              "disabled": {
                "background": "#191724",
                "border": "#1a1826",
                "foreground": "#4e4b63"
              },
              "inverted": {
                "background": "#e0def4",
                "border": "#191724",
                "foreground": "#1f1d2e"
              }
            },
            "variant": {
              "default": {
                "background": "#191724",
                "border": "#1c1a29",
                "foreground": "#75718e"
              },
              "hovered": {
                "background": "#1c1a29",
                "border": "#1c1a29",
                "foreground": "#75718e"
              },
              "pressed": {
                "background": "#1d1b2b",
                "border": "#1c1a29",
                "foreground": "#75718e"
              },
              "active": {
                "background": "#222031",
                "border": "#353347",
                "foreground": "#e0def4"
              },
              "disabled": {
                "background": "#191724",
                "border": "#1a1826",
                "foreground": "#4e4b63"
              },
              "inverted": {
                "background": "#e0def4",
                "border": "#191724",
                "foreground": "#1f1d2e"
              }
            },
            "on": {
              "default": {
                "background": "#1d1b2a",
                "border": "#232132",
                "foreground": "#e0def4"
              },
              "hovered": {
                "background": "#232132",
                "border": "#232132",
                "foreground": "#e0def4"
              },
              "pressed": {
                "background": "#2f2d40",
                "border": "#232132",
                "foreground": "#e0def4"
              },
              "active": {
                "background": "#403e53",
                "border": "#504d65",
                "foreground": "#e0def4"
              },
              "disabled": {
                "background": "#1d1b2a",
                "border": "#1e1c2c",
                "foreground": "#3b384f"
              },
              "inverted": {
                "background": "#e0def4",
                "border": "#191724",
                "foreground": "#3b394e"
              }
            },
            "accent": {
              "default": {
                "background": "#2f3739",
                "border": "#435255",
                "foreground": "#9cced7"
              },
              "hovered": {
                "background": "#435255",
                "border": "#435255",
                "foreground": "#9cced7"
              },
              "pressed": {
                "background": "#4e6164",
                "border": "#435255",
                "foreground": "#9cced7"
              },
              "active": {
                "background": "#5d757a",
                "border": "#6e8f94",
                "foreground": "#fbfdfd"
              },
              "disabled": {
                "background": "#2f3739",
                "border": "#3a4446",
                "foreground": "#85aeb5"
              },
              "inverted": {
                "background": "#fbfdfd",
                "border": "#171717",
                "foreground": "#587074"
              }
            },
            "positive": {
              "default": {
                "background": "#182e23",
                "border": "#254839",
                "foreground": "#5dc2a3"
              },
              "hovered": {
                "background": "#254839",
                "border": "#254839",
                "foreground": "#5dc2a3"
              },
              "pressed": {
                "background": "#2c5645",
                "border": "#254839",
                "foreground": "#5dc2a3"
              },
              "active": {
                "background": "#356b57",
                "border": "#40836c",
                "foreground": "#f9fdfb"
              },
              "disabled": {
                "background": "#182e23",
                "border": "#1e3b2e",
                "foreground": "#4ea287"
              },
              "inverted": {
                "background": "#f9fdfb",
                "border": "#000e00",
                "foreground": "#326552"
              }
            },
            "warning": {
              "default": {
                "background": "#50341a",
                "border": "#6d4d2b",
                "foreground": "#f5c177"
              },
              "hovered": {
                "background": "#6d4d2b",
                "border": "#6d4d2b",
                "foreground": "#f5c177"
              },
              "pressed": {
                "background": "#7e5a34",
                "border": "#6d4d2b",
                "foreground": "#f5c177"
              },
              "active": {
                "background": "#946e41",
                "border": "#b0854f",
                "foreground": "#fffcf9"
              },
              "disabled": {
                "background": "#50341a",
                "border": "#5e4023",
                "foreground": "#d2a263"
              },
              "inverted": {
                "background": "#fffcf9",
                "border": "#2c1600",
                "foreground": "#8e683c"
              }
            },
            "negative": {
              "default": {
                "background": "#431820",
                "border": "#612834",
                "foreground": "#ea6f92"
              },
              "hovered": {
                "background": "#612834",
                "border": "#612834",
                "foreground": "#ea6f92"
              },
              "pressed": {
                "background": "#71303f",
                "border": "#612834",
                "foreground": "#ea6f92"
              },
              "active": {
                "background": "#883c4f",
                "border": "#a44961",
                "foreground": "#fff9fa"
              },
              "disabled": {
                "background": "#431820",
                "border": "#52202a",
                "foreground": "#c75c79"
              },
              "inverted": {
                "background": "#fff9fa",
                "border": "#230000",
                "foreground": "#82384a"
              }
            }
          },
          "popover_shadow": {
            "blur": 4,
            "color": "#00000033",
            "offset": [
              1,
              2
            ]
          },
          "modal_shadow": {
            "blur": 16,
            "color": "#00000033",
            "offset": [
              0,
              2
            ]
          },
          "players": {
            "0": {
              "selection": "#9cced73d",
              "cursor": "#9cced7"
            },
            "1": {
              "selection": "#5dc2a33d",
              "cursor": "#5dc2a3"
            },
            "2": {
              "selection": "#9d76913d",
              "cursor": "#9d7691"
            },
            "3": {
              "selection": "#c4a7e63d",
              "cursor": "#c4a7e6"
            },
            "4": {
              "selection": "#c4a7e63d",
              "cursor": "#c4a7e6"
            },
            "5": {
              "selection": "#32748f3d",
              "cursor": "#32748f"
            },
            "6": {
              "selection": "#ea6f923d",
              "cursor": "#ea6f92"
            },
            "7": {
              "selection": "#f5c1773d",
              "cursor": "#f5c177"
            }
          },
          "syntax": {
            "comment": {
              "color": "#6e6a86"
            },
            "operator": {
              "color": "#31748f"
            },
            "punctuation": {
              "color": "#908caa"
            },
            "variable": {
              "color": "#e0def4"
            },
            "string": {
              "color": "#f6c177"
            },
            "type": {
              "color": "#9ccfd8"
            },
            "type.builtin": {
              "color": "#9ccfd8"
            },
            "boolean": {
              "color": "#ebbcba"
            },
            "function": {
              "color": "#ebbcba"
            },
            "keyword": {
              "color": "#31748f"
            },
            "tag": {
              "color": "#9ccfd8"
            },
            "function.method": {
              "color": "#ebbcba"
            },
            "title": {
              "color": "#f6c177"
            },
            "link_text": {
              "color": "#9ccfd8",
              "italic": false
            },
            "link_uri": {
              "color": "#ebbcba"
            }
          },
          "color_family": {
            "neutral": {
              "low": 11.568627450980392,
              "high": 91.37254901960785,
              "range": 79.80392156862746,
              "scaling_value": 1.2530712530712529
            },
            "red": {
              "low": 6.862745098039216,
              "high": 100,
              "range": 93.13725490196079,
              "scaling_value": 1.0736842105263158
            },
            "orange": {
              "low": 5.490196078431373,
              "high": 100,
              "range": 94.50980392156863,
              "scaling_value": 1.058091286307054
            },
            "yellow": {
              "low": 8.627450980392156,
              "high": 100,
              "range": 91.37254901960785,
              "scaling_value": 1.094420600858369
            },
            "green": {
              "low": 2.7450980392156863,
              "high": 100,
              "range": 97.25490196078431,
              "scaling_value": 1.028225806451613
            },
            "cyan": {
              "low": 0,
              "high": 100,
              "range": 100,
              "scaling_value": 1
            },
            "blue": {
              "low": 9.019607843137255,
              "high": 100,
              "range": 90.98039215686275,
              "scaling_value": 1.0991379310344827
            },
            "violet": {
              "low": 5.490196078431373,
              "high": 100,
              "range": 94.50980392156863,
              "scaling_value": 1.058091286307054
            },
            "magenta": {
              "low": 0,
              "high": 100,
              "range": 100,
              "scaling_value": 1
            }
          }
        }
    })
    .unwrap()
}

pub fn rose_pine_dawn() -> Theme {
    serde_json::from_value(json!({
      "name": "Rosé Pine Dawn",
      "is_light": true,
      "ramps": {},
      "lowest": {
        "base": {
          "default": {
            "background": "#dcd8d8",
            "border": "#dcd6d5",
            "foreground": "#575279"
          },
          "hovered": {
            "background": "#dcd6d5",
            "border": "#dcd6d5",
            "foreground": "#575279"
          },
          "pressed": {
            "background": "#efe6df",
            "border": "#dcd6d5",
            "foreground": "#575279"
          },
          "active": {
            "background": "#c1bac1",
            "border": "#a9a3b0",
            "foreground": "#575279"
          },
          "disabled": {
            "background": "#dcd8d8",
            "border": "#d0cccf",
            "foreground": "#938fa3"
          },
          "inverted": {
            "background": "#575279",
            "border": "#faf4ed",
            "foreground": "#c7c0c5"
          }
        },
        "variant": {
          "default": {
            "background": "#dcd8d8",
            "border": "#dcd6d5",
            "foreground": "#706c8c"
          },
          "hovered": {
            "background": "#dcd6d5",
            "border": "#dcd6d5",
            "foreground": "#706c8c"
          },
          "pressed": {
            "background": "#efe6df",
            "border": "#dcd6d5",
            "foreground": "#706c8c"
          },
          "active": {
            "background": "#c1bac1",
            "border": "#a9a3b0",
            "foreground": "#575279"
          },
          "disabled": {
            "background": "#dcd8d8",
            "border": "#d0cccf",
            "foreground": "#938fa3"
          },
          "inverted": {
            "background": "#575279",
            "border": "#faf4ed",
            "foreground": "#c7c0c5"
          }
        },
        "on": {
          "default": {
            "background": "#fef9f2",
            "border": "#e5e0df",
            "foreground": "#575279"
          },
          "hovered": {
            "background": "#e5e0df",
            "border": "#e5e0df",
            "foreground": "#575279"
          },
          "pressed": {
            "background": "#d4d0d2",
            "border": "#e5e0df",
            "foreground": "#575279"
          },
          "active": {
            "background": "#dbd5d4",
            "border": "#dbd3d1",
            "foreground": "#575279"
          },
          "disabled": {
            "background": "#fef9f2",
            "border": "#f6f1eb",
            "foreground": "#b1abb5"
          },
          "inverted": {
            "background": "#575279",
            "border": "#faf4ed",
            "foreground": "#d6d1d1"
          }
        },
        "accent": {
          "default": {
            "background": "#dde9eb",
            "border": "#c3d7db",
            "foreground": "#57949f"
          },
          "hovered": {
            "background": "#c3d7db",
            "border": "#c3d7db",
            "foreground": "#57949f"
          },
          "pressed": {
            "background": "#b6cfd3",
            "border": "#c3d7db",
            "foreground": "#57949f"
          },
          "active": {
            "background": "#a3c3c9",
            "border": "#8db6bd",
            "foreground": "#06090a"
          },
          "disabled": {
            "background": "#dde9eb",
            "border": "#d0e0e3",
            "foreground": "#72a5ae"
          },
          "inverted": {
            "background": "#06090a",
            "border": "#ffffff",
            "foreground": "#a8c7cd"
          }
        },
        "positive": {
          "default": {
            "background": "#dbeee7",
            "border": "#bee0d5",
            "foreground": "#3eaa8e"
          },
          "hovered": {
            "background": "#bee0d5",
            "border": "#bee0d5",
            "foreground": "#3eaa8e"
          },
          "pressed": {
            "background": "#b0dacb",
            "border": "#bee0d5",
            "foreground": "#3eaa8e"
          },
          "active": {
            "background": "#9bd0bf",
            "border": "#82c6b1",
            "foreground": "#060a09"
          },
          "disabled": {
            "background": "#dbeee7",
            "border": "#cde7de",
            "foreground": "#63b89f"
          },
          "inverted": {
            "background": "#060a09",
            "border": "#ffffff",
            "foreground": "#a1d4c3"
          }
        },
        "warning": {
          "default": {
            "background": "#ffebd6",
            "border": "#ffdab7",
            "foreground": "#e99d35"
          },
          "hovered": {
            "background": "#ffdab7",
            "border": "#ffdab7",
            "foreground": "#e99d35"
          },
          "pressed": {
            "background": "#fed2a6",
            "border": "#ffdab7",
            "foreground": "#e99d35"
          },
          "active": {
            "background": "#fbc891",
            "border": "#f7bc77",
            "foreground": "#330704"
          },
          "disabled": {
            "background": "#ffebd6",
            "border": "#ffe2c7",
            "foreground": "#f1ac57"
          },
          "inverted": {
            "background": "#330704",
            "border": "#ffffff",
            "foreground": "#fccb97"
          }
        },
        "negative": {
          "default": {
            "background": "#f1dfe3",
            "border": "#e6c6cd",
            "foreground": "#b4647a"
          },
          "hovered": {
            "background": "#e6c6cd",
            "border": "#e6c6cd",
            "foreground": "#b4647a"
          },
          "pressed": {
            "background": "#e0bac2",
            "border": "#e6c6cd",
            "foreground": "#b4647a"
          },
          "active": {
            "background": "#d8a8b3",
            "border": "#ce94a3",
            "foreground": "#0b0708"
          },
          "disabled": {
            "background": "#f1dfe3",
            "border": "#ecd2d8",
            "foreground": "#c17b8e"
          },
          "inverted": {
            "background": "#0b0708",
            "border": "#ffffff",
            "foreground": "#dbadb8"
          }
        }
      },
      "middle": {
        "base": {
          "default": {
            "background": "#fef9f2",
            "border": "#e5e0df",
            "foreground": "#575279"
          },
          "hovered": {
            "background": "#e5e0df",
            "border": "#e5e0df",
            "foreground": "#575279"
          },
          "pressed": {
            "background": "#d4d0d2",
            "border": "#e5e0df",
            "foreground": "#575279"
          },
          "active": {
            "background": "#dbd5d4",
            "border": "#dbd3d1",
            "foreground": "#575279"
          },
          "disabled": {
            "background": "#fef9f2",
            "border": "#f6f1eb",
            "foreground": "#b1abb5"
          },
          "inverted": {
            "background": "#575279",
            "border": "#faf4ed",
            "foreground": "#d6d1d1"
          }
        },
        "variant": {
          "default": {
            "background": "#fef9f2",
            "border": "#e5e0df",
            "foreground": "#706c8c"
          },
          "hovered": {
            "background": "#e5e0df",
            "border": "#e5e0df",
            "foreground": "#706c8c"
          },
          "pressed": {
            "background": "#d4d0d2",
            "border": "#e5e0df",
            "foreground": "#706c8c"
          },
          "active": {
            "background": "#dbd5d4",
            "border": "#dbd3d1",
            "foreground": "#575279"
          },
          "disabled": {
            "background": "#fef9f2",
            "border": "#f6f1eb",
            "foreground": "#b1abb5"
          },
          "inverted": {
            "background": "#575279",
            "border": "#faf4ed",
            "foreground": "#d6d1d1"
          }
        },
        "on": {
          "default": {
            "background": "#faf4ed",
            "border": "#fdf8f1",
            "foreground": "#575279"
          },
          "hovered": {
            "background": "#fdf8f1",
            "border": "#fdf8f1",
            "foreground": "#575279"
          },
          "pressed": {
            "background": "#fdf8f2",
            "border": "#fdf8f1",
            "foreground": "#575279"
          },
          "active": {
            "background": "#e6e1e0",
            "border": "#d0cccf",
            "foreground": "#575279"
          },
          "disabled": {
            "background": "#faf4ed",
            "border": "#fcf6ef",
            "foreground": "#efe6df"
          },
          "inverted": {
            "background": "#575279",
            "border": "#faf4ed",
            "foreground": "#ede9e5"
          }
        },
        "accent": {
          "default": {
            "background": "#dde9eb",
            "border": "#c3d7db",
            "foreground": "#57949f"
          },
          "hovered": {
            "background": "#c3d7db",
            "border": "#c3d7db",
            "foreground": "#57949f"
          },
          "pressed": {
            "background": "#b6cfd3",
            "border": "#c3d7db",
            "foreground": "#57949f"
          },
          "active": {
            "background": "#a3c3c9",
            "border": "#8db6bd",
            "foreground": "#06090a"
          },
          "disabled": {
            "background": "#dde9eb",
            "border": "#d0e0e3",
            "foreground": "#72a5ae"
          },
          "inverted": {
            "background": "#06090a",
            "border": "#ffffff",
            "foreground": "#a8c7cd"
          }
        },
        "positive": {
          "default": {
            "background": "#dbeee7",
            "border": "#bee0d5",
            "foreground": "#3eaa8e"
          },
          "hovered": {
            "background": "#bee0d5",
            "border": "#bee0d5",
            "foreground": "#3eaa8e"
          },
          "pressed": {
            "background": "#b0dacb",
            "border": "#bee0d5",
            "foreground": "#3eaa8e"
          },
          "active": {
            "background": "#9bd0bf",
            "border": "#82c6b1",
            "foreground": "#060a09"
          },
          "disabled": {
            "background": "#dbeee7",
            "border": "#cde7de",
            "foreground": "#63b89f"
          },
          "inverted": {
            "background": "#060a09",
            "border": "#ffffff",
            "foreground": "#a1d4c3"
          }
        },
        "warning": {
          "default": {
            "background": "#ffebd6",
            "border": "#ffdab7",
            "foreground": "#e99d35"
          },
          "hovered": {
            "background": "#ffdab7",
            "border": "#ffdab7",
            "foreground": "#e99d35"
          },
          "pressed": {
            "background": "#fed2a6",
            "border": "#ffdab7",
            "foreground": "#e99d35"
          },
          "active": {
            "background": "#fbc891",
            "border": "#f7bc77",
            "foreground": "#330704"
          },
          "disabled": {
            "background": "#ffebd6",
            "border": "#ffe2c7",
            "foreground": "#f1ac57"
          },
          "inverted": {
            "background": "#330704",
            "border": "#ffffff",
            "foreground": "#fccb97"
          }
        },
        "negative": {
          "default": {
            "background": "#f1dfe3",
            "border": "#e6c6cd",
            "foreground": "#b4647a"
          },
          "hovered": {
            "background": "#e6c6cd",
            "border": "#e6c6cd",
            "foreground": "#b4647a"
          },
          "pressed": {
            "background": "#e0bac2",
            "border": "#e6c6cd",
            "foreground": "#b4647a"
          },
          "active": {
            "background": "#d8a8b3",
            "border": "#ce94a3",
            "foreground": "#0b0708"
          },
          "disabled": {
            "background": "#f1dfe3",
            "border": "#ecd2d8",
            "foreground": "#c17b8e"
          },
          "inverted": {
            "background": "#0b0708",
            "border": "#ffffff",
            "foreground": "#dbadb8"
          }
        }
      },
      "highest": {
        "base": {
          "default": {
            "background": "#faf4ed",
            "border": "#fdf8f1",
            "foreground": "#575279"
          },
          "hovered": {
            "background": "#fdf8f1",
            "border": "#fdf8f1",
            "foreground": "#575279"
          },
          "pressed": {
            "background": "#fdf8f2",
            "border": "#fdf8f1",
            "foreground": "#575279"
          },
          "active": {
            "background": "#e6e1e0",
            "border": "#d0cccf",
            "foreground": "#575279"
          },
          "disabled": {
            "background": "#faf4ed",
            "border": "#fcf6ef",
            "foreground": "#efe6df"
          },
          "inverted": {
            "background": "#575279",
            "border": "#faf4ed",
            "foreground": "#ede9e5"
          }
        },
        "variant": {
          "default": {
            "background": "#faf4ed",
            "border": "#fdf8f1",
            "foreground": "#706c8c"
          },
          "hovered": {
            "background": "#fdf8f1",
            "border": "#fdf8f1",
            "foreground": "#706c8c"
          },
          "pressed": {
            "background": "#fdf8f2",
            "border": "#fdf8f1",
            "foreground": "#706c8c"
          },
          "active": {
            "background": "#e6e1e0",
            "border": "#d0cccf",
            "foreground": "#575279"
          },
          "disabled": {
            "background": "#faf4ed",
            "border": "#fcf6ef",
            "foreground": "#efe6df"
          },
          "inverted": {
            "background": "#575279",
            "border": "#faf4ed",
            "foreground": "#ede9e5"
          }
        },
        "on": {
          "default": {
            "background": "#fef9f2",
            "border": "#e5e0df",
            "foreground": "#575279"
          },
          "hovered": {
            "background": "#e5e0df",
            "border": "#e5e0df",
            "foreground": "#575279"
          },
          "pressed": {
            "background": "#d4d0d2",
            "border": "#e5e0df",
            "foreground": "#575279"
          },
          "active": {
            "background": "#dbd5d4",
            "border": "#dbd3d1",
            "foreground": "#575279"
          },
          "disabled": {
            "background": "#fef9f2",
            "border": "#f6f1eb",
            "foreground": "#b1abb5"
          },
          "inverted": {
            "background": "#575279",
            "border": "#faf4ed",
            "foreground": "#d6d1d1"
          }
        },
        "accent": {
          "default": {
            "background": "#dde9eb",
            "border": "#c3d7db",
            "foreground": "#57949f"
          },
          "hovered": {
            "background": "#c3d7db",
            "border": "#c3d7db",
            "foreground": "#57949f"
          },
          "pressed": {
            "background": "#b6cfd3",
            "border": "#c3d7db",
            "foreground": "#57949f"
          },
          "active": {
            "background": "#a3c3c9",
            "border": "#8db6bd",
            "foreground": "#06090a"
          },
          "disabled": {
            "background": "#dde9eb",
            "border": "#d0e0e3",
            "foreground": "#72a5ae"
          },
          "inverted": {
            "background": "#06090a",
            "border": "#ffffff",
            "foreground": "#a8c7cd"
          }
        },
        "positive": {
          "default": {
            "background": "#dbeee7",
            "border": "#bee0d5",
            "foreground": "#3eaa8e"
          },
          "hovered": {
            "background": "#bee0d5",
            "border": "#bee0d5",
            "foreground": "#3eaa8e"
          },
          "pressed": {
            "background": "#b0dacb",
            "border": "#bee0d5",
            "foreground": "#3eaa8e"
          },
          "active": {
            "background": "#9bd0bf",
            "border": "#82c6b1",
            "foreground": "#060a09"
          },
          "disabled": {
            "background": "#dbeee7",
            "border": "#cde7de",
            "foreground": "#63b89f"
          },
          "inverted": {
            "background": "#060a09",
            "border": "#ffffff",
            "foreground": "#a1d4c3"
          }
        },
        "warning": {
          "default": {
            "background": "#ffebd6",
            "border": "#ffdab7",
            "foreground": "#e99d35"
          },
          "hovered": {
            "background": "#ffdab7",
            "border": "#ffdab7",
            "foreground": "#e99d35"
          },
          "pressed": {
            "background": "#fed2a6",
            "border": "#ffdab7",
            "foreground": "#e99d35"
          },
          "active": {
            "background": "#fbc891",
            "border": "#f7bc77",
            "foreground": "#330704"
          },
          "disabled": {
            "background": "#ffebd6",
            "border": "#ffe2c7",
            "foreground": "#f1ac57"
          },
          "inverted": {
            "background": "#330704",
            "border": "#ffffff",
            "foreground": "#fccb97"
          }
        },
        "negative": {
          "default": {
            "background": "#f1dfe3",
            "border": "#e6c6cd",
            "foreground": "#b4647a"
          },
          "hovered": {
            "background": "#e6c6cd",
            "border": "#e6c6cd",
            "foreground": "#b4647a"
          },
          "pressed": {
            "background": "#e0bac2",
            "border": "#e6c6cd",
            "foreground": "#b4647a"
          },
          "active": {
            "background": "#d8a8b3",
            "border": "#ce94a3",
            "foreground": "#0b0708"
          },
          "disabled": {
            "background": "#f1dfe3",
            "border": "#ecd2d8",
            "foreground": "#c17b8e"
          },
          "inverted": {
            "background": "#0b0708",
            "border": "#ffffff",
            "foreground": "#dbadb8"
          }
        }
      },
      "popover_shadow": {
        "blur": 4,
        "color": "#2c2a4d33",
        "offset": [
          1,
          2
        ]
      },
      "modal_shadow": {
        "blur": 16,
        "color": "#2c2a4d33",
        "offset": [
          0,
          2
        ]
      },
      "players": {
        "0": {
          "selection": "#57949f3d",
          "cursor": "#57949f"
        },
        "1": {
          "selection": "#3eaa8e3d",
          "cursor": "#3eaa8e"
        },
        "2": {
          "selection": "#7c697f3d",
          "cursor": "#7c697f"
        },
        "3": {
          "selection": "#907aa93d",
          "cursor": "#907aa9"
        },
        "4": {
          "selection": "#907aa93d",
          "cursor": "#907aa9"
        },
        "5": {
          "selection": "#2a69833d",
          "cursor": "#2a6983"
        },
        "6": {
          "selection": "#b4647a3d",
          "cursor": "#b4647a"
        },
        "7": {
          "selection": "#e99d353d",
          "cursor": "#e99d35"
        }
      },
      "syntax": {
        "comment": {
          "color": "#9893a5"
        },
        "operator": {
          "color": "#286983"
        },
        "punctuation": {
          "color": "#797593"
        },
        "variable": {
          "color": "#575279"
        },
        "string": {
          "color": "#ea9d34"
        },
        "type": {
          "color": "#56949f"
        },
        "type.builtin": {
          "color": "#56949f"
        },
        "boolean": {
          "color": "#d7827e"
        },
        "function": {
          "color": "#d7827e"
        },
        "keyword": {
          "color": "#286983"
        },
        "tag": {
          "color": "#56949f"
        },
        "function.method": {
          "color": "#d7827e"
        },
        "title": {
          "color": "#ea9d34"
        },
        "link_text": {
          "color": "#56949f",
          "italic": false
        },
        "link_uri": {
          "color": "#d7827e"
        }
      },
      "color_family": {
        "neutral": {
          "low": 39.80392156862745,
          "high": 95.49019607843137,
          "range": 55.686274509803916,
          "scaling_value": 1.7957746478873242
        },
        "red": {
          "low": 0,
          "high": 100,
          "range": 100,
          "scaling_value": 1
        },
        "orange": {
          "low": 0,
          "high": 100,
          "range": 100,
          "scaling_value": 1
        },
        "yellow": {
          "low": 8.823529411764707,
          "high": 100,
          "range": 91.17647058823529,
          "scaling_value": 1.0967741935483872
        },
        "green": {
          "low": 0,
          "high": 100,
          "range": 100,
          "scaling_value": 1
        },
        "cyan": {
          "low": 0,
          "high": 100,
          "range": 100,
          "scaling_value": 1
        },
        "blue": {
          "low": 0,
          "high": 100,
          "range": 100,
          "scaling_value": 1
        },
        "violet": {
          "low": 0,
          "high": 100,
          "range": 100,
          "scaling_value": 1
        },
        "magenta": {
          "low": 0,
          "high": 100,
          "range": 100,
          "scaling_value": 1
        }
      }
    }))
    .unwrap()
}
