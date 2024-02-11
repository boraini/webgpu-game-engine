use glm::{Matrix4, Vector4};

pub fn identity_matrix() -> Matrix4<f32> {
    Matrix4::new(
        Vector4::new(1., 0., 0., 0.),
        Vector4::new(0., 1., 0., 0.),
        Vector4::new(0., 0., 1., 0.),
        Vector4::new(0., 0., 0., 1.),
    )
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MuckableMatrix(pub glm::Matrix4<f32>);

unsafe impl bytemuck::Zeroable for MuckableMatrix {
    fn zeroed() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

unsafe impl bytemuck::Pod for MuckableMatrix {}

impl Into<Matrix4<f32>> for MuckableMatrix {
    fn into(self) -> Matrix4<f32> {
        self.0
    }
}
