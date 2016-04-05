#version 330

uniform mat4 u_model_world;
uniform mat4 u_world_cam;
uniform mat4 u_projection;

in vec3 a_pos;

void main() {
  gl_Position = u_projection * u_world_cam * u_model_world * vec4(a_pos, 1.0);
}
