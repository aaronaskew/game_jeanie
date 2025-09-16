shader_type canvas_item;
render_mode unshaded;

uniform float curvature = 6.0;
uniform float vignette_multiplier = 0.7;
uniform float scanline_brightness = 0.1;
uniform float scanline_height = 180.0;
uniform vec3 frost:source_color = vec3(1.0,1.0,1.0);
uniform float lod: hint_range(0.0, 5.0) = 0.2;

uniform sampler2D screen_texture: hint_screen_texture, filter_linear_mipmap;

void fragment() {
	//vec2 centered_uv = vec2(UV.x, 1.0 - UV.y) * 2.0 - 1.0;
	vec2 centered_uv = SCREEN_UV * 2.0 - 1.0;
	vec2 uv_offset = centered_uv.yx / curvature;
	vec2 warped_uv = centered_uv + centered_uv * uv_offset * uv_offset;
	vec3 cutoff = vec3(step(abs(warped_uv.x), 1.0) * step(abs(warped_uv.y), 1.0));
	vec3 scanlines = vec3(sin(2.0 * warped_uv.y * scanline_height)
		* scanline_brightness + 1.0 - scanline_brightness);
	vec3 vignette = vec3(length(pow(abs(centered_uv), vec2(4.0)) / 0.9));

	vec3 screen_color = textureLod(screen_texture, (warped_uv + 1.0) / 2.0, lod).rgb * cutoff * scanlines * frost;
	screen_color -= vignette * vignette_multiplier;
	COLOR = vec4(screen_color, 1.0);
}