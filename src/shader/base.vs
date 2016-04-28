#version 330

uniform mat4 u_model_world;
uniform mat4 u_world_cam;
uniform mat4 u_projection;
uniform vec3 u_cam_pos;

in vec3 a_pos;
in vec3 a_norm;
in vec4 a_color;
in vec2 a_tex;

out vec3 cameraPos;
out vec4 worldSpacePos;
out vec4 cameraSpacePos;

void main() {
  cameraPos = u_cam_pos;
  worldSpacePos = u_model_world * vec4(a_pos, 1.0);
  cameraSpacePos = u_world_cam * worldSpacePos;
  gl_Position = u_projection * cameraSpacePos;
}
