#version 330

in vec3 cameraPos;
in vec4 worldSpacePos;
in vec4 cameraSpacePos;

in vec3 pos;
in vec3 norm;
in vec4 color;
in vec2 tex;

out vec4 o_color;

void main() {
  vec3 normal = normalize(norm);
  vec3 toCam = normalize(cameraPos - worldSpacePos.xyz);
  vec3 toLight = normalize(vec3(1.0, 20.0, 1.0) - worldSpacePos.xyz);
  float diffuseFactor = max(dot(normal, toLight), 0.0);
  vec3 diffColor = color.xyz * (0.1 + diffuseFactor);
  o_color = vec4(diffColor, color.a);
}
