use std::collections::HashMap;

use bevy::prelude::*;
pub struct MainPlugin;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ModalState {
    IsOpen,
    IsClosed,
}

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(ModalState::IsClosed)
            .add_startup_system(setup)
            .add_system(main_system)
            .add_system_set(SystemSet::on_enter(ModalState::IsOpen).with_system(modal_system))
            .add_system_set(SystemSet::on_enter(ModalState::IsClosed).with_system(modal_system));
    }
}

struct EntityData {
    root_entity: Entity,
    modal_entity: Option<Entity>,
}

#[derive(Default, Eq, PartialEq, Hash, Clone)]
struct Task {
    pub name: String,
    pub done: bool,
    pub list: String,
}

#[derive(Default, Eq, PartialEq, Hash, Clone)]
struct ChannelContents {
    pub list: Vec<String>,
    pub tasks: Vec<Task>,
}

impl ChannelContents {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            list: Vec::new(),
            tasks: Vec::new(),
        }
    }
}

#[derive(Component, Debug, Default, Clone, Copy)]
struct TaskText;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    _modal_state: Res<State<ModalState>>,
) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());
    let font = asset_server.load("fonts/Roboto-Regular.ttf");
    let channels = ["General", "Random"];
    let mut ch_contents = HashMap::new();
    for ch in channels {
        ch_contents.insert(
            ch,
            ChannelContents {
                list: vec!["TODO".to_string(), "Doing".to_string(), "Done".to_string()],
                tasks: vec![
                    Task {
                        name: "Task 1".to_string(),
                        done: false,
                        list: "TODO".to_string(),
                    },
                    Task {
                        name: "Task 2".to_string(),
                        done: false,
                        list: "Doing".to_string(),
                    },
                    Task {
                        name: "Task 3".to_string(),
                        done: false,
                        list: "TODO".to_string(),
                    },
                ],
            },
        );
    }
    // root node
    commands.spawn_bundle(ButtonBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            position_type: PositionType::Absolute,
            position: Rect::all(Val::Percent(10.0)),
            border: Rect::all(Val::Px(2.0)),
            ..Default::default()
        },
        color: Color::rgb(0.4, 0.4, 0.4).into(),
        ..Default::default()
    });
    let root_entity = commands
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
                            for channel in channels {
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
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Title
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Px(50.0)),
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                padding: Rect::all(Val::Px(10.)),
                                border: Rect::all(Val::Px(2.0)),
                                ..Default::default()
                            },
                            // color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.0), Val::Px(25.)),
                                    ..Default::default()
                                },
                                text: Text::with_section(
                                    "# Main Content",
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
                    // Main View
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::FlexEnd,
                                padding: Rect::all(Val::Px(10.)),
                                border: Rect::all(Val::Px(2.0)),
                                ..Default::default()
                            },
                            color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // list 表示
                            let ch_content = match ch_contents.get(&channels[0]) {
                                Some(cont) => cont,
                                None => unimplemented!(),
                            };

                            // list of task state
                            for list in ch_content.list.iter() {
                                parent
                                    .spawn_bundle(NodeBundle {
                                        style: Style {
                                            flex_direction: FlexDirection::ColumnReverse,
                                            justify_content: JustifyContent::FlexStart,
                                            size: Size::new(Val::Percent(30.0), Val::Undefined),
                                            padding: Rect::all(Val::Px(10.)),
                                            margin: Rect::all(Val::Px(10.)),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    })
                                    .insert(TaskText)
                                    .with_children(|parent| {
                                        parent.spawn_bundle(TextBundle {
                                            style: Style {
                                                size: Size::new(Val::Percent(100.0), Val::Px(25.)),
                                                margin: Rect {
                                                    bottom: Val::Px(10.),
                                                    ..Default::default()
                                                },
                                                ..Default::default()
                                            },
                                            text: Text::with_section(
                                                list,
                                                TextStyle {
                                                    font: font.clone(),
                                                    font_size: 25.,
                                                    color: Color::BLACK,
                                                },
                                                Default::default(),
                                            ),
                                            ..Default::default()
                                        });

                                        // task
                                        for task in ch_content
                                            .tasks
                                            .iter()
                                            .filter(|&task| &task.list == list)
                                        {
                                            // button
                                            parent
                                                .spawn_bundle(ButtonBundle {
                                                    style: Style {
                                                        size: Size::new(
                                                            Val::Percent(100.0),
                                                            Val::Px(40.),
                                                        ),
                                                        border: Rect::all(Val::Px(2.)),
                                                        margin: Rect {
                                                            top: Val::Px(5.),
                                                            ..Default::default()
                                                        },
                                                        ..Default::default()
                                                    },
                                                    color: Color::rgb(1., 1., 0.15).into(),
                                                    ..Default::default()
                                                })
                                                .with_children(|parent| {
                                                    parent.spawn_bundle(TextBundle {
                                                        style: Style {
                                                            size: Size::new(
                                                                Val::Percent(100.0),
                                                                Val::Px(20.),
                                                            ),
                                                            ..Default::default()
                                                        },
                                                        text: Text::with_section(
                                                            &task.name,
                                                            TextStyle {
                                                                font: font.clone(),
                                                                font_size: 20.,
                                                                color: Color::BLACK,
                                                            },
                                                            Default::default(),
                                                        ),
                                                        ..Default::default()
                                                    });
                                                });
                                        }
                                    });
                            }
                        });
                });
        })
        .id();
    commands.insert_resource(EntityData {
        root_entity,
        modal_entity: None,
    });
}

fn main_system(
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    // mut text_query: Query<&mut Text>,
    mut modal_state: ResMut<State<ModalState>>,
) {
    for (interaction, _children) in interaction_query.iter_mut() {
        // let text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                match modal_state.current() {
                    ModalState::IsClosed => modal_state.set(ModalState::IsOpen).unwrap(),
                    ModalState::IsOpen => modal_state.set(ModalState::IsClosed).unwrap(),
                }
            }
            Interaction::Hovered => {
                println!("hover");
            }
            Interaction::None => {}
        }
    }
}

fn modal_system(
    mut commands: Commands,
    // _asset_server: Res<AssetServer>,
    modal_state: Res<State<ModalState>>,
    entity_data: Res<EntityData>,
) {
    println!("modal_system: {:?}", modal_state);
    if modal_state.current() == &ModalState::IsOpen {
        // parent
        let modal_entity = commands
            .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    position_type: PositionType::Absolute,
                    position: Rect::all(Val::Px(0.0)),
                    padding: Rect::all(Val::Percent(10.0)),
                    border: Rect::all(Val::Px(2.0)),
                    ..Default::default()
                },
                color: Color::rgb(0.4, 0.4, 0.4).into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            position_type: PositionType::Relative,
                            border: Rect::all(Val::Px(2.0)),
                            ..Default::default()
                        },
                        color: Color::rgb(0.4, 0.4, 1.0).into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                ..Default::default()
                            },
                            color: Color::rgb(0.8, 0.8, 1.0).into(),
                            ..Default::default()
                        });
                    });
            })
            .id();
        commands
            .entity(entity_data.root_entity)
            .add_child(modal_entity);
        commands.insert_resource(EntityData {
            root_entity: entity_data.root_entity,
            modal_entity: Some(modal_entity),
        });
    } else {
        if let Some(modal_entity) = entity_data.modal_entity {
            commands.entity(modal_entity).despawn_recursive();
            commands.insert_resource(EntityData {
                root_entity: entity_data.root_entity,
                modal_entity: None,
            });
        }
    }
}
