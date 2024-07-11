use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::mouse;
use iced::{Border, Color, Element, Length, Point, Rectangle, Size, Command, Background};
use iced::window;

/// CustomHeader represents a customizable window header widget.
pub struct CustomHeader {
    width: Length,
    height: Length,
    background_color: Background,
    border_bottom: Option<(f32, Color)>,
}

impl CustomHeader {
    /// Creates a new CustomHeader with the specified width and height.
    pub fn new() -> Self {
        Self {
            width: Length::Fill,
            height: Length::Fixed(40.0),
            background_color: Background::Color(Color::WHITE),
            border_bottom: None,
        }
    }

    /// Sets the header width
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the header height
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the background color of the header.
    pub fn background_color(mut self, color: Background) -> Self {
        self.background_color = color;
        self
    }

    /// Adds a bottom border to the header with the specified width and color.
    pub fn border_bottom(mut self, width: f32, color: Color) -> Self {
        self.border_bottom = Some((width, color));
        self
    }
}

pub fn custom_header() -> CustomHeader {
    CustomHeader::new()
}

/// Message enum represents the possible actions that can be triggered by the header.
#[derive(Clone, Debug)]
pub enum Message {
    Minimize,
    Maximize,
    Close,
    Drag(Point),
}

impl<Theme, Renderer> Widget<Message, Theme, Renderer> for CustomHeader
where
    Renderer: renderer::Renderer,
{
    
    /// Defines the size of the widget.
    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    /// Computes the layout of the widget.
    fn layout(
        &self,
        _tree: &mut widget::Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let size = limits.resolve(self.width, self.height, Size::ZERO);
        layout::Node::new(size)
    }

    /// Draws the widget.
    fn draw(
        &self,
        _state: &widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
    
        // Draw background
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border::default(),
                ..renderer::Quad::default()
            },
            self.background_color,
        );
    
        // Draw border bottom if specified
        if let Some((border_width, border_color)) = self.border_bottom {
            let border_bounds = Rectangle {
                x: bounds.x,
                y: bounds.y + bounds.height - border_width,
                width: bounds.width,
                height: border_width,
            };
            renderer.fill_quad(
                renderer::Quad {
                    bounds: border_bounds,
                    border: Border::default(),
                    ..renderer::Quad::default()
                },
                border_color,
            );
        }
    
        // Draw buttons (full height, no radius, no padding)
        let button_width = 40.0; // You can adjust this value as needed
        let buttons = [
            ("Close", Color::from_rgb(1.0, 0.0, 0.0), Message::Close),
            ("Maximize", Color::from_rgb(0.0, 1.0, 0.0), Message::Maximize),
            ("Minimize", Color::from_rgb(1.0, 1.0, 0.0), Message::Minimize),
        ];
    
        for (i, (_, color, _)) in buttons.iter().enumerate() {
            let button_bounds = Rectangle {
                x: bounds.x + bounds.width - button_width * (i + 1) as f32,
                y: bounds.y,
                width: button_width,
                height: bounds.height,
            };
    
            renderer.fill_quad(
                renderer::Quad {
                    bounds: button_bounds,
                    border: Border::default(),
                    ..renderer::Quad::default()
                },
                *color,
            );
        }
    }
    
    fn on_event(
        &mut self,
        _state: &mut widget::Tree,
        event: iced::Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        _viewport: &Rectangle
    ) -> iced::event::Status {
        match event {
            iced::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if cursor.is_over(layout.bounds()) {
                    // Check if any button was clicked
                    let button_width = 40.0; // Same as in draw function
                    let buttons = [
                        ("Close", Color::from_rgb(1.0, 0.0, 0.0), Message::Close),
                        ("Maximize", Color::from_rgb(0.0, 1.0, 0.0), Message::Maximize),
                        ("Minimize", Color::from_rgb(1.0, 1.0, 0.0), Message::Minimize),
                    ];
    
                    for (i, (_, _, message)) in buttons.iter().enumerate() {
                        let button_bounds = Rectangle {
                            x: layout.bounds().x + layout.bounds().width - button_width * (i + 1) as f32,
                            y: layout.bounds().y,
                            width: button_width,
                            height: layout.bounds().height,
                        };
    
                        if cursor.is_over(button_bounds) {
                            shell.publish(message.clone());
                            return iced::event::Status::Captured;
                        }
                    }
    
                    // If not clicked on a button, start dragging
                    shell.publish(Message::Drag(cursor.position().unwrap_or_default()));
                    iced::event::Status::Captured
                } else {
                    iced::event::Status::Ignored
                }
            }
            _ => iced::event::Status::Ignored,
        }
    }
}

pub fn handle_header_message(message: Message) -> Command<Message> {
    match message {
        Message::Minimize => window::minimize(window::Id::MAIN, true),
        Message::Maximize => window::toggle_maximize(window::Id::MAIN),
        Message::Close => window::close(window::Id::MAIN),
        Message::Drag(_point) => window::drag(window::Id::MAIN),
    }
}

/// Implements the From trait to convert CustomHeader into an Element.
impl<'a, Message, Theme, Renderer> From<CustomHeader>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Message: 'a,
    CustomHeader: Widget<Message, Theme, Renderer>
{
    fn from(header: CustomHeader) -> Self {
        Self::new(header)
    }
}