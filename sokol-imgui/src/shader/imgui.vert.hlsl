cbuffer _21
{
    float2 _21_disp_size : packoffset(c0);
};

static float4 gl_Position;
static float2 position;
static float2 uv;
static float2 texcoord0;
static float4 color;
static float4 color0;

struct SPIRV_Cross_Input
{
    float2 position : TEXCOORD0;
    float2 texcoord0 : TEXCOORD1;
    float4 color0 : TEXCOORD2;
};

struct SPIRV_Cross_Output
{
    float2 uv : TEXCOORD0;
    float4 color : TEXCOORD1;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    gl_Position = float4(((position / _21_disp_size) - 0.5f.xx) * float2(2.0f, -2.0f), 0.5f, 1.0f);
    uv = texcoord0;
    color = color0;
}

SPIRV_Cross_Output main(SPIRV_Cross_Input stage_input)
{
    position = stage_input.position;
    texcoord0 = stage_input.texcoord0;
    color0 = stage_input.color0;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.uv = uv;
    stage_output.color = color;
    return stage_output;
}
