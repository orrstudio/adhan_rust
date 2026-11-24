# 🖥️ Сравнение GUI-инструментов для Adhan (Rust)

Цель: выбрать один инструмент для построения десктопного интерфейса на Rust.
Требования: Arch Linux + Wayland, только `GridLayout` (нет `BoxLayout`), dark theme, семисегментные шрифты, иконки, анимации мигания, Flatpak-дистрибутив, позже — Windows и мобильные платформы.

---

## 1. Iced

**Описание:** Elm-подобный (React-подобный) GUI-фреймворк для Rust. Работает через wgpu (GPU) или softbuffer. Декларативный стиль: состояние → представление → сообщения → обновление состояния.

**Примеры приложений:**
- Свой файловый менеджер, текстовый редактор, проигрыватель мультимедиа
- Промышленные интерфейсы, дашборды

**Плюсы:**
- Полностью на Rust, нет системных зависимостей GTK/Qt
- Хорошо работает под Wayland (через winit + wgpu)
- Архитектура близка к Kivy (состояние + события), что упростит портирование с Python
- Поддержка кастомных шрифтов и SVG иконок
- Анимации через `animation` и `subscription`
- Активное сообщество, регулярные обновления

**Минусы:**
- Меньше готовых виджетов, чем у GTK/Qt; много придётся писать sendiri (кнопки, переключатели, списки)
- Сложность с кастомной отрисовкой семисегментных часов (требует ручного рисования через canvas или использование текста с моноширинным шрифтом)
- Поддержка мобильных платформ ограничена (есть, но менее зрелая, чем у egui или нативных решений)

**Пример кода (упрощённый):**
```rust
use iced::{widget::{text, column}, Alignment, Application, Command, Settings, Element, Subscription, Time};

struct AdhanApp {
    current_time: String,
}

#[derive(Debug, Clone)]
enum Message {
    Tick(String),
}

impl Application for AdhanApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn title(&self) -> String { String::from("Adhan") }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(time) => {
                self.current_time = time;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            text(&self.current_time).size(50),
        ]
        .align_items(Alignment::Center)
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(500))
            .map(|_| Message::Tick(chrono::Local::now().format("%H:%M").to_string()))
    }
}

fn main() -> iced::Result {
    AdhanApp::run(Settings::default())
}
```

---

## 2. Slint

**Описание:** Декларативный UI-фреймворк с własным языком разметки `.slint`. Есть визуальный дизайнер (Slint Studio). Компилируется в нативный код.

**Примеры приложений:**
- Встраиваемые системы, приборы, панели управления
- Десктопные утилиты с кастомным дизайном

**Плюсы:**
- Визуальный дизайнер для быстрой прототипировки внешнего вида
- Декларативный `.slint` язык, легко разделять логику и представление
- Хорошая производительность (отрисовка через свой движок)
- Поддержка Wayland
- Русскоязычная документация растёт

**Минусы:**
- Меньше сообщества и библиотек, чем у iced/egui
- Кастомные виджеты (семисегментные часы) могут потребовать углубления в内部 API
- Мобильная поддержка ограничена
- Лицензия (возможны ограничения для закрытого исходного кода, нужно проверить)

**Пример кода (упрощённый):**
```slint
import { VerticalLayout, Text } from "std-widgets.slint";

export component MainWindow inherits Window {
    width: 715px;
    height: 1000px;
    title: "Adhan";

    VerticalLayout {
        alignment: center;
        spacing: 20px;

        Text {
            text: "04:04";
            color: green;
            font-size: 80px;
        }
    }
}
```

---

## 3. GTK4-rs (gtk-rs)

**Описание:** Привязки к GTK4 (biblioteka GNOME). Нативный вид для Linux/Gnome.

**Примеры приложений:**
- GNOME Text Editor, GNOME Maps, nombreuses приложения GNOME

**Плюсы:**
- Нативный вид под Gnome/Arch Linux (Wayland нативно)
- Огромное количество готовых виджетов (кнопки, списки, переключатели, настройки)
- Хорошая поддержка тем, доступность, internacionaisция
- Зрелое решение, много документации

**Минусы:**
- Тяжёлые системные зависимости (GTK, glib, pango, cairo и т.д.)
- Сложнее собирать Flatpak (нужно включать runtime)
- Архитектура не такая "чистая" как iced/Elm; больше callbacks и мутабельного состояния
- Поддержка Windows и мобильных платформ слабая или отсутствует
- Кастомная отрисовка семисегментных часов сложнее, чем в GPU-фреймворках

**Пример кода (упрощённый):**
```rust
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label};

fn main() {
    let app = Application::builder().application_id("com.orrstudio.Adhan").build();
    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Adhan")
            .default_width(715)
            .default_height(1000)
            .build();
        let label = Label::builder().label("04:04").build();
        window.set_child(Some(&label));
        window.present();
    });
    app.run();
}
```

---

## 4. egui

**Описание:** Мгновенный (immediate mode) GUI-фреймворк. Отрисовывает весь интерфейс каждый кадр.

**Примеры приложений:**
- Инструменты для разработчиков, дашборды, внутриигровые меню, прототипы

**Плюсы:**
- Очень простой в использовании, минимум boilerplate
- Высокая производительность
- Поддержка веба (wasm), десктопа, мобильных
- Легко кастомизировать

**Минусы:**
- Immediate mode не подходит для классического десктопного приложения с постоянным состоянием окон
- Нет встроенной поддержки Wayland (нужен бэкенд: winit, miniquad, glow и т.д.)
- Сложно реализовать нативное поведение окон (заголовок, ресайз, позиционирование)
- Меньше подходит для "приложения с настройками и сохранением состояния окна"

**Пример кода (упрощённый):**
```rust
egui::CentralPanel::default().show(ctx, |ui| {
    ui.heading("Adhan");
    ui.label("04:04");
});
```

---

## 📊 Сравнительная таблица

| Критерий | Iced | Slint | GTK4-rs | egui |
|----------|------|-------|---------|------|
| Язык разметки | Rust (код) | .slint | Rust (код/builder) | Rust (код) |
| Визуальный дизайнер | Нет | Да (Slint Studio) | Нет (Glade для GTK3, для GTK4 ограничено) | Нет |
| Wayland | Да (winit+wgpu) | Да | Да (нативно) | Через бэкенд |
| Архитектура | Elm/React (состояние+сообщения) | Декларативная | Императивная (callbacks) | Immediate mode |
| Готовые виджеты | Среднее | Среднее | Высокое | Низкое |
| Кастомная отрисовка (семисегмент) | Через canvas | Через свой рендеринг | Через cairo/canvas | Просто (рисование каждого кадра) |
| Flatpak | Хорошо | Хорошо | Сложно (runtime) | Хорошо |
| Windows | Да | Да | Ограничено | Да |
| Мобильные | Ограничено | Ограничено | Нет | Да (wasm/android/ios) |
| Сообщество | Активное | Маленькое | Большое | Активное |
| Кривая обучения | Средняя | Низкая | Высокая | Низкая |

---

## ✅ Моя рекомендация

Для проекта **Adhan** я рекомендую **Iced**:

1. **Архитектура близка к Kivy** — состояние + сообщения, что упростит портирование логики из Python-прототипа.
2. **Wayland-friendly** — работает через winit + wgpu, нет привязки к X11.
3. **Полностью на Rust** — нет тяжелых системных зависимостей GTK, проще собирать Flatpak и статический бинарник.
4. **Масштабируемость** — когда потребуются Windows и мобильные платформы, iced легче адаптировать, чем GTK4.

**Альтернатива:** если хочешь визуальный дизайнер для быстрой верстки таблицы молитв и настроек — посмотри **Slint**.

**Нужно подтверждение:** какой инструмент выбираем? После этого я создам `Cargo.toml` и скелет проекта.
