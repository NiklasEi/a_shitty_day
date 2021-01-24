// disable console opening on windows
#![windows_subsystem = "windows"]

use a_shitty_day_plugin::GamePlugin;
use bevy::prelude::*;

#[cfg(target_arch = "wasm32")]
use bevy_webgl2;

// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin};

#[bevy_main]
fn main() {
    let mut app = App::build();
    app.add_resource(Msaa { samples: 4 })
        .add_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "a_shitty_day".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    // app.add_plugin(FrameTimeDiagnosticsPlugin::default());
    // app.add_plugin(PrintDiagnosticsPlugin::default());

    app.add_plugin(GamePlugin).run();
}
