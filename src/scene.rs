use crate::material::Material;
use std::sync::Arc;

use wgpu::{
    BindGroupLayout,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::{GpuContext, buffer::GpuBuffer, material::ColoredObject, vertex::Vertex};

pub struct Scene {
    //for colored vertices only
    pub static_vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    static_vb: GpuBuffer,
    static_ib: GpuBuffer,
    static_transform_buffer: wgpu::Buffer,

    pub materials: Vec<Material>,
    sampler: wgpu::Sampler,

    gpu: Arc<GpuContext>,
}

impl Scene {
    pub(crate) fn new(gpu: Arc<GpuContext>) -> Self {
        let diffuse_sampler = gpu.device.create_sampler(&wgpu::SamplerDescriptor {
            label: None,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });
        let ret = Self {
            static_vertices: Vec::new(),
            indices: Vec::new(),
            static_vb: GpuBuffer::new(gpu.clone(), wgpu::BufferUsages::VERTEX),
            static_ib: GpuBuffer::new(gpu.clone(), wgpu::BufferUsages::INDEX),
            static_transform_buffer: gpu.device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[glam::Mat4::IDENTITY]),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            materials: Vec::new(),
            sampler: diffuse_sampler,
            gpu,
        };
        ret
    }

    pub fn add_static_object(
        &mut self,
        new_vertices: &[Vertex],
        new_indices: &[u16],
    ) -> ColoredObject {
        let start_index = self.indices.len() as u32;
        self.static_vertices.extend_from_slice(new_vertices);
        self.static_vb.append(bytemuck::cast_slice(new_vertices));

        self.indices.extend_from_slice(new_indices);
        self.static_ib.append(bytemuck::cast_slice(new_indices));
        ColoredObject {
            start_index,
            num_indices: new_indices.len() as u32,
        }
    }

    pub fn add_material(&mut self, image_path: &str, bind_group_layout: &BindGroupLayout) -> usize {
        let maybe_material = Material::new(image_path, &self.gpu, bind_group_layout, &self.sampler);
        let material = match maybe_material {
            Ok(material) => material,
            Err(e) => {
                eprintln!("Error: failed to load material: {}", e);
                return 0;
            }
        };
        self.materials.push(material);
        return self.materials.len() - 1;
    }

    pub fn render_static(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_vertex_buffer(0, self.static_vb.buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.static_transform_buffer.slice(..));
        render_pass.set_index_buffer(self.static_ib.buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..(self.static_ib.bytes_used / 2) as u32, 0, 0..1);
    }
    pub fn render_materials(&self, render_pass: &mut wgpu::RenderPass) {
        for material in &self.materials {
            material.render(render_pass);
        }
    }
}
