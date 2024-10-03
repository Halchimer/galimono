use projection::rh_ydown::{orthographic_gl, perspective_gl};
use ultraviolet::*;

struct OrthoProjectionParams(f32, f32, f32, f32);
struct PersperctiveProjectionParams(f32, f32);

pub enum CameraProjectionType {
    Orthographic(OrthoProjectionParams),
    Perspective(PersperctiveProjectionParams),
}

pub struct Camera {
    position: Vec3,
    direction: Vec3,

    up: Vec3,
    right: Vec3,

    projection: CameraProjectionType,
}

impl Camera {
    pub fn new(position: Vec3, direction: Vec3, projection: CameraProjectionType) -> Option<Self> {
        let right = Vec3::new(0.0, 1.0, 0.0).cross(direction).normalized();
        let up = direction.cross(right).normalized();
        Some(Self {
            position: position,
            direction: direction,

            up: up,
            right: right,

            projection: projection,
        })
    }

    pub fn get_lookat_matrix(&self) -> Mat4 {
        let A = Mat4::new(
            Vec4::new(self.right.x, self.up.x, self.direction.x, 0.0),
            Vec4::new(self.right.y, self.up.y, self.direction.y, 0.0),
            Vec4::new(self.right.z, self.up.z, self.direction.z, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        );
        let B = Mat4::new(
            Vec4::new(1.0, 0.0, 0.0, -self.position.x),
            Vec4::new(0.0, 1.0, 0.0, -self.position.y),
            Vec4::new(0.0, 0.0, 1.0, -self.position.z),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        );

        A * B
    }
    pub fn get_projection_matrix(&self, near: f32, far: f32) -> Mat4 {
        match &self.projection {
            CameraProjectionType::Orthographic(proj) => {
                orthographic_gl(proj.0, proj.1, proj.2, proj.3, near, far)
            }
            CameraProjectionType::Perspective(proj) => perspective_gl(proj.0, proj.1, near, far),
        }
    }
}
