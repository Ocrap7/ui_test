use vello::{
    kurbo::{Affine, Rect},
    peniko::{Brush, Color},
    util::{RenderContext, RenderSurface},
    RenderParams, Renderer, RendererOptions, Scene, SceneBuilder,
};
use winit::{dpi::PhysicalSize, event::Event, event_loop::EventLoop, window::Window};

fn create_window(event_loop: &winit::event_loop::EventLoopWindowTarget<()>) -> Window {
    use winit::{dpi::LogicalSize, window::WindowBuilder};
    WindowBuilder::new()
        .with_inner_size(LogicalSize::new(1044, 800))
        .with_resizable(true)
        .with_title("Vello demo")
        .build(&event_loop)
        .unwrap()
}

struct RenderState {
    size: PhysicalSize<u32>,
    surface: RenderSurface,
    renderer: Renderer,
    device: usize,
}

pub fn run(event_loop: EventLoop<()>, window: Window, mut render_ctx: RenderContext) {
    let mut scene = Scene::new();
    let mut state: Option<RenderState> = None;

    event_loop.run(move |event, _, _| match event {
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        Event::RedrawRequested(_) => {
            let Some(state) = &mut state else { return };

            let mut scene_builder = SceneBuilder::for_scene(&mut scene);
            scene_builder.fill(
                vello::peniko::Fill::NonZero,
                Affine::IDENTITY,
                &Brush::Solid(Color::AQUA),
                None,
                &Rect::from_origin_size(
                    (0.0, 0.0),
                    (state.size.width as f64, state.size.height as f64),
                ),
            );

            let params = RenderParams {
                width: state.size.width,
                height: state.size.height,
                base_color: Color::AQUA,
            };

            let surface_texture = state
                .surface
                .surface
                .get_current_texture()
                .expect("failed to get surface texture");

            vello::block_on_wgpu(
                &render_ctx.devices[state.device].device,
                state.renderer.render_to_surface_async(
                    &render_ctx.devices[state.device].device,
                    &render_ctx.devices[state.device].queue,
                    &scene,
                    &surface_texture,
                    &params,
                ),
            )
            .expect("failed to render to surface");

            surface_texture.present();
            render_ctx.devices[state.device]
                .device
                .poll(wgpu::Maintain::Poll);
        }
        Event::Resumed => {
            let surface = pollster::block_on(render_ctx.create_surface(
                &window,
                window.inner_size().width,
                window.inner_size().height,
            ));
            let render_options = RendererOptions {
                surface_format: Some(surface.format),
            };
            let device_idx = pollster::block_on(render_ctx.device(Some(&surface.surface))).unwrap();
            let renderer =
                Renderer::new(&render_ctx.devices[device_idx].device, &render_options).unwrap();

            state = Some(RenderState {
                size: window.inner_size(),
                surface,
                renderer,
                device: device_idx,
            })
        }
        _ => (),
    });
}

pub fn start() {
    let event_loop = EventLoop::new();
    let window = create_window(&event_loop);

    let render_ctx = RenderContext::new().unwrap();

    run(event_loop, window, render_ctx);
}
