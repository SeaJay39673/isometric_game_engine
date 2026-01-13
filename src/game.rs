use std::{
    collections::HashSet,
    sync::Arc,
    time::{Duration, Instant},
};

use ecs_core::spawn_entity;
use winit::{
    application::ApplicationHandler,
    event::StartCause,
    event_loop::{ControlFlow, EventLoop},
    keyboard::{NamedKey, SmolStr},
    window::Window,
};

use crate::{
    game_logic::{GameWorld, Position, Sprite, Velocity},
    graphics::Graphics,
    mesh::WorldMesh,
};

struct GameManager {
    last_frame: Instant,
    target_frame_duration: Duration,

    start: Instant,

    window: Option<Arc<Window>>,
    _fullscreen: bool,

    graphics: Option<Graphics>,
    world_mesh: Option<WorldMesh>,

    game_world: GameWorld,
    player: ecs_core::Entity,

    _pressed_named_keys: HashSet<NamedKey>,
    _pressed_keys: HashSet<SmolStr>,
    _cursor_location: (f32, f32),
}

impl GameManager {
    pub fn new() -> Self {
        let mut game_world = match GameWorld::new() {
            Ok(game_world) => game_world,
            Err(e) => panic!("{e}"),
        };
        let player = spawn_entity!(
            game_world.world,
            (
                Position {
                    x: 6.0,
                    y: 6.0,
                    z: 0.0,
                },
                Velocity {
                    x: -0.01,
                    y: -0.01,
                    z: 0.0,
                },
                Sprite {
                    texture_name: String::from("grass")
                }
            )
        );

        Self {
            last_frame: Instant::now(),
            target_frame_duration: Duration::from_secs_f64(1.0 / 120.0),
            start: Instant::now(),
            window: None,
            _fullscreen: false,
            graphics: None,
            world_mesh: None,

            game_world,
            player,

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

        self.game_world
            .world
            .system::<(Position, Velocity, Option<Sprite>), _>(|entity, (pos, vel, sprite)| {
                pos.x += vel.x;
                pos.y += vel.y;
                pos.z += vel.z;
                if let (Some(world_mesh), Some(sprite)) = (&mut self.world_mesh, sprite) {
                    world_mesh.update_entity(crate::game_logic::Entity {
                        entity: entity.clone(),
                        pos: [pos.x, pos.y, pos.z],
                        texture_name: sprite.texture_name.clone(),
                    });
                }
            });

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

        if self.world_mesh.is_none()
            && let Some(graphics) = &self.graphics
        {
            match WorldMesh::new(graphics, 0.1) {
                Ok(mut world_mesh) => {
                    world_mesh.update_chunk(self.game_world.chunk.clone());
                    self.game_world
                        .world
                        .entity_system::<(Position, Sprite), _>(
                            &self.player,
                            |entity, (pos, sprite)| {
                                world_mesh.update_entity(crate::game_logic::Entity {
                                    entity: entity.clone(),
                                    pos: [pos.x, pos.y, pos.z],
                                    texture_name: sprite.texture_name.clone(),
                                })
                            },
                        );
                    self.world_mesh = Some(world_mesh)
                }
                Err(e) => eprintln!("Error creating chunk meshes: {e}"),
            }
        }

        if self.graphics.is_some()
            && self.world_mesh.is_some()
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
                if let (Some(world_mesh), Some(graphics)) =
                    (&mut self.world_mesh, &mut self.graphics)
                {
                    let time_ms: u32 = self.start.elapsed().as_millis() as u32;
                    graphics.queue.write_buffer(
                        &world_mesh.time_buffer,
                        0,
                        bytemuck::bytes_of(&time_ms),
                    );
                    if let Err(e) = world_mesh.update(&graphics.device) {
                        eprintln!("Error updating world mesh: {e}");
                    }
                    if let Err(e) = graphics.render(Some(world_mesh)) {
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
