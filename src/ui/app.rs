//==============================================================================
// Open crates and libraries
//==============================================================================
use iced::{button, Button};
use iced::{pick_list, PickList};
use iced::{slider, Slider};
use iced::{image, Image, Text};
use iced::{Align, Column, Container, Element, Row, Sandbox};
use iced::{HorizontalAlignment, Length};

use fractal::color;
use fractal::definitions;
use fractal::definitions::FracArgs;
use fractal::definitions::{
  Gradient, GradientPreset, IterationStyle, IteratorKind};
use fractal::draw;

use ui::state::*;

//==============================================================================
// States of application parts
//==============================================================================
#[derive(Default)]
pub struct MainWindow {
  frac_state: FractalState,
  image_state: ImageState,
  basic_buttons: BasicButtons,
  log_increment_size: f64,
}

#[derive(Default)]
pub struct BasicButtons {
  navigation_buttons: NavigationButtons,
  fractal_adjustment_buttons: FractalAdjustmentButtons,
  image_adjustment_buttons: ImageAdjustmentButtons,
  change_color_button: button::State, // temporary
}

#[derive(Default)]
pub struct NavigationButtons {
  zoom_in_button: button::State,
  zoom_out_button: button::State,
  go_left_button: button::State,
  go_right_button: button::State,
  go_up_button: button::State,
  go_down_button: button::State,
}

#[derive(Default)]
pub struct FractalAdjustmentButtons {
  steps_up_button: button::State,
  steps_down_button: button::State,
  change_iteration_list: pick_list::State<IterationStyle>,
  change_iterator_list: pick_list::State<IteratorKind>,
  increment_slider: slider::State,
  re_up_button: button::State,
  re_down_button: button::State,
  im_up_button: button::State,
  im_down_button: button::State,
}

#[derive(Default)]
pub struct ImageAdjustmentButtons {
  pixel_up_button: button::State,
  pixel_down_button: button::State,
}
//==============================================================================
// Messages
//==============================================================================

#[derive(Debug, Clone, Copy)]
pub enum FracMessage {
  ZoomIn, ZoomOut,
  GoLeft, GoRight, GoUp, GoDown,
  PixelUp, PixelDown,
  StepsUp, StepsDown,
  IterationPicked(IterationStyle), IteratorPicked(IteratorKind),
  ChangeIncrementSize(f32),
  ReUp(f64), ReDown(f64), ImUp(f64), ImDown(f64),
}

#[derive(Debug, Clone, Copy)]
pub enum ImageMessage {
  ChangeColor,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
  Frac(FracMessage),
  Image(ImageMessage),
}

//==============================================================================
// Implementation
//==============================================================================

// Main Window

impl MainWindow {
  fn redraw_frac(&mut self, redraw_image: bool) {
    self.frac_state.fractal = draw::compute_fractal(self.frac_state.args);
    if redraw_image {
      self.redraw_only_image()
    }
  }

  fn redraw_only_image(&mut self) {
    let frac = &self.frac_state.fractal;
    let gradient = self.image_state.args.gradient.clone();
    let image = 
      color::color_fractal(frac, self.frac_state.args.steps, gradient);
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
      Message::Frac(fm) => {
        match fm {
          FracMessage::ZoomIn => {
            self.frac_state.args.field.radius *= 0.8;
          }
          FracMessage::ZoomOut => {
            self.frac_state.args.field.radius *= 1.2;
          }
          FracMessage::GoLeft => {
            let dx = self.frac_state.args.field.radius * 0.2;
            self.frac_state.args.field.center_re -= dx
          }
          FracMessage::GoRight => {
            let dx = self.frac_state.args.field.radius * 0.2;
            self.frac_state.args.field.center_re += dx
          }
          FracMessage::GoUp => {
            let dy = self.frac_state.args.field.radius * 0.2;
            self.frac_state.args.field.center_im -= dy
          }
          FracMessage::GoDown => {
            let dy = self.frac_state.args.field.radius * 0.2;
            self.frac_state.args.field.center_im += dy
          }
          FracMessage::PixelUp => self.frac_state.args.field.pixel_size *= 2,
          FracMessage::PixelDown => {
            let pix_size = self.frac_state.args.field.pixel_size;
            let new_size = std::cmp::max(pix_size / 2, 128);
            self.frac_state.args.field.pixel_size = new_size
          }
          FracMessage::StepsUp => self.frac_state.args.steps += 20,
          FracMessage::StepsDown => {
            let steps = std::cmp::max(self.frac_state.args.steps, 21);
            let new_steps = steps - 20;
            self.frac_state.args.steps = new_steps;
          }
          FracMessage::IterationPicked(kind) => {
            self.frac_state.args.iteration_style = kind
          },
          FracMessage::IteratorPicked(kind) => {
            self.frac_state.args.iterator_kind = kind
          },
          FracMessage::ChangeIncrementSize(val) => {
            self.log_increment_size = val as f64
          },
          FracMessage::ReUp(log_incr) =>
            self.frac_state.args.c_re += (2.0 as f64).powf(log_incr),
          FracMessage::ReDown(log_incr) =>
            self.frac_state.args.c_re -= (2.0 as f64).powf(log_incr),
          FracMessage::ImUp(log_incr) =>
            self.frac_state.args.c_im += (2.0 as f64).powf(log_incr),
          FracMessage::ImDown(log_incr) =>
            self.frac_state.args.c_im -= (2.0 as f64).powf(log_incr),
        };
        // Since the fractal arguments changed, we have to update the raw and
        // colored fractal
        self.redraw_frac(true)
      }
      Message::Image(im) => {
        match im {
          ImageMessage::ChangeColor => match self.image_state.current_preset {
            GradientPreset::Azul => {
              self.image_state.args.gradient = Gradient::svarog_gradient();
              self.image_state.current_preset = GradientPreset::Svarog
            }
            GradientPreset::Svarog => {
              self.image_state.args.gradient = Gradient::emperor_gradient();
              self.image_state.current_preset = GradientPreset::Emperor
            }
            GradientPreset::Emperor => {
              self.image_state.args.gradient = Gradient::gaia_gradient();
              self.image_state.current_preset = GradientPreset::Gaia
            }
            GradientPreset::Gaia => {
              self.image_state.args.gradient = Gradient::azul_gradient();
              self.image_state.current_preset = GradientPreset::Azul
            }
          },
        };
        self.redraw_only_image()
      }
    }
  }

  fn view(&mut self) -> Element<Message> {
    let image_handle = if self.image_state.args.better_resize {
      let pix_size = self.frac_state.args.field.pixel_size;
      let frac_image = 
        color::resize_fractal_image(&self.image_state.image, pix_size, 1000);
      image::Handle::from_pixels(1000, 1000, frac_image)
    } else {
      let pix_size = self.frac_state.args.field.pixel_size;
      let frac_image = self.image_state.image.clone();
      image::Handle::from_pixels(pix_size as u32, pix_size as u32, frac_image)
    };
    let image = Image::new(image_handle)
      .width(Length::Units(1000))
      .height(Length::Units(1000));
    let basic_buttons = 
      self.basic_buttons.view(self.frac_state.args, self.log_increment_size);
    Row::new()
      .padding(10)
      .align_items(Align::Center)
      .push(basic_buttons.width(Length::Units(500)))
      .push(Column::new().padding(10).spacing(10).push(image))
      .into()
  }
}

// Basic Buttons

impl<'a> BasicButtons {
  fn view(
    &'a mut self,
    frac_args: FracArgs,
    log_increment_size: f64,
  ) -> Column<'a, Message> {
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

    Column::new()
      .padding(10)
      .spacing(10)
      .push(self.navigation_buttons.view())
      .push(self.image_adjustment_buttons.view())
      .push(
        self.fractal_adjustment_buttons.view(
          frac_args.iteration_style,
          frac_args.iterator_kind,
          log_increment_size
        )
      )
      .push(Row::new().padding(row_pad).spacing(row_space).push(button(
        &mut self.change_color_button,
        "Change Color",
        Message::Image(ImageMessage::ChangeColor),
      )))
      .push(
        Row::new()
          .padding(row_pad)
          .spacing(row_space)
          .push(Text::new(format!(
            "Constant: {:.10} + {:.10} i",
            frac_args.c_re, frac_args.c_im
          ))),
      )
      .push(
        Row::new()
          .padding(row_pad)
          .spacing(row_space)
          .push(Text::new(format!(
            "Center: {:.10} + {:.10} i",
            frac_args.field.center_im, frac_args.field.center_re
          ))),
      )
      .push(
        Row::new()
          .padding(row_pad)
          .spacing(row_space)
          .push(Text::new(format!("Radius: {:.10}", frac_args.field.radius))),
      )
      .push(
        Row::new()
          .padding(row_pad)
          .spacing(row_space)
          .push(Text::new(format!("Steps: {}", frac_args.steps))),
      )
      .push(
        Row::new()
          .padding(row_pad)
          .spacing(row_space)
          .push(Text::new(format!(
            "Pixel width: {}",
            frac_args.field.pixel_size
          ))),
      )
  }
}

impl<'a> NavigationButtons {
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

    let buttons = Column::new()
      .padding(10)
      .spacing(10)
      .align_items(Align::Center)
      .push(
        Row::new()
          .padding(row_pad)
          .spacing(row_space)
          .push(button(
            &mut self.zoom_in_button,
            "+",
            Message::Frac(FracMessage::ZoomIn),
          ))
          .push(button(
            &mut self.zoom_out_button,
            "-",
            Message::Frac(FracMessage::ZoomOut),
          )),
      )
      .push(
        Row::new()
          .padding(row_pad)
          .spacing(row_space)
          .push(button(
            &mut self.go_left_button,
            "◄",
            Message::Frac(FracMessage::GoLeft),
          ))
          .push(button(
            &mut self.go_up_button,
            "▲",
            Message::Frac(FracMessage::GoUp),
          ))
          .push(button(
            &mut self.go_down_button,
            "▼",
            Message::Frac(FracMessage::GoDown),
          ))
          .push(button(
            &mut self.go_right_button,
            "►",
            Message::Frac(FracMessage::GoRight),
          )),
      );
    Container::new(buttons).into()
  }
}

impl<'a> ImageAdjustmentButtons {
  fn view(&mut self) -> Element<Message> {
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

    let buttons = Row::new()
      .padding(10)
      .spacing(10)
      .align_items(Align::Center)
      .push(button(
        &mut self.pixel_up_button,
        "More Detailed",
        Message::Frac(FracMessage::PixelUp),
      ))
      .push(button(
        &mut self.pixel_down_button,
        "Less Detailed",
        Message::Frac(FracMessage::PixelDown),
      ));
    Container::new(buttons).into()
  }
}

impl<'a> FractalAdjustmentButtons {
  fn view(
    &mut self,
    iteration: IterationStyle,
    iterator: IteratorKind,
    log_increment_size: f64,
  ) -> Element<Message> {
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

    let buttons = Column::new()
      .padding(10)
      .spacing(10)
      .align_items(Align::Center)
      .push(
        Row::new()
          .padding(row_pad)
          .spacing(row_space)
          .push(button(
            &mut self.steps_up_button,
            "More Steps",
            Message::Frac(FracMessage::StepsUp),
          ))
          .push(button(
            &mut self.steps_down_button,
            "Less Steps",
            Message::Frac(FracMessage::StepsDown),
          )),
      )
      .push(
        Row::new()
          .padding(row_pad)
          .spacing(row_space)
          .push(PickList::new(
            &mut self.change_iteration_list,
            definitions::ALL_ITERATIONS,
            Some(iteration),
            |kind| Message::Frac(FracMessage::IterationPicked(kind)),
            )
            .padding(8)
            .text_size(24)
          )
          .push(PickList::new(
            &mut self.change_iterator_list,
            definitions::ALL_ITERATORS,
            Some(iterator),
            |kind| Message::Frac(FracMessage::IteratorPicked(kind)),
          )
          .padding(8)
          .text_size(24)
        ),
      )
      .push(
        Row::new()
          .padding(row_pad)
          .spacing(row_space)
          .push(
            Slider::new(
                // increases go by 1/4 hence all the mess with 4.
                &mut self.increment_slider,
                -124.0..=4.0,
                (log_increment_size*4.) as f32,
                |val| Message::Frac(FracMessage::ChangeIncrementSize(val/4.)),
            )
          )
          .push(
            Text::new(format!(
              "increment: {:.10}", (2 as f64).powf(log_increment_size)))
            .size(24)
          )
      )
      .push(
        Row::new()
          .padding(row_pad)
          .spacing(row_space)
          .push(button(
            &mut self.re_up_button,
            "re+",
            Message::Frac(FracMessage::ReUp(log_increment_size)),
          ))
          .push(button(
            &mut self.re_down_button,
            "re-",
            Message::Frac(FracMessage::ReDown(log_increment_size)),
          ))
          .push(button(
            &mut self.im_up_button,
            "im+",
            Message::Frac(FracMessage::ImUp(log_increment_size)),
          ))
          .push(button(
            &mut self.im_down_button,
            "im-",
            Message::Frac(FracMessage::ImDown(log_increment_size)),
          )),
      );
    Container::new(buttons).into()
  }
}
