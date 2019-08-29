use crate::layout::{DynamicIds, WidgetHolder, WidgetId};

use conrod_core::{Point, UiCell};

use conrod_core::widget;
use conrod_core::widget::Widget;
use conrod_core::Scalar;

#[derive(WidgetCommon)]
#[allow(missing_copy_implementations)]
pub struct Matrix<'a, T> {
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    data: &'a mut DynamicIds<T>,
    style: Style,
    cols: usize,
    rows: usize,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    #[conrod(default = "0.0")]
    pub cell_pad_w: Option<Scalar>,
    #[conrod(default = "0.0")]
    pub cell_pad_h: Option<Scalar>,
}

pub struct State {
    len: usize,
}

impl<'a, T> Matrix<'a, T> {
    /// Create a widget matrix context.
    pub fn new(cols: usize, rows: usize, data: &'a mut DynamicIds<T>) -> Self {
        Matrix {
            common: widget::CommonBuilder::default(),
            data: data,
            style: Style::default(),
            cols: cols,
            rows: rows,
        }
    }

    /// A builder method for adding padding to the cell.
    pub fn cell_padding(mut self, w: Scalar, h: Scalar) -> Self {
        self.style.cell_pad_w = Some(w);
        self.style.cell_pad_h = Some(h);
        self
    }
}

impl<'a, T: Send + WidgetHolder> Widget for Matrix<'a, T> {
    type State = State;
    type Style = Style;
    type Event = Elements<'a, T>;

    fn init_state(&self, _: widget::id::Generator) -> Self::State {
        State { len: 0 }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

    /// Update the state of the Matrix.
    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs {
            id,
            state,
            rect,
            style,
            ui,
            ..
        } = args;
        let Matrix { cols, rows, .. } = self;

        let sz = cols * rows;
        self.data.resize(sz, &mut ui.widget_id_generator());
        if state.len != sz {
            state.update(|s| s.len = sz)
        }

        let cell_pad_w = style.cell_pad_w(&ui.theme);
        let cell_pad_h = style.cell_pad_h(&ui.theme);
        let (w, h) = rect.w_h();
        let elem_w = w / cols as Scalar;
        let elem_h = h / rows as Scalar;
        let (half_w, half_h) = (w / 2.0, h / 2.0);
        let x_min = -half_w + elem_w / 2.0;
        let x_max = half_w + elem_w / 2.0;
        let y_min = -half_h - elem_h / 2.0;
        let y_max = half_h - elem_h / 2.0;

        let [m_x, m_y] = ui.xy_of(id).unwrap();

        Elements {
            data: self.data,
            num_rows: rows,
            num_cols: cols,
            row: 0,
            col: 0,
            matrix_id: id,
            elem_w,
            elem_h,
            x_min,
            x_max,
            y_min,
            y_max,
            m_x,
            m_y,
        }
    }
}

/// The event type yielded by the `Matrix`.
///
/// This can be used to iterate over each element in the `Matrix`.
#[derive(Debug)]
pub struct Elements<'a, T> {
    data: &'a DynamicIds<T>,
    num_rows: usize,
    num_cols: usize,
    row: usize,
    col: usize,
    matrix_id: widget::Id,
    pub elem_w: Scalar,
    pub elem_h: Scalar,
    x_min: Scalar,
    x_max: Scalar,
    y_min: Scalar,
    y_max: Scalar,
    m_x: Scalar,
    m_y: Scalar,
}

impl<'a, T: Copy> Elements<'a, T> {
    pub fn xy_get(&self, idx: usize, idy: usize) -> Element<T> {
        let rel_x = conrod::utils::map_range(
            idx as Scalar,
            0.0,
            self.num_cols as Scalar,
            self.x_min,
            self.x_max,
        );
        let rel_y = conrod::utils::map_range(
            idy as Scalar,
            0.0,
            self.num_rows as Scalar,
            self.y_max,
            self.y_min,
        );

        Element {
            inner: self.data.get(idy * self.num_cols + idx),
            matrix_id: self.matrix_id,
            col: idx,
            row: idy,
            w: self.elem_w,
            h: self.elem_h,
            rel_x: rel_x,
            rel_y: rel_y,
            m_x: self.m_x,
            m_y: self.m_y,
        }
    }
}

/// Data necessary for instantiating a widget for a single `Matrix` element.
#[derive(Copy, Clone, Debug)]
pub struct Element<T> {
    /// The id generated for the widget.
    pub inner: T,
    /// The row number for the `Element`.
    pub row: usize,
    /// The column number for the `Element`.
    pub col: usize,
    /// The width of the element.
    pub w: Scalar,
    /// The height of the element.
    pub h: Scalar,
    /// The *x* position of the element relative to the centre of the `Matrix`.
    pub rel_x: Scalar,
    /// The *y* position of the element relative to the centre of the `Matrix`.
    pub rel_y: Scalar,
    /// The id of the `Matrix`, used for positioning.
    pub m_x: Scalar,
    pub m_y: Scalar,
    matrix_id: widget::Id,
}

impl<T> Element<T> {
    /// Sets the given widget as the widget to use for the item.
    ///
    /// Sets the:
    /// - position of the widget.
    /// - dimensions of the widget.
    /// - parent of the widget.
    /// - and finally sets the widget within the `Ui`.
    pub fn set<W>(self, widget: W, widget_id: WidgetId, ui: &mut UiCell) -> W::Event
    where
        W: Widget,
    {
        use conrod_core::Positionable;
        let Element {
            matrix_id,
            w: _,
            h: _,
            rel_x,
            rel_y,
            ..
        } = self;
        widget
            //.w_h(w, h)
            .x_y_relative_to(matrix_id, rel_x, rel_y)
            .set(widget_id, ui)
    }
}
