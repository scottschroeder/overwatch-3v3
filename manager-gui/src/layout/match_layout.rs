use crate::app::{App, AppAssets};
use crate::state::{MatchState, UiEvent};
use std::collections::VecDeque;

use super::{frame, WidgetId, FOOTER_HEIGHT};
use crate::layout::dynamic_matrix::Matrix;
use crate::layout::{DynamicIds, WidgetHolder};
use conrod::Scalar;

use conrod_core::UiCell;
use conrod_core::{
    color,
    widget::{self, Canvas, Text},
    Borderable, Colorable, Labelable, Positionable, Sizeable, Widget,
};
use overwatch::overwatch_3v3::Player;
use overwatch::{Hero, Role};

const CONTROL_HEIGHT: conrod::Scalar = 130.0;
const ROSTER_PLAYER_WIDTH: conrod::Scalar = 150.0;
const PORTRAIT_MINI_HEIGHT: conrod::Scalar = 50.0;
const PORTRAIT_FULL_HEIGHT: conrod::Scalar = 80.0;
const ROUND_RECORD_W_MIN: conrod::Scalar = PORTRAIT_MINI_HEIGHT * 4.0;
const ROUND_RECORD_W_MAX: conrod::Scalar = PORTRAIT_MINI_HEIGHT * 6.0;
const ROUND_RECORD_BORDER: conrod::Scalar = 1.0;
const MATCH_HISTORY_TITLE_HEIGHT: conrod::Scalar = 40.0;
const BATTLETAG_HEIGHT: conrod::Scalar = 25.0;

struct PlayerRosterIds {
    canvas: WidgetId,
    portrait_canvas: WidgetId,
    portrait_image: WidgetId,
    battletag_canvas: WidgetId,
    battletag_label: WidgetId,
    select_button: WidgetId,
}

impl PlayerRosterIds {
    pub fn new(gen: &mut conrod_core::widget::id::Generator) -> Self {
        Self {
            canvas: gen.next(),
            portrait_canvas: gen.next(),
            portrait_image: gen.next(),
            battletag_canvas: gen.next(),
            battletag_label: gen.next(),
            select_button: gen.next(),
        }
    }
}

#[derive(Clone, Copy)]
struct HeroSelectPortrait {
    portrait_canvas: WidgetId,
    portrait_image: WidgetId,
    overlay: WidgetId,
}

impl WidgetHolder for HeroSelectPortrait {
    fn allocate_ids(gen: &mut conrod_core::widget::id::Generator) -> Self {
        HeroSelectPortrait {
            portrait_canvas: gen.next(),
            portrait_image: gen.next(),
            overlay: gen.next(),
        }
    }
}

#[derive(Clone, Copy)]
struct MatchHistoryEntry {
    canvas: WidgetId,
    heros_canvas: WidgetId,
    heros: [WidgetId; 3],
    outcome_canvas: WidgetId,
    outcome_label: WidgetId,
}

impl WidgetHolder for MatchHistoryEntry {
    fn allocate_ids(gen: &mut conrod_core::widget::id::Generator) -> Self {
        Self {
            canvas: gen.next(),
            heros_canvas: gen.next(),
            heros: [gen.next(), gen.next(), gen.next()],
            outcome_canvas: gen.next(),
            outcome_label: gen.next(),
        }
    }
}

pub struct MatchIds {
    root: WidgetId,
    control_canvas: WidgetId,
    roster_players: [PlayerRosterIds; 3],
    round_outcome_canvas: WidgetId,
    round_outcome_toggle: WidgetId,
    round_outcome_submit: WidgetId,
    suggestion_canvas: WidgetId,
    suggestion_matrix: WidgetId,
    suggestion_dynamic: DynamicIds<HeroSelectPortrait>,
    match_canvas: WidgetId,
    match_history_canvas: WidgetId,
    match_history_title_canvas: WidgetId,
    match_history_title_label: WidgetId,
    match_history_data_canvas: WidgetId,
    match_history_matrix: WidgetId,
    match_history_dynamic: DynamicIds<MatchHistoryEntry>,
    hero_selection: HeroSelectionRoleIds,
    footer: WidgetId,
}

pub struct HeroSelectionRoleIds {
    canvas: WidgetId,
    matrix: WidgetId,
    dynamic: DynamicIds<HeroSelectPortrait>,
}

impl HeroSelectionRoleIds {
    pub fn new(gen: &mut conrod_core::widget::id::Generator) -> Self {
        Self {
            canvas: gen.next(),
            matrix: gen.next(),
            dynamic: DynamicIds::default(),
        }
    }
}

impl MatchIds {
    pub fn new(gen: &mut conrod_core::widget::id::Generator) -> Self {
        Self {
            root: gen.next(),

            control_canvas: gen.next(),
            roster_players: [
                PlayerRosterIds::new(gen),
                PlayerRosterIds::new(gen),
                PlayerRosterIds::new(gen),
            ],
            round_outcome_canvas: gen.next(),
            round_outcome_toggle: gen.next(),
            round_outcome_submit: gen.next(),
            suggestion_canvas: gen.next(),
            suggestion_matrix: gen.next(),
            match_canvas: gen.next(),
            match_history_canvas: gen.next(),
            match_history_title_canvas: gen.next(),
            match_history_title_label: gen.next(),
            match_history_data_canvas: gen.next(),
            match_history_matrix: gen.next(),
            match_history_dynamic: Default::default(),
            hero_selection: HeroSelectionRoleIds::new(gen),
            footer: gen.next(),
            suggestion_dynamic: Default::default(),
        }
    }
}

pub fn create_ui(app: &mut App, state: &MatchState, updates: &mut VecDeque<UiEvent>) {
    let ref mut ui = app.ui.set_widgets();
    let ref mut ids = app.ids;
    let ref assets = app.assets;
    let body = Canvas::new().color(color::TRANSPARENT).border(0.0);
    frame(ui, ids, ids.root.body, body);

    let blank_canvas = Canvas::new().color(color::TRANSPARENT).border(0.0);

    let round_record_w_max = ROUND_RECORD_W_MAX.min(ui.w_of(ids.root.body).unwrap() * 0.3);
    let round_record_width = ROUND_RECORD_W_MIN.max(round_record_w_max);

    // root canvas
    blank_canvas
        .clone()
        .color(color::GRAY)
        .flow_down(&[
            (
                ids.match_record.control_canvas,
                blank_canvas
                    .clone()
                    .color(color::LIGHT_CHARCOAL)
                    .length(CONTROL_HEIGHT)
                    .flow_right(&[
                        (
                            ids.match_record.roster_players[0].canvas,
                            blank_canvas
                                .clone()
                                .color(color::LIGHT_ORANGE)
                                .length(ROSTER_PLAYER_WIDTH),
                        ),
                        (
                            ids.match_record.roster_players[1].canvas,
                            blank_canvas
                                .clone()
                                .color(color::ORANGE)
                                .length(ROSTER_PLAYER_WIDTH),
                        ),
                        (
                            ids.match_record.roster_players[2].canvas,
                            blank_canvas
                                .clone()
                                .color(color::DARK_ORANGE)
                                .length(ROSTER_PLAYER_WIDTH),
                        ),
                        (ids.match_record.round_outcome_canvas, blank_canvas.clone()),
                    ]),
            ),
            (
                ids.match_record.suggestion_canvas,
                blank_canvas
                    .clone()
                    .color(color::BLACK)
                    .length(PORTRAIT_MINI_HEIGHT * 3.0),
            ),
            (
                ids.match_record.match_canvas,
                blank_canvas.clone().flow_right(&[
                    (
                        ids.match_record.match_history_canvas,
                        blank_canvas.clone().length(round_record_width).flow_down(&[
                            (
                                ids.match_record.match_history_title_canvas,
                                blank_canvas.clone().length(MATCH_HISTORY_TITLE_HEIGHT),
                            ),
                            (
                                ids.match_record.match_history_data_canvas,
                                blank_canvas.clone(),
                            ),
                        ]),
                    ),
                    (
                        ids.match_record.hero_selection.canvas,
                        blank_canvas.clone().color(color::BLACK),
                    ),
                ]),
            ),
            (
                ids.match_record.footer,
                blank_canvas
                    .clone()
                    .color(color::DARK_CHARCOAL)
                    .border(0.0)
                    .length(FOOTER_HEIGHT),
            ),
        ])
        .middle_of(ids.root.body)
        .wh_of(ids.root.body)
        .set(ids.match_record.root, ui);

    for player in Player::iter() {
        let pids = &ids.match_record.roster_players[player.index()];
        create_roster_hero(state, player, pids, updates, assets, ui);
    }

    let elements = Matrix::new(8, 4, &mut ids.match_record.hero_selection.dynamic)
        .middle_of(ids.match_record.hero_selection.canvas)
        .wh_of(ids.match_record.hero_selection.canvas)
        .set(ids.match_record.hero_selection.matrix, ui);

    let mut heros = Role::Tank
        .heros()
        .chain(Role::Dps.heros())
        .chain(Role::Support.heros());

    let dimm = elements
        .elem_h
        .min(elements.elem_w)
        .min(PORTRAIT_FULL_HEIGHT);
    for idy in 0..4 {
        for idx in 0..8 {
            let elem = elements.xy_get(idx, idy);
            if let Some(hero) = heros.next() {
                elem.set(
                    blank_canvas.clone().w_h(dimm, dimm),
                    elem.inner.portrait_canvas,
                    ui,
                );
                create_hero_selection_button(hero, state, assets, &elem.inner, updates, ui)
            }
        }
    }

    let sw = ui.w_of(ids.match_record.suggestion_canvas).unwrap();
    let suggestions = (sw / PORTRAIT_MINI_HEIGHT as f64) as usize;

    let elements = Matrix::new(suggestions, 3, &mut ids.match_record.suggestion_dynamic)
        .mid_left_of(ids.match_record.suggestion_canvas)
        .w(PORTRAIT_MINI_HEIGHT * suggestions as Scalar)
        .h_of(ids.match_record.suggestion_canvas)
        .set(ids.match_record.suggestion_matrix, ui);
    let mut heros = std::iter::repeat(Hero::Ana);
    let dimm = PORTRAIT_MINI_HEIGHT;
    for idy in 0..3 {
        for idx in 0..suggestions {
            let elem = elements.xy_get(idx, idy);
            if let Some(hero) = heros.next() {
                elem.set(
                    blank_canvas.clone().w_h(dimm, dimm),
                    elem.inner.portrait_canvas,
                    ui,
                );
                create_hero_selection_button(hero, state, assets, &elem.inner, updates, ui)
            }
        }
    }

    let outcome_color = |win: Option<bool>| match win {
        Some(true) => color::DARK_GREEN,
        Some(false) => color::LIGHT_RED,
        None => color::LIGHT_GRAY,
    };

    let outcome_label = |win: Option<bool>| match win {
        Some(true) => "victory",
        Some(false) => "defeat",
        None => "unknown",
    };

    // Match outcome
    for _ in widget::Toggle::new(false)
        .w_h(100.0, 40.0)
        .top_right_with_margins_on(ids.match_record.round_outcome_canvas, 20.0, 20.0)
        .label(outcome_label(state.get_win()))
        .color(outcome_color(state.get_win()))
        .set(ids.match_record.round_outcome_toggle, ui)
    {
        updates.push_front(UiEvent::RoundToggleOutcome)
    }

    if state.validate() {
        for _event in widget::Button::new()
            .label("Submit")
            .w_h(100.0, 40.0)
            .color(color::LIGHT_CHARCOAL)
            .bottom_right_with_margins_on(ids.match_record.round_outcome_canvas, 20.0, 20.0)
            .set(ids.match_record.round_outcome_submit, ui)
        {
            updates.push_front(UiEvent::RoundRecord)
        }
    }

    // Match History

    Text::new("match history")
        // style
        .font_size(ui.theme.font_size_medium)
        .center_justify()
        .w_of(ids.match_record.match_history_title_canvas)
        .middle_of(ids.match_record.match_history_title_canvas)
        .no_line_wrap()
        .set(ids.match_record.match_history_title_label, ui);

    // This should never be more than five, but in case we need to support ties, this is easy enough
    let rows = 5.max(state.match_len());

    let elements = Matrix::new(1, rows, &mut ids.match_record.match_history_dynamic)
        .middle_of(ids.match_record.match_history_data_canvas)
        .wh_of(ids.match_record.match_history_data_canvas)
        .set(ids.match_record.match_history_matrix, ui);

    let portrait_size = PORTRAIT_MINI_HEIGHT
        .min(elements.elem_h - (ROUND_RECORD_BORDER * 4.0))
        .min(round_record_width / (2.0 * 3.0));
    let heros_width = portrait_size * 3.0;
    for (ridx, round) in state.match_iter().enumerate() {
        let elem = elements.xy_get(0, ridx);
        elem.set(
            blank_canvas
                .clone()
                .border(ROUND_RECORD_BORDER)
                .w_h(elem.w, elem.h),
            elem.inner.canvas,
            ui,
        );

        blank_canvas
            .clone()
            .color(color::BLACK)
            .h(portrait_size)
            .w(heros_width)
            .mid_left_with_margin_on(elem.inner.canvas, 10.0)
            .set(elem.inner.heros_canvas, ui);

        for player in Player::iter() {
            let hero = round.get_hero(player);
            let img = assets.portraits[&hero];

            widget::Image::new(img)
                .h(portrait_size)
                .w(portrait_size)
                .x_relative_to(
                    elem.inner.heros_canvas,
                    portrait_size * ((player.index() as Scalar) - 1.0),
                )
                .set(elem.inner.heros[player.index()], ui);
        }

        let victory_bar = (round_record_width - heros_width) * 0.7;

        blank_canvas
            .clone()
            .color(outcome_color(Some(round.win)))
            .h(portrait_size * 0.7)
            .w(victory_bar)
            .mid_left_with_margin_on(elem.inner.canvas, heros_width + 20.0)
            .set(elem.inner.outcome_canvas, ui);

        Text::new(outcome_label(Some(round.win)))
            // style
            .font_size(ui.theme.font_size_medium)
            .center_justify()
            .w_of(elem.inner.outcome_canvas)
            .middle_of(elem.inner.outcome_canvas)
            .no_line_wrap()
            .set(elem.inner.outcome_label, ui);
    }
}

fn create_hero_selection_button(
    hero: Hero,
    state: &MatchState,
    assets: &AppAssets,
    ids: &HeroSelectPortrait,
    updates: &mut VecDeque<UiEvent>,
    ui: &mut UiCell,
) {
    let img = assets.portraits[&hero];
    let button = widget::Button::image(img)
        .middle_of(ids.portrait_canvas)
        .wh_of(ids.portrait_canvas);

    for _click in button.set(ids.portrait_image, ui) {
        updates.push_front(UiEvent::RoundSelectHero(hero))
    }

    if state.is_used(hero) {
        widget::Canvas::new()
            .color(color::Color::Rgba(0.3, 0.3, 0.3, 0.8))
            .middle_of(ids.portrait_canvas)
            .wh_of(ids.portrait_canvas)
            .set(ids.overlay, ui);
    }
}

fn create_roster_hero(
    state: &MatchState,
    player: Player,
    ids: &PlayerRosterIds,
    updates: &mut VecDeque<UiEvent>,
    assets: &AppAssets,
    ui: &mut conrod_core::UiCell,
) {
    if let Some(img) = state.get_hero(player).map(|h| assets.portraits[&h]) {
        let roster_h = ui.h_of(ids.canvas).unwrap();
        let portrait_buffer = (roster_h - PORTRAIT_FULL_HEIGHT - BATTLETAG_HEIGHT) / 2.0;
        widget::Canvas::new()
            .color(color::BLACK)
            .w_h(PORTRAIT_FULL_HEIGHT, PORTRAIT_FULL_HEIGHT)
            .mid_bottom_with_margin_on(ids.canvas, portrait_buffer + BATTLETAG_HEIGHT)
            .set(ids.portrait_canvas, ui);

        widget::Image::new(img)
            .wh_of(ids.portrait_canvas)
            .middle_of(ids.portrait_canvas)
            .set(ids.portrait_image, ui);
    }

    Text::new(state.get_battletag(player).as_str())
        // style
        .font_size(ui.theme.font_size_medium)
        .left_justify()
        .no_line_wrap()
        .mid_bottom_with_margin_on(ids.canvas, 10.0)
        .set(ids.battletag_label, ui);

    let clear = widget::Button::new()
        .color(color::TRANSPARENT)
        .wh_of(ids.canvas)
        .middle_of(ids.canvas);

    for _event in clear.set(ids.select_button, ui) {
        updates.push_front(UiEvent::RoundSelectPlayer(player))
    }
}
