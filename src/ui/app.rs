//==============================================================================
// Open crates and libraries
//==============================================================================
use iced::{Image, image, Text};
use iced::{button, Button};
use iced::{Align, Row, Column, Container, Element, Sandbox};
use iced::{HorizontalAlignment, Length};

use fractal::definitions::{Color, IterationStyle, IteratorKind};
use fractal::color;
use fractal::draw;

use ui::state::*;


//==============================================================================
// States of application parts
//==============================================================================
#[derive(Default)]
pub struct MainWindow {
  frac_state : FractalState,
  image_state : ImageState,
  basic_buttons : BasicButtons,
}

#[derive(Default)]
pub struct BasicButtons {
  zoom_in_button : button::State,
  zoom_out_button : button::State,
  go_left_button : button::State,
  go_right_button : button::State,
  go_up_button : button::State,
  go_down_button : button::State,
  pixel_up_button : button::State,
  pixel_down_button : button::State,
  steps_up_button : button::State,
  steps_down_button : button::State,
  change_color_button : button::State,
  change_iteration_button : button::State,
  change_iterator_button : button::State,
  re_up_button : button::State,
  re_down_button : button::State,
  im_up_button : button::State,
  im_down_button : button::State,
}

//==============================================================================
// Messages
//==============================================================================
#[derive(Debug, Clone, Copy)]
pub enum Message {
  ZoomIn,
  ZoomOut,
  GoLeft,
  GoRight,
  GoUp,
  GoDown,
  PixelUp,
  PixelDown,
  StepsUp,
  StepsDown,
  ChangeColor,
  ChangeIteration,
  ChangeIterator,
  ReUp,
  ReDown,
  ImUp,
  ImDown,
}


//==============================================================================
// Implementation
//==============================================================================

// Main Window

impl MainWindow {
  fn redraw_frac(&mut self, redraw_image : bool) {
    self.frac_state.fractal = draw::compute_fractal(self.frac_state.args);
    if redraw_image { self.redraw_only_image() }
  }

  fn redraw_only_image(&mut self) {
    let frac = &self.frac_state.fractal;
    let image = 
      color::color_fractal(
        frac, self.frac_state.args.steps, self.image_state.args.color);
    self.image_state.image = image
  }
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
        self.frac_state.args.field.radius *= 0.8;
      }
      Message::ZoomOut => {
        self.frac_state.args.field.radius *= 1.2;
      }
      Message::GoLeft => {
        let dx = self.frac_state.args.field.radius * 0.2;
        self.frac_state.args.field.re_center -= dx
      }
      Message::GoRight => {
        let dx = self.frac_state.args.field.radius * 0.2;
        self.frac_state.args.field.re_center += dx
      }
      Message::GoUp => {
        let dy = self.frac_state.args.field.radius * 0.2;
        self.frac_state.args.field.im_center -= dy
      }
      Message::GoDown => {
        let dy = self.frac_state.args.field.radius * 0.2;
        self.frac_state.args.field.im_center += dy
      }
      Message::PixelUp => {
        self.frac_state.args.field.pixel_size *= 2
      }
      Message::PixelDown => {
        let pix_size = self.frac_state.args.field.pixel_size;
        let new_size = std::cmp::max(pix_size / 2, 128);
        self.frac_state.args.field.pixel_size = new_size
      }
      Message::StepsUp => {
        self.frac_state.args.steps += 20
      }
      Message::StepsDown => {
        let steps = std::cmp::max(self.frac_state.args.steps, 21);
        let new_steps = steps-20;
        self.frac_state.args.steps = new_steps;
      }
      Message::ChangeColor => {
        match self.image_state.args.color {
          Color::Azul => {self.image_state.args.color = Color::Sunset}
          Color::Sunset => {self.image_state.args.color = Color::Sky}
          Color::Sky => {self.image_state.args.color = Color::Azul}
        }
      }
      Message::ChangeIteration => {
        match self.frac_state.args.iteration_style {
          IterationStyle::Mandelbrot => 
            {self.frac_state.args.iteration_style = IterationStyle::Julia}
          IterationStyle::Julia => 
            {self.frac_state.args.iteration_style = IterationStyle::Mandelbrot}
        }
      }
      Message::ChangeIterator => {
        match self.frac_state.args.iterator_kind {
          IteratorKind::Square => 
            {self.frac_state.args.iterator_kind = IteratorKind::Cube}
          IteratorKind::Cube => 
            {self.frac_state.args.iterator_kind = IteratorKind::Ship}
          IteratorKind::Ship => 
            {self.frac_state.args.iterator_kind = IteratorKind::Square}
        }
      }
      Message::ReUp => {
        self.frac_state.args.c_re += 0.01
      }
      Message::ReDown => {
        self.frac_state.args.c_re -= 0.01
      }
      Message::ImUp => {
        self.frac_state.args.c_im += 0.01
      }
      Message::ImDown => {
        self.frac_state.args.c_im -= 0.01
      }
    };
    // This should ideally only happen when needed
    self.redraw_frac(true)
  }

  fn view(&mut self) -> Element<Message> {
    let frac_image = self.image_state.image.clone();
    let pix_size = self.frac_state.args.field.pixel_size as u32;
    let image_handle = 
      image::Handle::from_pixels(pix_size, pix_size, frac_image);
    let image = 
      Image::new(image_handle)
      .width(Length::Units(1000)).height(Length::Units(1000));
    let basic_buttons = self.basic_buttons.view();
    Row::new().padding(10)
      .align_items(Align::Center)
      .push(basic_buttons)
      .push(image)
      .into()
  }
}

// Basic Buttons

impl BasicButtons {

  fn view(&mut self) -> Element<Message> {
    let row_space = 10;
    let row_pad = 10;

    let button = |state, label, message| {
      Button::new(
          state,
          Text::new(label)
              .width(Length::Fill)
              .horizontal_alignment(HorizontalAlignment::Center)
              .size(24),
      )
      .width(Length::Fill)
      .padding(8)
      .on_press(message)
      };

    let buttons =
      Column::new().padding(10).spacing(10)
        .align_items(Align::Center)
        .push( Row::new().padding(row_pad).spacing(row_space)
          .push( button(&mut self.zoom_in_button, "+", Message::ZoomIn) )
          .push( button(&mut self.zoom_out_button, "-", Message::ZoomOut) )
        )
        .push( Row::new().padding(row_pad).spacing(row_space)
          .push( button(&mut self.go_left_button, "◄", Message::GoLeft) )
          .push( button(&mut self.go_up_button, "▲", Message::GoUp) )
          .push( button(&mut self.go_down_button, "▼", Message::GoDown) )
          .push( button(&mut self.go_right_button, "►", Message::GoRight) )
        )
        .push( Row::new().padding(row_pad).spacing(row_space)
          .push( 
            button(&mut self.pixel_up_button, 
              "More Detailed", Message::PixelUp) )
          .push( 
            button(&mut self.pixel_down_button, 
              "Less Detailed", Message::PixelDown) )
        )
        .push( Row::new().padding(row_pad).spacing(row_space)
          .push( 
            button(&mut self.steps_up_button,
              "More Steps", Message::StepsUp) )
          .push( 
            button(&mut self.steps_down_button,
              "Less Steps", Message::StepsDown) )
        )
        .push( Row::new().padding(row_pad).spacing(row_space)
          .push( 
            button(&mut self.change_color_button,
              "Change Color", Message::ChangeColor) )
        )
        .push( Row::new().padding(row_pad).spacing(row_space)
          .push( 
            button(&mut self.change_iteration_button,
              "Change Iteration", Message::ChangeIteration) )
          .push( 
            button(&mut self.change_iterator_button,
              "Change Iterator", Message::ChangeIterator) )
        )
        .push( Row::new().padding(row_pad).spacing(row_space)
          .push( button(&mut self.re_up_button, "re+", Message::ReUp) )
          .push( button(&mut self.re_down_button, "re-", Message::ReDown) )
          .push( button(&mut self.im_up_button, "im+", Message::ImUp) )
          .push( button(&mut self.im_down_button, "im-", Message::ImDown) )
        )
      ;
    Container::new(buttons).width(Length::Fill).height(Length::Fill).into()
  }
}