shader_type canvas_item;
uniform vec2 direction = vec2(1.0, 0.0);  // 滚动方向
uniform float speed = 0.05;
uniform float scale = 1.0;                // 纹理缩放

void fragment() {
	vec2 uv = UV * scale;
    uv += direction * TIME * speed;
    uv = fract(uv);
    COLOR = texture(TEXTURE, uv);
	float range = 0.5 - distance(0.5, uv.x);
	COLOR.a *= uv.y * range;
}
