use crate::workout_data::effort::Effort;
use iced::Color;

static COLOR_GRADIENT: [&str; 40] = [
    "#0c0af0", "#0d12e7", "#0e1adf", "#0f23d7", "#102bce", "#1134c6", "#123cbe", "#1344b6",
    "#154dad", "#1655a5", "#175e9d", "#186694", "#196e8c", "#1a7784", "#1b7f7c", "#1d8873",
    "#1e906b", "#1f9963", "#20a15b", "#21a952", "#22b24a", "#23ba42", "#25c339", "#26cb31",
    "#27d329", "#28dc21", "#29e418", "#2aed10", "#2bf508", "#2dfe00", "#41e400", "#56cb00",
    "#6bb100", "#809800", "#957f00", "#aa6500", "#bf4c00", "#d43200", "#e91900", "#fe0000",
];
static MAX_WATTAGE: f64 = 500.0;

fn color_from_hex(hex: &str) -> Color {
    let hex_values = &hex[1..].chars().collect::<Vec<char>>();
    let red = u8::from_str_radix(&hex_values[0..2].iter().collect::<String>(), 16)
        .ok()
        .unwrap() as f32;
    let green = u8::from_str_radix(&hex_values[2..4].iter().collect::<String>(), 16)
        .ok()
        .unwrap() as f32;
    let blue = u8::from_str_radix(&hex_values[4..6].iter().collect::<String>(), 16)
        .ok()
        .unwrap() as f32;

    Color {
        r: red / 255.0,
        g: green / 255.0,
        b: blue / 255.0,
        a: 1.0,
    }
}

impl Effort {
    fn average_wattage(&self) -> f64 {
        (self.starting_value + self.ending_value) / 2.0
    }

    pub fn to_color(&self) -> Color {
        let (color_before, color_after) = select_colors_from_gradients(self.average_wattage());
        interpolate_colors(color_before, color_after)
    }
}

fn interpolate_colors(first: Color, second: Color) -> Color {
    Color {
        r: (first.r + second.r) / 2.0,
        g: (first.g + second.g) / 2.0,
        b: (first.b + second.b) / 2.0,
        a: (first.a + second.a) / 2.0,
    }
}

fn select_color_string_from_gradients<'a>(wattage: f64) -> (&'a str, &'a str) {
    let percent_of_max_wattage = min(wattage / MAX_WATTAGE, 1.0);
    let max_index_color_gradient = (COLOR_GRADIENT.len() - 1) as f64;

    let color_before =
        COLOR_GRADIENT[(percent_of_max_wattage * max_index_color_gradient).floor() as usize];

    let color_after =
        COLOR_GRADIENT[(percent_of_max_wattage * max_index_color_gradient).ceil() as usize];

    (color_before, color_after)
}

fn select_colors_from_gradients(wattage: f64) -> (Color, Color) {
    let (color_before, color_after) = select_color_string_from_gradients(wattage);
    (color_from_hex(color_before), color_from_hex(color_after))
}

fn min(a: f64, b: f64) -> f64 {
    if a > b {
        b
    } else {
        a
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod color_from_hex {
        use super::*;

        #[test]
        fn red() {
            assert_eq!(color_from_hex("#ff0000"), Color::from_rgb8(255, 0, 0));
        }
        #[test]
        fn blue() {
            assert_eq!(color_from_hex("#2e00ff"), Color::from_rgb8(46, 0, 255));
        }
        #[test]
        fn greenish() {
            assert_eq!(color_from_hex("#0df114"), Color::from_rgb8(13, 241, 20));
        }
    }

    mod effort {
        use super::*;
        #[test]
        fn average_wattage() {
            assert_eq!(Effort::new(0.0, 100.0, Some(60.0)).average_wattage(), 80.0)
        }
    }

    #[test]
    fn minimum() {
        assert_eq!(min(1.0, 2.0), 1.0)
    }

    #[test]
    fn interpolate_between_red_and_green() {
        let red = Color {
            r: 255.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        };
        let green = Color {
            r: 0.0,
            g: 255.0,
            b: 0.0,
            a: 0.0,
        };

        assert_eq!(
            interpolate_colors(red, green),
            Color {
                r: 127.5,
                g: 127.5,
                b: 0.0,
                a: 0.0,
            }
        )
    }

    mod select_color {
        use super::*;
        #[test]
        fn select_first_color() {
            assert_eq!(
                select_color_string_from_gradients(0.0),
                ("#0c0af0", "#0c0af0")
            );
        }
        #[test]
        fn select_first_and_second() {
            assert_eq!(
                select_color_string_from_gradients(1.0),
                ("#0c0af0", "#0d12e7")
            );
        }
        #[test]
        fn select_last() {
            assert_eq!(
                select_color_string_from_gradients(500.0),
                ("#fe0000", "#fe0000")
            );
        }
        #[test]
        fn select_middle() {
            assert_eq!(
                select_color_string_from_gradients(201.0),
                ("#1d8873", "#1e906b")
            );
        }
    }
}
