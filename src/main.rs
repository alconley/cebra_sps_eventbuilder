#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use cebra_sps_eventbuilder::EVBApp as CeBrASPSEVBApp;

enum Pane {
    CebraSPSEventBuilder,
}

struct TreeBehavior {
    cebra_sps_evb: CeBrASPSEVBApp,
}

impl egui_tiles::Behavior<Pane> for TreeBehavior {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        match pane {
            Pane::CebraSPSEventBuilder => "CeBrA+SPS EventBuilder".into(),
        }
    }

    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        _tile_id: egui_tiles::TileId,
        pane: &mut Pane,
    ) -> egui_tiles::UiResponse {
        match pane {
            Pane::CebraSPSEventBuilder => {
                self.cebra_sps_evb.ui(ui);
            }
        }

        if ui
            .add(egui::Button::new("Drag me!").sense(egui::Sense::drag()))
            .drag_started()
        {
            egui_tiles::UiResponse::DragStarted
        } else {
            egui_tiles::UiResponse::None
        }
    }
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([425.0, 250.0])
            .with_min_inner_size([425.0, 250.0]),
        ..Default::default()
    };

    let mut tree = create_tree();

    let mut behavior = TreeBehavior {
        cebra_sps_evb: CeBrASPSEVBApp::default(),
    };

    eframe::run_simple_native("CeBrA+SPS Eventbuilder", native_options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            tree.ui(&mut behavior, ui);
        });
    })

}

fn create_tree() -> egui_tiles::Tree<Pane> {

    let mut tiles = egui_tiles::Tiles::default();

    let mut tabs = vec![];
    tabs.push(tiles.insert_pane(Pane::CebraSPSEventBuilder));

    let root = tiles.insert_tab_tile(tabs);

    egui_tiles::Tree::new("my_tree", root, tiles)
}


// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(cebra_sps_eventbuilder::EVBApp::new(cc, false))),
            )
            .await
            .expect("failed to start eframe");
    });
}
