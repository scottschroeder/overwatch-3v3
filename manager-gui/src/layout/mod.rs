use crate::app::App;
use crate::layout::roster_layout::RosterIds;
use crate::state::State;
use std::collections::VecDeque;

pub type WidgetId = conrod_core::widget::Id;

const HEADER_HEIGHT: conrod::Scalar = 30.0;
const FOOTER_HEIGHT: conrod::Scalar = 50.0;
pub const BACKGROUND_RGB: [f32; 3] = [0.0625, 0.46875, 0.3125];
pub const BACKGROUND: conrod_core::Color =
    conrod_core::Color::Rgba(BACKGROUND_RGB[0], BACKGROUND_RGB[1], BACKGROUND_RGB[2], 1.0);
pub const BATTLETAG_COLOR: conrod_core::Color = conrod_core::color::LIGHT_BLUE;

pub use dynamic_ids::{DynamicIds, WidgetHolder};

mod dynamic_ids {

    use conrod_core::widget::id::Generator;

    pub trait WidgetHolder {
        fn allocate_ids(gen: &mut Generator) -> Self;
    }

    impl<T> WidgetHolder for DynamicIds<T> {
        fn allocate_ids(_gen: &mut Generator) -> Self {
            DynamicIds::default()
        }
    }

    #[derive(Debug)]
    pub struct DynamicIds<T> {
        inner: Vec<T>,
    }

    impl<T> Default for DynamicIds<T> {
        fn default() -> DynamicIds<T> {
            DynamicIds { inner: Vec::new() }
        }
    }

    impl<T> DynamicIds<T> {
        pub fn len(&self) -> usize {
            self.inner.len()
        }
    }

    impl<T: WidgetHolder> DynamicIds<T> {
        pub fn resize(&mut self, size: usize, gen: &mut Generator) {
            while self.inner.len() <= size {
                self.inner.push(T::allocate_ids(gen))
            }
        }
    }

    impl<T: Copy> DynamicIds<T> {
        pub fn get(&self, idx: usize) -> T {
            self.inner[idx]
        }
    }
}

pub struct Ids {
    root: RootIds,
    roster: roster_layout::RosterIds,
    match_record: match_layout::MatchIds,
}

impl Ids {
    pub fn new(gen: &mut conrod_core::widget::id::Generator) -> Ids {
        Ids {
            root: RootIds::new(gen),
            roster: RosterIds::new(gen),
            match_record: MatchIds::new(gen),
        }
    }
}

pub struct RootIds {
    root: WidgetId,
    header: WidgetId,
    body: WidgetId,
}

impl RootIds {
    pub fn new(gen: &mut conrod_core::widget::id::Generator) -> RootIds {
        RootIds {
            root: gen.next(),
            header: gen.next(),
            body: gen.next(),
        }
    }
}

pub fn create_ui(app: &mut App, state: &mut State) -> bool {
    trace!("Update UI");
    let mut updates = VecDeque::new();

    match state {
        State::RosterSelect(roster_state, _) => {
            self::roster_layout::create_ui(app, roster_state, &mut updates)
        },
        State::Match(match_state, _) => self::match_layout::create_ui(app, match_state, &mut updates),
        State::Exit => (),
        _ => unimplemented!("unknown window state, can not draw UI"),
    }

    let update = !updates.is_empty();
    state.transform(updates.drain(..));
    update
}

use crate::layout::match_layout::MatchIds;

use conrod_core::{
    color, widget::Canvas, Borderable, Colorable, Labelable, Positionable, Sizeable, UiCell, Widget,
};

fn frame(ui: &mut conrod_core::UiCell, ids: &Ids, body_id: WidgetId, body: Canvas) {
    let header = Canvas::new()
        .color(color::DARK_CHARCOAL)
        .border(0.0)
        .length(HEADER_HEIGHT);

    Canvas::new()
        .color(BACKGROUND)
        .border(0.0)
        .flow_down(&[(ids.root.header, header), (body_id, body)])
        .set(ids.root.root, ui);
}

mod dynamic_matrix;

mod roster_layout;

mod match_layout;
