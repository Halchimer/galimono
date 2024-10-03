use std::mem::size_of;

use beryllium::*;
use ogl33::*;

mod eventhandler;
mod glbuffers;
mod renderer;
mod shader;
mod texture;
mod vertex;
mod window;

pub fn start(window: &window::Window) {
    let vao = glbuffers::VAO::new().unwrap();
    vao.bind();

    let vbo = glbuffers::Buffer::new(glbuffers::BufferType::VBO).unwrap();
    vbo.bind();

    let ebo = glbuffers::Buffer::new(glbuffers::BufferType::EBO).unwrap();
    ebo.bind();

    const VERTICES: [[f32; 5]; 4] = [
        [-0.5, -0.5, -0.5, 0.0, 1.0],
        [0.5, -0.5, -0.5, 1.0, 1.0],
        [0.5, 0.5, -0.5, 1.0, 0.0],
        [-0.5, 0.5, -0.5, 0.0, 0.0],
    ];
    const INDICES: [u32; 6] = [0, 1, 3, 1, 2, 3];

    vbo.send_data(bytemuck::cast_slice(&VERTICES), GL_STATIC_DRAW);
    ebo.send_data(bytemuck::cast_slice(&INDICES), GL_STATIC_DRAW);

    let pos_attribute = glbuffers::VertexAttribute::new(
        0,
        3,
        size_of::<[f32; 5]>().try_into().unwrap(),
        0,
        GL_FLOAT,
    );
    pos_attribute.enable();

    let tex_attribute = glbuffers::VertexAttribute::new(
        1,
        2,
        size_of::<[f32; 5]>().try_into().unwrap(),
        size_of::<f32>() as isize * 3,
        GL_FLOAT,
    );
    tex_attribute.enable();

    let texture0 = texture::Texture::new().unwrap();
    let texLoadResult = texture0.load_from_file("squalala.png");
    match texLoadResult {
        Err(_) => panic!("Unable to load image file."),
        Ok(_) => (),
    }
    texture0.bind();
    unsafe {
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST as i32);
    }

    let vert_shader =
        shader::Shader::create_from_file(shader::ShaderType::Vert, "shaders/vertex.glsl").unwrap();

    let frag_shader =
        shader::Shader::create_from_file(shader::ShaderType::Frag, "shaders/fragment.glsl")
            .unwrap();

    let shader_prog = shader::ShaderProgram::new().unwrap();
    shader_prog.attach_shader(&vert_shader);
    shader_prog.attach_shader(&frag_shader);
    shader_prog.link();

    frag_shader.delete();
    vert_shader.delete();

    shader_prog.use_program();

    window.get_glWindow().set_swap_interval(SwapInterval::Vsync);
}

pub fn update(window: &window::Window) {
    // RENDERING
    unsafe {
        glClear(GL_COLOR_BUFFER_BIT);

        glDrawElements(GL_TRIANGLES, 2, GL_UNSIGNED_INT, 6 as *const _);
    }
    window.get_glWindow().swap_window();
}

fn main() {
    let mut win = window::Window::create_window(
        "Test Window",
        WindowPosition::Centered,
        800,
        600,
        start,
        update,
    )
    .unwrap();

    unsafe {
        load_gl_with(|f_name| win.get_glWindow().get_proc_address(f_name));
    }

    win.init();
}
