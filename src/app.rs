use eframe::egui::{Context, TextEdit, Ui};
use eframe::{egui, Frame};
use eframe::egui::WidgetType::TextEdit;

pub struct App {
    prompt: String,
    output: String,
    evaluated: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            prompt: String::new(),
            output: String::new(),
            evaluated: String::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            // TODO:  get cursor position from multiline
            TextEdit::multiline(&mut self.prompt).show(ui);
            TextEdit::singleline(&mut self.output).show(ui);
        });
        egui::CentralPanel::default().show(ctx, |ui| {

        });
        // reevaluate if prompt changes
        if self.evaluated != self.prompt {
            self.evaluated = self.prompt.clone();
            self.output = crate::qalculate::eval(self.prompt.clone());
        }
    }

    fn btn(&mut self, ui: &mut Ui, label: String, expr: String) {
        if ui.button(label).clicked() {
            // TODO: insert expr at cursor position in prompt
        }
    }
}
