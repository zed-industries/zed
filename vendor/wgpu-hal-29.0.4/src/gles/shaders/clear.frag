uniform vec4 color;
//Hack: Some WebGL implementations don't find "color" otherwise.
uniform vec4 color_workaround;
out vec4 frag;
void main() {
  frag = color + color_workaround;
}
