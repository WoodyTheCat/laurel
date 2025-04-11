use iced::{
    advanced::{
        text,
        widget::{tree, Tree},
        Layout, Widget,
    },
    overlay, Alignment, Length, Padding, Point, Size, Vector,
};
use laurel_common::{Element, LaurelTheme};

use super::{modal::modal_overlay::ModalOverlay, view_port::ViewPortMessage};

pub struct MainView<'a, Message, Renderer>
where
    Renderer: text::Renderer,
{
    child: Element<'a, Message, Renderer>,
    modal: Option<Element<'a, Message, Renderer>>,
}

impl<'a, Message, Renderer> MainView<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer,
{
    pub fn new(
        child: Element<'a, Message, Renderer>,
        modal: Option<Element<'a, Message, Renderer>>,
    ) -> Self {
        Self { child, modal }
    }
}

#[derive(Default)]
struct State {
    // content: String
}

impl<'a, Message, Renderer> Widget<Message, LaurelTheme, Renderer>
    for MainView<'a, Message, Renderer>
where
    Message: 'a + Clone + ViewPortMessage,
    Renderer: text::Renderer,
    Renderer: iced::advanced::Renderer,
    <Renderer as iced::advanced::text::Renderer>::Font: From<iced::Font>,
{
    fn children(&self) -> Vec<Tree> {
        if let Some(value) = self.modal.as_ref() {
            return vec![Tree::new(&self.child), Tree::new(value)];
        }
        vec![Tree::new(&self.child)]
    }

    fn diff(&self, tree: &mut Tree) {
        if let Some(value) = self.modal.as_ref() {
            return tree.diff_children(&[&self.child, value]);
        }
        tree.diff_children(std::slice::from_ref(&self.child))
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<iced::advanced::overlay::Element<'b, Message, LaurelTheme, Renderer>> {
        if self.modal.is_none() {
            return self.child.as_widget_mut().overlay(
                &mut tree.children[0],
                layout.children().next().unwrap(),
                renderer,
                translation, // + layout.position()
            );
        }
        if let Some(modal) = self.modal.as_mut() {
            if let Some(child) = tree.children.get_mut(1) {
                let overlay = overlay::Element::new(Box::new(ModalOverlay::new(
                    modal,
                    child,
                    layout.bounds().size(),
                    Some(Message::dismiss_modal()),
                )));
                return Some(overlay);
            }
        }
        None
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        let limits = limits.width(self.child.as_widget().size().width).loose();

        let mut content = self
            .child
            .as_widget()
            .layout(&mut tree.children[0], renderer, &limits);
        let padding = Padding::default().top(20.0);
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

impl<'a, Message, Renderer> From<MainView<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: iced::advanced::renderer::Renderer + iced::advanced::text::Renderer + 'a,
    Message: 'a + Clone + ViewPortMessage,
    <Renderer as iced::advanced::text::Renderer>::Font: From<iced::Font>,
{
    fn from(child: MainView<'a, Message, Renderer>) -> Self {
        Self::new(child)
    }
}
