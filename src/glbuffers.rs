use ogl33::*;
use std::mem::size_of_val;

pub struct VAO(GLuint);
impl VAO {
    pub fn new() -> Option<VAO> {
        let mut out = VAO(0);
        unsafe {
            glGenVertexArrays(1, &mut out.0);
        }

        match out.0 {
            0 => return None,
            _ => return Some(out),
        }
    }

    pub fn bind(&self) {
        unsafe {
            glBindVertexArray(self.0);
        }
    }

    pub fn unbind() {
        unsafe {
            glBindVertexArray(0);
        }
    }
}

#[derive(Clone, Copy)]
pub enum BufferType {
    VBO = GL_ARRAY_BUFFER as isize,
    EBO = GL_ELEMENT_ARRAY_BUFFER as isize,
}

pub struct Buffer(pub GLuint, BufferType);
impl Buffer {
    pub fn new(buffer_type: BufferType) -> Option<Buffer> {
        let mut out = Buffer(0, buffer_type);

        unsafe {
            glGenBuffers(1, &mut out.0);
        }

        match out.0 {
            0 => return None,
            _ => return Some(out),
        }
    }
    pub fn bind(&self) {
        unsafe {
            glBindBuffer(self.1 as GLenum, self.0);
        }
    }
    pub fn unbind(buffer_type: BufferType) {
        unsafe {
            glBindBuffer(buffer_type as GLenum, 0);
        }
    }

    pub fn send_data(&self, data: &[u8], usage: GLenum) {
        unsafe {
            glBindBuffer(self.1 as GLenum, self.0);
            glBufferData(
                self.1 as GLenum,
                size_of_val(&data) as isize,
                data.as_ptr().cast(),
                usage,
            );
        }
    }
}

pub struct VertexAttribute(GLuint);
impl VertexAttribute {
    pub fn new(
        id: GLuint,
        size: GLint,
        msize: GLsizei,
        moffset: isize,
        datatype: GLenum,
    ) -> VertexAttribute {
        unsafe {
            glVertexAttribPointer(id, size, datatype, GL_FALSE, msize, moffset as *const _);
        }
        VertexAttribute(id)
    }

    pub fn enable(&self) {
        unsafe {
            glEnableVertexAttribArray(self.0);
        }
    }
}

pub fn triangle_gen() {}
pub fn quad_gen(pos: [f32; 3], width: f32, height: f32) -> Option<VAO> {
    let out = VAO::new();
    let vao = out.unwrap();
    vao.bind();

    let vbo = Buffer::new(BufferType::VBO).unwrap();
    vbo.bind();

    let vertices: [[f32; 5]; 4] = [
        [pos[0], pos[1], pos[2], 0.0, 1.0],
        [pos[0], pos[1] + height, pos[2], 0.0, 0.0],
        [pos[0] + width, pos[1] + height, pos[2], 1.0, 0.0],
        [pos[0] + width, pos[1], pos[2], 1.0, 1.0],
    ];

    let indices: [u32; 6] = [0, 1, 2, 0, 2, 3];

    let ebo = Buffer::new(BufferType::EBO).unwrap();
    ebo.bind();

    vbo.send_data(bytemuck::cast_slice(&vertices), GL_STATIC_DRAW);
    ebo.send_data(bytemuck::cast_slice(&indices), GL_STATIC_DRAW);

    VAO::unbind();
    Some(vao)
}
