//==============================================================================
// Open crates and libraries
//==============================================================================
use iced::{Image, image, Text};
use iced::{button, Button};
use iced::{Align, Row, Column, Element, Sandbox};

use fractal::color;
use fractal::draw;
use fractal::definitions::*;

use ui::state::*;

#[derive(Default)]
pub struct MainWindow {
  frac_state : FractalState,
  image_state : ImageState,
  zoom_in_button : button::State,
  zoom_out_button : button::State,
  go_left_button : button::State,
  go_right_button : button::State,
  go_down_button : button::State,
  go_up_button : button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
  ZoomIn,
  ZoomOut,
  GoLeft,
  GoRight,
  GoDown,
  GoUp,
}

impl Sandbox for MainWindow {
  type Message = Message;

  fn new() -> Self {
    Self::default()
  }

  fn title(&self) -> String {
    String::from("Fractalisator")
  }

  fn update(&mut self, message: Message) {
    match message {
      Message::ZoomIn => {
        self.frac_state.args.field.radius *= 0.9;
      }
      Message::ZoomOut => {
        self.frac_state.args.field.radius *= 1.1;
      }
      Message::GoLeft => {
        let dx = self.frac_state.args.field.radius * 0.1;
        self.frac_state.args.field.re_center += dx
      }
      Message::GoRight => {
        let dx = self.frac_state.args.field.radius * 0.1;
        self.frac_state.args.field.re_center -= dx
      }
      Message::GoUp => {
        let dy = self.frac_state.args.field.radius * 0.1;
        self.frac_state.args.field.im_center += dy
      }
      Message::GoDown => {
        let dy = self.frac_state.args.field.radius * 0.1;
        self.frac_state.args.field.im_center -= dy
      }
    }
  }

  fn view(&mut self) -> Element<Message> {
    let frac = draw::compute_fractal(self.frac_state.args);
    let frac_image = color::color_fractal(&frac);
    let image = image::Handle::from_pixels(2000, 2000, frac_image);
    let col1 = Column::new().padding(10)
      .align_items(Align::Center)
      .push(
        Row::new()
        .push(
          Button::new(&mut self.zoom_in_button, Text::new("Zoom In"))
            .on_press(Message::ZoomIn),
        )
        .push(
          Button::new(&mut self.zoom_out_button, Text::new("Zoom Out"))
            .on_press(Message::ZoomOut),
        ))
      .push(
        Row::new()
        .push(
          Button::new(&mut self.go_left_button, Text::new("Left"))
            .on_press(Message::GoLeft),
        )
        .push(
          Button::new(&mut self.go_right_button, Text::new("Right"))
            .on_press(Message::GoRight),
        ))
      .push(
        Row::new()
        .push(
          Button::new(&mut self.go_up_button, Text::new("Up"))
            .on_press(Message::GoUp),
        )
        .push(
          Button::new(&mut self.go_down_button, Text::new("Down"))
            .on_press(Message::GoDown),
        ))
    ;
    Row::new()
      .push(col1)
      .push(Image::new(image))
      .into()
  }
}