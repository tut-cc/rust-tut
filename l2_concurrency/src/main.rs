use iced::widget::{button, column, image, row, text};
use iced::{Center, Element, Length, Task, Theme};

pub fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("Mandelbrot Set")
        .theme(|_state: &App| Theme::Dark)
        .run()
}

#[derive(Debug, Clone, Copy)]
struct Complex {
    re: f32, // 実部
    im: f32, // 虚部
}

impl Complex {
    fn norm_sqr(&self) -> f32 {
        self.re * self.re + self.im * self.im
    }
}

impl std::ops::Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let re = self.re + rhs.re;
        let im = self.im + rhs.im;
        Complex { re, im }
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        // (a + bi) (c + di) = (ac - bd) + (ad + bc)i
        let re = self.re * rhs.re - self.im * rhs.im;
        let im = self.re * rhs.im + self.im * rhs.re;
        Complex { re, im }
    }
}

const IMAGE_SIZE: usize = 500;

#[derive(Debug, Clone)]
enum Message {
    Generate,
    ImageGenerated(image::Handle),
    IncreaseIterations,
    DecreaseIterations,
}

struct App {
    image_handle: Option<image::Handle>,
    max_iter: usize,
    is_loading: bool,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let app = Self {
            image_handle: None,
            max_iter: 0,
            is_loading: true,
        };

        let task = Task::perform(generate_mandelbrot(app.max_iter), Message::ImageGenerated);

        (app, task)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Generate => {
                if self.is_loading {
                    Task::none()
                } else {
                    self.is_loading = true;
                    Task::perform(generate_mandelbrot(self.max_iter), Message::ImageGenerated)
                }
            }
            Message::ImageGenerated(handle) => {
                self.image_handle = Some(handle);
                self.is_loading = false;
                Task::none()
            }
            Message::IncreaseIterations => {
                self.max_iter = (self.max_iter + 5).min(100);
                Task::none()
            }
            Message::DecreaseIterations => {
                self.max_iter = self.max_iter.saturating_sub(5);
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let controls = row![
            button("クオリティを下げる").on_press(Message::DecreaseIterations),
            button("クオリティを上げる").on_press(Message::IncreaseIterations),
            button("生成").on_press_maybe((!self.is_loading).then_some(Message::Generate)),
        ]
        .spacing(10)
        .align_y(Center);

        let quality = text(format!("クオリティ: {} (0-100)", self.max_iter));

        let plot: Element<Message> = if let Some(handle) = &self.image_handle {
            image(handle)
                .width(Length::Fixed(IMAGE_SIZE as f32))
                .height(Length::Fixed(IMAGE_SIZE as f32))
                .into()
        } else {
            text("計算中...").size(30).into()
        };

        column![controls, quality, plot]
            .padding(20)
            .spacing(15)
            .into()
    }
}

async fn generate_mandelbrot(max_iter: usize) -> image::Handle {
    let mut pixels = vec![0u8; IMAGE_SIZE * IMAGE_SIZE * 4];

    let scale = 128.0;

    let x_min = -2.0 / scale;
    let x_max = 1.0 / scale;
    let y_min = -1.5 / scale;
    let y_max = 1.5 / scale;

    // scale = 1.0 => x/y_diff in [0, 0]
    // scale = 2.0 => x/y_diff in [-r/2, r/2]     [-1.5, 1.5]
    // scale = 4.0 => x/y_diff in [-3*r/4, 3*r/4] [-1.5-0.75, 1.5+0.75]
    // scale = 8.0 => x/y_diff in [-7*r/8, 7*r/8] [-1.5-0.75, 1.5+0.75]

    let x_diff = -0.50;
    let y_diff = -0.525;

    for y in 0..IMAGE_SIZE {
        for x in 0..IMAGE_SIZE {
            let cx = x_min + (x as f32 / IMAGE_SIZE as f32) * (x_max - x_min) + x_diff;
            let cy = y_min + (y as f32 / IMAGE_SIZE as f32) * (y_max - y_min) + y_diff;

            let c = Complex { re: cx, im: cy };
            let mut z = Complex { re: 0.0, im: 0.0 };
            let mut iter = 0;

            while z.norm_sqr() <= 4.0 && iter < max_iter {
                z = z * z + c;
                iter += 1;
            }

            // row-major index for the flat array
            let idx = (y * IMAGE_SIZE + x) * 4;

            if iter < max_iter {
                let brightness = iter as f32 / max_iter as f32;
                pixels[idx] = (brightness * 30.0) as u8; // red
                pixels[idx + 1] = (brightness * 140.0) as u8; // green
                pixels[idx + 2] = (brightness * 255.0) as u8; // blue
                pixels[idx + 3] = 255; // alpha
            } else {
                pixels[idx] = 0;
                pixels[idx + 1] = 0;
                pixels[idx + 2] = 0;
                pixels[idx + 3] = 255;
            }
        }
    }

    image::Handle::from_rgba(IMAGE_SIZE as u32, IMAGE_SIZE as u32, pixels)
}
