// disable console opening on windows
#![windows_subsystem = "windows"]

use bevy::prelude::*;
use mini_jam_72_plugin::GamePlugin;

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
            title: "mini_jam_72".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    // app.add_plugin(FrameTimeDiagnosticsPlugin::default());
    // app.add_plugin(PrintDiagnosticsPlugin::default());

    app.add_plugin(GamePlugin).run();
}
