use crate::gui::style;
use crate::workout_data::workout;
use crate::{gui::mrc_creator::WorkoutMessage, workout_data::effort};
use iced::widget::canvas;
use iced::widget::text::Shaping;
use iced::{Color, Element, Length, Point, Rectangle, Renderer, Size, Theme};
use std::cell::RefCell;
#[derive(Default)]
pub struct Visualizer {
    cache: canvas::Cache,
    workout: RefCell<workout::Workout>,
}

impl Visualizer {
    pub fn view(&self, workout: workout::Workout) -> impl Into<Element<'_, WorkoutMessage>> {
        self.overwrite_workout(workout);
        self.cache.clear();
        canvas::Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
    }
    fn overwrite_workout(&self, new_workout: workout::Workout) {
        let mut workout = self.workout.borrow_mut();
        *workout = new_workout;
    }
}

impl canvas::Program<WorkoutMessage> for &Visualizer {
    type State = ();
    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let draw_all = self.cache.draw(renderer, bounds.size(), |frame| {
            draw_backround(frame);
            draw_efforts(frame, bounds, &self.workout.borrow().efforts);
            draw_pink_border(frame);
            draw_summary_statistic(
                frame,
                &bounds,
                self.workout.borrow().average_intensity(),
                self.workout.borrow().total_time_of_workout(),
            )
        });

        vec![draw_all]
    }
}

fn draw_backround(frame: &mut canvas::Frame) {
    let background = canvas::Path::rectangle(Point::ORIGIN, frame.size());
    frame.fill(&background, Color::from_rgb8(0x40, 0x44, 0x4B));
}
fn draw_efforts(frame: &mut canvas::Frame, bounds: Rectangle, efforts: &[effort::Effort]) {
    for (shape, color) in compute_boxes_for_efforts(&bounds, efforts) {
        let drawn_shape = shape.draw();
        frame.fill(&drawn_shape, color);
    }
}
fn draw_pink_border(frame: &mut canvas::Frame) {
    frame.stroke(
        &canvas::Path::rectangle(Point::ORIGIN, frame.size()),
        canvas::Stroke::default()
            .with_color(style::PURPLE)
            .with_width(3.0),
    );
}
fn draw_summary_statistic(
    frame: &mut canvas::Frame,
    bounds: &'_ Rectangle,
    average_intensity: f64,
    duration_in_minutes: f64,
) {
    let text_size_with_buffer = style::TEXT_SIZE * 1.25;
    let offset_from_left: f32 = bounds.width * 0.85;

    frame.fill_text(pink_text(
        format!("Average Wattage: {:.1}", average_intensity),
        Point {
            x: offset_from_left,
            y: text_size_with_buffer,
        },
    ));
    frame.fill_text(pink_text(
        format!("Duration: {} ", duration_in_minutes),
        Point {
            x: offset_from_left,
            y: 2.0 * text_size_with_buffer,
        },
    ));
}

fn pink_text(text: String, position: iced::Point) -> canvas::Text {
    canvas::Text {
        content: text,
        position,
        color: style::PINK,
        size: iced::Pixels(style::TEXT_SIZE),
        font: iced::Font::default(),
        line_height: iced::widget::text::LineHeight::default(),
        shaping: Shaping::Basic,
        horizontal_alignment: iced::alignment::Horizontal::Center,
        vertical_alignment: iced::alignment::Vertical::Center,
    }
}

fn compute_boxes_for_efforts(
    bounds: &'_ Rectangle,
    efforts: &[effort::Effort],
) -> Vec<(Box<dyn Drawable>, Color)> {
    compute_shapes_to_draw(bounds, efforts)
        .into_iter()
        .zip(duplicate_element_in_iterator(
            &mut compute_colors_of_shapes(efforts).into_iter(),
        ))
        .collect()
}
fn compute_colors_of_shapes(efforts: &[effort::Effort]) -> Vec<Color> {
    efforts.iter().map(|effort| effort.to_color()).collect()
}

fn duplicate_element_in_iterator<T>(iterator: &mut dyn Iterator<Item = T>) -> Vec<T>
where
    T: Clone,
{
    iterator
        .flat_map(|element| vec![element.clone(), element].into_iter())
        .collect()
}

fn compute_shapes_to_draw(
    bounds: &'_ Rectangle,
    efforts: &[effort::Effort],
) -> Vec<Box<dyn Drawable>> {
    let durations = efforts
        .iter()
        .map(|effort| effort.duration_in_minutes as f32)
        .collect();
    let starting_values: Vec<f32> = efforts
        .iter()
        .map(|effort| effort.starting_value as f32)
        .collect();
    let ending_values: Vec<f32> = efforts
        .iter()
        .map(|effort| effort.ending_value as f32)
        .collect();

    let offset_between_durations = 1.0;

    compute_starting_dimensions_x(bounds.size().width, durations, offset_between_durations)
        .into_iter()
        .zip(
            compute_starting_dimensions_y(
                bounds.size().height,
                starting_values.clone(),
                offset_between_durations,
                starting_values.iter().chain(ending_values.iter()).copied().fold(f32::NAN, f32::max)
            ),
        )
        .zip(
            compute_starting_dimensions_y(
                bounds.size().height,
                ending_values.clone(),
                offset_between_durations,
                starting_values.iter().chain(ending_values.iter()).copied().fold(f32::NAN, f32::max)
            ),
        )
        .flat_map(
            |((x_dimensions, y_dimensions_starting), y_dimensions_ending)| -> Vec<Box<dyn Drawable+'static>>{
                vec![
                    Box::new(RectangleToDraw::new(
                        x_dimensions,
                        if y_dimensions_ending.height > y_dimensions_starting.height {y_dimensions_starting} else {y_dimensions_ending},
                        bounds.size(),
                    )),
                    Box::new(TriangleToDraw::new(
                        x_dimensions,
                        y_dimensions_starting,
                        y_dimensions_ending,
                        bounds.size(),
                    )),
                ]
            },
        )
        .collect::<Vec<Box<dyn Drawable>>>()
}

fn compute_starting_dimensions_x(
    length_of_frame: f32,
    durations: Vec<f32>,
    offset_between_durations: f32,
) -> Vec<RectangleXDimensions> {
    let ratio_duration_to_frame: f32 =
        compute_ratio_of_duration_to_frame(length_of_frame, offset_between_durations, &durations);

    let widths = durations
        .iter()
        .map(|&current_duration| current_duration * ratio_duration_to_frame);

    compute_starting_points_of_efforts(
        &offset_between_durations,
        &ratio_duration_to_frame,
        &durations,
    )
    .zip(widths)
    .map(|(starting_point, width)| RectangleXDimensions::new(starting_point, width))
    .collect()
}

fn compute_ratio_of_duration_to_frame(
    length_of_frame: f32,
    offset_between_durations: f32,
    durations: &[f32],
) -> f32 {
    (length_of_frame - (offset_between_durations * durations.len() as f32))
        / durations.iter().sum::<f32>()
}

fn compute_starting_points_of_efforts<'a>(
    offset_between_durations: &'a f32,
    ratio_duration_to_frame: &'a f32,
    durations: &'a [f32],
) -> impl Iterator<Item = f32> + 'a {
    std::iter::once(&0.0_f32).chain(durations.iter()).scan(
        0.0_f32,
        move |last_starting_point, &current_duration| {
            *last_starting_point = *last_starting_point
                + offset_between_durations
                + current_duration * ratio_duration_to_frame;
            Some(*last_starting_point)
        },
    )
}

fn compute_starting_dimensions_y(
    length_of_frame: f32,
    efforts: Vec<f32>,
    offset_between_efforts: f32,
    max: f32,
) -> Vec<RectangleYDimensions> {
    let ratio_effort_to_frame = ((length_of_frame * 0.90) - offset_between_efforts) / max;
    let heigths = efforts
        .iter()
        .map(|&current_effort| current_effort * ratio_effort_to_frame);

    let starting_points_y = vec![offset_between_efforts; efforts.len()].into_iter();

    starting_points_y
        .zip(heigths)
        .map(|(starting_point, height)| RectangleYDimensions::new(starting_point, height))
        .collect()
}

#[derive(Clone, Debug, Default, PartialEq, Copy)]
struct RectangleXDimensions {
    starting_point: f32,
    width: f32,
}

impl RectangleXDimensions {
    fn new(starting_point: f32, width: f32) -> Self {
        Self {
            starting_point,
            width,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Copy)]
struct RectangleYDimensions {
    starting_point: f32,
    height: f32,
}

impl RectangleYDimensions {
    fn new(starting_point: f32, height: f32) -> Self {
        Self {
            starting_point,
            height,
        }
    }
}

trait Drawable {
    fn draw(&self) -> canvas::Path;
}
#[derive(Clone, Debug, PartialEq)]
struct RectangleToDraw {
    top_left: Point,
    size: Size,
}

impl RectangleToDraw {
    fn new(
        dimensions_x: RectangleXDimensions,
        dimensions_y: RectangleYDimensions,
        size_of_frame: Size,
    ) -> Self {
        Self {
            top_left: Point::new(
                dimensions_x.starting_point,
                size_of_frame.height - dimensions_y.height - dimensions_y.starting_point,
            ),
            size: Size::new(dimensions_x.width, dimensions_y.height),
        }
    }
}

impl Drawable for RectangleToDraw {
    fn draw(&self) -> canvas::Path {
        canvas::Path::rectangle(self.top_left, self.size)
    }
}

struct TriangleToDraw {
    point_1: Point,
    point_2: Point,
    point_3: Point,
}

impl TriangleToDraw {
    fn new(
        x_dimensions: RectangleXDimensions,
        y_dimensions_starting: RectangleYDimensions,
        y_dimensions_ending: RectangleYDimensions,
        frame: Size,
    ) -> Self {
        if y_dimensions_starting.height > y_dimensions_ending.height {
            Self {
                point_1: Point::new(
                    x_dimensions.starting_point,
                    mirror_y(
                        y_dimensions_ending.starting_point + y_dimensions_ending.height,
                        frame,
                    ),
                ),
                point_2: Point::new(
                    x_dimensions.starting_point,
                    mirror_y(
                        y_dimensions_starting.starting_point + y_dimensions_starting.height,
                        frame,
                    ),
                ),

                point_3: Point::new(
                    x_dimensions.starting_point + x_dimensions.width,
                    mirror_y(
                        y_dimensions_ending.starting_point + y_dimensions_ending.height,
                        frame,
                    ),
                ),
            }
        } else {
            Self {
                point_1: Point::new(
                    x_dimensions.starting_point,
                    mirror_y(
                        y_dimensions_starting.starting_point + y_dimensions_starting.height,
                        frame,
                    ),
                ),
                point_2: Point::new(
                    x_dimensions.starting_point + x_dimensions.width,
                    mirror_y(
                        y_dimensions_starting.starting_point + y_dimensions_starting.height,
                        frame,
                    ),
                ),
                point_3: Point::new(
                    x_dimensions.starting_point + x_dimensions.width,
                    mirror_y(
                        y_dimensions_ending.starting_point + y_dimensions_ending.height,
                        frame,
                    ),
                ),
            }
        }
    }
}

fn mirror_y(point: f32, frame: Size) -> f32 {
    frame.height - point
}

impl Drawable for TriangleToDraw {
    fn draw(&self) -> canvas::Path {
        canvas::Path::new(|p| {
            p.move_to(self.point_1);
            p.line_to(self.point_2);
            p.line_to(self.point_3);
            p.line_to(self.point_1);
            p.close();
        })
    }
}

#[cfg(test)]
mod test {
    use super::{
        compute_ratio_of_duration_to_frame, compute_starting_dimensions_x,
        compute_starting_dimensions_y, compute_starting_points_of_efforts, RectangleToDraw,
        RectangleXDimensions, RectangleYDimensions,
    };
    use iced::{Point, Size};

    #[test]
    fn test_get_starting_coordinates_x() {
        assert_eq!(
            compute_starting_dimensions_x(100.0, vec![10.0, 20.0, 40.0, 10.0], 0.1),
            vec![
                RectangleXDimensions::new(0.1, 12.45),
                RectangleXDimensions::new(12.65, 24.9),
                RectangleXDimensions::new(37.65, 49.8),
                RectangleXDimensions::new(87.55, 12.45)
            ]
        )
    }
    #[test]
    fn test_compute_ratio_of_duration_to_frame() {
        assert_eq!(
            compute_ratio_of_duration_to_frame(100.0, 0.1, &vec![10.0, 20.0, 40.0, 10.0]),
            1.245
        )
    }
    #[test]
    fn test_compute_starting_points_of_efforts() {
        assert_eq!(
            compute_starting_points_of_efforts(&0.1, &1.245, &[10.0, 20.0, 40.0, 10.0])
                .collect::<Vec<f32>>(),
            vec![0.1, 12.65, 37.65, 87.55, 100.1]
        )
    }
    #[test]
    fn test_get_starting_coordinates_y() {
        assert_eq!(
            compute_starting_dimensions_y(278.888_9, vec![100.0, 200.0, 250.0, 100.0], 1.0, 250.0),
            // TODO: Make comparison round.
            vec![
                RectangleYDimensions::new(1.0, 99.99999),
                RectangleYDimensions::new(1.0, 199.99998),
                RectangleYDimensions::new(1.0, 249.99998),
                RectangleYDimensions::new(1.0, 99.99999)
            ]
        )
    }

    #[test]
    fn test_rectangle_to_draw() {
        assert_eq!(
            RectangleToDraw::new(
                RectangleXDimensions {
                    starting_point: 0.1,
                    width: 10.0
                },
                RectangleYDimensions {
                    starting_point: 0.1,
                    height: 30.0
                },
                Size::new(100.0, 300.0)
            ),
            RectangleToDraw {
                top_left: Point::new(0.1, 269.9),
                size: Size::new(10.0, 30.0)
            }
        )
    }
}
