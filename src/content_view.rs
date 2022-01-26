use cacao::{
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
        description.set_text("Quickly press CapsLock or Fn to activate keyboard layout with the same ordinal");
        description.set_text_alignment(cacao::text::TextAlign::Justified);

        let instructions = Label::new();
        instructions.set_font(&Font::system(10.));
        instructions.set_text("In Preferences -> Keyboard set 'Press 🌐' to 'Do Nothing' ");
        instructions.set_text_alignment(cacao::text::TextAlign::Justified);
        let instructions2 = Label::new();
        instructions2.set_font(&Font::system(10.));
        instructions2
            .set_text("In Preferences -> Keyboard -> Modifier Keys... set Caps Lock to 🌐 Globe");
        instructions2.set_text_alignment(cacao::text::TextAlign::Justified);

        view.add_subview(&title);
        view.add_subview(&description);
        view.add_subview(&instructions);
        view.add_subview(&instructions2);

        LayoutConstraint::activate(&[
            title.top.constraint_equal_to(&view.top).offset(PADDING * 4.),
            title.left.constraint_equal_to(&view.left).offset(PADDING),
            title
                .right
                .constraint_equal_to(&view.right)
                .offset(-PADDING),
            description
                .top
                .constraint_equal_to(&title.bottom)
                .offset(PADDING),
            description
                .left
                .constraint_equal_to(&view.left)
                .offset(PADDING),
            description
                .right
                .constraint_equal_to(&view.right)
                .offset(-PADDING),
            instructions
                .top
                .constraint_equal_to(&description.bottom)
                .offset(PADDING),
            instructions
                .left
                .constraint_equal_to(&view.left)
                .offset(PADDING),
            instructions
                .right
                .constraint_equal_to(&view.right)
                .offset(-PADDING),
            instructions2
                .top
                .constraint_equal_to(&instructions.bottom),
            instructions2
                .left
                .constraint_equal_to(&view.left)
                .offset(PADDING),
            instructions2
                .right
                .constraint_equal_to(&view.right)
                .offset(-PADDING),
        ]);
    }
}
