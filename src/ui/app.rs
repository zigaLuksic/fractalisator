//==============================================================================
// Open crates and libraries
//==============================================================================
use image as imagetool;

use iced::{button, Button};
use iced::{image, Image, Text};
use iced::{pick_list, PickList};
use iced::{slider, Slider};
use iced::{Align, Column, Container, Element, Row, Sandbox};
use iced::{Background, Color, HorizontalAlignment, Length, Vector};

use fractal::color;
use fractal::definitions;
use fractal::definitions::FracArgs;
use fractal::definitions::{Gradient, GradientPreset, IterationStyle, IteratorKind};
use fractal::draw;

use ui::state::*;

//==============================================================================
// Application layout definitions
//==============================================================================
#[derive(Copy, Clone)]
pub enum Layout {
  FracOptions,
  ColorOptions,
}

//==============================================================================
// States of application parts
//==============================================================================
pub struct MainWindow {
  frac_state: FractalState,
  image_state: ImageState,
  displayed_image: Vec<u8>,
  app_state: AppState,
  frac_layout: FracLayout,
  color_layout: ColorLayout,
  save_button: button::State,
  to_frac_layout_button: button::State,
  to_color_layout_button: button::State,
}

impl Default for MainWindow {
  fn default() -> Self {
    let default_frac_state = FractalState::default();
    let default_image_state = ImageState::default();
    let pix_size = default_frac_state.args.field.pixel_size;
    let displayed_image = color::resize_fractal_image(&default_image_state.image.clone(), pix_size, 1000);
    MainWindow {
      frac_state: default_frac_state,
      image_state: default_image_state,
      displayed_image: displayed_image,
      app_state: AppState::default(),
      frac_layout: FracLayout::default(),
      color_layout: ColorLayout::default(),
      save_button: button::State::default(),
      to_frac_layout_button: button::State::default(),
      to_color_layout_button: button::State::default(),
    }
  }
}

pub struct AppState {
  layout: Layout,
  log_increment_size: f64,
}

impl Default for AppState {
  fn default() -> Self {
    AppState {
      layout: Layout::FracOptions,
      log_increment_size: -6.,
    }
  }
}

#[derive(Default)]
pub struct FracLayout {
  navigation_buttons: NavigationButtons,
  fractal_adjustment_buttons: FractalAdjustmentButtons,
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

#[derive(Default)]
pub struct ColorLayout {
  steps_up_button: button::State,
  steps_down_button: button::State,
  image_adjustment_buttons: ImageAdjustmentButtons,
  change_color_button: button::State, // temporary
}

//==============================================================================
// Messages
//==============================================================================

#[derive(Debug, Clone, Copy)]
pub enum FracMsg {
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
  IterationPicked(IterationStyle),
  IteratorPicked(IteratorKind),
  ReUp(f64),
  ReDown(f64),
  ImUp(f64),
  ImDown(f64),
}

#[derive(Debug, Clone, Copy)]
pub enum ImgMsg {
  ChangeColor,
}

#[derive(Debug, Clone, Copy)]
pub enum AppMsg {
  SaveImage,
  ChangeToFracLayout,
  ChangeToColorLayout,
  ChangeIncrementSize(f32),
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
  Frac(FracMsg),
  Image(ImgMsg),
  App(AppMsg),
}

//==============================================================================
// Auxiliary Definitions
//==============================================================================

fn button<'a, Message: Clone>(state: &'a mut button::State, label: &str) -> Button<'a, Message> {
  Button::new(
    state,
    Text::new(label)
      .width(Length::Fill)
      .horizontal_alignment(HorizontalAlignment::Center)
      .size(24),
  )
  .padding(8)
  .width(Length::Fill)
}

pub enum ButtonStyle {
  Primary,
  // Secondary,
}

impl button::StyleSheet for ButtonStyle {
  fn active(&self) -> button::Style {
    button::Style {
      background: Some(Background::Color(match self {
        ButtonStyle::Primary => Color::from_rgb(0.11, 0.42, 0.87),
        // Button::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
      })),
      border_radius: 12.0,
      shadow_offset: Vector::new(1.0, 1.0),
      text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
      ..button::Style::default()
    }
  }

  fn hovered(&self) -> button::Style {
    button::Style {
      text_color: Color::WHITE,
      shadow_offset: Vector::new(1.0, 2.0),
      ..self.active()
    }
  }
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
    let image = color::color_fractal(frac, self.frac_state.args.steps, gradient);
    self.image_state.image = image.clone();
    let pix_size = self.frac_state.args.field.pixel_size;
    self.displayed_image = color::resize_fractal_image(&image, pix_size, 1000);
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
          FracMsg::ZoomIn => {
            self.frac_state.args.field.radius *= 0.8;
          }
          FracMsg::ZoomOut => {
            self.frac_state.args.field.radius *= 1.2;
          }
          FracMsg::GoLeft => {
            let dx = self.frac_state.args.field.radius * 0.2;
            self.frac_state.args.field.center_re -= dx
          }
          FracMsg::GoRight => {
            let dx = self.frac_state.args.field.radius * 0.2;
            self.frac_state.args.field.center_re += dx
          }
          FracMsg::GoUp => {
            let dy = self.frac_state.args.field.radius * 0.2;
            self.frac_state.args.field.center_im -= dy
          }
          FracMsg::GoDown => {
            let dy = self.frac_state.args.field.radius * 0.2;
            self.frac_state.args.field.center_im += dy
          }
          FracMsg::PixelUp => self.frac_state.args.field.pixel_size *= 2,
          FracMsg::PixelDown => {
            let pix_size = self.frac_state.args.field.pixel_size;
            let new_size = std::cmp::max(pix_size / 2, 128);
            self.frac_state.args.field.pixel_size = new_size
          }
          FracMsg::StepsUp => self.frac_state.args.steps += 20,
          FracMsg::StepsDown => {
            let steps = std::cmp::max(self.frac_state.args.steps, 21);
            let new_steps = steps - 20;
            self.frac_state.args.steps = new_steps;
          }
          FracMsg::IterationPicked(kind) => self.frac_state.args.iteration_style = kind,
          FracMsg::IteratorPicked(kind) => self.frac_state.args.iterator_kind = kind,
          FracMsg::ReUp(log_incr) => self.frac_state.args.c_re += (2.0 as f64).powf(log_incr),
          FracMsg::ReDown(log_incr) => self.frac_state.args.c_re -= (2.0 as f64).powf(log_incr),
          FracMsg::ImUp(log_incr) => self.frac_state.args.c_im += (2.0 as f64).powf(log_incr),
          FracMsg::ImDown(log_incr) => self.frac_state.args.c_im -= (2.0 as f64).powf(log_incr),
        };
        // Since the fractal arguments changed, we have to update the raw and
        // colored fractal
        self.redraw_frac(true)
      }
      Message::Image(im) => {
        match im {
          ImgMsg::ChangeColor => match self.image_state.current_preset {
            GradientPreset::Azul => {
              self.image_state.args.gradient = Gradient::svarog();
              self.image_state.current_preset = GradientPreset::Svarog
            }
            GradientPreset::Svarog => {
              self.image_state.args.gradient = Gradient::emperor();
              self.image_state.current_preset = GradientPreset::Emperor
            }
            GradientPreset::Emperor => {
              self.image_state.args.gradient = Gradient::gaia();
              self.image_state.current_preset = GradientPreset::Gaia
            }
            GradientPreset::Gaia => {
              self.image_state.args.gradient = Gradient::azul();
              self.image_state.current_preset = GradientPreset::Azul
            }
          },
        };
        self.redraw_only_image()
      }
      Message::App(ap) => match ap {
        AppMsg::ChangeIncrementSize(val) => self.app_state.log_increment_size = val as f64,
        AppMsg::ChangeToColorLayout => self.app_state.layout = Layout::ColorOptions,
        AppMsg::ChangeToFracLayout => self.app_state.layout = Layout::FracOptions,
        AppMsg::SaveImage => {
          imagetool::ImageBuffer::<imagetool::Rgba<u8>, Vec<u8>>::from_vec(
            self.frac_state.args.field.pixel_size as u32,
            self.frac_state.args.field.pixel_size as u32,
            color::bgra_to_rgba(&self.image_state.image),
          )
          .unwrap()
          .save("fractal_image.png")
          .unwrap()
        },
      },
    }
  }

  fn view(&mut self) -> Element<Message> {
    let image_handle = image::Handle::from_pixels(1000, 1000, self.displayed_image.clone());
    let image = Image::new(image_handle)
      .width(Length::Units(1000))
      .height(Length::Units(1000));
    let layout = match self.app_state.layout {
      Layout::FracOptions => self
        .frac_layout
        .view(self.frac_state.args, self.app_state.log_increment_size)
        .width(Length::Units(500)),
      Layout::ColorOptions => self
        .color_layout
        .view(self.frac_state.args)
        .width(Length::Units(500)),
    };
    let layout_buttons = Row::new()
      .padding(10)
      .spacing(10)
      .width(Length::Units(500))
      .push(
        button(&mut self.to_frac_layout_button, "Fractal Options")
          .on_press(Message::App(AppMsg::ChangeToFracLayout))
          .style(ButtonStyle::Primary),
      )
      .push(
        button(&mut self.to_color_layout_button, "Color Options")
          .on_press(Message::App(AppMsg::ChangeToColorLayout))
          .style(ButtonStyle::Primary),
      );
    let save_button = Row::new()
      .padding(10)
      .spacing(10)
      .width(Length::Units(500))
      .push(
        button(&mut self.save_button, "Save Fractal")
          .on_press(Message::App(AppMsg::SaveImage))
          .style(ButtonStyle::Primary),
      );
    Row::new()
      .padding(10)
      .align_items(Align::Center)
      .push(
        Column::new()
          .align_items(Align::Start)
          .padding(10)
          .spacing(10)
          .push(layout_buttons)
          .push(layout)
          .push(save_button),
      )
      .push(Column::new().padding(10).spacing(10).push(image))
      .into()
  }
}

// -----------------------------------------------------------------------------
// Frac Layout
// -----------------------------------------------------------------------------

impl<'a> FracLayout {
  fn view(&'a mut self, frac_args: FracArgs, log_increment_size: f64) -> Column<'a, Message> {
    let row_space = 10;
    let row_pad = 10;

    Column::new()
      .padding(10)
      .spacing(10)
      .push(self.navigation_buttons.view())
      .push(self.fractal_adjustment_buttons.view(
        frac_args.iteration_style,
        frac_args.iterator_kind,
        log_increment_size,
      ))
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

    let buttons = Column::new()
      .padding(10)
      .spacing(10)
      .align_items(Align::Center)
      .push(
        Row::new()
          .padding(row_pad)
          .spacing(row_space)
          .push(button(&mut self.zoom_in_button, "+").on_press(Message::Frac(FracMsg::ZoomIn)))
          .push(button(&mut self.zoom_out_button, "-").on_press(Message::Frac(FracMsg::ZoomOut))),
      )
      .push(
        Row::new()
          .padding(row_pad)
          .spacing(row_space)
          .push(button(&mut self.go_left_button, "◄").on_press(Message::Frac(FracMsg::GoLeft)))
          .push(button(&mut self.go_up_button, "▲").on_press(Message::Frac(FracMsg::GoUp)))
          .push(button(&mut self.go_down_button, "▼").on_press(Message::Frac(FracMsg::GoDown)))
          .push(button(&mut self.go_right_button, "►").on_press(Message::Frac(FracMsg::GoRight))),
      );
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

    let buttons = Column::new()
      .padding(10)
      .spacing(10)
      .align_items(Align::Center)
      .push(
        Row::new()
          .padding(10)
          .spacing(10)
          .push(
            PickList::new(
              &mut self.change_iteration_list,
              definitions::ALL_ITERATIONS,
              Some(iteration),
              |kind| Message::Frac(FracMsg::IterationPicked(kind)),
            )
            .padding(8)
            .text_size(24),
          )
          .push(
            PickList::new(
              &mut self.change_iterator_list,
              definitions::ALL_ITERATORS,
              Some(iterator),
              |kind| Message::Frac(FracMsg::IteratorPicked(kind)),
            )
            .padding(8)
            .text_size(24),
          ),
      )
      .push(
        Row::new()
          .padding(row_pad)
          .spacing(row_space)
          .push(Slider::new(
            // increases go by 1/4 hence all the mess with 4.
            &mut self.increment_slider,
            -124.0..=4.0,
            (log_increment_size * 4.) as f32,
            |val| Message::App(AppMsg::ChangeIncrementSize(val / 4.)),
          ))
          .push(
            Text::new(format!(
              "increment: {:.10}",
              (2 as f64).powf(log_increment_size)
            ))
            .size(24),
          ),
      )
      .push(
        Row::new()
          .padding(row_pad)
          .spacing(row_space)
          .push(
            button(&mut self.re_up_button, "re+")
              .on_press(Message::Frac(FracMsg::ReUp(log_increment_size))),
          )
          .push(
            button(&mut self.re_down_button, "re-")
              .on_press(Message::Frac(FracMsg::ReDown(log_increment_size))),
          )
          .push(
            button(&mut self.im_up_button, "im+")
              .on_press(Message::Frac(FracMsg::ImUp(log_increment_size))),
          )
          .push(
            button(&mut self.im_down_button, "im-")
              .on_press(Message::Frac(FracMsg::ImDown(log_increment_size))),
          ),
      );
    Container::new(buttons).into()
  }
}

// -----------------------------------------------------------------------------
// Color Layout
// -----------------------------------------------------------------------------

impl<'a> ColorLayout {
  fn view(&'a mut self, frac_args: FracArgs) -> Column<'a, Message> {
    let row_space = 10;
    let row_pad = 10;

    Column::new()
      .padding(10)
      .spacing(10)
      .push(self.image_adjustment_buttons.view())
      .push(
        Row::new()
          .padding(row_pad)
          .spacing(row_space)
          .push(
            button(&mut self.steps_up_button, "More Steps")
              .on_press(Message::Frac(FracMsg::StepsUp)),
          )
          .push(
            button(&mut self.steps_down_button, "Less Steps")
              .on_press(Message::Frac(FracMsg::StepsDown)),
          ),
      )
      .push(
        Row::new().padding(row_pad).spacing(row_space).push(
          button(&mut self.change_color_button, "Change Color")
            .on_press(Message::Image(ImgMsg::ChangeColor)),
        ),
      )
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

impl<'a> ImageAdjustmentButtons {
  fn view(&mut self) -> Element<Message> {
    let buttons = Row::new()
      .padding(10)
      .spacing(10)
      .align_items(Align::Center)
      .push(
        button(&mut self.pixel_up_button, "More Detailed")
          .on_press(Message::Frac(FracMsg::PixelUp)),
      )
      .push(
        button(&mut self.pixel_down_button, "Less Detailed")
          .on_press(Message::Frac(FracMsg::PixelDown)),
      );
    Container::new(buttons).into()
  }
}
