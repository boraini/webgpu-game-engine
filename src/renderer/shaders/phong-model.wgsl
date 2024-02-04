const near: f32 = 1.0f;
const far: f32 = -1.0f;
const camera_position: vec4f = vec4f(0, 0, 2, 1);

struct ProjectionMatrix {
    @location(0) matrix: mat4x4f,
    @location(1) matrix_inverse: mat4x4f,
}

struct Material {
    @location(0) ka : vec3f,
    @location(1) kd : vec3f,
    @location(2) ks : vec3f,
    @location(3) shininess : f32,
}

struct PointLight {
    @location(0) position : vec4f,
    @location(1) color : vec3f,
}

struct ViewInfo {
    @location(0) position : vec4f,
}

struct VertexOut {
  @builtin(position) position : vec4f,
  @location(1) normal : vec4f,
  @location(2) view_relative : vec4f,
  @location(3) light_relative : vec4f,
}

// Object Bindings
@group(0) @binding(0)
var<uniform> projection_matrix : ProjectionMatrix;
@group(1) @binding(0)
var<uniform> material : Material;
@group(2) @binding(0)
var<uniform> light : PointLight;
@group(2) @binding(1)
var<uniform> view_info : ViewInfo;

@vertex
fn vertex_main(@location(0) position : vec4f, @location(1) normal : vec4f, @location(2) uv : vec4f) -> VertexOut {
    var output: VertexOut;
    output.position = projection_matrix.matrix * position;
    output.normal = normalize(normal);
    output.view_relative = view_info.position - position;
    output.light_relative = light.position - position;
    return output;
}

@fragment
fn fragment_main(frag_data: VertexOut) -> @location(0) vec4f {
    var normal = normalize(frag_data.normal);
    var view_direction = normalize(frag_data.view_relative);
    var light_direction = normalize(frag_data.light_relative);
    var reflection_direction = normalize(2 * normal * dot(light_direction, normal) - light_direction);
    // var halfway_direction = 0.5 * (light_direction + view_direction);
    // var attenuation = 16.0 / (dot(frag_data.view_relative, frag_data.view_relative) * dot(frag_data.light_relative, frag_data.light_relative));
    
    var ambient = material.ka;
    var lambertian_diffuse = max(0.0, dot(light_direction, normal)) * light.color * material.kd;
    var phong_specular = pow(max(0.0, dot(reflection_direction, view_direction)), material.shininess) * light.color * material.ks;
    //var blinn_phong_specular = pow(max(0.0, dot(normal, halfway_direction)), material.shininess);

    var result = ambient + lambertian_diffuse + phong_specular;
    return vec4f(result, 1);
}