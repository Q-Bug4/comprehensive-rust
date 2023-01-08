use std::fmt::Write;

pub trait Widget {
    /// Natural width of `self`.
    fn width(&self) -> usize;

    /// Draw the widget into a buffer.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

    /// Draw the widget on standard output.
    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{}", &buffer);
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
    callback: Box<dyn FnMut()>,
}

impl Button {
    fn new(label: &str, callback: Box<dyn FnMut()>) -> Button {
        Button {
            label: Label::new(label),
            callback,
        }
    }
}

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    fn new(title: &str) -> Window {
        Window {
            title: title.to_owned(),
            widgets: Vec::new(),
        }
    }

    fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }
}


impl Widget for Label {
    fn width(&self) -> usize {
        10
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        buffer.write_str(&self.label).expect("buffer write error");
        buffer.write_str("\n").expect("write \\n error");
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        20
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let width = self.width();
        let border_middle_len = width - 2;
        let mut border = format!("+{:-^border_middle_len$}+\n", "");
        buffer.write_str(&border).expect("buffer write error");
        buffer.write_str(&format!("|{: ^border_middle_len$}|\n", &self.label.label)).expect("buffer write error");
        buffer.write_str(&border).expect("buffer write error");
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        40
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let width = self.width();
        let border_middle_len = width - 2;
        let mut border = format!("+{:-^border_middle_len$}+\n", "");
        buffer.write_str(&border).expect("buffer write error");
        for widget in &self.widgets {
            let mut content: String = String::new();
            widget.draw_into(&mut content);
            for line in content.split("\n") {
                if line.is_empty() {
                    continue
                }
                buffer.write_str(&format!("|{: <border_middle_len$}|\n", line)).expect("buffer write error");
            }
        }
        buffer.write_str(&border).expect("buffer write error");
    }
}

fn main() {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new(
        "Click me!",
        Box::new(|| println!("You clicked the button!")),
    )));
    window.draw();
}