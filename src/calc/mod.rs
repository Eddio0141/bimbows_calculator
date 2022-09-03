pub mod value_handler;

use eframe::*;
use egui::*;
use serde::{Deserialize, Serialize};

use self::value_handler::ValueHandler;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct CalcApp {
    #[serde(skip)]
    pub value_handler: ValueHandler,
}

impl App for CalcApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.input_handler(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            self.value_display(ui);
        });
    }
}

impl CalcApp {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }

    fn value_display(&self, ui: &mut Ui) {
        let text =
            RichText::new(self.value_handler.main_display()).background_color(Color32::WHITE);
        let text = Label::new(text);

        ui.add_sized([ui.available_size().x, 40.], text);
    }

    fn input_handler(&mut self, ctx: &Context) {
        if ctx.input().key_pressed(Key::Escape) {
            self.clear_all();
        } else if ctx.input().key_pressed(Key::Num0) {
            self.append_digit(0);
        } else if ctx.input().key_pressed(Key::Num1) {
            self.append_digit(1);
        } else if ctx.input().key_pressed(Key::Num2) {
            self.append_digit(2);
        } else if ctx.input().key_pressed(Key::Num3) {
            self.append_digit(3);
        } else if ctx.input().key_pressed(Key::Num4) {
            self.append_digit(4);
        } else if ctx.input().key_pressed(Key::Num5) {
            self.append_digit(5);
        } else if ctx.input().key_pressed(Key::Num6) {
            self.append_digit(6);
        } else if ctx.input().key_pressed(Key::Num7) {
            self.append_digit(7);
        } else if ctx.input().key_pressed(Key::Num8) {
            self.append_digit(8);
        } else if ctx.input().key_pressed(Key::Num9) {
            self.append_digit(9);
        }
    }

    fn append_digit(&mut self, digit: u8) {
        self.value_handler.append_digit(digit);
    }

    fn clear_all(&mut self) {
        self.value_handler.clear_all();
    }
}
