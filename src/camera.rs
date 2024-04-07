use std::ops::Range;

use glam::{EulerRot, Mat4, Quat, Vec3};
use winit::{
    event::KeyEvent,
    keyboard::{KeyCode, PhysicalKey},
};

#[derive(Debug, Clone)]
pub struct Camera {
    position: Vec3,
    oriention: Quat,
    fov: f32,
    aspect_ratio: f32,
    clip: Range<f32>,
    is_dirty: bool,
    move_sensitivity: f32,
    rotate_sensitivity: f32,
    view_m: Mat4,
    perspective_m: Mat4,
}

impl Default for Camera {
    fn default() -> Self {
        let mut c = Camera {
            position: -10.0 * Vec3::Z,
            oriention: Quat::IDENTITY,
            fov: 45_f32.to_radians(),
            aspect_ratio: 1.,
            clip: 0.1..1000.,
            is_dirty: true,
            move_sensitivity: 10.,
            rotate_sensitivity: 0.1,
            view_m: Mat4::ZERO,
            perspective_m: Mat4::ZERO,
        };
        c.update();
        c
    }
}

impl Camera {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_position(mut self, position: Vec3) -> Self {
        self.position = position;
        self.is_dirty = true;
        self
    }

    pub fn with_fov(mut self, fov_in_degree: f32) -> Self {
        assert!(fov_in_degree > 1. && fov_in_degree < 89.);
        self.fov = fov_in_degree.to_radians();
        self.is_dirty = true;
        self
    }

    pub fn with_aspect_ratio(mut self, aspect_ratio: f32) -> Self {
        assert!(aspect_ratio > 0.1 && aspect_ratio < 10.);
        self.aspect_ratio = aspect_ratio;
        self.is_dirty = true;
        self
    }

    pub fn with_clip(mut self, clip: Range<f32>) -> Self {
        assert!(clip.start > 1e-6);
        self.clip = clip;
        self.is_dirty = true;
        self
    }

    pub fn with_up_lookat(mut self, up: Vec3, look_at: Vec3) -> Self {
        let up = up.normalize();
        let direction = (look_at - self.position).normalize();
        assert!(up.dot(direction) < 0.01);
        self.oriention = Quat::from_mat4(&Mat4::look_at_rh(self.position, look_at, up));
        self.is_dirty = true;
        self
    }

    pub fn with_move_sensitivity(mut self, sensitivity: f32) -> Self {
        self.move_sensitivity = sensitivity;
        self.is_dirty = true;
        self
    }

    pub fn with_rotate_sensitivity(mut self, sensitivity: f32) -> Self {
        self.rotate_sensitivity = sensitivity;
        self.is_dirty = true;
        self
    }
}

impl Camera {
    pub fn update(&mut self) {
        if self.is_dirty {
            self.view_m = Mat4::from_rotation_translation(self.oriention, self.position);
            self.perspective_m =
                Mat4::perspective_lh(self.fov, self.aspect_ratio, self.clip.start, self.clip.end);
            self.is_dirty = false;
        }
    }

    /// moving in the camera coordinate system
    pub fn moving(&mut self, v: Vec3) {
        let v = self.oriention * v;
        self.position += v;
        self.is_dirty = true;
    }

    pub fn rotating(&mut self, r: Vec3) {
        let q = Quat::from_euler(EulerRot::XYZ, r.x, r.y, r.z);
        self.oriention *= q;
        self.oriention = self.oriention.normalize();
        self.is_dirty = true;
    }

    pub fn view_matrix(&self) -> &Mat4 {
        &self.view_m
    }

    pub fn perspective_matrix(&self) -> &Mat4 {
        &self.perspective_m
    }

    pub fn on_keyboard_event(&mut self, event: KeyEvent) {
        if let PhysicalKey::Code(code) = event.physical_key {
            match code {
                KeyCode::KeyW => self.moving(Vec3::Y * self.move_sensitivity),
                KeyCode::KeyS => self.moving(-Vec3::Y * self.move_sensitivity),
                KeyCode::KeyA => self.moving(-Vec3::X * self.move_sensitivity),
                KeyCode::KeyD => self.moving(Vec3::X * self.move_sensitivity),
                KeyCode::KeyQ => self.moving(-Vec3::Z * self.move_sensitivity),
                KeyCode::KeyE => self.moving(Vec3::Z * self.move_sensitivity),

                KeyCode::KeyI => self.rotating(Vec3::Y * self.rotate_sensitivity),
                KeyCode::KeyK => self.rotating(-Vec3::Y * self.rotate_sensitivity),
                KeyCode::KeyL => self.rotating(Vec3::X * self.rotate_sensitivity),
                KeyCode::KeyJ => self.rotating(-Vec3::X * self.rotate_sensitivity),
                KeyCode::KeyU => self.rotating(Vec3::Z * self.rotate_sensitivity),
                KeyCode::KeyO => self.rotating(-Vec3::Z * self.rotate_sensitivity),

                _ => {}
            }
        }
    }
}
