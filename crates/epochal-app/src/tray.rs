use chrono::{Datelike, Local};
use image::{ImageBuffer, Rgb, RgbImage};

const ICON_SIZE: u32 = 48;

#[derive(Clone, Debug)]
pub enum TrayAction {
    ShowWindow,
    Quit,
}

#[cfg(target_os = "linux")]
mod platform {
    use super::TrayAction;
    use ksni::{self, MenuItem};
    use std::thread;

    pub struct TrayManager {
        _join: thread::JoinHandle<()>,
        _handle: ksni::Handle<EpochalTray>,
    }

    struct EpochalTray {
        icon: ksni::Icon,
        sender: std::sync::mpsc::Sender<TrayAction>,
    }

    impl ksni::Tray for EpochalTray {
        fn activate(&mut self, _x: i32, _y: i32) {
            let _ = self.sender.send(TrayAction::ShowWindow);
        }

        fn title(&self) -> String {
            "Epochal".into()
        }

        fn tool_tip(&self) -> ksni::ToolTip {
            ksni::ToolTip {
                title: "Epochal".into(),
                description: "Epochal - Calendar Application".into(),
                ..Default::default()
            }
        }

        fn icon_pixmap(&self) -> Vec<ksni::Icon> {
            vec![self.icon.clone()]
        }

        fn menu(&self) -> Vec<MenuItem<Self>> {
            let open = ksni::menu::StandardItem {
                label: "Open Epochal".into(),
                activate: Box::new(|this: &mut EpochalTray| {
                    let _ = this.sender.send(TrayAction::ShowWindow);
                }),
                ..Default::default()
            };

            let quit = ksni::menu::StandardItem {
                label: "Quit".into(),
                activate: Box::new(|this: &mut EpochalTray| {
                    let _ = this.sender.send(TrayAction::Quit);
                }),
                ..Default::default()
            };

            vec![open.into(), quit.into()]
        }
    }

    impl TrayManager {
        pub fn new(
            sender: std::sync::mpsc::Sender<TrayAction>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            let icon = super::generate_calendar_icon_argb32();
            let service = ksni::TrayService::new(EpochalTray { icon, sender });
            let handle = service.handle();
            let join = thread::spawn(move || {
                // Blocks until the service exits
                let _ = service.run();
            });
            Ok(Self {
                _join: join,
                _handle: handle,
            })
        }
    }
}

#[cfg(not(target_os = "linux"))]
mod platform {
    use super::TrayAction;
    use std::thread;
    use tray_icon::menu::{Menu, MenuEvent, MenuItem};
    use tray_icon::{Icon, TrayIcon, TrayIconBuilder};

    pub struct TrayManager {
        _tray_icon: TrayIcon,
    }

    impl TrayManager {
        pub fn new(
            sender: std::sync::mpsc::Sender<TrayAction>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            let icon = super::generate_calendar_icon_rgba()?;

            let menu = Menu::new();
            let show_item = MenuItem::new("Open Epochal", true, None);
            let quit_item = MenuItem::new("Quit", true, None);
            menu.append(&show_item)?;
            menu.append(&quit_item)?;

            let show_id = show_item.id().clone();
            let quit_id = quit_item.id().clone();

            // spawn menu event loop to forward events to GTK main thread
            let rx = MenuEvent::receiver();
            thread::spawn(move || {
                while let Ok(event) = rx.recv() {
                    if event.id == show_id {
                        let _ = sender.send(TrayAction::ShowWindow);
                    } else if event.id == quit_id {
                        let _ = sender.send(TrayAction::Quit);
                    }
                }
            });

            let tray_icon = TrayIconBuilder::new()
                .with_title("Epochal")
                .with_tooltip("Epochal - Calendar Application")
                .with_icon(icon)
                .with_menu(Box::new(menu))
                .build()?;

            Ok(Self {
                _tray_icon: tray_icon,
            })
        }
    }
}

pub use platform::TrayManager;

fn generate_calendar_canvas() -> RgbImage {
    let mut img: RgbImage = ImageBuffer::new(ICON_SIZE, ICON_SIZE);

    // Fill background with white
    for pixel in img.pixels_mut() {
        *pixel = Rgb([255, 255, 255]);
    }

    // Draw calendar border (simple black rectangle)
    draw_border(&mut img);

    // Draw calendar header (red bar at top)
    draw_header(&mut img);

    // Draw the day number
    let now = Local::now();
    let day = now.day();
    draw_day_number(&mut img, day);

    img
}

#[cfg(target_os = "linux")]
fn generate_calendar_icon_argb32() -> ksni::Icon {
    let img = generate_calendar_canvas();
    // Convert RGB to ARGB32 with opaque alpha for SNI pixmap
    let mut data = Vec::with_capacity((ICON_SIZE * ICON_SIZE * 4) as usize);
    for p in img.pixels() {
        let [r, g, b] = p.0;
        data.extend_from_slice(&[255, r, g, b]); // ARGB32
    }
    ksni::Icon {
        width: ICON_SIZE as i32,
        height: ICON_SIZE as i32,
        data,
    }
}

#[cfg(not(target_os = "linux"))]
fn generate_calendar_icon_rgba() -> Result<tray_icon::Icon, Box<dyn std::error::Error>> {
    let img = generate_calendar_canvas();
    let icon_data = img.into_raw();
    let icon = tray_icon::Icon::from_rgba(icon_data, ICON_SIZE, ICON_SIZE)?;
    Ok(icon)
}

fn draw_border(img: &mut RgbImage) {
    let black = Rgb([0, 0, 0]);

    // Top and bottom borders
    for x in 0..ICON_SIZE {
        img.put_pixel(x, 0, black);
        img.put_pixel(x, ICON_SIZE - 1, black);
    }

    // Left and right borders
    for y in 0..ICON_SIZE {
        img.put_pixel(0, y, black);
        img.put_pixel(ICON_SIZE - 1, y, black);
    }
}

fn draw_header(img: &mut RgbImage) {
    let red = Rgb([220, 20, 20]);
    let header_height = ICON_SIZE / 6;

    for y in 1..header_height {
        for x in 1..(ICON_SIZE - 1) {
            img.put_pixel(x, y, red);
        }
    }
}

fn draw_day_number(img: &mut RgbImage, day: u32) {
    let black = Rgb([0, 0, 0]);
    let center_x = ICON_SIZE / 2;
    let center_y = ICON_SIZE / 2 + 4; // Slightly below center

    match day {
        1 => draw_digit_1(img, center_x, center_y, black),
        2 => draw_digit_2(img, center_x, center_y, black),
        3 => draw_digit_3(img, center_x, center_y, black),
        4 => draw_digit_4(img, center_x, center_y, black),
        5 => draw_digit_5(img, center_x, center_y, black),
        6 => draw_digit_6(img, center_x, center_y, black),
        7 => draw_digit_7(img, center_x, center_y, black),
        8 => draw_digit_8(img, center_x, center_y, black),
        9 => draw_digit_9(img, center_x, center_y, black),
        10..=19 => {
            draw_digit_1(img, center_x - 6, center_y, black);
            draw_digit_by_value(img, center_x + 6, center_y, black, day % 10);
        }
        20..=29 => {
            draw_digit_2(img, center_x - 6, center_y, black);
            draw_digit_by_value(img, center_x + 6, center_y, black, day % 10);
        }
        30..=31 => {
            draw_digit_3(img, center_x - 6, center_y, black);
            draw_digit_by_value(img, center_x + 6, center_y, black, day % 10);
        }
        _ => {
            draw_digit_by_value(img, center_x, center_y, black, day % 10);
        }
    }
}

fn draw_digit_by_value(img: &mut RgbImage, x: u32, y: u32, color: Rgb<u8>, digit: u32) {
    match digit {
        0 => draw_digit_0(img, x, y, color),
        1 => draw_digit_1(img, x, y, color),
        2 => draw_digit_2(img, x, y, color),
        3 => draw_digit_3(img, x, y, color),
        4 => draw_digit_4(img, x, y, color),
        5 => draw_digit_5(img, x, y, color),
        6 => draw_digit_6(img, x, y, color),
        7 => draw_digit_7(img, x, y, color),
        8 => draw_digit_8(img, x, y, color),
        9 => draw_digit_9(img, x, y, color),
        _ => {}
    }
}

// Simple 5x7 pixel font for digits
fn draw_digit_0(img: &mut RgbImage, cx: u32, cy: u32, color: Rgb<u8>) {
    let pattern = [
        [0, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [0, 1, 1, 1, 0],
    ];
    draw_pattern(img, cx, cy, &pattern, color);
}

fn draw_digit_1(img: &mut RgbImage, cx: u32, cy: u32, color: Rgb<u8>) {
    let pattern = [
        [0, 0, 1, 0, 0],
        [0, 1, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 1, 1, 1, 0],
    ];
    draw_pattern(img, cx, cy, &pattern, color);
}

fn draw_digit_2(img: &mut RgbImage, cx: u32, cy: u32, color: Rgb<u8>) {
    let pattern = [
        [0, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
        [0, 0, 0, 0, 1],
        [0, 0, 0, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 1, 0, 0, 0],
        [1, 1, 1, 1, 1],
    ];
    draw_pattern(img, cx, cy, &pattern, color);
}

fn draw_digit_3(img: &mut RgbImage, cx: u32, cy: u32, color: Rgb<u8>) {
    let pattern = [
        [0, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
        [0, 0, 0, 0, 1],
        [0, 0, 1, 1, 0],
        [0, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [0, 1, 1, 1, 0],
    ];
    draw_pattern(img, cx, cy, &pattern, color);
}

fn draw_digit_4(img: &mut RgbImage, cx: u32, cy: u32, color: Rgb<u8>) {
    let pattern = [
        [0, 0, 0, 1, 0],
        [0, 0, 1, 1, 0],
        [0, 1, 0, 1, 0],
        [1, 0, 0, 1, 0],
        [1, 1, 1, 1, 1],
        [0, 0, 0, 1, 0],
        [0, 0, 0, 1, 0],
    ];
    draw_pattern(img, cx, cy, &pattern, color);
}

fn draw_digit_5(img: &mut RgbImage, cx: u32, cy: u32, color: Rgb<u8>) {
    let pattern = [
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0],
        [1, 0, 0, 0, 0],
        [1, 1, 1, 1, 0],
        [0, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [0, 1, 1, 1, 0],
    ];
    draw_pattern(img, cx, cy, &pattern, color);
}

fn draw_digit_6(img: &mut RgbImage, cx: u32, cy: u32, color: Rgb<u8>) {
    let pattern = [
        [0, 0, 1, 1, 0],
        [0, 1, 0, 0, 0],
        [1, 0, 0, 0, 0],
        [1, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [0, 1, 1, 1, 0],
    ];
    draw_pattern(img, cx, cy, &pattern, color);
}

fn draw_digit_7(img: &mut RgbImage, cx: u32, cy: u32, color: Rgb<u8>) {
    let pattern = [
        [1, 1, 1, 1, 1],
        [0, 0, 0, 0, 1],
        [0, 0, 0, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 1, 0, 0, 0],
        [0, 1, 0, 0, 0],
        [0, 1, 0, 0, 0],
    ];
    draw_pattern(img, cx, cy, &pattern, color);
}

fn draw_digit_8(img: &mut RgbImage, cx: u32, cy: u32, color: Rgb<u8>) {
    let pattern = [
        [0, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [0, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [0, 1, 1, 1, 0],
    ];
    draw_pattern(img, cx, cy, &pattern, color);
}

fn draw_digit_9(img: &mut RgbImage, cx: u32, cy: u32, color: Rgb<u8>) {
    let pattern = [
        [0, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [0, 1, 1, 1, 1],
        [0, 0, 0, 0, 1],
        [0, 0, 0, 1, 0],
        [0, 1, 1, 0, 0],
    ];
    draw_pattern(img, cx, cy, &pattern, color);
}

fn draw_pattern(img: &mut RgbImage, cx: u32, cy: u32, pattern: &[[u8; 5]; 7], color: Rgb<u8>) {
    let start_x = cx.saturating_sub(2);
    let start_y = cy.saturating_sub(3);

    for (row, line) in pattern.iter().enumerate() {
        for (col, &pixel) in line.iter().enumerate() {
            if pixel == 1 {
                let x = start_x + col as u32;
                let y = start_y + row as u32;
                if x < ICON_SIZE && y < ICON_SIZE {
                    img.put_pixel(x, y, color);
                }
            }
        }
    }
}
