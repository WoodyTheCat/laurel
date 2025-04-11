use iced::advanced::{
    self,
    layout::{self, Layout},
    overlay, renderer,
    widget::{self},
};
use iced::{alignment::Alignment, Border, Color, Length, Rectangle, Size};
use iced::{mouse, Vector};
use laurel_common::{Element, LaurelTheme};

pub struct ModalOverlay<'a, 'b, Message, Renderer> {
    content: &'b mut Element<'a, Message, Renderer>,
    tree: &'b mut widget::Tree,
    size: Size,
    on_dismiss: Option<Message>,
}

impl<'a, 'b, Message, Renderer> ModalOverlay<'a, 'b, Message, Renderer> {
    pub fn new(
        content: &'b mut Element<'a, Message, Renderer>,
        tree: &'b mut widget::Tree,
        size: Size,
        on_dismiss: Option<Message>,
    ) -> Self {
        Self {
            content,
            tree,
            size,
            on_dismiss,
        }
    }
}

impl<'a, 'b, Message, Renderer> overlay::Overlay<Message, LaurelTheme, Renderer>
    for ModalOverlay<'a, 'b, Message, Renderer>
where
    Renderer: advanced::Renderer,
    Message: Clone,
{
    fn layout(&mut self, renderer: &Renderer, _bounds: Size) -> layout::Node {
        let limits = layout::Limits::new(Size::ZERO, self.size)
            .width(Length::Fill)
            .height(Length::Fill);

        let mut child = self
            .content
            .as_widget()
            .layout(self.tree, renderer, &limits);

        child.align_mut(Alignment::Center, Alignment::Center, limits.max());

        let node = layout::Node::with_children(self.size, vec![child]); // mut
                                                                        // node.move_to();

        node
    }

    // fn on_event(
    //     &mut self,
    //     event: Event,
    //     layout: Layout<'_>,
    //     cursor: mouse::Cursor,
    //     renderer: &Renderer,
    //     clipboard: &mut dyn Clipboard,
    //     shell: &mut Shell<'_, Message>,
    // ) -> event::Status {
    //     let content_bounds = layout.children().next().unwrap().bounds();
    //
    //     if let Some(message) = self.on_dismiss.as_ref() {
    //         if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = &event {
    //             if !cursor.is_over(content_bounds) {
    //                 shell.publish(message.clone());
    //                 return event::Status::Captured;
    //             }
    //         }
    //     }
    //
    //     self.content.as_widget_mut().on_event(
    //         self.tree,
    //         event,
    //         layout.children().next().unwrap(),
    //         cursor,
    //         renderer,
    //         clipboard,
    //         shell,
    //         &layout.bounds(),
    //     )
    // }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &LaurelTheme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: Border {
                    width: 0.0,
                    color: Color::TRANSPARENT,
                    ..Default::default()
                },
                ..Default::default()
            },
            Color {
                a: 0.80,
                ..Color::BLACK
            },
        );

        self.content.as_widget().draw(
            self.tree,
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor,
            &layout.bounds(),
        );
    }

    fn operate(
        &mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        self.content.as_widget().operate(
            self.tree,
            layout.children().next().unwrap(),
            renderer,
            operation,
        );
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            self.tree,
            layout.children().next().unwrap(),
            cursor,
            viewport,
            renderer,
        )
    }

    fn overlay<'c>(
        &'c mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'c, Message, LaurelTheme, Renderer>> {
        self.content.as_widget_mut().overlay(
            self.tree,
            layout.children().next().unwrap(),
            renderer,
            Vector::ZERO,
        )
    }
}
