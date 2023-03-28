#[derive(Default)]
pub struct HomerApp {
    expression: String,
    upper: String,
    lower: String,
    n: String,
}


impl HomerApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    fn plot(&self, ui: &mut egui::Ui) -> egui::Response {
        use egui::plot::{Line, PlotPoints};
        use meval::*;
        use std::str::FromStr;

        let upper: f64 = match self.upper.parse() {
            Ok(x) => x,
            Err(_) => 1.0,
        };

        let lower: f64 = match self.lower.parse() {
            Ok(x) => x,
            Err(_) => -1.0,
        };

        let n = 128;
        let line = Line::new(
            (0..=n)
                .map(|i| {

                    let x = egui::remap(i as f64, 0.0..=n as f64, lower..=upper);
                    let expr = match Expr::from_str(&self.expression) {
                        Ok(x) => x,
                        Err(_) => Expr::from_str("0").unwrap(),
                    };
                    let mut ctx = Context::new();
                    ctx.var("x", x);
                    [x, expr.eval_with_context(ctx).unwrap()]

                })
                .collect::<PlotPoints>(),
        );
        egui::plot::Plot::new("plot")
            .show_axes([true, true])
            .allow_drag(true)
            .allow_zoom(true)
            .allow_scroll(true)
            .center_x_axis(true)
            .center_y_axis(true)
            .width(600.0)
            .height(400.0)
            .data_aspect(1.0)
            .show(ui, |plot_ui| plot_ui.line(line))
            .response
    }


}

impl eframe::App for HomerApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

         egui::CentralPanel::default().show(ctx, |ui| {
            
            ui.horizontal(|ui| {
                
                self.plot(ui);
                
            });


            ui.text_edit_singleline(&mut self.expression);

            ui.horizontal( |ui| {
                ui.label("Upper bound: ");
                ui.add_sized( [20.0, 9.0], egui::TextEdit::singleline(&mut self.upper) );
            });

            ui.horizontal( |ui| {
                ui.label("Lower bound: ");
                ui.add_sized( [20.0, 9.0], egui::TextEdit::singleline(&mut self.lower) );
            });
            
            ui.horizontal( |ui| {
                ui.label("n = ");
                ui.add_sized( [20.0, 9.0], egui::TextEdit::singleline(&mut self.n) );
            });

        });


    }



    
}
