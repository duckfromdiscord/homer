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
        use egui::plot::{Line, PlotPoints, Points};
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

        let n: u8 = match self.n.parse() {
            Ok(x) => x,
            Err(_) => 3,
        };

        let resolution = 128;

        let expr = match Expr::from_str(&self.expression) {
            Ok(x) => x,
            Err(_) => Expr::from_str("0").unwrap(),
        };

        // Function Line
        let line = Line::new(
            (0..=resolution)
                .map(|i| {

                    let x = egui::remap(i as f64, 0.0..=resolution as f64, lower..=upper);
                    let mut ctx = Context::new();
                    ctx.var("x", x);
                    [x, expr.eval_with_context(ctx).unwrap()]

                })
                .collect::<PlotPoints>(),
        );

        // Midpoint

        let mp_dx: f64 = (upper-lower)/(n as f64);
        let mut mp_subints: Vec<[f64; 2]> = vec![];
        let mut mp_idx: f64 = lower;

        for i in 0..n {
            mp_idx = (lower) + (i as f64 * mp_dx);
            let x = ( (mp_idx) + (mp_idx + mp_dx) ) / 2.0;
            let mut ctx = Context::new();
            ctx.var("x", x);
            mp_subints.push( [x, expr.eval_with_context(ctx).unwrap()] );
        }
        
        let mp_plotpoints: PlotPoints = PlotPoints::new(mp_subints);

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
            .show(ui, |plot_ui| {
                plot_ui.line(line);
                plot_ui.points(Points::new(mp_plotpoints));
            })
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


            self.expression = "x^2".to_string();
            self.upper = "3".to_string();
            self.lower = "0".to_string();
            self.n = "4".to_string();

        });


    }



    
}
