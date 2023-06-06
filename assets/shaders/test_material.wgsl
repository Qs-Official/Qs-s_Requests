#import bevy_sprite::mesh2d_types
#import bevy_sprite::mesh2d_view_bindings


@fragment
fn fragment(
	#import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
	return vec4<f32>(0.0,1.0,0.0,1.0);
}
