#version 330 core
in vec4 colour;
in vec4 gl_FragCoord;   // but actually use UVs hey
in vec2 uv;
flat in uint fs_mode;

out vec4 frag_colour;

#define PI 3.1415926535897932384626433832795
#define ROOT2INV 0.70710678118654752440084436210484903928

uniform sampler2D tex;
uniform float time;

vec3 hsv2rgb(vec3 c)
{
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

//	Classic Perlin 3D Noise 
//	by Stefan Gustavson
//
vec4 permute(vec4 x){return mod(((x*34.0)+1.0)*x, 289.0);}
vec4 taylorInvSqrt(vec4 r){return 1.79284291400159 - 0.85373472095314 * r;}
vec3 fade(vec3 t) {return t*t*t*(t*(t*6.0-15.0)+10.0);}

float cnoise(vec3 P){
  vec3 Pi0 = floor(P); // Integer part for indexing
  vec3 Pi1 = Pi0 + vec3(1.0); // Integer part + 1
  Pi0 = mod(Pi0, 289.0);
  Pi1 = mod(Pi1, 289.0);
  vec3 Pf0 = fract(P); // Fractional part for interpolation
  vec3 Pf1 = Pf0 - vec3(1.0); // Fractional part - 1.0
  vec4 ix = vec4(Pi0.x, Pi1.x, Pi0.x, Pi1.x);
  vec4 iy = vec4(Pi0.yy, Pi1.yy);
  vec4 iz0 = Pi0.zzzz;
  vec4 iz1 = Pi1.zzzz;

  vec4 ixy = permute(permute(ix) + iy);
  vec4 ixy0 = permute(ixy + iz0);
  vec4 ixy1 = permute(ixy + iz1);

  vec4 gx0 = ixy0 / 7.0;
  vec4 gy0 = fract(floor(gx0) / 7.0) - 0.5;
  gx0 = fract(gx0);
  vec4 gz0 = vec4(0.5) - abs(gx0) - abs(gy0);
  vec4 sz0 = step(gz0, vec4(0.0));
  gx0 -= sz0 * (step(0.0, gx0) - 0.5);
  gy0 -= sz0 * (step(0.0, gy0) - 0.5);

  vec4 gx1 = ixy1 / 7.0;
  vec4 gy1 = fract(floor(gx1) / 7.0) - 0.5;
  gx1 = fract(gx1);
  vec4 gz1 = vec4(0.5) - abs(gx1) - abs(gy1);
  vec4 sz1 = step(gz1, vec4(0.0));
  gx1 -= sz1 * (step(0.0, gx1) - 0.5);
  gy1 -= sz1 * (step(0.0, gy1) - 0.5);

  vec3 g000 = vec3(gx0.x,gy0.x,gz0.x);
  vec3 g100 = vec3(gx0.y,gy0.y,gz0.y);
  vec3 g010 = vec3(gx0.z,gy0.z,gz0.z);
  vec3 g110 = vec3(gx0.w,gy0.w,gz0.w);
  vec3 g001 = vec3(gx1.x,gy1.x,gz1.x);
  vec3 g101 = vec3(gx1.y,gy1.y,gz1.y);
  vec3 g011 = vec3(gx1.z,gy1.z,gz1.z);
  vec3 g111 = vec3(gx1.w,gy1.w,gz1.w);

  vec4 norm0 = taylorInvSqrt(vec4(dot(g000, g000), dot(g010, g010), dot(g100, g100), dot(g110, g110)));
  g000 *= norm0.x;
  g010 *= norm0.y;
  g100 *= norm0.z;
  g110 *= norm0.w;
  vec4 norm1 = taylorInvSqrt(vec4(dot(g001, g001), dot(g011, g011), dot(g101, g101), dot(g111, g111)));
  g001 *= norm1.x;
  g011 *= norm1.y;
  g101 *= norm1.z;
  g111 *= norm1.w;

  float n000 = dot(g000, Pf0);
  float n100 = dot(g100, vec3(Pf1.x, Pf0.yz));
  float n010 = dot(g010, vec3(Pf0.x, Pf1.y, Pf0.z));
  float n110 = dot(g110, vec3(Pf1.xy, Pf0.z));
  float n001 = dot(g001, vec3(Pf0.xy, Pf1.z));
  float n101 = dot(g101, vec3(Pf1.x, Pf0.y, Pf1.z));
  float n011 = dot(g011, vec3(Pf0.x, Pf1.yz));
  float n111 = dot(g111, Pf1);

  vec3 fade_xyz = fade(Pf0);
  vec4 n_z = mix(vec4(n000, n100, n010, n110), vec4(n001, n101, n011, n111), fade_xyz.z);
  vec2 n_yz = mix(n_z.xy, n_z.zw, fade_xyz.y);
  float n_xyz = mix(n_yz.x, n_yz.y, fade_xyz.x); 
  return 2.2 * n_xyz;
}

float rand(float n){return fract(sin(n) * 43758.5453123);}

float noise(float p){
	float fl = floor(p);
  float fc = fract(p);
	return mix(rand(fl), rand(fl + 1.0), fc);
}

float f1d(float p){
  return 
    1.000 * noise(p) +
    0.500 * noise(p*2.0 + 15123.34521) +
    0.250 * noise(p*4.0 + 13418.23523) +
    0.125 * noise(p*8.0 + 19023.52627) /
    1.875
    ;
}

// between 0 and 1
// then its -ln for mountain mode

float slowstart(float t) {
    return 1.0 - (1.0 - t)*(1.0 - t);
}
float slowstop(float t) {
    return t*t;
}

float quadraticInOut(float t) {
  float p = 2.0 * t * t;
  return t < 0.5 ? p : -p + (4.0 * t) - 1.0;
}
float exponentialInOut(float t) {
  return t == 0.0 || t == 1.0
    ? t
    : t < 0.5
      ? +0.5 * pow(2.0, (20.0 * t) - 10.0)
      : -0.5 * pow(2.0, 10.0 - (t * 20.0)) + 1.0;
}

void main() {
    switch(fs_mode) {
        case 0u:
        frag_colour = colour;
        break;
        case 1u:
        frag_colour = texture(tex, uv) * colour;
        break;
        default:
        frag_colour = vec4(0., 1., 0., 1.);
        break;
        case 2u:
        // gem
        float x = (0.5-uv.x);
        float y = (0.5-uv.y);

        float md = abs(x) + abs(y);

        if (md > 0.5) {
          frag_colour = vec4(1., 1., 1., 0);
          return;
        }
        if (md > 0.25) {
          frag_colour = vec4(colour.xyz, 1.0 - md);
        } else {
          frag_colour = vec4(colour.xyz, 0.75);
        }
        break;
        case 3u:
        // shadow
        x = (0.5-uv.x);
        y = (0.5-uv.y);
        float r = sqrt(x*x+y*y);
        if (r > 0.5) {
          frag_colour = vec4(0., 0., 0., 0.);
        } else {
          frag_colour = vec4(0., 0., 0., 0.25);
        }

        break;
        case 4u:
        x = (0.5-uv.x);
        y = (0.5-uv.y);
        r = sqrt(x*x+y*y);

        float density = (1.0 - uv.y) * (2.0*(0.5 - r));
        density = min(density * 2.0, 1.0);
        float nz = cnoise(vec3(10.0*uv.x, 10.0*uv.y, 10.0*time));
        float nr = cnoise(vec3(10.0*uv.x, 10.0*uv.y, 10.0*time + 1.0));
        float ng = cnoise(vec3(10.0*uv.x, 10.0*uv.y, 10.0*time + 2.0));
        float nb = cnoise(vec3(10.0*uv.x, 10.0*uv.y, 10.0*time + 3.0));
        density = density + 0.1*nz;
        frag_colour = vec4(hsv2rgb(vec3(0.0 + 0.05 * nb, 0.9 + 0.1* nr, 0.9 + 0.1*ng)), density);
        // frag_colour = vec4(colour.x + 0.5*nr, colour.y + 0.3*ng, colour.z + 0.1*nb, density);
        // frag_colour = vec4(colour.xyz, density * 0.4 + density*nz + 0.2*nz);

        break;
        case 1000u:
        float h1 = 0.6 -0.3 * log(f1d(uv.x * 3 + time * 0.1));
      
        float h2 = 0.5 -0.2 * log(f1d(1238+(uv.x * 4) + (12238+time) * 0.2));
        float h3 = 0.4 -0.1 * log(f1d(7633+(uv.x * 5) + (55645+time) * 0.3));

        h1 = 1 - h1;
        h2 = 1 - h2;
        h3 = 1 - h3;

        // float h = 0.4 + 0.2 * f1d(uv.x * 10 + time * 1);
        if (uv.y > h3) {
          frag_colour = vec4(0.55, 0.39, 0.25, 1.0);
        } else if (uv.y > h2) {
          frag_colour = vec4(0.53, 0.33, 0.25, 1.0);
        } else if (uv.y > h1){
          frag_colour = vec4(0.5, 0.3, 0.25, 1.0);
        } else {
          frag_colour = vec4(0.55, 0.55, 0.9, 1.0);
        }
        break;
        case 1001u:
        // or if the geometry, it actually splits into 4 and they swap places, or flip and flip UVs
        // for transition diamonds shrink revealing next thing
        // if L1 dist > t - tlast kind of thing
        // the flag in crash team racing: with some kind of domain warping applied
        float theta = PI/4;
        float up = cos(theta) * uv.x - sin(theta) * uv.y;
        float vp = sin(theta) * uv.x + cos(theta) * uv.y;

        up = up + time * 0.02;
        vp = vp + time * 0.0015;

        up *= 5.0;
        vp *= 5.0;

        up = mod(up, 1.0);
        vp = mod(vp, 1.0);

        if (up < 0.5 ^^ vp < 0.5) {
          frag_colour = vec4(0.6, 0.1, 0.6, 1.0);
        } else {
          frag_colour = vec4(0.3, 0.1, 0.6, 1.0);
        }

        break;
    }
}