use crate::workout_data::workout;
use crate::{gui::crm_creator::WorkoutMessage, workout_data::effort};
use iced::{canvas, Color, Element, Length, Point, Rectangle, Size};
#[derive(Default)]
pub struct Visualizer {
    cache: canvas::Cache,
    workout: workout::Workout,
}

impl Visualizer {
    pub fn view(&'_ mut self, workout: workout::Workout) -> impl Into<Element<'_, WorkoutMessage>> {
        self.workout = workout;
        self.cache.clear();
        canvas::Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
    }
}

impl canvas::Program<WorkoutMessage> for Visualizer {
    fn draw(&self, bounds: Rectangle, _cursor: canvas::Cursor) -> Vec<canvas::Geometry> {
        let draw_all = self.cache.draw(bounds.size(), |frame| {
            let background = canvas::Path::rectangle(Point::ORIGIN, frame.size());
            frame.fill(&background, Color::from_rgb8(0x40, 0x44, 0x4B));

            for rectangle in &draw_efforts(&bounds, &self.workout.efforts) {
                let drawn_rectangle = rectangle.draw();
                frame.fill(&drawn_rectangle, Color::from_rgb8(255, 255, 255));
            }
        });

        vec![draw_all]
    }
}

fn draw_efforts(bounds: &Rectangle, efforts: &[effort::Effort]) -> Vec<RectangleToDraw> {
    let durations = efforts
        .iter()
        .map(|effort| effort.duration_in_minutes.to_float() as f32)
        .collect();

    let efforts = efforts
        .iter()
        .map(|effort| effort.starting_value.to_float() as f32)
        .collect();

    let offset_between_durations = 1.0;

    compute_starting_dimensions_x(bounds.size().width, durations, offset_between_durations)
        .into_iter()
        .zip(
            compute_starting_dimensions_y(bounds.size().height, efforts, offset_between_durations)
                .into_iter(),
        )
        .map(|(x_dimensions, y_dimensions)| {
            RectangleToDraw::new(x_dimensions, y_dimensions, bounds.size())
        })
        .collect::<Vec<RectangleToDraw>>()
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
    durations: &Vec<f32>,
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
) -> Vec<RectangleYDimensions> {
    let ratio_effort_to_frame = ((length_of_frame * 0.95) - offset_between_efforts)
        / efforts.iter().copied().fold(f32::NAN, f32::max);
    let heigths = efforts
        .iter()
        .map(|&current_effort| current_effort * ratio_effort_to_frame);

    let starting_points_y = vec![offset_between_efforts; efforts.len()].into_iter();

    starting_points_y
        .zip(heigths)
        .map(|(starting_point, height)| RectangleYDimensions::new(starting_point, height))
        .collect()
}

#[derive(Clone, Debug, Default, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq)]
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
    fn draw(&self) -> canvas::Path {
        canvas::Path::rectangle(self.top_left, self.size)
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
            compute_starting_dimensions_y(100.0, vec![100.0, 200.0, 250.0, 100.0], 0.1),
            vec![
                RectangleYDimensions::new(0.1, 37.960003),
                RectangleYDimensions::new(0.1, 75.920006),
                RectangleYDimensions::new(0.1, 94.9),
                RectangleYDimensions::new(0.1, 37.960003)
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
