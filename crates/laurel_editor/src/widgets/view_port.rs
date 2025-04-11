use iced::{
    advanced::{
        text,
        widget::{tree, Tree},
        Layout, Widget,
    },
    Alignment, Padding, Point,
};
use laurel_common::{Element, LaurelTheme};

use crate::core::window::VirtualWindow;

pub struct ViewPort<'a, Message, Renderer>
where
    Renderer: text::Renderer,
{
    child: Element<'a, Message, Renderer>,
    window: &'a VirtualWindow,
}

impl<'a, Message, Renderer> ViewPort<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer,
{
    pub fn new(
        child: Element<'a, Message, Renderer>,
        _modal: Option<Element<'a, Message, Renderer>>,
        window: &'a VirtualWindow,
    ) -> Self {
        Self { child, window }
    }
}

#[derive(Default)]
struct State {
    // content: String
}

pub trait ViewPortMessage {
    fn view_change(height: f32, width: f32) -> Self;
    fn dismiss_modal() -> Self;
    fn set_textbox_focus(is_focused: bool) -> Self;
}

impl<'a, Message, Renderer> Widget<Message, LaurelTheme, Renderer>
    for ViewPort<'a, Message, Renderer>
where
    Message: 'a + Clone + ViewPortMessage,
    Renderer: text::Renderer,
    Renderer: iced::advanced::Renderer,
    <Renderer as iced::advanced::text::Renderer>::Font: From<iced::Font>,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.child)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.child))
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: iced::Vector,
    ) -> Option<iced::advanced::overlay::Element<'b, Message, LaurelTheme, Renderer>> {
        self.child.as_widget_mut().overlay(
            &mut tree.children[0],
            layout.children().next().unwrap(),
            renderer,
            translation,
        )
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn size(&self) -> iced::Size<iced::Length> {
        self.child.as_widget().size()
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        let limits = limits.width(self.child.as_widget().size().width).loose();

        let mut content = self.child.as_widget().layout(tree, renderer, &limits);
        let padding = Padding::ZERO;
        let size = content.size();
        // let size = limits.pad(padding).resolve(content.size());

        content.move_to_mut(Point::new(padding.left, padding.top));
        content.align_mut(Alignment::Start, Alignment::Start, size);

        iced::advanced::layout::Node::with_children(size.expand(padding), vec![content])
        // self.child.as_widget().layout(renderer, limits)
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced::advanced::widget::Operation,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.child.as_widget().operate(
                &mut tree.children[0],
                layout.children().next().unwrap(),
                renderer,
                operation,
            );
        });
    }

    // fn on_event(
    //     &mut self,
    //     tree: &mut Tree,
    //     event: Event,
    //     layout: Layout<'_>,
    //     cursor: iced::mouse::Cursor,
    //     renderer: &Renderer,
    //     clipboard: &mut dyn iced::advanced::Clipboard,
    //     shell: &mut iced::advanced::Shell<'_, Message>,
    //     viewport: &iced::Rectangle,
    // ) -> iced::event::Status {
    //     let bounds = layout.bounds();
    //     let _state = &mut tree.state;
    //
    //     match event {
    //         Event::Window(window::Event::Resized {
    //             width: _,
    //             height: _,
    //         }) => shell.publish(Message::view_change(bounds.height, bounds.width)),
    //         Event::Window(window::Event::RedrawRequested(_)) => {
    //             // ! This runs a bunch, I'm not sure if this affects performace.
    //             if !self.window.is_same(bounds.width, bounds.height) {
    //                 shell.publish(Message::view_change(bounds.height, bounds.width))
    //             }
    //         }
    //         _ => (),
    //     }
    //
    //     self.child.as_widget_mut().on_event(
    //         &mut tree.children[0],
    //         event,
    //         layout.children().next().unwrap(),
    //         cursor,
    //         renderer,
    //         clipboard,
    //         shell,
    //         viewport,
    //     )
    // }

    fn mouse_interaction(
        &self,
        _state: &iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        _viewport: &iced::Rectangle,
        _renderer: &Renderer,
    ) -> iced::advanced::mouse::Interaction {
        let mouse_is_over = cursor.is_over(layout.bounds());
        if mouse_is_over {
            return iced::mouse::Interaction::Text;
        }
        iced::mouse::Interaction::default()
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &LaurelTheme,
        style: &iced::advanced::renderer::Style,
        layout: Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        self.child.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor,
            viewport,
        )
    }
}

impl<'a, Message, Renderer> From<ViewPort<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: iced::advanced::renderer::Renderer + iced::advanced::text::Renderer + 'a,
    Message: 'a + Clone + ViewPortMessage,
    <Renderer as iced::advanced::text::Renderer>::Font: From<iced::Font>,
{
    fn from(child: ViewPort<'a, Message, Renderer>) -> Self {
        Self::new(child)
    }
}
