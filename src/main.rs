// Вариант приложения Adhan на Iced GUI

use iced::{
    Alignment, Application, Command, Settings, window, Length, Font, font::{Family, Weight},
};
use iced::widget::{text, Column, Container, Row, container};
use std::borrow::Cow;
use std::time::Duration;

mod logic;
mod data;
mod ui;
mod utils;

pub const APP_NAME: &str = "Adhan";
pub const APP_VERSION: &str = "0.1.0";
pub const APP_AUTHOR: &str = "Oruc Qafarov";

use logic::clock_functions::get_formatted_time;
use logic::date_formatted::get_date_parts;
use logic::prayer_times::PrayerTimesManager;

const DSEG7_BOLD: &[u8] = include_bytes!("../fonts/DSEG-Classic/DSEG7Classic-Bold.ttf");
const DSEG7_LIGHT: &[u8] = include_bytes!("../fonts/DSEG-Classic/DSEG7Classic-Light.ttf");
const DALEK_BOLD: &[u8] = include_bytes!("../fonts/DSEG-Classic/DalekPinpointBold.ttf");
const GOTHIC_REGULAR: &[u8] = include_bytes!("../fonts/DSEG-Classic/SourceCodePro-Regular.ttf");
const MATERIAL_SYMBOLS: &[u8] = include_bytes!("../fonts/MaterialSymbolsOutlined[FILL,GRAD,opsz,wght].ttf");

fn black_background(_theme: &iced::Theme) -> container::Appearance {
    container::Appearance {
        background: Some(iced::Background::Color(iced::Color::BLACK)),
        ..Default::default()
    }
}

fn divider_background(_theme: &iced::Theme) -> container::Appearance {
    container::Appearance {
        background: Some(iced::Background::Color(iced::Color::from_rgb(0.4, 0.4, 0.4))),
        ..Default::default()
    }
}

struct AdhanApp {
    time: String,
    colon_visible: bool,
    window_width: f32,
    prayer_times: Option<logic::prayer_times::PrayerTimes>,
    // Шрифты для разных частей даты
    font_dseg_bold: Font,
    font_dseg_light: Font,
    font_dalek_bold: Font,
    font_gothic_reg: Font,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick,
    Resized(f32),
}

impl Application for AdhanApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn title(&self) -> String {
        String::from(APP_NAME)
    }

    fn new(_flags: ()) -> (AdhanApp, Command<Message>) {
        // Загружаем шрифты из байтов проекта
        let fs = iced_graphics::text::font_system();
        fs.write()
            .unwrap()
            .load_font(Cow::Owned(DSEG7_BOLD.to_vec()));
        fs.write()
            .unwrap()
            .load_font(Cow::Owned(DSEG7_LIGHT.to_vec()));
        fs.write()
            .unwrap()
            .load_font(Cow::Owned(MATERIAL_SYMBOLS.to_vec()));

        eprintln!("[FONT DEBUG] DSEG7-Bold loaded ({} bytes)", DSEG7_BOLD.len());
        eprintln!("[FONT DEBUG] DSEG7-Light loaded ({} bytes)", DSEG7_LIGHT.len());
        eprintln!("[FONT DEBUG] Material Symbols loaded ({} bytes)", MATERIAL_SYMBOLS.len());

        let pt_mgr = PrayerTimesManager::new();
        let prayer_times = pt_mgr.fetch_prayer_times().ok();

        (
            AdhanApp {
                time: get_formatted_time(true),
                colon_visible: true,
                window_width: 715.0,
                prayer_times,
                font_dseg_bold: Font {
                    family: Family::Name("DSEG7 Classic"),
                    weight: Weight::Bold,
                    ..Font::DEFAULT
                },
                font_dseg_light: Font {
                    family: Family::Name("DSEG7 Classic"),
                    weight: Weight::Light,
                    ..Font::DEFAULT
                },
                font_dalek_bold: Font {
                    family: Family::Name("Dalek Pinpoint Bold"),
                    weight: Weight::Bold,
                    ..Font::DEFAULT
                },
                font_gothic_reg: Font {
                    family: Family::Name("Source Code Pro"),
                    weight: Weight::Normal,
                    ..Font::DEFAULT
                },
            },
            Command::none(),
        )
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick => {
                self.colon_visible = !self.colon_visible;
                self.time = get_formatted_time(self.colon_visible);
                Command::none()
            }
            Message::Resized(width) => {
                self.window_width = width;
                Command::none()
            }
        }
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::batch(vec![
            iced::time::every(Duration::from_millis(500)).map(|_| Message::Tick),
            iced::subscription::events().map(|event| {
                if let iced::Event::Window(_, iced::window::Event::Resized { width, .. }) = event {
                    Message::Resized(width as f32)
                } else {
                    Message::Tick
                }
            }),
        ])
    }

    fn view(&self) -> iced::Element<'_, Message> {
        // Имя family из TTF: "DSEG7 Classic", вес: Bold
        let dseg_font = Font {
            family: Family::Name("DSEG7 Classic"),
            weight: Weight::Bold,
            ..Font::DEFAULT
        };

        let time_text = self.time.clone();

        // Размер шрифта часов зависит от ширины окна (как в оригинале: width / 3.5)
        // Ограничиваем: не меньше 40, не больше 250 пикселей
        let clock_font_size = (self.window_width / 3.5).clamp(40.0, 250.0);

        let clock = text(time_text)
            .size(clock_font_size)
            .font(dseg_font)
            .style(iced::Color::from_rgb(0.0, 1.0, 0.0));

        // Дата из частей с разными шрифтами (как в оригинале на Python)
        // Дата из частей с разными шрифтами (как в оригинале на Python)
        // Базовый размер шрифта зависит от ширины окна (как в Python: base_font_size = width * 0.04)
        let base_font_size = self.window_width * 0.04;
        let date_parts = get_date_parts(base_font_size);
        let date_row = Row::new()
            .push(date_part_widget(&date_parts[0], font_for_part(&date_parts[0], self)))
            .push(date_part_widget(&date_parts[1], font_for_part(&date_parts[1], self)))
            .push(date_part_widget(&date_parts[2], font_for_part(&date_parts[2], self)))
            .push(date_part_widget(&date_parts[3], font_for_part(&date_parts[3], self)))
            .push(date_part_widget(&date_parts[4], font_for_part(&date_parts[4], self)))
            .push(date_part_widget(&date_parts[5], font_for_part(&date_parts[5], self)))
            .push(date_part_widget(&date_parts[6], font_for_part(&date_parts[6], self)))
            .push(date_part_widget(&date_parts[7], font_for_part(&date_parts[7], self)))
            .push(date_part_widget(&date_parts[8], font_for_part(&date_parts[8], self)))
            .spacing(0)
            .align_items(Alignment::Center);

        let tahr = text(format!("Təhəccüd   {}", self.prayer_times.as_ref().map(|p| p.midnight.as_str()).unwrap_or("--:--")))
            .size(32)
            .font(dseg_font)
            .style(iced::Color::from_rgb(0.6, 0.6, 0.6));

        let imsak = text(format!("İmsak   {}", self.prayer_times.as_ref().map(|p| p.fajr.as_str()).unwrap_or("--:--")))
            .size(32)
            .font(dseg_font)
            .style(iced::Color::from_rgb(0.6, 0.6, 0.6));

        let gunesh = text(format!("Günəş   {}", self.prayer_times.as_ref().map(|p| p.sunrise.as_str()).unwrap_or("--:--")))
            .size(32)
            .font(dseg_font)
            .style(iced::Color::from_rgb(0.6, 0.6, 0.6));

        let gunorta = text(format!("Günorta  {}", self.prayer_times.as_ref().map(|p| p.dhuhr.as_str()).unwrap_or("--:--")))
            .size(32)
            .font(dseg_font)
            .style(iced::Color::from_rgb(0.6, 0.6, 0.6));

        let ikindi = text(format!("İkindi   {}", self.prayer_times.as_ref().map(|p| p.asr.as_str()).unwrap_or("--:--")))
            .size(32)
            .font(dseg_font)
            .style(iced::Color::from_rgb(0.6, 0.6, 0.6));

        let axsam = text(format!("Axşam    {}", self.prayer_times.as_ref().map(|p| p.maghrib.as_str()).unwrap_or("--:--")))
            .size(32)
            .font(dseg_font)
            .style(iced::Color::from_rgb(0.6, 0.6, 0.6));

        let geca = text(format!("Gecə     {}", self.prayer_times.as_ref().map(|p| p.isha.as_str()).unwrap_or("--:--")))
            .size(32)
            .font(dseg_font)
            .style(iced::Color::from_rgb(0.6, 0.6, 0.6));

        // Иконки для блока времени до следующей молитвы (Material Symbols)
        let mosque_icon = Font {
            family: Family::Name("Material Symbols Outlined"),
            weight: Weight::Normal,
            ..Font::DEFAULT
        };
        let bell_icon = Font {
            family: Family::Name("Material Symbols Outlined"),
            weight: Weight::Normal,
            ..Font::DEFAULT
        };

        // Размер иконок как в Python: base_font_size * 0.5, где base_font_size = width * 0.04
        // Для 715px это примерно 14-28 пикселей
        let icon_font_size = (self.window_width * 0.04 * 0.5).clamp(20.0, 60.0);

        // Коды иконок Material Symbols: \ueab2 = prayer_times (мечеть), \uf353 = bell (колокольчик)
        let mosque_text = text("\u{eab2}")
            .size(icon_font_size)
            .font(mosque_icon)
            .style(iced::Color::from_rgb(0.8, 0.6, 0.0));

        let bell_text = text("\u{f353}")
            .size(icon_font_size)
            .font(bell_icon)
            .style(iced::Color::from_rgb(0.8, 0.6, 0.0));

        // Время до следующей молитвы (будет вычисляться из API)
        let time_until_prayer = text("01:49")
            .size(64)
            .font(dseg_font)
            .style(iced::Color::from_rgb(1.0, 0.0, 0.0));

        // Линия-разделитель между часами и датой
        let divider0 = Container::new(iced::widget::Space::new(Length::Fill, Length::Fixed(2.0)))
            .style(divider_background);

        // Линия-разделитель между датой и временем до молитвы
        let divider1 = Container::new(iced::widget::Space::new(Length::Fill, Length::Fixed(2.0)))
            .style(divider_background);

        // Линия-разделитель между временем до молитвы и списком молитв
        let divider2 = Container::new(iced::widget::Space::new(Length::Fill, Length::Fixed(2.0)))
            .style(divider_background);

        // Блок времени до следующей молитвы с иконками
        let prayer_time_row = Row::new()
            .push(mosque_text)
            .push(time_until_prayer)
            .push(bell_text)
            .spacing(20)
            .align_items(Alignment::Center);

        let content = Column::new()
            .push(clock)
            .push(divider0)
            .push(date_row)
            .push(divider1)
            .push(prayer_time_row)
            .push(divider2)
            .push(tahr)
            .push(imsak)
            .push(gunesh)
            .push(gunorta)
            .push(ikindi)
            .push(axsam)
            .push(geca)
            .spacing(15)
            .align_items(Alignment::Center)
            .max_width(715.0);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(black_background)
            .into()
    }
}

// Вспомогательная функция для выбора шрифта по типу FontRef
fn font_for_part(part: &logic::date_formatted::DatePart, app: &AdhanApp) -> Font {
    match part.font {
        logic::date_formatted::FontRef::Dseg7Bold => app.font_dseg_bold.clone(),
        logic::date_formatted::FontRef::Dseg7Light => app.font_dseg_light.clone(),
        logic::date_formatted::FontRef::GothicReg => app.font_gothic_reg.clone(),
        logic::date_formatted::FontRef::DalekBold => app.font_dalek_bold.clone(),
    }
}

// Вспомогательная функция для создания текстового виджета части даты с нужным шрифтом
fn date_part_widget(part: &logic::date_formatted::DatePart, font: Font) -> iced::Element<'static, Message> {
    text(part.text.clone())
        .size(part.font_size)
        .font(font)
        .style(part.color)
        .into()
}

fn main() -> iced::Result {
    AdhanApp::run(Settings {
        window: window::Settings {
            size: iced::Size::new(715.0, 1000.0),
            resizable: true,
            ..Default::default()
        },
        default_text_size: iced::Pixels(20.0),
        ..Default::default()
    })
}
