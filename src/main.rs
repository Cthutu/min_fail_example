use eframe::{
    egui::{
        CentralPanel, Color32, CtxRef, FontData, FontDefinitions, FontFamily, Hyperlink, Label,
        Layout, RichText, ScrollArea, Separator, TextStyle, Ui, Vec2,
    },
    epi::{App, Frame, Storage},
    run_native, NativeOptions,
};

const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

struct Headlines {
    articles: Vec<NewsCardData>,
}

struct NewsCardData {
    title: String,
    desc: String,
    url: String,
}

impl App for Headlines {
    fn setup(&mut self, ctx: &CtxRef, _frame: &Frame, _storage: Option<&dyn Storage>) {
        self.configure_fonts(ctx);
    }

    fn update(&mut self, ctx: &CtxRef, _frame: &Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| self.render_news_cards(ui));
        });
    }

    fn name(&self) -> &str {
        "Headlines"
    }
}

impl Headlines {
    fn new() -> Self {
        let iter = (0..20).map(|a| NewsCardData {
            title: format!("title{}", a),
            desc: format!("desc{}", a),
            url: format!("https://example.com/{}", a),
        });

        Headlines {
            articles: Vec::from_iter(iter),
        }
    }

    fn configure_fonts(&self, ctx: &CtxRef) {
        // Create a font definition object
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            "MesloLGS".to_string(),
            FontData::from_static(include_bytes!("../MesloLGS_NF_Regular.ttf")),
        );
        font_def
            .family_and_size
            .insert(TextStyle::Heading, (FontFamily::Proportional, 35.0));
        font_def
            .family_and_size
            .insert(TextStyle::Body, (FontFamily::Proportional, 20.0));
        font_def
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "MesloLGS".to_string());

        ctx.set_fonts(font_def);
    }

    fn render_news_cards(&self, ui: &mut Ui) {
        for a in &self.articles {
            ui.add_space(PADDING);
            let title = format!("► {}", a.title);
            ui.colored_label(WHITE, title);

            ui.add_space(PADDING);
            let desc = Label::new(RichText::new(&a.desc).text_style(TextStyle::Button));
            ui.add(desc);

            ui.style_mut().visuals.hyperlink_color = CYAN;
            ui.add_space(PADDING);
            ui.with_layout(Layout::right_to_left(), |ui| {
                ui.add(Hyperlink::from_label_and_url("read more 🔗", &a.url));
            });

            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }
}

fn main() {
    let app = Headlines::new();
    let win_option = NativeOptions {
        initial_window_size: Some(Vec2::new(540.0, 960.0)),
        ..Default::default()
    };

    run_native(Box::new(app), win_option)
}
