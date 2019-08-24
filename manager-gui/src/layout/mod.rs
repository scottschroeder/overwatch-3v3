use crate::app::App;
use crate::layout::roster_layout::RosterIds;
use crate::state::State;
use std::collections::VecDeque;

pub type WidgetId = conrod_core::widget::Id;

const HEADER_HEIGHT: conrod::Scalar = 30.0;
const FOOTER_HEIGHT: conrod::Scalar = 100.0;
pub const BACKGROUND_RGB: [f32; 3] = [0.0625, 0.46875, 0.3125];
pub const BACKGROUND: conrod_core::Color =
    conrod_core::Color::Rgba(BACKGROUND_RGB[0], BACKGROUND_RGB[1], BACKGROUND_RGB[2], 1.0);
pub const BATTLETAG_COLOR: conrod_core::Color = conrod_core::color::LIGHT_BLUE;

pub struct Ids {
    root: RootIds,
    roster: roster_layout::RosterIds,
}

impl Ids {
    pub fn new(gen: &mut conrod_core::widget::id::Generator) -> Ids {
        Ids {
            root: RootIds::new(gen),
            roster: RosterIds::new(gen),
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
        State::RosterSelect(roster_state) => {
            self::roster_layout::create_ui(app, roster_state, &mut updates)
        },
        State::Match(match_state) => self::match_layout::create_ui(app, match_state, &mut updates),
        State::Exit => (),
        _ => unimplemented!("unknown window state, can not draw UI"),
    }

    let update = !updates.is_empty();
    state.transform(updates.drain(..));
    update
}

use conrod_core::{
    color,
    widget::{self, Canvas},
    Borderable, Colorable, Labelable, Positionable, Sizeable, Widget,
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

mod roster_layout;
mod match_layout {

    use crate::app::App;
    use crate::state::{MatchState, UiEvent};
    use std::collections::VecDeque;

    use super::{frame, WidgetId, FOOTER_HEIGHT, HEADER_HEIGHT};
    use conrod_core::{
        color,
        widget::{self, Canvas, Text, TextBox},
        Borderable, Colorable, Labelable, Positionable, Sizeable, Widget,
    };

    pub fn create_ui(app: &mut App, state: &MatchState, updates: &mut VecDeque<UiEvent>) {
        let ref mut ui = app.ui.set_widgets();
        let ref ids = app.ids;
        let body = Canvas::new().color(color::TRANSPARENT).border(0.0);
        frame(ui, ids, ids.root.body, body);
    }
}
