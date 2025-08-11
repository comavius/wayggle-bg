// --------- License Information ---------
// This file is distributed in
// https://www.shadertoy.com/view/3XdXRr,
// under the CC0 license.
// ---------------------------------------

// CC0: Just another cube
// Glowtracers are great for compact coding, but I wanted to see how much
// I could squeeze a more normal raymarcher in terms of characters used.

// Twigl: https://twigl.app?ol=true&ss=-OW-y9xgRgWubwKcn0Nd

// == Globals ==
// Single-letter variable names are used to save characters (code golfing).
mat2 R;// A 2D rotation matrix, calculated once per frame in mainImage and used by D.
float d=1.// Stores the most recent distance to the scene from the ray's position.
,z// Stores the total distance traveled along the ray.
,G=9.// "Glow" variable. Tracks the closest the ray comes to the object (for volumetric glow effect).
,M=1e-3
;
// == Distance Function (SDF - Signed Distance Field) ==
// This function calculates the shortest distance from a given point 'p' to the scene geometry.
// A positive result means the point is outside an object, negative is inside, and zero is on the surface.
// This is the core of "raymarching", as it tells us the largest safe step we can take along a ray.
float D(vec3 p){
    // Apply two rotations to the point's coordinates. This twists the space the object
    // exists in, making the simple cube shape appear more complex and animated.
    p.xy*=R;
    p.xz*=R;
    
    // Create a higher-frequency version of the coordinate for detailed surface patterns.
    vec3 S=sin(123.*p);
    
    // This creates a volumetric glow effect by tracking the minimum distance
    // to either the existing glow value or a glowing shell around the object.
    G=min(
        G
        // The glowing shell
        ,max(
            abs(length(p)-.6)
            // The main object distance calculation:
            // 1. A superquadric (rounded cube shape) is created using an L8-norm.
            // The expression `pow(dot(p=p*p*p*p,p),.125)` is a golfed version of
            // `pow(pow(p.x,8)+pow(p.y,8)+pow(p.z,8), 1./8.)`.
            // The `- .5` defines the object's size.
            ,d=pow(dot(p*=p*p*p,p),.125)-.5
            // 2. Surface detail subtraction. This creates small surface variations
            // using high-frequency sine waves for more appealing reflections.
            -pow(1.+S.x*S.y*S.z,8.)/1e5
        )
    );
    
    return d;
}

// == Main Render Function ==
// This function is called for every pixel on the screen to determine its color.
// 'o' is the final output color (rgba). 'C' is the input pixel coordinate (xy).
void mainImage(out vec4 o,vec2 C){
    // Single-letter variable names are used to save characters (code golfing).
    vec3 p// The current point in 3D space along the ray.
    ,O// Multi-purpose vector: color accumulator, then normal vector, then final color.
    ,r=iResolution// 'r' holds screen resolution, later re-used for the epsilon vector and reflection.
    // 'I' is the Ray Direction vector. It's calculated once per pixel.
    // This converts the 2D screen coordinate 'C' into a 3D direction, creating the camera perspective.
    ,I=normalize(vec3(C-.5*r.xy,r.y))
    // Base glow color (dark bluish tint).
    ,B=vec3(1,2,9)*M
    ;
    
    // == Raymarching Loop ==
    // This loop "marches" a ray from the camera out into the scene to find what it hits.
    // It uses a golfed structure where the body of the loop updates the ray position 'p',
    // and the "advancement" step moves the ray forward.
    for(
        // -- Initializer (runs once before the loop) --
        // Calculate the rotation matrix for this frame based on time.
        R=mat2(cos(.3*iTime+vec4(0,11,33,0)))
        // -- Condition --
        // Loop while total distance 'z' is less than 9 and we are not yet touching a surface (d > 1e-3).
        ;z<9.&&d>M
        // -- Advancement --
        // The ray advances by the safe distance 'd' returned by D(p).
        // The result of D(p) is also assigned to the global 'd' inside the function.
        ;z+=D(p)
    )
    // -- Loop Body --
    // Calculate the current position 'p' in world space.
    // The camera starts at (0,0,-2) and points forward.
    p=z*I
    ,p.z-=2.
    ;
    
    // -- Hit Condition --
    // If the loop finished because z exceeded the max distance, we hit nothing. Otherwise, we hit the surface.
    if(z<9.){
        // -- Calculate Surface Normal --
        // Estimate the gradient ∇D at the hit point 'p' via central differences on the SDF D.
        // We use ε = 1e-3 and loop over each axis (x, y, z):
        //   • Zero r, then set r[i] = ε.
        //   • Compute O[i] = D(p + r) – D(p – r).
        // After the loop, O holds the unnormalized normal vector.
        for(
            int i// axis index: 0→x, 1→y, 2→z
            ;i<3
            ;O[i++]=D(p+r)-D(p-r)
        )
        r-=r// clear r to vec3(0)
        ,r[i]=M// set only the i-th component
        ;
        
        // -- Lighting and Shading --
        // 'z' is re-purposed to store a fresnel factor (1 - cos(angle)) for edge brightness.
        // `dot(O, I)` calculates how much the surface faces away from the camera.
        // O is also normalized here to become a proper normal vector.
        z=1.+dot(O=normalize(O),I);
        
        // 'r' is re-purposed to store the reflection vector.
        r=reflect(I,O);
        
        // Calculate a point 'C' along the reflection vector 'r' to sample a background color.
        // For upward reflections (r.y > 0), this finds the intersection with the plane y=5.
        C=(p+r*(5.-p.y)/abs(r.y)).xz;
        
        // Calculate the final color 'O' of the hit point.
        O=
        // Multiply by the fresnel factor squared for stronger edge reflections.
        z*z*
        // Use a ternary operator to decide the color based on where the reflection ray goes.
        (
            // If the reflection vector points upward...
            r.y>0.
            // ...sample a procedural "sky" with a radial gradient and blue tint.
            ?5e2*smoothstep(5.,4.,d=sqrt(length(C*C))+1.)*d*B
            // ...otherwise, sample a "floor" with a deep blue exponential falloff.
            :exp(-2.*length(C))*(B/M-1.)
        )
        // Add rim lighting (brighter on upward-facing surfaces).
        +pow(1.+O.y,5.)*B
        ;
    }
    
    // == Tonemapping & Output ==
    // Apply final effects and map the High Dynamic Range (HDR) color to a displayable range.
    // Add glow contribution: smaller G values (closer ray passes) create a brighter blue glow.
    o=sqrt(O+B/G).xyzx;
}