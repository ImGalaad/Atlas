#version 330

in vec2 fragTexCoord;

uniform sampler2D texture0;
uniform vec2 res;
uniform vec2 cam_pos;

// uniform float time_of_day;
// uniform float game_tick;
uniform float zoom;

void main() {
    vec2 center = vec2(0.5, 0.5);

    // Zoom
    vec2 zoom_uv = center + (fragTexCoord - center) * (1.0 / zoom);
    zoom_uv.y *= -1;

    gl_FragColor = texture(texture0, zoom_uv);
}
