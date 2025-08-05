pub fn shader_toy_adaptor(fragment_shader: String) -> String {
    return format!(
        "
        #version 300 es
        precision highp float;
        
        // uniforms
        uniform float u_time;
        uniform vec2 u_resolution;
        
        // input: vertex position
        in vec2 v_position;

        // output: pixel color
        out vec4 fragColor;

        // Shadertoy style uniforms definition
        float iTime;
        vec3 iResolution;

        // Shadertoy main function
        {fragment_shader}

        // GLSL main function
        void main() {{
            iResolution=vec3(u_resolution,u_resolution.x/u_resolution.y);
            iTime=u_time;
            vec2 w=v_position*u_resolution.xy;
            mainImage(fragColor,w);
        }}
        "
    );
}
