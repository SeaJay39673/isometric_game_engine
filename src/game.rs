use std::{
    collections::HashSet,
    sync::Arc,
    time::{Duration, Instant},
};

use winit::{
    application::ApplicationHandler,
    event::StartCause,
    event_loop::{ControlFlow, EventLoop},
    keyboard::{NamedKey, SmolStr},
    window::Window,
};

use crate::{graphics::Graphics, mesh::ChunkMeshes};

struct GameManager {
    last_frame: Instant,
    target_frame_duration: Duration,

    start: Instant,

    window: Option<Arc<Window>>,
    _fullscreen: bool,

    graphics: Option<Graphics>,
    chunk_meshes: Option<ChunkMeshes>,

    _pressed_named_keys: HashSet<NamedKey>,
    _pressed_keys: HashSet<SmolStr>,
    _cursor_location: (f32, f32),
}

impl GameManager {
    pub fn new() -> Self {
        Self {
            last_frame: Instant::now(),
            target_frame_duration: Duration::from_secs_f64(1.0 / 120.0),
            start: Instant::now(),
            window: None,
            _fullscreen: false,
            graphics: None,
            chunk_meshes: None,
            _pressed_named_keys: HashSet::new(),
            _pressed_keys: HashSet::new(),
            _cursor_location: (0.0, 0.0),
        }
    }
}

impl ApplicationHandler for GameManager {
    fn new_events(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        cause: winit::event::StartCause,
    ) {
        let now = Instant::now();
        let next_frame_time = self.last_frame + self.target_frame_duration;

        if now >= next_frame_time || matches!(cause, StartCause::Init) {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
            self.last_frame = now;
        }
        event_loop.set_control_flow(ControlFlow::WaitUntil(
            self.last_frame + self.target_frame_duration,
        ));
    }

    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            if let Ok(window) = event_loop.create_window(
                Window::default_attributes()
                    .with_maximized(true)
                    .with_visible(false),
            ) {
                self.window = Some(Arc::new(window));
            }
        }

        if self.graphics.is_none()
            && let Some(window) = &self.window
        {
            if let Ok(graphics) = pollster::block_on(Graphics::new(window)) {
                self.graphics = Some(graphics);
            }
        }

        if self.chunk_meshes.is_none()
            && let Some(graphics) = &self.graphics
        {
            match ChunkMeshes::new(graphics, 2, 8, 0.1) {
                Ok(chunk_meshes) => self.chunk_meshes = Some(chunk_meshes),
                Err(e) => eprintln!("Error creating chunk meshes: {e}"),
            }
        }

        if self.graphics.is_some()
            && self.chunk_meshes.is_some()
            && let Some(window) = &self.window
        {
            window.set_visible(true);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        use winit::event::WindowEvent::*;
        match event {
            CloseRequested => event_loop.exit(),
            KeyboardInput { event: _, .. } => {}
            CursorMoved { position: _, .. } => {}
            MouseInput {
                state: _,
                button: _,
                ..
            } => {}
            RedrawRequested => {
                if let (Some(chunk_meshes), Some(graphics)) =
                    (&mut self.chunk_meshes, &mut self.graphics)
                {
                    let time_ms: u32 = self.start.elapsed().as_millis() as u32;
                    graphics.queue.write_buffer(
                        &chunk_meshes.time_buffer,
                        0,
                        bytemuck::bytes_of(&time_ms),
                    );
                    if let Err(e) = graphics.render(Some(chunk_meshes)) {
                        eprintln!("Error rendering graphics: {e}");
                    }
                }
            }
            Resized(size) => {
                if let Some(graphics) = &mut self.graphics {
                    graphics.resize(size.width, size.height);
                }
            }
            _ => {}
        };
    }
}

pub struct Game {
    event_loop: EventLoop<()>,
    game_manager: GameManager,
}

impl Game {
    pub fn new() -> anyhow::Result<Self> {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);
        let game_manager = GameManager::new();
        Ok(Self {
            event_loop,
            game_manager,
        })
    }
    pub fn run(mut self) -> anyhow::Result<()> {
        self.event_loop.run_app(&mut self.game_manager)?;
        Ok(())
    }
}
