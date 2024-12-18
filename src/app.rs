use eframe::egui::{Context, Id, TextEdit, Ui};
use eframe::{egui, Frame};
use eframe::egui::text_edit::TextCursorState;

pub struct App {
    prompt: String,
    output: String,
    evaluated: String,
    cursor: TextCursorState
}

impl Default for App {
    fn default() -> Self {
        Self {
            prompt: String::new(),
            output: String::new(),
            evaluated: String::new(),
            cursor: TextCursorState::default()
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        ctx.set_pixels_per_point(3.0);
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            let res = ui.add_sized([ui.available_width(), 20.0], TextEdit::multiline(&mut self.prompt));
            if let Some(state) = TextEdit::load_state(ctx, res.id) {
                self.cursor = state.cursor;
            }
            ui.add_sized([ui.available_width(), 20.0], TextEdit::singleline(&mut self.output));
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.horizontal(|ui| {
                    self.btn(ui, "1".to_string(), "1".to_string());
                    self.btn(ui, "2".to_string(), "2".to_string());
                    self.btn(ui, "3".to_string(), "3".to_string());
                });
                ui.horizontal(|ui| {
                    self.btn(ui, "4".to_string(), "4".to_string());
                    self.btn(ui, "5".to_string(), "5".to_string());
                    self.btn(ui, "6".to_string(), "6".to_string());
                });
                ui.horizontal(|ui| {
                    self.btn(ui, "7".to_string(), "7".to_string());
                    self.btn(ui, "8".to_string(), "8".to_string());
                    self.btn(ui, "9".to_string(), "9".to_string());
                });
                ui.horizontal(|ui| {
                    self.btn(ui, "0".to_string(), "0".to_string());
                });
            });
            ui.label(" ");
            ui.horizontal(|ui| {
                self.btn(ui, "+".to_string(), "+".to_string());
                self.btn(ui, "-".to_string(), "-".to_string());
            });
            ui.horizontal(|ui| {
                self.btn(ui, "*".to_string(), "*".to_string());
                self.btn(ui, "/".to_string(), "/".to_string());
            });
            ui.horizontal(|ui| {
                self.btn(ui, "(".to_string(), "(".to_string());
                self.btn(ui, ")".to_string(), ")".to_string());
            });
            ui.horizontal(|ui| {
                self.btn(ui, "sin".to_string(), "sin(".to_string());
                self.btn(ui, "cos".to_string(), "cos(".to_string());
                self.btn(ui, "tan".to_string(), "tan(".to_string());
                self.btn(ui, "cot".to_string(), "cot(".to_string());
            });
            ui.horizontal(|ui| {
                self.btn(ui, "log".to_string(), "log(".to_string());
                self.btn(ui, "ln".to_string(), "ln(".to_string());
            });
            ui.horizontal(|ui| {
                self.btn(ui, "sqrt".to_string(), "sqrt(".to_string());
                self.btn(ui, "exp".to_string(), "^".to_string());
            });
            ui.horizontal(|ui| {
                self.btn(ui, "pi".to_string(), "pi".to_string());
                self.btn(ui, "e".to_string(), "e".to_string());
                self.btn(ui, "i".to_string(), "i".to_string());
            });
        });
        // reevaluate if prompt changes
        if self.evaluated != self.prompt {
            self.evaluated = self.prompt.clone();
            self.output = crate::qalculate::eval(self.prompt.clone());
        }
    }

}

impl App {
    fn btn(&mut self, ui: &mut Ui, label: String, expr: String) {
        if ui.button(label).clicked() {
            // TODO: insert expr at cursor position in prompt
            self.prompt.push_str(&expr);
        }
    }
}
