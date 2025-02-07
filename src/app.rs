use egui::TextStyle;

#[derive(Default, PartialEq, Eq)]
enum Action {
    #[default]
    Action,
    BonusAction,
    Reaction,
}

#[derive(Default, PartialEq, Eq)]
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
        Self {
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

        result *= self.level as f64;
        result *= if self.verbal { 20.0 } else { 1.0 };
        result *= if self.somatic { 30.0 } else { 1.0 };
        result *= match self.action_variant {
            Action::Action => 6.0,
            Action::BonusAction => 2.0,
            Action::Reaction => 0.5,
        };
        result = match self.material_sign {
            BonusSign::Additive => result + self.material as f64,
            BonusSign::Multiplicative => result * self.material as f64,
        };
        result = match self.bonus_sign {
            BonusSign::Additive => result + self.bonus as f64,
            BonusSign::Multiplicative => result * self.bonus as f64,
        };
        result *= match self.dice_type {
            Dice::None => 1.0,
            _ => self.dice_amount as f64,
        };

        result /= match self.dice_type {
            Dice::None => 1.0,
            dice => dice as u32 as f64,
        };

        self.result = format!("{}", result);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
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
                        ui.label("Level");
                        ui.add_space(ui.available_width() - 48.0);
                        ui.add(egui::DragValue::new(&mut self.level).range(1..=11));
                    });
                    ui.separator();
                    ui.vertical(|ui| {
                        ui.label("Action Type");
                        ui.horizontal(|ui| {
                            ui.selectable_value(&mut self.action_variant, Action::Action, "Action");
                            ui.selectable_value(
                                &mut self.action_variant,
                                Action::BonusAction,
                                "Bonus Action",
                            );
                            ui.selectable_value(
                                &mut self.action_variant,
                                Action::Reaction,
                                "Reaction",
                            );
                        });
                    });
                    ui.separator();
                    ui.vertical(|ui| {
                        ui.label("Components");
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut self.verbal, "Verbal");
                            ui.checkbox(&mut self.somatic, "Somatic");
                        });
                    });
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Material Component");
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
                        ui.label("Dice");
                        ui.horizontal(|ui| {
                            ui.label("Amount");
                            ui.add(egui::DragValue::new(&mut self.dice_amount));

                            ui.separator();

                            egui::ComboBox::from_label("Type")
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
                        ui.label("Additional Bonus");
                        ui.add_space(38.0);

                        ui.selectable_value(&mut self.bonus_sign, BonusSign::Additive, "+");
                        ui.selectable_value(&mut self.bonus_sign, BonusSign::Multiplicative, "*");

                        ui.add_space(22.0);
                        ui.add(egui::DragValue::new(&mut self.bonus));
                    });
                    ui.separator();

                    ui.label("Result");

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
