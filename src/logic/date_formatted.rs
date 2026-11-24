// Форматирование дат (григорианская, хиджрская)
// Точно по оригиналу Python: logic/date_formatted.py

use chrono::{Datelike, Local, Weekday};

// Часть даты с её шрифтом, размером и цветом
#[derive(Clone)]
pub struct DatePart {
    pub text: String,
    pub font: FontRef,      // Какой шрифт использовать
    pub font_size: f32,     // Размер шрифта
    pub color: iced::Color, // Цвет текста
}

#[derive(Clone, Copy)]
pub enum FontRef {
    Dseg7Bold,   // Хиджрский год, день, григорианский день, год
    Dseg7Light,  // Хиджрский год, месяц, день, григорианский день, месяц, год
    GothicReg,   // Хиджрский месяц, григорианский месяц
    DalekBold,   // День недели римскими цифрами
}

/// Возвращает части даты для отображения в одной строке
/// Формат (точно по оригиналу Python logic/date_formatted.py):
/// Хиджрский год (DSEG7-Light) + Хиджрский месяц (GothicRegular) + Хиджрский день (DSEG7-Light)
/// + Разделитель + День недели (DalekBold) + Разделитель + День (DSEG7-Light) + Точка + Месяц (GothicRegular) + Год (DSEG7-Light)
/// Итог: 1448/VI/6 - VI - 17/XI/2026
pub fn get_date_parts(base_font_size: f32) -> Vec<DatePart> {
    let now = Local::now();
    let weekday_idx = now.weekday().number_from_monday() - 1; // 0-6

    // Römische Zahlen für Wochentage
    let weekday_roman = match weekday_idx {
        0 => "I", 1 => "II", 2 => "III", 3 => "IV", 4 => "V", 5 => "VI", 6 => "VII",
        _ => "I",
    };

    // Römische Zahlen für Monate
    let month_roman = match now.month() {
        1 => "I", 2 => "II", 3 => "III", 4 => "IV", 5 => "V", 6 => "VI",
        7 => "VII", 8 => "VIII", 9 => "IX", 10 => "X", 11 => "XI", 12 => "XII",
        _ => "I",
    };

    // Хиджрская дата (заглушка, будет из API)
    let hijri_year = "1448";
    let hijri_month = "VI";
    let hijri_day = "6";

    // Григорианская дата
    let day = now.day();
    let month = now.month();
    let year = now.year();

    let yellow = iced::Color::from_rgb(1.0, 1.0, 0.0);

    // Все части используют одинаковый размер (как в Python: base_font_size * 1.0)
    let size = base_font_size;

    vec![
        // Хиджрский год (DSEG7-Light)
        DatePart {
            text: hijri_year.to_string(),
            font: FontRef::Dseg7Light,
            font_size: size,
            color: yellow,
        },
        // Хиджрский месяц (GothicRegular)
        DatePart {
            text: format!("/{}/", hijri_month),
            font: FontRef::GothicReg,
            font_size: size,
            color: yellow,
        },
        // Хиджрский день (DSEG7-Light) без пробела в конце
        DatePart {
            text: hijri_day.to_string(),
            font: FontRef::Dseg7Light,
            font_size: size,
            color: yellow,
        },
        // Разделитель " - " между хиджрой и днём недели
        DatePart {
            text: " - ".to_string(),
            font: FontRef::DalekBold,
            font_size: size,
            color: yellow,
        },
        // День недели римскими (DalekBold) без пробелов
        DatePart {
            text: weekday_roman.to_string(),
            font: FontRef::DalekBold,
            font_size: size,
            color: yellow,
        },
        // Разделитель между днём недели и григорианской датой
        DatePart {
            text: " - ".to_string(),
            font: FontRef::DalekBold,
            font_size: size,
            color: yellow,
        },
        // День месяца (DSEG7-Light)
        DatePart {
            text: day.to_string(),
            font: FontRef::Dseg7Light,
            font_size: size,
            color: yellow,
        },
        // Месяц римскими (GothicRegular)
        DatePart {
            text: format!("/{}/", month_roman),
            font: FontRef::GothicReg,
            font_size: size,
            color: yellow,
        },
        // Год (DSEG7-Light)
        DatePart {
            text: year.to_string(),
            font: FontRef::Dseg7Light,
            font_size: size,
            color: yellow,
        },
    ]
}
