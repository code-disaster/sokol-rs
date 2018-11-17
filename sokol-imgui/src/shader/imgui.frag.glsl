#version 330
#ifdef GL_ARB_shading_language_420pack
#extension GL_ARB_shading_language_420pack : require
#endif

uniform sampler2D tex;

layout(location = 0) out vec4 frag_color;
in vec2 uv;
in vec4 color;

void main()
{
    frag_color = texture(tex, uv) * color;
}

