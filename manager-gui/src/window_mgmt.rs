use crate::app::App;
use crate::layout;
use crate::state::State;
use crate::support;
use glium::glutin;
use glium::Surface;
use rusttype;
use winit::Event::WindowEvent;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub fn load_font() -> rusttype::Font<'static> {
    //let font_data = include_bytes!("../../assets/fonts/NotoSans/NotoSans-Regular.ttf");
    let font_data = include_bytes!("../../assets/fonts/Overwatch/bignoodletoo.ttf");
    //let font_data = include_bytes!("../../assets/fonts/Overwatch/koverwatch.ttf");
    let collection =
        rusttype::FontCollection::from_bytes(font_data as &[u8]).expect("font was invalid?");

    collection
        .into_font()
        .expect("expected loading embedded font to succeed")
}

pub fn init_window() -> (glutin::EventsLoop, App) {
    // Create window.
    let events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Overwatch 3v3 Elimination - Team Manager")
        .with_dimensions((WIDTH, HEIGHT).into());
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let display =
        glium::Display::new(window, context, &events_loop).expect("unable to create new window");

    let display = support::GliumDisplayWinitWrapper(display);

    // Create UI and other components.
    let mut app = App::new(display, &events_loop);

    // Add font.
    app.ui.fonts.insert(load_font());

    (events_loop, app)
}

pub fn main_window_loop(mut events: glutin::EventsLoop, mut app: App) {
    let mut event_loop = support::EventLoop::new();

    let mut state = State::new();

    debug!("Starting event loop.");

    'main: loop {
        for event in event_loop.next(&mut events) {
            trace!("top of main loop: {:?}", event);
            if let State::Exit = state {
                info!("exiting...");
                break 'main;
            }

            if let support::WindowEvent::Glutin(gevent) = event {
                if let Some(gevent) = support::convert_event(gevent.clone(), &app.display) {
                    app.ui.handle_event(gevent);
                    event_loop.needs_update();
                }

                match gevent {
                    glium::glutin::Event::WindowEvent { event, .. } => match event {
                        // Break from the loop upon `Escape`.
                        glium::glutin::WindowEvent::CloseRequested
                        | glium::glutin::WindowEvent::KeyboardInput {
                            input:
                                glium::glutin::KeyboardInput {
                                    virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => {
                            state = State::Exit;
                            break 'main;
                        },
                        _ => (),
                    },
                    _ => (),
                }
            }

            let needs_update = layout::create_ui(&mut app, &mut state);
            if needs_update {
                event_loop.needs_update();
            }

            // Render the `Ui` and then display it on the screen.
            if let Some(primitives) = app.ui.draw_if_changed() {
                app.renderer.fill(&app.display.0, primitives, &app.images);
                let mut target = app.display.0.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                app.renderer
                    .draw(&app.display.0, &mut target, &app.images)
                    .unwrap();
                target.finish().unwrap();
            }

            trace!("bottom of main loop");
        }
    }
}
