use cacao::{
    color::Color,
    layout::{Layout, LayoutConstraint},
    text::{Font, Label},
    view::{View, ViewDelegate},
};

#[derive(Default)]
pub struct ContentView;

const PADDING: f32 = 10.;

impl ViewDelegate for ContentView {
    const NAME: &'static str = "Lightswitch Keyboard Layout switcher";

    fn did_load(&mut self, view: View) {
        let title = Label::new();
        title.set_font(&Font::system(15.));
        title.set_text("Lightswitch keyboard layout switcher");
        title.set_text_alignment(cacao::text::TextAlign::Center);

        let description = Label::new();
        description.set_font(&Font::system(10.));
        description.set_text("Press CapsLock or Fn certain number of times to activate keyboard layout with the same ordinal");
        description.set_text_alignment(cacao::text::TextAlign::Center);

        view.add_subview(&title);
        view.add_subview(&description);

        LayoutConstraint::activate(&[
            title.top.constraint_equal_to(&view.center_y).offset(-30.),
            title.left.constraint_equal_to(&view.left).offset(PADDING),
            title
                .right
                .constraint_equal_to(&view.right)
                .offset(-PADDING),
            description
                .top
                .constraint_equal_to(&title.bottom)
                .offset(20.),
            description
                .left
                .constraint_equal_to(&view.left)
                .offset(PADDING),
            description
                .right
                .constraint_equal_to(&view.right)
                .offset(-PADDING),
        ]);
    }
}
