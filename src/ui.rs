use bevy::prelude::*;

use crate::{UiTextures, WinSize};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, setup_ui_system);
    }
}

fn setup_ui_system(mut commands: Commands, ui_textures: Res<UiTextures>, win_size: Res<WinSize>) {
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
                    parent.spawn_bundle(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(40.), Val::Px(40.)),
                            ..Default::default()
                        },
                        image: ui_textures.heart_full.clone().into(),
                        ..Default::default()
                    });
                    parent.spawn_bundle(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(40.), Val::Px(40.)),
                            ..Default::default()
                        },
                        image: ui_textures.heart_full.clone().into(),
                        ..Default::default()
                    });
                    parent.spawn_bundle(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(40.), Val::Px(40.)),
                            ..Default::default()
                        },
                        image: ui_textures.heart_full.clone().into(),
                        ..Default::default()
                    });
                });
        });
}
