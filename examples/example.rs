use bevy_ui_text_picker::*;
use bevy::prelude::*;

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .commands()
        .spawn_bundle(NodeBundle {
            color: UiColor(Color::MAROON),
            style: Style {
                margin: Rect::all(Val::Auto),
                padding: Rect::all(Val::Px(10.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            builder.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Hello, ".to_string(),
                            style: TextStyle {
                                font: asset_server.load("FiraMono-Medium.ttf"),
                                font_size: 32.0,
                                color: Color::WHITE,
                            },
                        },
                        TextSection {
                            value: "World".to_string(), 
                            style: TextStyle {
                                font: asset_server.load("FiraMono-Medium.ttf"),
                                font_size: 32.0,
                                color: Color::WHITE,
                            }
                        },
                    ],
                    alignment: TextAlignment { vertical: VerticalAlign::Center, horizontal:HorizontalAlign::Center },
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(TextRects::default());
        });
}

fn report(
    windows: Res<Windows>,
    mut query: Query<(&mut Text, &TextRects)>,
) {
    windows
    .get_primary()
    .and_then(|window| window.cursor_position())
    .map(|pointer_position| {
        query.for_each_mut(|(mut text, text_rects)| {
            if !text_rects.is_empty() {
                let picked_section = text_rects.iter()
                .find(|(section, rect)| {
                    (rect.min.x..rect.max.x).contains(&pointer_position.x)
                    && (rect.min.y..rect.max.y).contains(&pointer_position.y)
                })
                .map(|&x| x.0);
                for (i, section) in text.sections.iter_mut().enumerate() {
                    section.style.color = if Some(i) == picked_section {
                        Color::RED
                    } else {
                        Color::WHITE
                    };
                }
            
            }
        });
    });
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(BevyTextPickerPlugin)
    .add_startup_system(setup)
    .add_system(report)
    .run();
}