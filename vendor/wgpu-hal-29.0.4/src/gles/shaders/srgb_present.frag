#version 300 es
precision mediump float;
in vec2 uv;
uniform sampler2D present_texture;
out vec4 frag;
vec4 linear_to_srgb(vec4 linear) {
    vec3 color_linear = linear.rgb;
    vec3 selector = ceil(color_linear - 0.0031308); // 0 if under value, 1 if over
    vec3 under = 12.92 * color_linear;
    vec3 over = 1.055 * pow(color_linear, vec3(0.41666)) - 0.055;
    vec3 result = mix(under, over, selector);
    return vec4(result, linear.a);
}
void main() {
  frag = linear_to_srgb(texture(present_texture, uv));
}