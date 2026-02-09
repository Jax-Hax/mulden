use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct LinkedButton {
    pub linked_menu: Entity,
}

#[derive(Component, Debug, Clone, Default)]
#[require(BackgroundColor, Button)]
pub struct DefaultColors {
    pub background: Color,
    pub border: Color,
    pub hover: Color,
    pub press: Color,
}

#[derive(Component, Debug, Clone, Default)]
#[require(Visibility)]
pub struct Menu;

#[derive(Component)]
#[require(Menu)]
pub struct MainMenuUi;

pub fn new_menu(bgc: Color, bdc: Color) -> impl Bundle {
    (
        Menu,
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(6),
            ..default()
        },
        BackgroundColor(bgc),
        BorderColor::all(bdc),
        Visibility::Visible,
    )
}

pub fn setup_main_menu(mut commands: Commands) {
    let bgc: Color = Color::srgb(0.0, 0.0, 0.0);
    let bdc: Color = Color::srgb(1.0, 1.0, 1.0);
    let menu: Entity = commands.spawn((MainMenuUi, new_menu(bgc, bdc))).id();

    let selection_node = commands
        .spawn(Node {
            width: percent(100),
            height: percent(80),
            border: UiRect::all(px(5)),
            border_radius: BorderRadius::ZERO,
            // horizontally center child buttons
            justify_content: JustifyContent::Center,
            // vertically center child buttons
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            top: percent(20),

            ..default()
        })
        .id();
    commands.entity(menu).add_children(&[selection_node]);

    let button1 = commands
        .spawn(new_button("Singleplayer", Color::WHITE, 33.0))
        .id();
    commands.entity(selection_node).add_children(&[button1]);

    let button2 = commands
        .spawn(new_button("Multiplayer", Color::WHITE, 33.0))
        .id();
    commands.entity(selection_node).add_children(&[button2]);

    let text1 = commands
        .spawn(new_text((35.0, 15.0), "MULDEN", Color::WHITE, 50.0))
        .id();
    commands.entity(menu).add_children(&[text1]);
}

fn new_text(
    position: (f32, f32),
    text: &str,
    text_color: Color,
    font_size: f32,
) -> impl Bundle {
    (
        Node {
            width: percent(30),
            height: px(65),
            border: UiRect::all(px(5)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            border_radius: BorderRadius::ZERO,
            position_type: PositionType::Absolute,
            left: percent(position.0),
            top: percent(position.1),
            ..default()
        },
        BorderColor::all(Color::WHITE),
        BackgroundColor(Color::BLACK),
        children![(
            Text::new(text),
            TextFont {
                font_size,
                ..default()
            },
            TextColor(text_color),
            TextShadow::default(),
        )],
    )
}

fn new_button(text: &str, text_color: Color, font_size: f32) -> impl Bundle {
    (
        Button,
        Node {
            width: percent(20),
            height: px(65),
            border: UiRect::all(px(5)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            border_radius: BorderRadius::ZERO,
            ..default()
        },
        BorderColor::all(Color::WHITE),
        BackgroundColor(Color::BLACK),
        DefaultColors {background: Color::BLACK, border : Color::WHITE, hover: Color::srgb(0.0, 1.0, 0.0), press: Color::srgb(1.0, 0.0, 0.0)},
        children![(
            Text::new(text),
            TextFont {
                font_size,
                ..default()
            },
            TextColor(text_color),
            TextShadow::default(),
        )],
    )
}