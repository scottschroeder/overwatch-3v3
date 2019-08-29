use crate::app::App;
use crate::state::{RosterSelectState, UiEvent};
use std::collections::VecDeque;

use super::{frame, WidgetId, FOOTER_HEIGHT, HEADER_HEIGHT};
use conrod_core::{
    color,
    widget::{self, Canvas, Text},
    Borderable, Colorable, Labelable, Positionable, Sizeable, Widget,
};
use overwatch::overwatch_3v3::Player;

const LOGIN_WIDTH: conrod::Scalar = 300.0;
const LOGIN_HEIGHT: conrod::Scalar = 200.0;
const BATTLETAG_PADDING: conrod::Scalar = 5.0;
const LOGIN_LOWER_SECTION_HEIGHT: conrod::Scalar = (LOGIN_HEIGHT - HEADER_HEIGHT) / 3.0;

const INTERACTIVE_HEIGHT: conrod::Scalar = 100.00;
const BATTLETAG_INTERACTIVE_WIDTH: conrod::Scalar = 200.00;
const BATTLETAG_DISPLAY_WIDTH: conrod::Scalar = 150.0;
const PLAY_BUTTON_WIDTH: conrod::Scalar = 50.0;
const PLAY_BUTTON_HIGHT: conrod::Scalar = 30.0;

mod textbox {
    use crate::layout::WidgetId;

    const TEXTBOX_HEIGHT: Scalar = 30.0;
    const TEXTBOX_PADDING: Scalar = 10.0;

    use conrod_core::{
        widget::{self, Canvas, Text, TextBox},
        Borderable, Colorable, Labelable, Positionable, Scalar, Sizeable, Widget,
    };

    #[derive(Debug, Clone, Copy)]
    pub struct TextboxIds {
        pub canvas: WidgetId,
        pub textbox: WidgetId,
        pub label: WidgetId,
    }

    impl TextboxIds {
        pub fn new(gen: &mut conrod_core::widget::id::Generator) -> Self {
            Self {
                canvas: gen.next(),
                textbox: gen.next(),
                label: gen.next(),
            }
        }
    }

    pub fn textbox_field<F: FnMut(String)>(
        text: &str,
        mut update: F,
        ids: TextboxIds,
        width: conrod::Scalar,
        ui: &mut conrod_core::UiCell,
    ) -> bool {
        use conrod_core::widget::text_box::Event as TextBoxEvent;

        let events = TextBox::new(&text)
            // style
            .w_h(width, TEXTBOX_HEIGHT)
            .font_size(ui.theme.font_size_medium)
            .left_justify()
            .pad_text(5.0)
            // position
            .mid_bottom_of(ids.canvas)
            .set(ids.textbox, ui);

        let mut enter_pressed = false;

        for event in events.into_iter() {
            match event {
                TextBoxEvent::Update(s) => {
                    update(s);
                },
                TextBoxEvent::Enter => {
                    enter_pressed = true;
                    break;
                },
            }
        }
        enter_pressed
    }

    pub fn textbox_label(text: &str, ids: TextboxIds, ui: &mut conrod_core::UiCell) {
        Text::new(text)
            // style
            .font_size(ui.theme.font_size_small)
            .center_justify()
            .no_line_wrap()
            // position
            //.mid_left_with_margin_on(ids.canvas, TEXTBOX_PADDING)
            .top_left_with_margin_on(ids.canvas, TEXTBOX_PADDING)
            .set(ids.label, ui);
    }
}

pub struct RosterIds {
    root: WidgetId,
    header_canvas: WidgetId,
    header_label: WidgetId,

    interactive_canvas: WidgetId,
    input_canvas: WidgetId,
    input: textbox::TextboxIds,
    input_add_button: WidgetId,

    roster_canvas: WidgetId,
    roster_display_area: WidgetId,
    roster_players: [PlayerRosterIds; 3],
    battletags_canvas: WidgetId,

    footer_canvas: WidgetId,
    play_button: WidgetId,
    play_label: WidgetId,
}

impl RosterIds {
    pub fn new(gen: &mut conrod_core::widget::id::Generator) -> Self {
        Self {
            root: gen.next(),
            header_canvas: gen.next(),
            header_label: gen.next(),
            interactive_canvas: gen.next(),
            input_canvas: gen.next(),
            input: textbox::TextboxIds::new(gen),
            input_add_button: gen.next(),
            roster_canvas: gen.next(),
            roster_display_area: gen.next(),
            roster_players: [
                PlayerRosterIds::new(gen),
                PlayerRosterIds::new(gen),
                PlayerRosterIds::new(gen),
            ],
            battletags_canvas: gen.next(),
            footer_canvas: gen.next(),
            play_button: gen.next(),
            play_label: gen.next(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PlayerRosterIds {
    pub canvas: WidgetId,
    pub battletag: WidgetId,
    pub button: WidgetId,
    pub label: WidgetId,
}

impl PlayerRosterIds {
    pub fn new(gen: &mut conrod_core::widget::id::Generator) -> Self {
        Self {
            canvas: gen.next(),
            battletag: gen.next(),
            button: gen.next(),
            label: gen.next(),
        }
    }
}

pub fn create_ui(app: &mut App, state: &RosterSelectState, updates: &mut VecDeque<UiEvent>) {
    let ref mut ui = app.ui.set_widgets();
    let ref ids = app.ids;
    let body = Canvas::new().color(color::TRANSPARENT).border(0.0);
    frame(ui, ids, ids.root.body, body);

    let interactive_canvas = Canvas::new()
        .color(color::LIGHT_CHARCOAL)
        .border(0.0)
        .length(INTERACTIVE_HEIGHT);

    let interactive_split = Canvas::new()
        .color(color::TRANSPARENT)
        .border(0.0)
        .pad(20.0);

    let battletags_matrix = Canvas::new()
        // style
        .color(color::TRANSPARENT)
        .border(0.0)
        .border_color(color::BLACK);

    let footer_canvas = Canvas::new()
        // style
        .color(color::DARK_CHARCOAL)
        .border(0.0)
        .length(FOOTER_HEIGHT);

    // root canvas
    Canvas::new()
        .color(color::GREY)
        .flow_down(&[
            (
                ids.roster.interactive_canvas,
                interactive_canvas.flow_right(&[
                    (
                        ids.roster.input_canvas,
                        interactive_split
                            .clone()
                            .length(BATTLETAG_INTERACTIVE_WIDTH),
                    ),
                    (ids.roster.roster_canvas, interactive_split.clone()),
                ]),
            ),
            (ids.roster.battletags_canvas, battletags_matrix.clone()),
            (ids.roster.footer_canvas, footer_canvas.clone()),
        ])
        .middle_of(ids.root.body)
        .wh_of(ids.root.body)
        .set(ids.roster.root, ui);

    // Battletag input
    Canvas::new()
        .color(color::TRANSPARENT)
        .border(0.0)
        .w(BATTLETAG_INTERACTIVE_WIDTH * 0.8)
        .h(INTERACTIVE_HEIGHT * 0.6)
        .mid_bottom_of(ids.roster.input_canvas)
        .set(ids.roster.input.canvas, ui);

    Text::new("BattleTag")
        // style
        .font_size(ui.theme.font_size_medium)
        .left_justify()
        .no_line_wrap()
        .top_left_with_margin_on(ids.roster.input_canvas, BATTLETAG_PADDING / 2.0)
        .align_left_of(ids.roster.input.textbox)
        .set(ids.roster.input.label, ui);

    let battletag_enter = textbox::textbox_field(
        &state.battletag,
        |s| updates.push_front(UiEvent::RecordBattletag(s)),
        ids.roster.input,
        BATTLETAG_INTERACTIVE_WIDTH * 0.8,
        ui,
    );
    if battletag_enter {
        updates.push_front(UiEvent::EnterBattleTag);
    }

    // Roster Display
    Canvas::new()
        .color(color::RED)
        .border(0.0)
        .w(BATTLETAG_DISPLAY_WIDTH * 3.0)
        .h(INTERACTIVE_HEIGHT)
        .mid_left_of(ids.roster.roster_canvas)
        .flow_right(&[
            (
                ids.roster.roster_players[0].canvas,
                interactive_split.clone().color(color::LIGHT_ORANGE),
            ),
            (
                ids.roster.roster_players[1].canvas,
                interactive_split.clone().color(color::ORANGE),
            ),
            (
                ids.roster.roster_players[2].canvas,
                interactive_split.clone().color(color::DARK_ORANGE),
            ),
        ])
        .set(ids.roster.roster_display_area, ui);

    for player in Player::iter() {
        let pids = &ids.roster.roster_players[player.index()];
        create_roster_battletag(state, player, pids, updates, ui);
    }

    if state.ready_to_play() {
        let play = widget::Button::new()
            .color(color::ORANGE)
            .w_h(PLAY_BUTTON_WIDTH, PLAY_BUTTON_HIGHT)
            .mid_right_with_margin_on(
                ids.roster.footer_canvas,
                (FOOTER_HEIGHT - PLAY_BUTTON_HIGHT) / 2.0,
            )
            .label("play")
            .align_middle_y_of(ids.roster.footer_canvas);

        for _event in play.set(ids.roster.play_button, ui) {
            updates.push_front(UiEvent::RosterPlay)
        }
    }
}

fn create_roster_battletag(
    state: &RosterSelectState,
    player: Player,
    ids: &PlayerRosterIds,
    updates: &mut VecDeque<UiEvent>,
    ui: &mut conrod_core::UiCell,
) {
    Text::new(&format!("{}", player))
        // style
        .font_size(ui.theme.font_size_medium)
        .left_justify()
        .no_line_wrap()
        .top_left_with_margin_on(ids.canvas, BATTLETAG_PADDING / 2.0)
        .set(ids.label, ui);

    if let Some(bt) = state.get_battletag(player) {
        Text::new(bt)
            // style
            .font_size(ui.theme.font_size_medium)
            .left_justify()
            .no_line_wrap()
            .align_left_of(ids.label)
            .set(ids.battletag, ui);

        let clear = widget::Button::new()
            .color(color::TRANSPARENT)
            .wh_of(ids.canvas)
            .middle_of(ids.canvas);

        for _event in clear.set(ids.button, ui) {
            updates.push_front(UiEvent::RemoveFromRoster(player))
        }
    }
}
