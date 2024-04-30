use gtk::prelude::*;

fn build_ui(app: &gtk::Application) {
    let glade_src = include_str!("layout.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let window: gtk::Window = builder.object("applicationwindow1").unwrap();

    window.set_application(Some(app));

    // Inputs
    let message_input: gtk::Entry = builder.object("message_input").unwrap();

    // Submit button
    let button: gtk::Button = builder.object("generate_btn").unwrap();

    // Outputs
    let is_dead_switch: gtk::Switch = builder.object("is_dead_switch").unwrap();

    let message_output: gtk::Label = builder.object("message_output").unwrap();

    let image_output: gtk::Image = builder.object("image_output").unwrap();
    let image_output_clone = image_output.clone(); // low-cost clone

    button.connect_clicked(move |_| {
        message_output.set_text(&format!("{}\n \\\n \\", message_input.text().as_str()));
        image_output_clone.show();
        let is_dead = is_dead_switch.is_active();

        if is_dead {
            image_output_clone.set_from_file(Some("./images/cat_dead.png"))
        } else {
            image_output_clone.set_from_file(Some("./images/cat.png"))
        }
        image_output_clone.show();
    });

    window.show_all();
    image_output.hide();
}

fn main() {
    let application = gtk::Application::new(Some("com.catsay-gui-glade"), Default::default());

    application.connect_activate(build_ui);

    application.run();
}
