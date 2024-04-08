#version 150      

in vec3 position;
in vec3 normal;

out vec3 v_normal;      

uniform mat4 view;
uniform mat4 perspective; 

void main() {
    v_normal = transpose(inverse(mat3(view))) * normal;      
    gl_Position = perspective * view * vec4(position, 1.0);
}