#![feature(return_position_impl_trait_in_trait)]



use dui_core::{
    layout::get_id_manger,
    simple_text::FontManager,
    view::{
        BackgroundImpl, Element, FrameImpl, PaddingImpl, Text, HStack, View,
    }, Alignment, platform::{get_color, set_blur},
};
use dui_util::Rf;
use vello::{
    kurbo::{Affine, Rect},
    peniko::{Brush, Color},
    util::{RenderContext, RenderSurface},
    RenderParams, Renderer, RendererOptions, Scene, SceneBuilder,
};
use winit::{
    dpi::{PhysicalSize, Size},
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};

fn create_window(event_loop: &winit::event_loop::EventLoopWindowTarget<()>) -> Window {
    use winit::{dpi::LogicalSize, window::WindowBuilder};
    WindowBuilder::new()
        .with_inner_size(LogicalSize::new(1044, 800))
        .with_resizable(true)
        .with_title("Vello demo")
        .build(event_loop)
        .unwrap()
}

struct RenderState {
    size: PhysicalSize<u32>,
    surface: RenderSurface,
    renderer: Renderer,
    device: usize,
}

struct MyView;

impl Element for MyView {
    fn body(&self) -> impl View {
        HStack::from((

            Text::new("Hello World")
                .background(Color::GREEN)
                .padding(5.0)
                .background(Color::RED),

            Text::new("Hello World")
                .background(Color::GREEN)
                .padding(5.0)
                .background(Color::RED),

        ))
        .padding(1.0)
        .frame_min_max((100.0, 100.0), (5000.0, 5000.0))
        .align(Alignment::TOP_LEADING)
    }
}

impl MyView {}

pub fn run(event_loop: EventLoop<()>, window: Window, mut render_ctx: RenderContext) {
    let mut scene = Scene::new();
    let mut state: Option<RenderState> = None;

    set_blur(&window);

    window.request_redraw();
    window.focus_window();

    let font_manager = Rf::new(FontManager::new());

    event_loop.run(move |event, _, _| {
        let resize = |state: &mut RenderState, size: (u32, u32), set: bool| {
            let phys_size = PhysicalSize::new(size.0, size.1);
            if phys_size == window.inner_size() && phys_size == state.size {
                return;
            }

            state.surface.config.width = size.0;
            state.surface.config.height = size.1;

            state.surface.surface.configure(
                &render_ctx.devices[state.device].device,
                &state.surface.config,
            );

            state.size = phys_size;

            if set {
                window.set_inner_size(Size::Physical(phys_size));
            } else {
            }
        };

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => {
                    resize(state.as_mut().unwrap(), (size.width, size.height), false);
                    window.request_redraw();
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                // window.focus_window();
            }
            Event::RedrawRequested(_) => {
                let Some(state) = &mut state else { return };
                let mut scene_builder = SceneBuilder::for_scene(&mut scene);

                let mut path = Vec::with_capacity(512);
                path.push(0);

                // let path = Rc::new(path);

                let mut lctx = dui_core::drawing::LayoutContext {
                    font_manager: font_manager.clone(),

                    path: &mut path,
                    scale_factor: window.scale_factor(),
                };

                let m = MyView;
                let size = m.body().layout(
                    &mut lctx,
                    Rect::from_origin_size(
                        (0.0, 0.0),
                        (state.size.width as f64, state.size.height as f64),
                    ),
                );

                resize(state, (size.width() as _, size.height() as _), true);

                scene_builder.fill(
                    vello::peniko::Fill::NonZero,
                    Affine::IDENTITY,
                    &Brush::Solid(get_color()),
                    None,
                    &Rect::from_origin_size(
                        (0.0, 0.0),
                        (state.size.width as f64, state.size.height as f64),
                    ),
                );

                let surface_texture = state
                    .surface
                    .surface
                    .get_current_texture()
                    .expect("failed to get surface texture");
                surface_texture.present();

                render_ctx.devices[state.device]
                    .device
                    .poll(wgpu::Maintain::Poll);

                let devidx = state.device;

                window.set_inner_size(Size::Physical(PhysicalSize {
                    width: size.width() as _,
                    height: size.height() as _,
                }));

                let dctx = dui_core::drawing::DrawingContext {
                    builder: Rf::new(scene_builder),
                    path: Rf::new(path),

                    font_manager: font_manager.clone(),

                    background_brush: Color::TRANSPARENT.into(),
                    fill_brush: Color::BLACK.into(),
                    foreground_color: Color::BLACK.into(),

                    bounding: Rect::from_origin_size(
                        (0.0, 0.0),
                        (state.size.width as f64, state.size.height as f64),
                    ),
                    first: true,

                    scale_factor: window.scale_factor(),
                };

                m.view().draw(dctx);

                println!("{:#?}", get_id_manger());

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
                    &render_ctx.devices[devidx].device,
                    state.renderer.render_to_surface_async(
                        &render_ctx.devices[devidx].device,
                        &render_ctx.devices[devidx].queue,
                        &scene,
                        &surface_texture,
                        &params,
                    ),
                )
                .expect("failed to render to surface");

                //     rtx.send(()).unwrap();

                surface_texture.present();
                render_ctx.devices[state.device]
                    .device
                    .poll(wgpu::Maintain::Poll);
            }
            Event::Resumed => {
                let mut surface = pollster::block_on(render_ctx.create_surface(
                    &window,
                    window.inner_size().width,
                    window.inner_size().height,
                ));
                let render_options = RendererOptions {
                    surface_format: Some(surface.format),
                };
                let device_idx =
                    pollster::block_on(render_ctx.device(Some(&surface.surface))).unwrap();
                let renderer =
                    Renderer::new(&render_ctx.devices[device_idx].device, &render_options).unwrap();

                #[cfg(target_os = "macos")]
                unsafe {
                    surface
                        .surface
                        .as_hal_mut::<wgpu_hal::api::Metal, _, _>(|surface| {
                            if let Some(surface) = surface {
                                surface.present_with_transaction = true
                            }
                        });
                }

                // println!("{:?}", surface.config.present_mode);
                // surface.config.present_mode = PresentMode::AutoNoVsync;
                // surface
                //     .surface
                //     .configure(&render_ctx.devices[device_idx].device, &surface.config);

                state = Some(RenderState {
                    size: window.inner_size(),
                    surface,
                    renderer,
                    device: device_idx,
                })
            }
            _ => (),
        }
    });
}

pub fn start() {
    let event_loop = EventLoop::new();
    let window = create_window(&event_loop);

    let render_ctx = RenderContext::new().unwrap();

    run(event_loop, window, render_ctx);
}
