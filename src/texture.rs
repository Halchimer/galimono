use beryllium::*;
use image::{self, EncodableLayout};
use ogl33::*;

pub struct Texture(GLuint);

impl Texture {
    pub fn new() -> Option<Texture> {
        let mut out = 0;
        unsafe {
            glGenTextures(1, &mut out);
        }
        Some(Texture(out))
    }
    pub fn bind(&self) {
        unsafe {
            glBindTexture(GL_TEXTURE_2D, self.0);
        }
    }
    pub fn delete(&mut self) {
        unsafe {
            glDeleteTextures(1, [self.0].as_ptr());
        }
    }

    pub fn load_from_file(&self, path: &str) -> Result<(), image::ImageError> {
        self.bind();
        let image_data = image::open(path)?.into_rgba8();
        unsafe {
            glTexImage2D(
                GL_TEXTURE_2D,
                0,
                GL_RGBA.try_into().unwrap(),
                image_data.width().try_into().unwrap(),
                image_data.height().try_into().unwrap(),
                0,
                GL_RGBA,
                GL_UNSIGNED_BYTE,
                image_data.as_bytes().as_ptr() as *const _,
            );
        }
        Ok(())
    }
    pub fn Enable() {
        unsafe {
            glEnable(GL_TEXTURE_2D);
        }
    }
}
