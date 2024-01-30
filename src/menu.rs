use bevy::prelude::*;

use crate::{game::assets::MyAssets, AppState};

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

const NORMAL_BUTTON: Color = Color::rgb(0.5, 0.5, 0.5);
const HOVERED_BUTTON: Color = Color::rgb(1., 1., 1.);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), setup_menu)
            .add_systems(
                OnExit(AppState::Menu),
                crate::utility::despawn_screen::<OnMenuScreen>,
            );
        // .add_systems(OnExit(AppState::Menu), systems);

        // Generalized button handling
        app.add_systems(
            Update,
            (menu_action, button_system).run_if(in_state(AppState::Menu)),
        );
    }
}

#[derive(Component)]
struct OnMenuScreen;

#[derive(Component)]
pub struct MenuCamera;

// All actions that can be triggered from a button click
#[derive(Component, PartialEq)]
enum MenuButtonAction {
    Play,
    Quit,
}

fn setup_menu(mut commands: Commands, assets: Res<MyAssets>) {
    let button_style = Style {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };

    commands
        .spawn((
            Name::new("MainMenu"),
            NodeBundle {
                style: Style {
                    // left: Val::Percent(15.),
                    // right: Val::Percent(15.),
                    // top: Val::Percent(10.),
                    // bottom: Val::Percent(10.),
                    width: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    // aspect_ratio: Some(1.5),
                    ..default()
                },
                ..default()
            },
            OnMenuScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Background"),
                NodeBundle {
                    style: Style {
                        width: Val::Px(500.0),
                        height: Val::Px(125.0),
                        margin: UiRect::top(Val::VMin(5.)),

                        // position_type: PositionType::Absolute,
                        // left: Val::Px(0.),
                        // right: Val::Px(0.),
                        // top: Val::Px(0.),
                        // bottom: Val::Px(0.),
                        // width: Val::Px(500.0),
                        // height: Val::Px(125.0),
                        // width: Val::Px(500.0),
                        // height: Val::Px(125.0),
                        // margin: UiRect::top(Val::VMin(5.)),
                        ..default()
                    },
                    // a `NodeBundle` is transparent by default, so to see the image we have to its color to `WHITE`
                    // background_color: Color::WHITE.into(),
                    ..default()
                },
                UiImage::new(assets.menu_background.clone()),
            ));

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    for (action, text) in [
                        (MenuButtonAction::Play, assets.menu_play.clone()),
                        (MenuButtonAction::Quit, assets.menu_quit.clone()),
                    ] {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    image: UiImage::new(text),
                                    ..default()
                                },
                                // UiImage::new(text),
                                action,
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Px(300.),
                                            height: Val::Px(100.),
                                            ..default()
                                        },
                                        ..default()
                                    },
                                ));
                            });
                            // .with_children(|parent| {
                            //     parent.spawn(TextBundle::from_section(
                            //         text,
                            //         button_text_style.clone(),
                            //     ));
                            // });
                    }
                });
        });
}

#[derive(Component)]
struct SelectedOption;

// This system handles changing all buttons color based on mouse interaction
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<bevy::app::AppExit>,
    mut appstate: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => app_exit_events.send(bevy::app::AppExit),
                MenuButtonAction::Play => {
                    appstate.set(AppState::Game);
                }
            }
        }
    }
}
