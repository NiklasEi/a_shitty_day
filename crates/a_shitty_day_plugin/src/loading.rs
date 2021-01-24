use crate::assets::font_monogram;
use crate::{AppState, STAGE};
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::sprite::TextureAtlasBuilder;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<TileSpriteHandles>()
            .on_state_enter(STAGE, AppState::Loading, start_loading.system())
            .on_state_update(STAGE, AppState::Loading, load.system())
            .on_state_exit(STAGE, AppState::Loading, clean_up_loading.system());
    }
}

struct LoadingIndicator;

#[derive(Default, Clone)]
struct TileSpriteHandles {
    handles: Vec<HandleUntyped>,
    atlas_loaded: bool,
}

struct LoadedAssets {
    atlas: Handle<TextureAtlas>,
}

fn start_loading(
    commands: &mut Commands,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut tile_sprite_handles: ResMut<TileSpriteHandles>,
    asset_server: ResMut<AssetServer>,
) {
    tile_sprite_handles.handles = asset_server.load_folder("textures").unwrap();
    let font = asset_server.load(font_monogram());
    let material = color_materials.add(Color::NONE.into());
    commands
        .spawn(CameraUiBundle::default())
        // root node
        .spawn(NodeBundle {
            style: Style {
                ..Default::default()
            },
            material,
            ..Default::default()
        })
        .with(LoadingIndicator)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    value: "Loading...".to_owned(),
                    font,
                    style: TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.6, 0.6, 0.6),
                        ..Default::default()
                    },
                },
                ..Default::default()
            });
        });
}

fn load(
    commands: &mut Commands,
    mut sprite_handles: ResMut<TileSpriteHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<AppState>>,
) {
    if sprite_handles.atlas_loaded {
        state.set_next(AppState::Menu);
    }
    // Lets load all our textures from our folder!
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        for handle in sprite_handles.handles.iter() {
            let texture = textures.get(handle).unwrap();
            texture_atlas_builder.add_texture(handle.clone_weak().typed::<Texture>(), &texture);
        }

        let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
        let atlas_handle = texture_atlases.add(texture_atlas);

        commands.insert_resource(LoadedAssets {
            atlas: atlas_handle,
        });
        sprite_handles.atlas_loaded = true;
    }
}

fn clean_up_loading(commands: &mut Commands, text_query: Query<Entity, With<LoadingIndicator>>) {
    for remove in text_query.iter() {
        commands.despawn_recursive(remove);
    }
}
