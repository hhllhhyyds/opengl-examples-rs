#version 140

in vec3 v_normal;
in vec3 v_eye_to_light;

out vec4 color;

uniform vec3 u_light;

void main() {
    vec3 c_surface = vec3(0.8, 0.7, 0.6);
    vec3 c_cool = vec3(0.0, 0.0, 0.55) + 0.25 * c_surface;
    vec3 c_warm = vec3(0.3, 0.3, 0.0) + 0.25 * c_surface;
    vec3 c_highlight = vec3(1.0, 1.0, 1.0);
    vec3 n_u_light = normalize(u_light);
    float t = (dot(v_normal, n_u_light) + 1.0) / 2.0;
    vec3 r = 2.0 * dot(v_normal, n_u_light) * v_normal - n_u_light;
    vec3 v = v_eye_to_light;
    float s = clamp(100.0 * dot(r, v) - 97.0, 0.0, 1.0);
    color = vec4(s * c_highlight + (1.0 - s) * (t * c_warm + (1.0 - t) * c_cool), 1.0);
}