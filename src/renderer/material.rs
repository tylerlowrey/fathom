use std::ops::{Deref};
use bevy::prelude::*;
use crate::assets::materials::Material;
use crate::renderer::mesh::{GpuMeshes, Mesh};
use crate::renderer::pipeline::Pipelines;
use crate::renderer::{Renderable, RendererState};


#[derive(Resource)]
pub struct DefaultMaterial(pub(crate) Handle<Material>);

impl Deref for DefaultMaterial {
    type Target = Handle<Material>;
    fn deref(&self) -> &Handle<Material> {
        &self.0
    }
}

#[derive(Component)]
pub struct MeshMaterial {
    material: Handle<Material>
}

pub fn render_mesh_with_material(
    renderable_entities: Query<(&Mesh, &MeshMaterial), With<Renderable>>,
    pipelines: Res<Pipelines>,
    gpu_meshes: Res<GpuMeshes>,
    renderer_state: Res<RendererState>,
) {
    let device = &renderer_state.device;
    let queue = &renderer_state.queue;
    let surface = &renderer_state.surface;
    let frame = surface.get_current_texture().unwrap();
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Main rendering command encoder")
    });
    for (mesh, mesh_material) in &renderable_entities {
        let material_handle = mesh_material.material.clone();
        let pipeline_id = pipelines.get_pipeline_id_by_material(material_handle)
            .unwrap_or_else(|| {
                panic!("Unable to get pipeline_id from the given material handle");
            });
        let pipeline_opt = pipelines.registered_pipelines.get(pipeline_id);
        let vertex_buffer_opt = gpu_meshes.buffers_map.get(&mesh.vertex_buffer_id);


        if let (Some(pipeline), Some(vertex_buffer)) = (pipeline_opt, vertex_buffer_opt) {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            render_pass.set_pipeline(pipeline);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

            if let Some((_, _, uniform_bind_group)) = pipelines.render_pipeline_state.get(pipeline_id) {
                render_pass.set_bind_group(0, &uniform_bind_group, &[]);
            }

            if let Some(Some(index_buffer)) = &mesh.has_indices().then(|| gpu_meshes.buffers_map.get(&mesh.index_buffer_id)) {
                render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(0..mesh.num_indices() as u32, 0, 0..1);
            } else {
                render_pass.draw(0..mesh.num_vertices() as u32, 0..1);
            }
        } else {
            error!("Unable to perform render pass for pipeline_id={} | vertex_buffer_id={}", pipeline_id, mesh.vertex_buffer_id);
        }
    }

    queue.submit(Some(encoder.finish()));
    frame.present();

}