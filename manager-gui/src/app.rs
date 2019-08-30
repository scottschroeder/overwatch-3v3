use crate::layout;
use crate::support;

use glium;
use glium::glutin;

use crate::image_util;
use crate::image_util::ImageId;
use overwatch::Hero;
use std::collections::BTreeMap;
use match_history::MatchDb;

pub struct App {
    pub ui: conrod_core::Ui,
    pub display: support::GliumDisplayWinitWrapper,
    pub ids: layout::Ids,
    pub images: image_util::ImageMap,
    pub renderer: conrod_glium::Renderer,
    pub assets: AppAssets,
}

pub struct AppAssets {
    pub portraits: BTreeMap<Hero, ImageId>,
}

impl App {
    pub fn new(window: support::GliumDisplayWinitWrapper, _events: &glutin::EventsLoop) -> Self {
        let window_dimm = window
            .0
            .gl_window()
            .window()
            .get_inner_size()
            .expect("expected getting window size to succeed.");

        // Create UI.
        let mut ui = conrod_core::UiBuilder::new([window_dimm.width, window_dimm.height]).build();
        let renderer = conrod_glium::Renderer::new(&window.0)
            .expect("expected loading conrod glium renderer to succeed.");

        // The image map describing each of our widget->image mappings (in our case, none).
        let mut image_map = image_util::ImageMap::new();

        let portraits = image_util::load_ow_portraits(&mut image_map, &window.0);

        let ids = layout::Ids::new(&mut ui.widget_id_generator());

        App {
            ui: ui,
            display: window,
            ids: ids,
            images: image_map,
            renderer: renderer,
            assets: AppAssets { portraits },
        }
    }
}
