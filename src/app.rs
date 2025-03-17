use egui::TextStyle;
use i18n_embed::{
    LanguageLoader,
    fluent::{FluentLanguageLoader, fluent_language_loader},
};
use i18n_embed_fl::fl;
use log::info;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Localizations;

#[derive(Default, PartialEq, Eq)]
enum Action {
    #[default]
    Action,
    BonusAction,
    Reaction,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum BonusSign {
    Additive,
    #[default]
    Multiplicative,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
enum Dice {
    #[default]
    None = 1,
    D4 = 4,
    D6 = 6,
    D8 = 8,
    D10 = 10,
    D12 = 12,
    D20 = 20,
    D100 = 100,
}

pub struct Calculator {
    localization_loader: FluentLanguageLoader,
    level: u32,
    action_variant: Action,
    verbal: bool,
    somatic: bool,
    material: u32,
    material_sign: BonusSign,
    bonus: u32,
    bonus_sign: BonusSign,
    dice_amount: u32,
    dice_type: Dice,
    result: String,
}

impl Default for Calculator {
    fn default() -> Self {
        let loader: FluentLanguageLoader = fluent_language_loader!();
        loader
            .load_languages(&Localizations, &[loader.fallback_language().clone()])
            .unwrap();

        info!("{:?}", loader.available_languages(&Localizations));

        Self {
            localization_loader: loader,
            level: 1,
            action_variant: Default::default(),
            verbal: true,
            somatic: true,
            material: 1,
            material_sign: Default::default(),
            bonus: 1,
            bonus_sign: Default::default(),
            dice_amount: 0,
            dice_type: Default::default(),
            result: "".to_owned(),
        }
    }
}

impl Calculator {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for Calculator {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let mut result: f64 = 1.0;

        result *= if self.level == 0 {
            0.5
        } else {
            self.level as f64
        };
        result *= if self.verbal { 20.0 } else { 1.0 };
        result *= if self.somatic { 30.0 } else { 1.0 };
        result *= match self.action_variant {
            Action::Action => 6.0,
            Action::BonusAction => 2.0,
            Action::Reaction => 0.5,
        };
        result *= match self.dice_type {
            Dice::None => 1.0,
            _ => self.dice_amount as f64,
        };

        result /= match self.dice_type {
            Dice::None => 1.0,
            dice => dice as u32 as f64,
        };

        result = match self.material_sign {
            BonusSign::Additive => result + self.material as f64,
            BonusSign::Multiplicative => result * self.material as f64,
        };

        result = match self.bonus_sign {
            BonusSign::Additive => result + self.bonus as f64,
            BonusSign::Multiplicative => result * self.bonus as f64,
        };

        self.result = format!("{:.0}", result);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button(fl!(self.localization_loader, "file"), |ui| {
                    ui.menu_button(fl!(self.localization_loader, "language"), |ui| {
                        if ui.button("English").clicked() {
                            i18n_embed::select(
                                &self.localization_loader,
                                &Localizations,
                                &["en-US".parse().unwrap()],
                            );
                            ui.close_menu();
                        }
                        if ui.button("Русский").clicked() {
                            i18n_embed::select(
                                &self.localization_loader,
                                &Localizations,
                                &["ru-RU".parse().unwrap()],
                            );
                            ui.close_menu();
                        }
                    });
                    if ui.button(fl!(self.localization_loader, "quit")).clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        self.localization_loader
                            .current_language()
                            .language
                            .as_str(),
                    );
                    egui::warn_if_debug_build(ui);
                    ui.label(env!("CARGO_PKG_VERSION"));
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
                |ui| {
                    ui.horizontal(|ui| {
                        ui.label(fl!(self.localization_loader, "spell-level"));
                        ui.add_space(ui.available_width() - 48.0);
                        ui.add(egui::DragValue::new(&mut self.level).range(0..=11));
                    });
                    ui.separator();
                    ui.vertical(|ui| {
                        ui.label(fl!(self.localization_loader, "action-type"));
                        ui.horizontal(|ui| {
                            ui.selectable_value(
                                &mut self.action_variant,
                                Action::Action,
                                fl!(self.localization_loader, "action"),
                            );
                            ui.selectable_value(
                                &mut self.action_variant,
                                Action::BonusAction,
                                fl!(self.localization_loader, "bonus-action"),
                            );
                            ui.selectable_value(
                                &mut self.action_variant,
                                Action::Reaction,
                                fl!(self.localization_loader, "reaction"),
                            );
                        });
                    });
                    ui.separator();
                    ui.vertical(|ui| {
                        ui.label(fl!(self.localization_loader, "components"));
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut self.verbal, fl!(self.localization_loader, "verbal"));
                            ui.checkbox(
                                &mut self.somatic,
                                fl!(self.localization_loader, "somatic"),
                            );
                        });
                    });
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label(fl!(self.localization_loader, "material-component"));
                        ui.add_space(16.0);

                        ui.selectable_value(&mut self.material_sign, BonusSign::Additive, "+");
                        ui.selectable_value(
                            &mut self.material_sign,
                            BonusSign::Multiplicative,
                            "*",
                        );

                        ui.add_space(22.0);
                        ui.add(egui::DragValue::new(&mut self.material));
                    });
                    ui.separator();
                    ui.vertical(|ui| {
                        ui.label(fl!(self.localization_loader, "dice"));
                        ui.horizontal(|ui| {
                            ui.label(fl!(self.localization_loader, "amount"));
                            ui.add(egui::DragValue::new(&mut self.dice_amount));

                            ui.separator();

                            egui::ComboBox::from_label(fl!(self.localization_loader, "type"))
                                .selected_text(format!("{:?}", self.dice_type))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.dice_type, Dice::None, "None");
                                    ui.selectable_value(&mut self.dice_type, Dice::D4, "D4");
                                    ui.selectable_value(&mut self.dice_type, Dice::D6, "D6");
                                    ui.selectable_value(&mut self.dice_type, Dice::D8, "D8");
                                    ui.selectable_value(&mut self.dice_type, Dice::D10, "D10");
                                    ui.selectable_value(&mut self.dice_type, Dice::D12, "D12");
                                    ui.selectable_value(&mut self.dice_type, Dice::D20, "D20");
                                    ui.selectable_value(&mut self.dice_type, Dice::D100, "D100");
                                });
                        });
                    });
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label(fl!(self.localization_loader, "additional-bonus"));
                        ui.add_space(38.0);

                        ui.selectable_value(&mut self.bonus_sign, BonusSign::Additive, "+");
                        ui.selectable_value(&mut self.bonus_sign, BonusSign::Multiplicative, "*");

                        ui.add_space(22.0);
                        ui.add(egui::DragValue::new(&mut self.bonus));
                    });
                    ui.separator();

                    ui.label(fl!(self.localization_loader, "result"));

                    ui.add_space(4.0);

                    ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::singleline(&mut self.result)
                            .font(TextStyle::Monospace)
                            .horizontal_align(egui::Align::Center)
                            .vertical_align(egui::Align::Center),
                    )
                },
            );
        });
    }
}
