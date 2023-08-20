#[derive(Debug, Clone, Default)]
pub struct AboutWindow {}

impl AboutWindow {
    fn name(&self) -> &'static str {
        "About DaV"
    }

    pub fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .resizable(false)
            .open(open)
            .show(ctx, |ui| {
                self.ui(ui);
            });
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        use egui::special_emojis::{OS_APPLE, OS_LINUX, OS_WINDOWS};

        ui.heading("Aeon");
        ui.label(format!(
            "egui is an immediate mode GUI library written in Rust. egui runs both on the web and natively on {}{}{}. \
            On the web it is compiled to WebAssembly and rendered with WebGL.{}",
            OS_APPLE, OS_LINUX, OS_WINDOWS,
            if cfg!(target_arch = "wasm32") {
                " Everything you see is rendered as textured triangles. There is no DOM, HTML, JS or CSS. Just Rust."
            } else {""}
        ));
        ui.label("egui is designed to be easy to use, portable, and fast.");

        ui.add_space(12.0); // ui.separator();
        ui.heading("Immediate mode");
        about_immediate_mode(ui);
    }
}

fn about_immediate_mode(ui: &mut egui::Ui) {
    ui.style_mut().spacing.interact_size.y = 0.0; // hack to make `horizontal_wrapped` work better with text.

    ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label("Immediate mode is a GUI paradigm that lets you create a GUI with less code and simpler control flow. For example, this is how you create a ");
            let _ = ui.small_button("button");
            ui.label(" in egui:");
        });

    ui.add_space(8.0);

    ui.label("Note how there are no callbacks or messages, and no button state to store.");

    ui.label("Immediate mode has its roots in gaming, where everything on the screen is painted at the display refresh rate, i.e. at 60+ frames per second. \
        In immediate mode GUIs, the entire interface is laid out and painted at the same high rate. \
        This makes immediate mode GUIs especially well suited for highly interactive applications.");

    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("More about immediate mode ");
        ui.hyperlink_to("here", "https://github.com/emilk/egui#why-immediate-mode");
        ui.label(".");
    });
}
