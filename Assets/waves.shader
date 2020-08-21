shader_type spatial;

void vertex() {
	VERTEX.y += sin(TIME*4.0 + VERTEX.x * 50.0) * 0.2;
	VERTEX.y += cos(TIME*2.0 + VERTEX.z * 22.0) * 0.4;
}


void fragment(){
	ROUGHNESS = 0.1;
	METALLIC = 0.5;
	ALBEDO = vec3(0,0, 0.5);
}
