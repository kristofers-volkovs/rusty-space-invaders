use bevy::prelude::*;

use crate::components::HeartImage;
use crate::{PlayerState, UiTextures, WinSize};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, setup_ui_system)
            .add_system(heart_image_update_system);
    }
}

fn setup_ui_system(
    mut commands: Commands,
    ui_textures: Res<UiTextures>,
    win_size: Res<WinSize>,
    player_state: Res<PlayerState>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(130.), Val::Percent(100.)),
                        align_items: AlignItems::FlexEnd,
                        justify_content: JustifyContent::SpaceBetween,
                        border: Rect::all(Val::Px(5.)),
                        ..Default::default()
                    },
                    color: Color::rgba(0., 0., 0., 0.).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    for i in 1..=player_state.max_health {
                        let bundle = heart_image_bundle(ui_textures.heart_full.clone().into());
                        if i > player_state.health {
                            let bundle = heart_image_bundle(ui_textures.heart_empty.clone().into());
                        }
                        parent.spawn_bundle(bundle).insert(HeartImage);
                    }
                });
        });
}

fn heart_image_bundle(image: UiImage) -> ImageBundle {
    ImageBundle {
        style: Style {
            size: Size::new(Val::Px(40.), Val::Px(40.)),
            ..Default::default()
        },
        image,
        ..Default::default()
    }
}

fn heart_image_update_system(
    player_state: Res<PlayerState>,
    ui_textures: Res<UiTextures>,
    mut query: Query<&mut UiImage, With<HeartImage>>,
) {
    for (idx, mut image) in query.iter_mut().enumerate() {
        if idx >= player_state.health {
            image.0 = ui_textures.heart_empty.clone().into();
        }
    }
}
