use std::collections::HashMap;

use bevy::prelude::*;

fn main() {
    #[cfg(debug_assertions)]
    {
        // use std::fs;
        // fs::create_dir("build").unwrap();
    }
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MainPlugin)
        .run();
}

struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        // app.add_system(hello_world);
        app.add_startup_system(setup);
    }
}

#[derive(Eq, PartialEq, Hash)]
struct ChannelContents {}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());
    let font = asset_server.load("fonts/Roboto-Regular.ttf");
    let channels = ["General", "Random"];
    let mut ch_contents = HashMap::new();
    for ch in channels {
        ch_contents.insert(ch, ChannelContents {});
    }
    // root node
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                margin: Rect::all(Val::Px(0.)),
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // sidebar
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(20.0), Val::Percent(100.0)),
                        // border: Rect::all(Val::Px(2.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        flex_grow: 1.0,
                        ..Default::default()
                    },
                    color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // left vertical fill (content)
                    let title_height = 7.0;
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(title_height)),
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // text
                            parent.spawn_bundle(TextBundle {
                                style: Style {
                                    margin: Rect {
                                        left: Val::Px(10.0),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                text: Text::with_section(
                                    "Channel List",
                                    TextStyle {
                                        font: font.clone(),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                    Default::default(),
                                ),
                                ..Default::default()
                            });
                        });
                    // channel list
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::ColumnReverse,
                                size: Size::new(
                                    Val::Percent(100.0),
                                    Val::Percent(100.0 - title_height),
                                ),
                                // overflow: Overflow::Hidden,
                                ..Default::default()
                            },
                            color: Color::rgb(0.10, 0.10, 0.10).into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            let channel_font_size = 20.0;
                            for (_index, &channel) in channels.iter().enumerate() {
                                parent.spawn_bundle(TextBundle {
                                    style: Style {
                                        flex_shrink: 0.,
                                        size: Size::new(Val::Undefined, Val::Px(channel_font_size)),
                                        margin: Rect {
                                            left: Val::Px(channel_font_size),
                                            top: Val::Px(channel_font_size / 2.),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    },
                                    text: Text::with_section(
                                        channel,
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: channel_font_size,
                                            color: Color::WHITE,
                                        },
                                        Default::default(),
                                    ),
                                    ..Default::default()
                                });
                            }
                        });
                });
            // main contents
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        justify_content: JustifyContent::FlexStart,
                        size: Size::new(Val::Percent(80.0), Val::Percent(100.0)),
                        padding: Rect::all(Val::Px(10.)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn_bundle(TextBundle {
                        style: Style {
                            size: Size::new(Val::Undefined, Val::Px(25.)),
                            ..Default::default()
                        },
                        text: Text::with_section(
                            "Main Content",
                            TextStyle {
                                font: font.clone(),
                                font_size: 25.,
                                color: Color::BLACK,
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
        });
}
