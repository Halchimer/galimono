use ogl33::*;
use std::fmt;
use std::fs;

#[derive(Clone, Copy)]
pub enum ShaderType {
    Vert = GL_VERTEX_SHADER as isize,
    Frag = GL_FRAGMENT_SHADER as isize,
}
impl fmt::Display for ShaderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ShaderType::Vert => write!(f, "Vertex Shader"),
            ShaderType::Frag => write!(f, "Fragment Shader"),
        }
    }
}

pub struct Shader {
    pub gl_shader: GLuint,
    pub shader_type: ShaderType,
}

impl Shader {
    pub fn create_from_file(shader_type: ShaderType, path: &str) -> Result<Self, String> {
        let shader = Shader {
            gl_shader: unsafe { glCreateShader(shader_type.clone() as GLenum) },
            shader_type: shader_type,
        };
        let source = fs::read_to_string(path).expect("Unable to read shader source file...");

        unsafe {
            glShaderSource(
                shader.gl_shader,
                1,
                &(source.as_bytes().as_ptr().cast()),
                &(source.len().try_into().unwrap()),
            );
            glCompileShader(shader.gl_shader);

            let mut success = 0;
            glGetShaderiv(shader.gl_shader, GL_COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                glGetShaderInfoLog(shader.gl_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!(
                    "{} Compile Error: {} at path {}",
                    shader_type,
                    String::from_utf8_lossy(&v),
                    source,
                );
            }
        }

        Ok(shader)
    }

    pub fn delete(&self) {
        unsafe {
            glDeleteShader(self.gl_shader);
        }
    }
}

pub struct ShaderProgram(GLuint);

impl ShaderProgram {
    pub fn new() -> Option<Self> {
        Some(Self(unsafe { glCreateProgram() }))
    }
    pub fn attach_shader(&self, shader: &Shader) {
        unsafe {
            glAttachShader(self.0, shader.gl_shader);
        }
    }
    pub fn link(&self) {
        unsafe {
            glLinkProgram(self.0);

            let mut success = 0;
            glGetProgramiv(self.0, GL_LINK_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                glGetProgramInfoLog(self.0, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
            }
        }
    }
    pub fn use_program(&self) {
        unsafe {
            glUseProgram(self.0);
        }
    }
}
