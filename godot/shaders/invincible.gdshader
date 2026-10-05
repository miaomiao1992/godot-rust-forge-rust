shader_type canvas_item;

const vec2 center = vec2(0.5);
uniform float radius: hint_range(0.0, 1.0) = 0.5;
uniform float width: hint_range(0.0, 1.0) = 0.2;
uniform vec2 hight_light_center = vec2(0.0);

void fragment() {
    float circle_main = step(distance(UV, center), 0.5);
	float start = width * (0.05 * sin(TIME * 5.0) + 0.45);
	float circle_inner = smoothstep(start, 0.7, distance(UV, center));
	float main_shape = circle_main * circle_inner;
	
	float hightlight_shape = 1.0 - distance(UV, hight_light_center);
	hightlight_shape = 1.0;
	COLOR.a = main_shape * hightlight_shape;
	
	COLOR.rgb = COLOR.rgb;
}