use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Quat, Vec3};

// This is confusing. Even though in the shader it only uses slot 2,
// it actually uses 4 consecutive slots to pass the indiviual columns of the matrix
const ATTRIBUTES: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![
    2 => Float32x4,
    3 => Float32x4,
    4 => Float32x4,
    5 => Float32x4,
];
pub const TRANSFORM_MATRIX_DESC: wgpu::VertexBufferLayout = wgpu::VertexBufferLayout {
    array_stride: std::mem::size_of::<glam::Mat4>() as u64,
    step_mode: wgpu::VertexStepMode::Instance,
    attributes: &ATTRIBUTES,
};

pub struct InterpolatedPose {
    target: Transform,
    current: Transform,
    start_time: u64,
    duration: u64,
}

impl InterpolatedPose {
    pub fn new(transform: Transform) -> Self {
        Self {
            target: transform.clone(),
            current: transform,
            start_time: 0,
            duration: 0,
        }
    }
    pub fn update_target(&mut self, transform: &Transform, current_time: u64, duration: u64) {
        self.current = self.target.clone();
        self.target = transform.clone();
        self.start_time = current_time;
        self.duration = duration;
    }

    pub fn move_target_absolute(&mut self, new_position: Vec3, current_time: u64, duration: u64) {
        self.current = self.target.clone();
        self.target.position = new_position;
        self.start_time = current_time;
        self.duration = duration;
    }

    pub fn interpolate(&self, current_time: u64) -> Mat4 {
        let elapsed = current_time.saturating_sub(self.start_time);
        let interpolate_point: f32 = elapsed as f32 / self.duration as f32;
        let transform = if self.duration == 0 || interpolate_point > 1.0 {
            &self.target
        } else {
            &Transform {
                position: self.target.position * interpolate_point
                    + self.current.position * (1.0 - interpolate_point),
                rotation: self.target.rotation * interpolate_point
                    + self.current.rotation * (1.0 - interpolate_point),
                scale: self.target.scale * interpolate_point
                    + self.current.scale * (1.0 - interpolate_point),
            }
        };
        Mat4::from_scale_rotation_translation(
            transform.scale,
            transform.rotation,
            transform.position,
        )
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn from_vector(position: &Vec3, color: [f32; 3]) -> Self {
        Self {
            position: position.to_array(),
            color,
        }
    }
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        pub const ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
                0 => Float32x3,
                1 => Float32x3,
        ];
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct TextureVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}
impl TextureVertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        pub const ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
                0 => Float32x3,
                1 => Float32x2,
        ];
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TextureVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}

#[derive(Clone)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}
