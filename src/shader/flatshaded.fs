#version 330

in vec3 cameraPos;
in vec4 worldSpacePos;
in vec4 cameraSpacePos;

out vec4 o_color;

void main() {
  vec3 toCam = normalize(cameraPos - worldSpacePos.xyz);
  vec3 toLight = normalize(vec3(1.0, 1.0, 1.0) - worldSpacePos.xyz);
  float diffuseFactor = max(dot(toCam, toLight), 0.0);
  vec3 color = vec3(1.0, 1.0, 1.0) * (0.1 + diffuseFactor);
  o_color = vec4(color, 1.0);
}
