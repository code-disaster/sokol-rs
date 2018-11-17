#version 330
#ifdef GL_ARB_shading_language_420pack
#extension GL_ARB_shading_language_420pack : require
#endif

uniform vec4 Uniforms[1];
layout(location = 0) in vec2 position;
out vec2 uv;
layout(location = 1) in vec2 texcoord0;
out vec4 color;
layout(location = 2) in vec4 color0;

void main()
{
    gl_Position = vec4(((position / Uniforms[0].xy) - vec2(0.5)) * vec2(2.0, -2.0), 0.5, 1.0);
    uv = texcoord0;
    color = color0;
}

