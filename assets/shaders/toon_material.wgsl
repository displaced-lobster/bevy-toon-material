#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::pbr_bindings
#import bevy_pbr::mesh_bindings

#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
#import bevy_pbr::pbr_ambient
#import bevy_pbr::shadows
#import bevy_pbr::fog
#import bevy_pbr::pbr_functions

struct ToonMaterial {
    color: vec4<f32>,
    glossiness: f32,
    receive_shadows: u32,
};

@group(1) @binding(0)
var<uniform> toon: ToonMaterial;

struct FragmentInput {
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    let is_orthographic = view.projection[3].w == 1.0;
    let view_direction = normalize(view.world_position.xyz);
    // let view_direction = calculate_view(in.world_position, is_orthographic);
    let view_z = dot(vec4<f32>(
        view.inverse_view[0].z,
        view.inverse_view[1].z,
        view.inverse_view[2].z,
        view.inverse_view[3].z
    ), in.world_position);
    let cluster_index = fragment_cluster_index(in.frag_coord.xy, view_z, is_orthographic);
    let offset_and_counts = unpack_offset_and_counts(cluster_index);

    var direct_light: vec3<f32> = vec3<f32>(0.0);

    // Point lights
    for (var i: u32 = offset_and_counts[0]; i < offset_and_counts[0] + offset_and_counts[1]; i = i + 1u) {
        let light_id = get_light_id(i);
        let light = &point_lights.data[light_id];
        let light_color = normalize((*light).color_inverse_square_range.rgb);
        let light_to_frag = normalize((*light).position_radius.xyz - in.world_position.xyz);
        let NdotL = saturate(dot(in.world_normal, light_to_frag));

        var shadow = 1.0;

        if ((toon.receive_shadows & 1u) != 0u
            && (point_lights.data[light_id].flags & POINT_LIGHT_FLAGS_SHADOWS_ENABLED_BIT) != 0u) {
            shadow = fetch_point_shadow(light_id, in.world_position, in.world_normal);
        }

        let light_intensity = smoothstep(0.0, 0.01, NdotL * shadow);

        // specular reflection
        let half_vector = normalize(light_to_frag + view_direction);
        let NdotH = saturate(dot(in.world_normal, half_vector));
        let specular_intensity = pow(NdotH * light_intensity, 1000.0 / toon.glossiness);
        let specular_intensity_smooth = smoothstep(0.05, 0.1, specular_intensity);

        // Rim lighting
        let rim_dot = 1.0 - dot(view_direction, in.world_normal);
        let rim_amount = 0.6;
        let rim_threshold = 0.2;
        var rim_intensity = rim_dot * pow(NdotL, rim_threshold);

        rim_intensity = smoothstep(rim_amount - 0.01, rim_amount + 0.01, rim_intensity);

        direct_light += (light_intensity + specular_intensity_smooth + rim_intensity) * light_color;
    }

    return vec4<f32>(toon.color.rgb * (lights.ambient_color.rgb + direct_light), toon.color.a);
}
