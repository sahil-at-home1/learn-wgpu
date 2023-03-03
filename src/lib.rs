use wgpu::{PrimitiveState};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use rand::Rng;

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,
    color: wgpu::Color,
    render_pipelines: Vec<wgpu::RenderPipeline>,
    render_pipeline_idx: usize,
}

impl State {
    // Creating some of the wgpu types requires async code
    async fn new(window: Window) -> State {
        let size = window.inner_size();
        // the instance is a handle to the GPU
        let instance = wgpu::Instance::new(
            wgpu::InstanceDescriptor { 
                backends: wgpu::Backends::all(), 
                dx12_shader_compiler: Default::default(),
            }
        );
        // create the surface to present to
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let options = wgpu::RequestAdapterOptions{
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };
        let adapter = instance.request_adapter(&options).await.unwrap();
        let mut limits = wgpu::Limits::default();
        if cfg!(target_arch = "wasm32") {
            limits = wgpu::Limits::downlevel_webgl2_defaults();
        }
        let desc = wgpu::DeviceDescriptor {
            features: wgpu::Features::empty(),
            limits: limits,
            label: None,
        };
        let (device, queue) = adapter.request_device(&desc, None).await.unwrap();
        // configure the surface
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .filter(|f| f.describe().srgb)
            .next()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);
        // set a default background color
        let color = wgpu::Color{
            r: 1.0, 
            g: 1.0, 
            b: 1.0, 
            a: 1.0
        };
        // load in shaders
        let shader_source = wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into());
        let shader_desc = wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: shader_source,
        };
        let shader = device.create_shader_module(shader_desc);
        // create render pipeline
        let render_pipeline_layout_desc = wgpu::PipelineLayoutDescriptor{
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        };
        let render_pipeline_layout = device.create_pipeline_layout(&render_pipeline_layout_desc);
        let vertex_state = wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[],
        };
        let fragment_state = wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })]
        };
        let primitive_state = PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        };
        let multisample_state = wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        };
        let render_pipeline_desc1 = wgpu::RenderPipelineDescriptor{
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: vertex_state,
            fragment: Some(fragment_state),
            primitive: primitive_state,
            depth_stencil: None,
            multisample: multisample_state,
            multiview: None,
        };
        let render_pipeline1 = device.create_render_pipeline(&render_pipeline_desc1);
        // challenge render pipeline
        let vertex_state2 = wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[],
        };
        let fragment_state2 = wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main2",
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })]
        };
        let render_pipeline_desc2 = wgpu::RenderPipelineDescriptor{
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: vertex_state2,
            fragment: Some(fragment_state2),
            primitive: primitive_state,
            depth_stencil: None,
            multisample: multisample_state,
            multiview: None,
        };
        let render_pipeline2 = device.create_render_pipeline(&render_pipeline_desc2);
        let render_pipelines = vec![render_pipeline1, render_pipeline2];
        let render_pipeline_idx = 0;
        return State {
            window,
            surface,
            device,
            queue,
            config,
            size,
            color,
            render_pipelines, 
            render_pipeline_idx, 
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config)
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::MouseInput { 
                button: MouseButton::Left, 
                state: ElementState::Pressed, 
                .. 
            } => {
                self.color = wgpu::Color{
                    r: rand::thread_rng().gen_range(0.0..1.0),
                    g: rand::thread_rng().gen_range(0.0..1.0),
                    b: rand::thread_rng().gen_range(0.0..1.0),
                    a: 1.0,
                };
                self.window().request_redraw();
                true
            },
            WindowEvent::KeyboardInput { 
                input: KeyboardInput {
                    scancode: 0x39, 
                    state: ElementState::Pressed,
                    ..
                },
                ..
            } => {
                self.render_pipeline_idx = if self.render_pipeline_idx > 0 { 0 } else { 1 };
                true
            },
            _ => false,
        }
    }

    fn update(&mut self) {
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let texture_desc = wgpu::TextureViewDescriptor::default();
        let view = output.texture.create_view(&texture_desc);
        let encoder_desc = wgpu::CommandEncoderDescriptor{label: Some("Render Encoder")};
        let mut encoder = self.device.create_command_encoder(&encoder_desc);
        // prepare render pass
        let ops = wgpu::Operations{
            load: wgpu::LoadOp::Clear(self.color), 
            store: true
        };
        let color_attachment = wgpu::RenderPassColorAttachment{
                view: &view,
                resolve_target: None,
                ops: ops,
            };
        let render_pass_desc = wgpu::RenderPassDescriptor{
            label: Some("Render Pass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: None,
        };
        let mut render_pass = encoder.begin_render_pass(&render_pass_desc);
        render_pass.set_pipeline(&self.render_pipelines[self.render_pipeline_idx]);
        render_pass.draw(0..3, 0..1);
        // need to release mut borrow before calling finish on encoder
        drop(render_pass);
        // submit command buffer (as an iter) to render queue
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}


pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(window).await;

    event_loop.run(move | event, _, control_flow | {
        match event {
            Event::WindowEvent {ref event, window_id} 
                if window_id == state.window().id() => {
                    if !state.input(event) {
                        match event {
                            WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                                input: KeyboardInput {
                                        state: ElementState::Pressed,
                                        virtual_keycode: Some(VirtualKeyCode::Escape),
                                        ..
                                    },
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            WindowEvent::Resized(physical_size) => {
                                state.resize(*physical_size);
                            }
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                state.resize(**new_inner_size)
                            }
                            _ => {}
                        }
                    }
                }
            Event::RedrawRequested(window_id) 
                if window_id == state.window().id() => {
                    state.update();
                    match state.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once
                // unless we request it
                state.window().request_redraw();
            }
            _ => {}
        }
    });
}

