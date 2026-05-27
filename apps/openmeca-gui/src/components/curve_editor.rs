use iced::{
    Color, Element, Length, Rectangle, Renderer, mouse,
    widget::canvas::{self, Frame, Path, Stroke},
};
use meca_hid::curve::{Curve, MAX_VALUE};

use crate::Message;

pub struct CurveEditor<'a> {
    pub curve: &'a Curve,
    pub raw: u16,
    pub accent: Color,
}

#[derive(Default)]
pub struct CurveEditorState {
    drag: Option<usize>,
}

// Returns (padding, draw_w, draw_h) from canvas bounds
fn layout(bounds: Rectangle) -> (f32, f32, f32) {
    let padding = 0.0;
    (
        padding,
        bounds.width - padding * 2.0,
        bounds.height - padding * 2.0,
    )
}

// Map a u16 curve value to canvas y-axis
fn val_to_y(val: u16, padding: f32, h: f32) -> f32 {
    padding + h - (val as f32 / MAX_VALUE as f32) * h
}

// Map canvas y → u16 value (clamped)
fn y_to_val(y: f32, padding: f32, h: f32) -> u16 {
    let frac = 1.0 - (y - padding) / h;
    (frac.clamp(0.0, 1.0) * MAX_VALUE as f32).round() as u16
}

// Map a u16 x position (0-4096) -> canvas x-axis
fn xval_to_x(val: u16, padding: f32, w: f32) -> f32 {
    padding + (val as f32 / MAX_VALUE as f32) * w
}

impl<'a> canvas::Program<Message> for CurveEditor<'a> {
    type State = CurveEditorState;

    fn update(
        &self,
        state: &mut Self::State,
        event: &iced::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        let (padding, w, h) = layout(bounds);
        let points = self.curve.points();

        let ctrl_canvas: Vec<(f32, f32)> = (2..=6)
            .map(|i| {
                (
                    xval_to_x(points[i].x, padding, w),
                    val_to_y(points[i].y, padding, h),
                )
            })
            .collect();

        match event {
            canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(pos) = cursor.position_in(bounds) {
                    // efind nearest control point within 16px
                    let nearest = ctrl_canvas
                        .iter()
                        .enumerate()
                        .filter_map(|(i, &(cx, cy))| {
                            let d = ((pos.x - cx).powi(2) + (pos.y - cy).powi(2)).sqrt();
                            if d < 16.0 { Some((i, d)) } else { None }
                        })
                        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

                    if let Some((i, _)) = nearest {
                        state.drag = Some(i);
                        return Some(canvas::Action::capture());
                    }
                }

                None
            }

            canvas::Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if let Some(index) = state.drag {
                    if let Some(pos) = cursor.position_in(bounds) {
                        let new_y = y_to_val(pos.y, padding, h);
                        return Some(
                            canvas::Action::publish(Message::SetCurvePoint(index, new_y))
                                .and_capture(),
                        );
                    }
                }

                None
            }

            canvas::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                if state.drag.take().is_some() {
                    return Some(canvas::Action::capture());
                }

                None
            }

            _ => None,
        }
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let (padding, w, h) = layout(bounds);
        let points = self.curve.points();

        // Background
        frame.fill_rectangle(
            iced::Point::new(padding, padding),
            iced::Size::new(w, h),
            Color::from_rgba(0.0, 0.0, 0.0, 0.5),
        );

        // Grid lines
        for i in 1..=5 {
            let frac: f32 = (i as f32) / 6.0;
            let grid_y = padding + h * (1.0 - frac);
            let grid_x = padding + w * frac;

            let horizontal_line = Path::line(
                iced::Point::new(padding, grid_y),
                iced::Point::new(padding + w, grid_y),
            );

            frame.stroke(
                &horizontal_line,
                Stroke::default()
                    .with_color(Color::from_rgba(1.0, 1.0, 1.0, 0.06))
                    .with_width(1.0),
            );

            let vertical_line = Path::line(
                iced::Point::new(grid_x, padding),
                iced::Point::new(grid_x, padding + h),
            );

            frame.stroke(
                &vertical_line,
                Stroke::default()
                    .with_color(Color::from_rgba(1.0, 1.0, 1.0, 0.06))
                    .with_width(1.0),
            );
        }

        // Linear curve for reference
        let diagonal = Path::line(
            iced::Point::new(padding, padding + h),
            iced::Point::new(padding + w, padding),
        );

        frame.stroke(
            &diagonal,
            Stroke::default()
                .with_color(Color::from_rgba(1.0, 1.0, 1.0, 0.10))
                .with_width(1.0),
        );

        // Deadzone shading
        let dz_bottom = self.curve.bottom_dz_percentage() as f32 / 100.0;
        let dz_top = self.curve.top_dz_percentage() as f32 / 100.0;

        if dz_bottom > 0.0 {
            let dz_w = w * dz_bottom;

            frame.fill_rectangle(
                iced::Point::new(padding, padding),
                iced::Size::new(dz_w, h),
                Color::from_rgba(1.0, 0.3, 0.3, 0.05),
            );
        }

        if dz_top < 1.0 {
            let dz_x = padding + w * dz_top;

            frame.fill_rectangle(
                iced::Point::new(dz_x, padding),
                iced::Size::new(w - (dz_x - padding), h),
                Color::from_rgba(1.0, 0.3, 0.3, 0.05),
            );
        }

        // Curve path
        let curve_path = Path::new(|b| {
            let start = iced::Point::new(
                xval_to_x(points[0].x, padding, w),
                val_to_y(points[0].y, padding, h),
            );

            b.move_to(start);

            for i in 1..points.len() {
                let point = iced::Point::new(
                    xval_to_x(points[i].x, padding, w),
                    val_to_y(points[i].y, padding, h),
                );

                b.line_to(point);
            }
        });

        frame.stroke(
            &curve_path,
            Stroke::default().with_color(self.accent).with_width(2.0),
        );

        // Configurable points
        let cursor_pos = cursor.position_in(bounds);
        let ctrl_canvas: Vec<(f32, f32, usize)> = (2..=6)
            .map(|i| {
                (
                    xval_to_x(points[i].x, padding, w),
                    val_to_y(points[i].y, padding, h),
                    i - 2,
                )
            })
            .collect();

        for (cx, cy, i) in &ctrl_canvas {
            let hovered = cursor_pos.map_or(false, |p| {
                ((p.x - cx).powi(2) + (p.y - cy).powi(2)).sqrt() < 16.0
            });

            let dragging = state.drag == Some(*i);
            let radius = if dragging {
                7.0
            } else if hovered {
                6.0
            } else {
                5.0
            };

            let circle = Path::circle(iced::Point::new(*cx, *cy), radius);

            frame.fill(&circle, self.accent);

            // White when hovered or dragging
            if hovered || dragging {
                frame.stroke(
                    &circle,
                    Stroke::default().with_color(Color::WHITE).with_width(1.5),
                );
            }
        }

        // Live input
        let raw_frac = self.raw as f32 / MAX_VALUE as f32;
        let cursor_x = padding + raw_frac * w;
        let interpolated_y = val_to_y(self.curve.interpolate(self.raw), padding, h);

        let vertical_line = Path::line(
            iced::Point::new(cursor_x, padding),
            iced::Point::new(cursor_x, padding + h),
        );

        frame.stroke(
            &vertical_line,
            Stroke::default()
                .with_color(Color::from_rgba(1.0, 1.0, 1.0, 0.25))
                .with_width(1.0),
        );

        let dot = Path::circle(iced::Point::new(cursor_x, interpolated_y), 4.0);
        frame.fill(&dot, Color::WHITE);

        vec![frame.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if state.drag.is_some() {
            return mouse::Interaction::Grabbing;
        }

        if let Some(pos) = cursor.position_in(bounds) {
            let (padding, w, h) = layout(bounds);
            let points = self.curve.points();

            let near = (2..=6).any(|i| {
                let cx = xval_to_x(points[i].x, padding, w);
                let cy = val_to_y(points[i].y, padding, h);
                ((pos.x - cx).powi(2) + (pos.y - cy).powi(2)).sqrt() < 16.0
            });

            if near {
                return mouse::Interaction::Grab;
            }
        }

        mouse::Interaction::default()
    }
}

pub fn curve_editor<'a>(
    curve: &'a Curve,
    raw: u16,
    accent: Color,
    height: f32,
) -> Element<'a, Message> {
    canvas::Canvas::new(CurveEditor { curve, raw, accent })
        .width(Length::Fill)
        .height(height)
        .into()
}
