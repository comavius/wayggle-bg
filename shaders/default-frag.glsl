#version 300 es
precision highp float;

uniform float u_time;
in vec2 v_position;
out vec4 fragColor;

vec2 random(vec2 st){
    st=vec2(dot(st,vec2(127.1,311.7)),
    dot(st,vec2(269.5,183.3)));
    return-1.+2.*fract(sin(st)*43758.5453123);
}

void main(){
    float scale=5.;
    vec2 pos=v_position*scale;
    
    vec2 cell=floor(pos);
    
    float min_dist=100.;
    
    for(int y=-1;y<=1;y++){
        for(int x=-1;x<=1;x++){
            vec2 neighbor_cell=cell+vec2(x,y);
            
            vec2 feature_point=neighbor_cell+random(neighbor_cell);
            
            feature_point+=.4*sin(u_time*.7+random(neighbor_cell)*6.28);
            
            float dist=length(pos-feature_point);
            
            min_dist=min(min_dist,dist);
        }
    }
    
    float mono=pow(min_dist,2.5);
    
    fragColor=vec4(vec3(mono),1.);
}