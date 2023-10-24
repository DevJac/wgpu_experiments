fn main() {
    let instance: wgpu::Instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let adapter: wgpu::Adapter =
        pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default()))
            .unwrap();
    let (device, queue): (wgpu::Device, wgpu::Queue) =
        pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None))
            .unwrap();
    let shader_module = device.create_shader_module(wgpu::include_wgsl!(
        "../../shaders/fundamentals_compute.wgsl"
    ));
    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: None,
        module: &shader_module,
        entry_point: "compute_main",
    });
    let input: Vec<f32> = vec![1.0, 3.0, 5.0];
    let input_byte_size = (input.len() * std::mem::size_of::<f32>()) as u64;
    let work_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("work_buffer"),
        size: input_byte_size,
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    queue.write_buffer(&work_buffer, 0, bytemuck::cast_slice(&input));
    let result_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("result_buffer"),
        size: input_byte_size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &compute_pipeline.get_bind_group_layout(0),
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &work_buffer,
                offset: 0,
                size: None,
            }),
        }],
    });
    let mut command_encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut compute_pass =
            command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        compute_pass.set_pipeline(&compute_pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.dispatch_workgroups(input.len() as u32, 1, 1);
    }
    command_encoder.copy_buffer_to_buffer(&work_buffer, 0, &result_buffer, 0, input_byte_size);
    queue.submit([command_encoder.finish()]);
    {
        let result_buffer_slice = result_buffer.slice(..);
        result_buffer_slice.map_async(wgpu::MapMode::Read, |_| {});
        device.poll(wgpu::Maintain::Wait);
        let result_buffer_mapped_range = result_buffer_slice.get_mapped_range();
        let output: &[f32] = bytemuck::cast_slice(&result_buffer_mapped_range);
        dbg!(output);
    }
    result_buffer.unmap();
}
