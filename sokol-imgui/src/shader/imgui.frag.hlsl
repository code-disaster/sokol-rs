Texture2D<float4> tex;
SamplerState _tex_sampler;

static float4 frag_color;
static float2 uv;
static float4 color;

struct SPIRV_Cross_Input
{
    float2 uv : TEXCOORD0;
    float4 color : TEXCOORD1;
};

struct SPIRV_Cross_Output
{
    float4 frag_color : SV_Target0;
};

void frag_main()
{
    frag_color = tex.Sample(_tex_sampler, uv) * color;
}

SPIRV_Cross_Output main(SPIRV_Cross_Input stage_input)
{
    uv = stage_input.uv;
    color = stage_input.color;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.frag_color = frag_color;
    return stage_output;
}
