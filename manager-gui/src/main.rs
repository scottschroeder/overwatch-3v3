//! A simple demonstration of how to construct and use Canvasses by splitting up the window.

#[macro_use]
extern crate conrod_core;
extern crate conrod_glium;
#[macro_use]
extern crate conrod_winit;
extern crate find_folder;
extern crate glium;
extern crate image;

use conrod_core::{Colorable, Labelable, Positionable, Sizeable, Ui, Scalar};
use glium::Surface;

use overwatch::overwatch_3v3::{Match, Roster};
use overwatch::{
    overwatch_3v3::{CompBuilder, Player},
    BattleTag, Hero, HeroPool,
};
use std::collections::BTreeMap;
use std::mem;
use std::path;

mod support;

type ImageMap = conrod_core::image::Map<glium::texture::Texture2d>;
type ImageId = conrod_core::image::Id;
type WidgetId = conrod_core::widget::Id;

const OVERWATCH_PORTRAITS: &str = "images/overwatch/portraits/";

mod window_mgmt {
    use glium::glutin;
    use rusttype;
    use crate::support;
    use crate::app::App;

    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;

    pub fn load_font() -> rusttype::Font<'static> {
        let font_data = include_bytes!("../../assets/fonts/NotoSans/NotoSans-Regular.ttf");
        let collection = rusttype::FontCollection::from_bytes(font_data as &[u8])
            .expect("font was invalid?");

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
        let display = glium::Display::new(window, context, &events_loop)
            .expect("unable to create new window");

        let display = support::GliumDisplayWinitWrapper(display);

        // Create UI and other components.
        let mut app = App::new(display, &events_loop);

        // Add font.
        app.ui.fonts.insert(load_font());

        (events_loop, app)
    }

    pub fn main_window_loop(events: glutin::EventsLoop, mut app: App) {

    }
}
mod state {
    enum State {
        Match
    }
}

mod layout {
    pub struct Ids;
}

mod app {
    use conrod;
    use glium;
    use glium::glutin;
    use crate::support;
    use crate::layout;

    type ImageMap = conrod_core::image::Map<glium::texture::Texture2d>;

    pub struct App {
        pub ui: conrod_core::Ui,
        pub display: support::GliumDisplayWinitWrapper,
        pub ids: layout::Ids,
        pub images: ImageMap,
        pub renderer: conrod_glium::Renderer,
    }

    impl App {
        pub fn new(window: support::GliumDisplayWinitWrapper, events: &glutin::EventsLoop) -> Self {
            let window_dimm = window.0
                .gl_window()
                .window()
                .get_inner_size()
                .expect("expected getting window size to succeed.");

            // Create UI.
            let mut ui = conrod_core::UiBuilder::new([window_dimm.width, window_dimm.height]).build();
            let mut renderer = conrod_glium::Renderer::new(&window.0)
                .expect("expected loading conrod glium renderer to succeed.");

            // The image map describing each of our widget->image mappings (in our case, none).
            let mut image_map = ImageMap::new();
            // Instantiate the generated list of widget identifiers.

            App {
                ui: ui,
                display: window,
                ids: layout::Ids,
                images: image_map,
                renderer: renderer,
            }
        }
    }
}

fn main() {
    color_backtrace::install();
    let (mut events_loop, mut app) = window_mgmt::init_window();

    let assets = find_folder::Search::KidsThenParents(3, 5)
        .for_folder("assets")
        .unwrap();

    let ids = &mut Ids::new(app.ui.widget_id_generator());

    let portraits = load_ow_portraits(&mut app.images, &app.display.0, &assets);
    let player_widgets = vec![
        PlayerWidgets::new(&mut app.ui),
        PlayerWidgets::new(&mut app.ui),
        PlayerWidgets::new(&mut app.ui),
    ];

    let history_widgets = vec![
        HistoryWidgets::new(&mut app.ui),
        HistoryWidgets::new(&mut app.ui),
        HistoryWidgets::new(&mut app.ui),
        HistoryWidgets::new(&mut app.ui),
        HistoryWidgets::new(&mut app.ui),
    ];

    let mut builder_state = MatchState::new(Roster::default());

    // Poll events from the window.
    let mut event_loop = support::EventLoop::new();
    'main: loop {
        // Handle all events.
        for event in event_loop.next(&mut events_loop) {
            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = support::convert_event(event.clone(), &app.display) {
                app.ui.handle_event(event);
                event_loop.needs_update();
            }

            match event {
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
                    } => break 'main,
                    _ => (),
                },
                _ => (),
            }
        }

        // Instantiate all widgets in the GUI.
        builder_widgets(
            app.ui.set_widgets(),
            ids,
            &mut builder_state,
            &portraits,
            &player_widgets,
            &history_widgets,
        );

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = app.ui.draw_if_changed() {
            app.renderer.fill(&app.display.0, primitives, &app.images);
            let mut target = app.display.0.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            app.renderer.draw(&app.display.0, &mut target, &app.images).unwrap();
            target.finish().unwrap();
        }
    }
}

fn load_ow_portraits(
    image_map: &mut ImageMap,
    display: &glium::Display,
    assets: &path::Path,
) -> BTreeMap<Hero, ImageId> {
    let portrait_dir = assets.join(OVERWATCH_PORTRAITS);
    overwatch::Hero::iter()
        .map(|h| {
            let mut hero_portrait = portrait_dir.clone();
            hero_portrait.push(h.blizzard_name());
            hero_portrait.set_extension("png");
            //let img = conrod_core::image::open(hero_portrait).unwrap();
            let img = load_image(display, hero_portrait);
            let id = image_map.insert(img);
            (h, id)
        })
        .collect()
}

// Load an image from our assets folder as a texture we can draw to the screen.
fn load_image<P>(display: &glium::Display, path: P) -> glium::texture::Texture2d
    where
        P: AsRef<std::path::Path>,
{
    let path = path.as_ref();
    let rgba_image = image::open(&std::path::Path::new(&path)).unwrap().to_rgba();
    let image_dimensions = rgba_image.dimensions();
    let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(
        &rgba_image.into_raw(),
        image_dimensions,
    );
    //let texture = glium::texture::SrgbTexture2d::new(display, raw_image).unwrap();
    let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
    texture
}

#[derive(Debug, Default)]
struct MatchState {
    selected_player: Player,
    builder: CompBuilder,
    history: Match,
    used_heros: HeroPool,
}

impl MatchState {
    fn new(roster: Roster) -> MatchState {
        MatchState {
            builder: CompBuilder::new(roster),
            ..MatchState::default()
        }
    }

    fn win_color(&self) -> conrod_core::color::Color {
        match self.builder.get_win() {
            Some(true) => conrod_core::color::GREEN,
            _ => conrod_core::color::LIGHT_GREEN,
        }
    }
    fn lose_color(&self) -> conrod_core::color::Color {
        match self.builder.get_win() {
            Some(false) => conrod_core::color::RED,
            _ => conrod_core::color::LIGHT_RED,
        }
    }

    fn press_win(&mut self) {
        match self.builder.get_win() {
            Some(true) => self.builder.clear_win(),
            _ => self.builder.set_win(true),
        }
    }
    fn press_lose(&mut self) {
        match self.builder.get_win() {
            Some(false) => self.builder.clear_win(),
            _ => self.builder.set_win(false),
        }
    }
    fn press_finalize(&mut self) {
        let mut builder = CompBuilder::new(self.builder.roster());
        mem::swap(&mut self.builder, &mut builder);
        let r = builder.finalize().unwrap();
        self.history.insert_round(r).unwrap();
        self.used_heros = self.history.used_heros();
        self.selected_player = Player::One;
    }

    fn select_hero(&mut self, hero: Hero) {
        if self.used_heros.contains(&hero) {
            return;
        }
        self.clear_hero_selection(self.selected_player);
        self.builder.set_player(self.selected_player, hero);
        self.used_heros.insert(hero);
        self.selected_player = self.selected_player.cycle_next();
    }

    #[inline]
    fn clear_hero_selection(&mut self, player: Player) {
        if let Some(hero) = self.builder.get_hero(player) {
            self.used_heros.remove(&hero);
            self.builder.clear_hero(player);
        }
    }

    fn press_select_player(&mut self, player: Player) {
        self.selected_player = player;
        self.clear_hero_selection(player);
    }
}

fn max_square(id: WidgetId, ui: &Ui) -> Option<Scalar> {
    ui.h_of(id)
        .and_then(|h| {
            ui.w_of(id).map(|w| (w, h))
        })
        .map(|(w, h)| {
            w.min(h)
        })
}

#[derive(Debug)]
struct PlayerWidgets {
    canvas: WidgetId,
    portrait_border_canvas: WidgetId,
    portrait_canvas: WidgetId,
    battletag_canvas: WidgetId,
    select_button: WidgetId,
    portrait_image: WidgetId,
    battletag_text: WidgetId,
}

#[derive(Debug)]
struct HistoryWidgets {
    hero1: WidgetId,
    hero2: WidgetId,
    hero3: WidgetId,
    hero1_portrait: WidgetId,
    hero2_portrait: WidgetId,
    hero3_portrait: WidgetId,
    outcome: WidgetId,
    edit: WidgetId,
}

impl HistoryWidgets {
    fn new(ui: &mut Ui) -> HistoryWidgets {
        let mut gen = ui.widget_id_generator();
        HistoryWidgets {
            hero1: gen.next(),
            hero2: gen.next(),
            hero3: gen.next(),
            hero1_portrait: gen.next(),
            hero2_portrait: gen.next(),
            hero3_portrait: gen.next(),
            outcome: gen.next(),
            edit: gen.next(),
        }
    }
    fn player_widgets(&self, player: Player) -> (WidgetId, WidgetId) {
        match player {
            Player::One => (self.hero1, self.hero1_portrait),
            Player::Two => (self.hero2, self.hero2_portrait),
            Player::Three => (self.hero3, self.hero3_portrait),
        }
    }
}

type Split<'a> = (WidgetId, conrod_core::widget::Canvas<'a>);

impl PlayerWidgets {
    fn new(ui: &mut Ui) -> PlayerWidgets {
        let mut gen = ui.widget_id_generator();
        PlayerWidgets {
            canvas: gen.next(),
            portrait_border_canvas: gen.next(),
            portrait_canvas: gen.next(),
            battletag_canvas: gen.next(),
            select_button: gen.next(),
            portrait_image: gen.next(),
            battletag_text: gen.next(),
        }
    }

    fn portrait<'a>(&self, active: bool) -> Split {
        let color = if active {
            conrod_core::color::LIGHT_ORANGE
        } else {
            conrod_core::color::TRANSPARENT
        };
        (
            self.portrait_border_canvas,
            conrod_core::widget::Canvas::new()
                .color(color)
                .length_weight(0.8),
        )
    }
    fn battletag<'a>(&self) -> Split<'a> {
        (
            self.battletag_canvas,
            conrod_core::widget::Canvas::new()
                .color(conrod_core::color::TRANSPARENT)
                //.label(bt.as_str())
                //.title_bar(bt.as_str())
                .length_weight(0.2),
        )
    }
}

fn builder_widgets(
    ref mut ui: conrod_core::UiCell,
    ids: &mut Ids,
    builder_state: &mut MatchState,
    portraits: &BTreeMap<Hero, ImageId>,
    players: &[PlayerWidgets],
    match_history: &[HistoryWidgets],
) {
    use conrod_core::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};

    let player_splits = players
        .iter()
        .zip(Player::iter())
        .map(|(pw, p)| {
            vec![pw.portrait(builder_state.selected_player == p), pw.battletag()]
        })
        .collect::<Vec<_>>();

    let player_canvas = players
        .iter()
        .zip(Player::iter())
        .enumerate()
        .map(|(idx, (pw, p))| {
            (
                pw.canvas,
                widget::Canvas::new()
                    .color(color::TRANSPARENT)
                    //.pad(20.0)
                    .flow_down(&player_splits[idx]),
            )
        })
        .collect::<Vec<_>>();

    // Construct our main `Canvas` tree.
    widget::Canvas::new()
        .flow_down(&[
            (
                ids.builder_header,
                widget::Canvas::new()
                    .color(color::BLUE)
                    .pad_bottom(20.0)
                    .length_weight(0.10),
            ),
            (
                ids.builder_controller,
                widget::Canvas::new()
                    .flow_right(&[
                        (
                            ids.builder_controller_reset,
                            widget::Canvas::new()
                                .color(color::ORANGE)
                                .pad(20.0)
                                .length_weight(1.0 / 8.0),
                        ),
                        (
                            ids.builder_controller_players,
                            widget::Canvas::new()
                                .color(color::ORANGE)
                                .flow_right(&player_canvas)
                                .length_weight(4.0 / 8.0),
                        ),
                        (
                            ids.builder_controller_finalize,
                            widget::Canvas::new()
                                .color(color::ORANGE)
                                .pad(20.0)
                                .length_weight(3.0 / 8.0)
                                .flow_right(&[
                                    (
                                        ids.builder_lose,
                                        widget::Canvas::new().color(color::TRANSPARENT).pad(20.0),
                                    ),
                                    (
                                        ids.builder_win,
                                        widget::Canvas::new().color(color::TRANSPARENT).pad(20.0),
                                    ),
                                    (
                                        ids.builder_finalize,
                                        widget::Canvas::new().color(color::TRANSPARENT).pad(20.0),
                                    ),
                                ]),
                        ),
                    ])
                    .length_weight(0.25),
            ),
            (
                ids.builder_suggestions_pane,
                widget::Canvas::new().color(color::BLUE).length_weight(0.2),
            ),
            (
                ids.builder_lower_pane,
                widget::Canvas::new()
                    .color(color::BLACK)
                    .length_weight(0.6)
                    .flow_right(&[
                        (
                            ids.builder_match_history,
                            widget::Canvas::new()
                                .color(color::BLACK)
                                .length_weight(0.2)
                                .flow_down(&[
                                    (
                                        ids.builder_match_history_title,
                                        widget::Canvas::new()
                                            .color(color::TRANSPARENT)
                                            .length_weight(0.2),
                                    ),
                                    (
                                        ids.builder_match_history_data,
                                        widget::Canvas::new()
                                            .color(color::TRANSPARENT)
                                            .length_weight(0.8),
                                    ),
                                ]),
                        ),
                        (
                            ids.builder_heros,
                            widget::Canvas::new().color(color::BLACK).length_weight(0.4),
                        ),
                    ]),
            ),
            (
                ids.builder_footer,
                widget::Canvas::new().color(color::BLUE).length_weight(0.1),
            ),
        ])
        .set(ids.master, ui);

    for (pw, p) in players.iter().zip(Player::iter()) {
        if let Some(image_id) = builder_state.builder.get_hero(p).map(|h| portraits[&h]) {
            let max_dimm = max_square(pw.portrait_border_canvas, &ui).unwrap() * 0.8;

            widget::Canvas::new()
                .color(color::BLACK)
                .w_h(max_dimm, max_dimm)
                .middle_of(pw.portrait_border_canvas)
                .set(pw.portrait_canvas, ui);

            widget::Image::new(image_id)
                .wh_of(pw.portrait_canvas)
                .middle_of(pw.portrait_canvas)
                .set(pw.portrait_image, ui);
        }

        let bt = builder_state.builder.get_battletag(p);
        //        widget::Canvas::new()
        //            .color(color::TRANSPARENT)
        //            .wh_of(pw.battletag_canvas)
        //            .middle_of(pw.battletag_canvas)
        //            .label(bt.as_str())
        //            .set(pw.battletag_text, ui)
        //            ;
        let bt_h = ui.h_of(pw.battletag_canvas).unwrap() * 0.7;
        let bt_w = ui.w_of(pw.battletag_canvas).unwrap() - bt_h;
        widget::Text::new(bt.as_str())
            .color(color::ORANGE.complement())
            .font_size(bt_h as u32)
            .h(bt_h)
            .w(bt_w)
            .center_justify()
            //.middle_of(pw.battletag_canvas)
            .mid_top_of(pw.battletag_canvas)
            .set(pw.battletag_text, ui);

        let button = widget::Button::new()
            .color(color::TRANSPARENT)
            .wh_of(pw.canvas)
            .middle_of(pw.canvas);

        for _click in button.set(pw.select_button, ui) {
            builder_state.press_select_player(p)
        }
    }

    for _click in widget::Button::new()
        .color(builder_state.lose_color())
        .wh_of(ids.builder_lose)
        .middle_of(ids.builder_lose)
        .set(ids.builder_lose_button, ui)
        {
            builder_state.press_lose()
        }

    for _click in widget::Button::new()
        .color(builder_state.win_color())
        .wh_of(ids.builder_win)
        .middle_of(ids.builder_win)
        .set(ids.builder_win_button, ui)
        {
            builder_state.press_win()
        }

    if builder_state.builder.validate() {
        for _click in widget::Button::new()
            .color(color::BLUE)
            .wh_of(ids.builder_finalize)
            .middle_of(ids.builder_finalize)
            .set(ids.builder_finalize_button, ui)
            {
                builder_state.press_finalize()
            }
    }

    widget::Text::new("Match History")
        .color(color::WHITE)
        .wh_of(ids.builder_match_history_title)
        .middle_of(ids.builder_match_history_title)
        .center_justify()
        .set(ids.builder_match_history_title_text, ui);

    let mut match_history_elements = widget::Matrix::new(1, 5)
        .middle_of(ids.builder_match_history_data)
        .wh_of(ids.builder_match_history_data)
        .set(ids.builder_match_history_data_matrix, ui);

    for (ridx, round) in builder_state.history.iter().enumerate() {
        let ref round_widgets = match_history[ridx];
        let elem = match_history_elements.next(ui).unwrap();
        let c = if round.win { color::GREEN } else { color::RED };
        let match_canvas = &[
            (
                round_widgets.hero1,
                widget::Canvas::new().color(color::BLACK),
            ),
            (
                round_widgets.hero2,
                widget::Canvas::new().color(color::BLACK),
            ),
            (
                round_widgets.hero3,
                widget::Canvas::new().color(color::BLACK),
            ),
            (round_widgets.outcome, widget::Canvas::new().color(c)),
            (round_widgets.edit, widget::Canvas::new().color(color::BLUE)),
        ];
        let canvas = widget::Canvas::new()
            .color(color::RED)
            .wh_of(elem.widget_id)
            .middle_of(elem.widget_id)
            .flow_right(match_canvas);
        elem.set(canvas, ui);

        for player in Player::iter() {
            let hero = round.get_hero(player);
            let (canvas_id, portrait_id) = round_widgets.player_widgets(player);
            widget::Image::new(portraits[&hero])
                .wh_of(canvas_id)
                .middle_of(canvas_id)
                .set(portrait_id, ui);
        }
    }

    let hero_portraits_cols = 7;
    let hero_portraits_rows = 5;

    let mut hero_matrix = widget::Matrix::new(hero_portraits_cols, hero_portraits_rows)
        .middle_of(ids.builder_heros)
        .wh_of(ids.builder_heros)
        .set(ids.builder_hero_matrix, ui);

    let mut hero_used_matrix = widget::Matrix::new(hero_portraits_cols, hero_portraits_rows)
        .middle_of(ids.builder_heros)
        .wh_of(ids.builder_heros)
        .set(ids.builder_hero_used_matrix, ui);

    let matrix_dimm = max_square(ids.builder_hero_matrix, &ui).unwrap();
    let min_rc = std::cmp::min(hero_portraits_rows, hero_portraits_cols);
    let elem_dimm = matrix_dimm / (min_rc as f64);

    for (hero, img) in portraits {
        let elem = hero_matrix.next(ui).unwrap(); // should only fail if we add more heros
        let used_elem = hero_used_matrix.next(ui).unwrap(); // should only fail if we add more heros

        let button = widget::Button::image(*img)
            .middle_of(elem.widget_id)
            .w_h(elem_dimm, elem_dimm);

        for _click in elem.set(button, ui) {
            builder_state.select_hero(*hero);
        }
        if builder_state.used_heros.contains(&hero) {
            let canvas = widget::Canvas::new()
                .color(color::Color::Rgba(0.3, 0.3, 0.3, 0.8))
                .parent(elem.widget_id)
                .middle_of(elem.widget_id)
                .w_h(elem_dimm, elem_dimm);
            used_elem.set(canvas, ui);
        }
    }
}

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    struct Ids {
        master,
        // Builder
        builder_header,
        builder_controller,
        builder_controller_reset,
        builder_controller_players,
        builder_controller_finalize,
        builder_suggestions,
        builder_heros,
        builder_hero_matrix,
        builder_hero_used_matrix,

        builder_suggestions_pane,

        builder_lose,
        builder_lose_button,
        builder_win,
        builder_win_button,
        builder_finalize,
        builder_finalize_button,
        builder_lower_pane,
        builder_match_history,
        builder_match_history_title,
        builder_match_history_title_text,
        builder_match_history_data,
        builder_match_history_data_matrix,

        builder_footer,
    }
}
