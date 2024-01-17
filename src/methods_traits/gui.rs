#![allow(dead_code)]
pub trait Widget {
    /// Natural width of `self`.
    fn width(&self) -> usize;

    /// Draw the widget into a buffer.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

    /// Draw the widget on standard output.
    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{buffer}");
    }
}

pub struct Label {
    label: String,
}

impl Label {
    fn new(label: &str) -> Label {
        Label {
            label: label.to_owned(),
        }
    }
}

pub struct Button {
    label: Label,
}

impl Button {
    fn new(label: &str) -> Button {
        Button {
            label: Label::new(label),
        }
    }
}

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    fn new(title: &str) -> Self {
        Self {
            title: title.to_owned(),
            widgets: Vec::new(),
        }
    }

    fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }

    fn inner_width(&self) -> usize {
        std::cmp::max(
            self.title.chars().count(),
            self.widgets.iter().map(|w| w.width()).max().unwrap_or(0),
        )
    }
}

// Implement `Widget` for `Window`.
impl Widget for Window {
    fn width(&self) -> usize {
        self.inner_width() + 8
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let inner_width = self.width();
        writeln!(buffer, "+-{:-<inner_width$}-+", "").unwrap(); // left aligned
        writeln!(buffer, "| {:^inner_width$} |", &self.title).unwrap(); // centered
        writeln!(buffer, "+={:=<inner_width$}=+", "").unwrap(); // right aligned

        // send widget to buffer
        let mut inner = String::new();
        for widget in &self.widgets {
            widget.draw_into(&mut inner);
        }
        for line in inner.lines() {
            writeln!(buffer, "| {:<inner_width$} |", line).unwrap(); // left aligned
        }
        writeln!(buffer, "+-{:-<inner_width$}-+", "").unwrap();
    }
}

// Implement `Widget` for `Button`.
impl Widget for Button {
    fn width(&self) -> usize {
        self.label.width() + 4
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let width = self.width();
        let mut label = String::new();
        self.label.draw_into(&mut label);

        writeln!(buffer, "+{:-<width$}+", "").unwrap(); // left aligned
        for line in label.lines() {
            writeln!(buffer, "|{:^width$}|", &line).unwrap(); // centered
        }
        writeln!(buffer, "+{:-<width$}+", "").unwrap(); // right aligned
    }
}

// Implement `Widget` for `Label`.
impl Widget for Label {
    fn width(&self) -> usize {
        self.label
            .lines()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0)
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        writeln!(buffer, "{}", self.label).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // cargo test test_window_draw -- --nocapture
    #[test]
    fn test_window_draw() {
        let mut window = Window::new("Rust GUI Demo 1.23");
        window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
        window.add_widget(Box::new(Button::new("Click me!")));
        window.draw();
    }
}
