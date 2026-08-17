// A triangle that fills the whole screen
vec2[3] TRIANGLE_POS = vec2[](
  vec2( 0.0, -3.0),
  vec2(-3.0,  1.0),
  vec2( 3.0,  1.0)
);
void main() {
  gl_Position = vec4(TRIANGLE_POS[gl_VertexID], 0.0, 1.0);
}