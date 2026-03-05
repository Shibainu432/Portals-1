uniform float is_warped;

void main() {
	int world = eye_world;

    vec3 warped_pos = mix(v_pos_unwarped, warp_position(v_pos_unwarped), is_warped);

	travel(world, eye, warped_pos);
	travel(world, warped_pos, v_center_unwarped);

	color = v_colors[world];
	if (color.a < 0.5) {
		discard;
	}

	color.rgb *= v_ambient_factor + v_diffuse_factor * max(dot(v_normal, light_dir), 0.0);
}
