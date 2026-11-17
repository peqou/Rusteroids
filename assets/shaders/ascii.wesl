import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;

@group(0) @binding(0)
var screen_texture: texture_2d<f32>;

@group(0) @binding(1)
var texture_sampler: sampler;

struct AsciiEffect {
    strength: f32,
}

@group(0) @binding(2)
var<uniform> settings: AsciiEffect;

// Набор символов от тёмного к яркому:
//   . : - + * # @
// Размер символа: 4x6
const GLYPHS: array<u32, 8> = array<u32, 8>(
    0x000000u,
    0x000200u,
    0x002200u,
    0x000F00u,
    0x004F40u,
    0x096F69u,
    0x0BF6FDu,
    0xE15D96u
);

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let resolution = vec2<f32>(
        textureDimensions(screen_texture)
    );

    let cell_size = vec2<f32>(6.0, 9.0);
    let pixel = in.uv * resolution;
    let cell = floor(pixel / cell_size);

    let sample_uv =
        (cell + vec2<f32>(0.5)) * cell_size / resolution;

    let color = textureSample(
        screen_texture,
        texture_sampler,
        sample_uv
    );

    let brightness = dot(
        color.rgb,
        vec3<f32>(0.2126, 0.7152, 0.0722)
    );

    // Отличаем объекты от почти чёрного космоса
    let strongest = max(
        color.r,
        max(color.g, color.b)
    );

    if strongest < 0.035 {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }

    // Усиливаем слабое освещение
    let boosted = sqrt(clamp(
        brightness * 3.0,
        0.0,
        1.0
    ));

    let level = u32(clamp(
        floor(boosted * 6.0) + 1.0,
        1.0,
        7.0
    ));

    let glyph = GLYPHS[level];

    let local = fract(pixel / cell_size);
    let gx = u32(floor(local.x * 4.0));
    let gy = u32(floor(local.y * 6.0));

    let bit_index = gy * 4u + gx;
    let ink = f32((glyph >> bit_index) & 1u);

    // Сохраняем оттенок, но делаем цвет ярче
    let normalized = color.rgb / max(strongest, 0.001);
    let illuminated = normalized * (0.55 + boosted * 0.75);

    let background = vec3<f32>(0.0);

    let ascii_color = mix(
        background,
        illuminated * ink,
        settings.strength
    );

    return vec4<f32>(ascii_color, 1.0);
}
