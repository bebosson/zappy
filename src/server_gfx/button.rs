pub mod craftbutton{

    use bevy::{
        diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, input::mouse::MouseMotion, prelude::*, sprite::Anchor, text::{BreakLineOn, Text2dBounds}, ui::RelativeCursorPosition, window::PrimaryWindow, winit::WinitSettings
    };

    use crate::{env::env::RessCommandId, map::map::Position, sprite_player::sprite_player::Cell};
    
    pub struct Craftbutton;
    
    impl Plugin for Craftbutton{
        fn build(&self, app: &mut App) {
            app
            .insert_resource(WinitSettings::desktop_app())
            // .add_plugins(DefaultPlugins)
            .add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Startup, setup_button)
            .add_systems(Update, (text_update_system, mouse_motion, cursor_position));
            // .add_systems(
            //     Update,
            //     (animate_translation, animate_rotation, animate_scale),
            // );
    
        }
    }
    #[derive(Component)]
    struct AnimateTranslation;
    
    #[derive(Component)]
    struct AnimateRotation;
    
    #[derive(Component)]
    struct AnimateScale;

    #[derive(Component)]
    struct Textatrou;


    fn mouse_motion(
        mut motion_evr: EventReader<MouseMotion>,
    ) {
        for ev in motion_evr.read() {
            println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
        }
    }

    fn cursor_position(
        q_windows: Query<&Window, With<PrimaryWindow>>,
    ) {
        // Games typically only have one window (the primary window)
        if let Some(position) = q_windows.single().cursor_position() {
            println!("Cursor is inside the primary window, at {:?}", position);
        } else {
            println!("Cursor is not in the game window.");
        }
    }
    
    fn setup_button(mut commands: Commands, asset_server: Res<AssetServer>) {
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        let text_style = TextStyle {
            font: font.clone(),
            font_size: 60.0,
            color: Color::WHITE,
        };
        let text_alignment = TextAlignment::Center;
        // 2d camera
        // commands.spawn(Camera2dBundle::default());
        // Demonstrate changing translation
        // commands.spawn((
        //     Text2dBundle {
        //         text: Text::from_section("translation", text_style.clone())
        //             .with_alignment(text_alignment),
        //         ..default()
        //     },
        //     AnimateTranslation,
        // ));
        // // Demonstrate changing rotation
        // commands.spawn((
        //     Text2dBundle {
        //         text: Text::from_section("rotation", text_style.clone()).with_alignment(text_alignment),
        //         ..default()
        //     },
        //     AnimateRotation,
        // ));
        // // Demonstrate changing scale
        // commands.spawn((
        //     Text2dBundle {
        //         text: Text::from_section("scale", text_style).with_alignment(text_alignment),
        //         ..default()
        //     },
        //     AnimateScale,
        // ));
        // Demonstrate text wrapping
        let slightly_smaller_text_style = TextStyle {
            font,
            font_size: 12.0,
            color: Color::WHITE,
        };
        let box_size = Vec2::new(300.0, 200.0);
        let box_position = Vec2::new(0.0, -250.0);
        commands
            .spawn((SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(box_size.x, box_size.y)),
                    ..default()
                },
                transform: Transform::from_translation(box_position.extend(0.0)),
                ..default()
                
            }))
            .with_children(|builder| {
                builder.spawn(Text2dBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "this text wraps in the box\n(Unicode linebreaks)",
                            slightly_smaller_text_style.clone(),
                        )],
                        
                        alignment: TextAlignment::Left,
                        linebreak_behavior: BreakLineOn::WordBoundary,
                    },
                    text_2d_bounds: Text2dBounds {
                        // Wrap text in the rectangle
                        size: box_size,
                    },
                    // ensure the text is drawn on top of the box
                    transform: Transform::from_translation(Vec3::Z), // here text position need to depend of box_position
                    ..default()
                },
                );
            });
    
        let other_box_size = Vec2::new(300.0, 200.0);
        let other_box_position = Vec2::new(320.0, -250.0);
        commands
            .spawn((SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.20, 0.3, 0.70),
                    custom_size: Some(Vec2::new(other_box_size.x, other_box_size.y)),
                    ..default()
                },
                transform: Transform::from_translation(other_box_position.extend(0.0)),
                ..default()
            },Textatrou))
            .with_children(|builder| {
                builder.spawn(Text2dBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "FPS",
                            slightly_smaller_text_style.clone(),
                        ),
                        TextSection::new(
                            "",
                            slightly_smaller_text_style.clone(),
                        )],
                        alignment: TextAlignment::Left,
                        linebreak_behavior: BreakLineOn::AnyCharacter,
                    },
                    text_2d_bounds: Text2dBounds {
                        // Wrap text in the rectangle
                        size: other_box_size,
                    },
                    // ensure the text is drawn on top of the box
                    transform: Transform::from_translation(Vec3::Z),
                    ..default()
                });
            }).insert(RelativeCursorPosition::default());
    
        // for (text_anchor, color) in [
        //     (Anchor::TopLeft, Color::RED),
        //     (Anchor::TopRight, Color::GREEN),
        //     (Anchor::BottomRight, Color::BLUE),
        //     (Anchor::BottomLeft, Color::YELLOW),
        // ] {
        //     commands.spawn(Text2dBundle {
        //         text: Text {
        //             sections: vec![TextSection::new(
        //                 format!(" Anchor::{text_anchor:?} "),
        //                 TextStyle {
        //                     color,
        //                     ..slightly_smaller_text_style.clone()
        //                 },
        //             )],
        //             ..Default::default()
        //         },
        //         transform: Transform::from_translation(250. * Vec3::Y),
        //         text_anchor,
        //         ..default()
        //     });
        // }
    }
    
    // fn animate_translation(
    //     time: Res<Time>,
    //     mut query: Query<&mut Transform, (With<Text>, With<AnimateTranslation>)>,
    // ) {
    //     for mut transform in &mut query {
    //         transform.translation.x = 100.0 * time.elapsed_seconds().sin() - 400.0;
    //         transform.translation.y = 100.0 * time.elapsed_seconds().cos();
    //     }
    // }
    
    // fn animate_rotation(
    //     time: Res<Time>,
    //     mut query: Query<&mut Transform, (With<Text>, With<AnimateRotation>)>,
    // ) {
    //     for mut transform in &mut query {
    //         transform.rotation = Quat::from_rotation_z(time.elapsed_seconds().cos());
    //     }
    // }
    
    // fn animate_scale(
    //     time: Res<Time>,
    //     mut query: Query<&mut Transform, (With<Text>, With<AnimateScale>)>,
    // ) {
    //     // Consider changing font-size instead of scaling the transform. Scaling a Text2D will scale the
    //     // rendered quad, resulting in a pixellated look.
    //     for mut transform in &mut query {
    //         transform.translation = Vec3::new(400.0, 0.0, 0.0);
    
    //         let scale = (time.elapsed_seconds().sin() + 1.1) * 2.0;
    //         transform.scale.x = scale;
    //         transform.scale.y = scale;
    //     }
    // } 

    fn text_update_system(
        q_parent: Query<(&Transform, With<Textatrou>, &Children)>,
        mut q_child: Query<&mut Text>,
        relative_cursor_position_query: Query<&RelativeCursorPosition>,
    ) {
        
        let relative_cursor_position = relative_cursor_position_query.single();
        // get the properties of each squad
        for (_, _, children) in q_parent.iter() {
            // `children` is a collection of Entity IDs
            for &child in children.iter() {

                // get the health of each child unit
                let mut text = q_child.get_mut(child).unwrap();
                text.sections[1].value =
                if let Some(relative_cursor_position) = relative_cursor_position.normalized {
                    format!(
                        "({:.1}, {:.1})",
                        relative_cursor_position.x, relative_cursor_position.y
                    )
                } else {
                    "unknown".to_string()
                };
    
                text.sections[1].style.color = if relative_cursor_position.mouse_over() {
                    Color::rgb(0.1, 0.9, 0.1)
                } else {
                    Color::rgb(0.9, 0.1, 0.1)
                };
                // do something
            }
        }
    }

    // fn relative_cursor_position_system(
    //     relative_cursor_position_query: Query<&RelativeCursorPosition>,
    //     mut output_query: Query<&mut Text>,
    // ) {
    //     let relative_cursor_position = relative_cursor_position_query.single();
    
    //     let mut output = output_query.single_mut();
    
    //     output.sections[1].value =
    //         if let Some(relative_cursor_position) = relative_cursor_position.normalized {
    //             format!(
    //                 "({:.1}, {:.1})",
    //                 relative_cursor_position.x, relative_cursor_position.y
    //             )
    //         } else {
    //             "unknown".to_string()
    //         };
    
    //     output.sections[1].style.color = if relative_cursor_position.mouse_over() {
    //         Color::rgb(0.1, 0.9, 0.1)
    //     } else {
    //         Color::rgb(0.9, 0.1, 0.1)
    //     };
    // }
    

    fn button_system(
        mut interaction_query: Query<
        (
            &Interaction,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Transform>,
    ) {
        for (interaction, children) in &mut interaction_query {
            let mut text = text_query.get_mut(children[0]).unwrap();
            match *interaction {
                Interaction::Pressed => {
                    
                }
                Interaction::Hovered => {
                    println!("{:?}", text);
                }
                Interaction::None => {
                  
                }
            }
        }
    }

    // fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //     // ui camera
    //     commands.spawn(Camera2dBundle::default());
    //     commands
    //         .spawn(NodeBundle {
    //             style: Style {
    //                 width: Val::Percent(100.0),
    //                 height: Val::Percent(100.0),
    //                 align_items: AlignItems::Center,
    //                 justify_content: JustifyContent::Center,
    //                 ..default()
    //             },
    //             ..default()
    //         })
    //         .with_children(|parent| {
    //             parent
    //                 .spawn(ButtonBundle {
    //                     style: Style {
    //                         width: Val::Px(150.0),
    //                         height: Val::Px(65.0),
    //                         border: UiRect::all(Val::Px(5.0)),
    //                         // horizontally center child text
    //                         justify_content: JustifyContent::Center,
    //                         // vertically center child text
    //                         align_items: AlignItems::Center,
    //                         ..default()
    //                     },
    //                     border_color: BorderColor(Color::BLACK),
    //                     background_color: NORMAL_BUTTON.into(),
    //                     ..default()
    //                 })
    //                 .with_children(|parent| {
    //                     parent.spawn(TextBundle::from_section(
    //                         "Button",
    //                         TextStyle {
    //                             font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    //                             font_size: 40.0,
    //                             color: Color::rgb(0.9, 0.9, 0.9),
    //                         },
    //                     ));
    //                 });
    //         });
    // }


    // fn text_update_system(
    //     mut query: Query<&mut Text>,
    //     diagnostics: Res<DiagnosticsStore>,
    // ) {
    //     for mut text in &mut query {
    //         if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
    //             if let Some(value) = fps.smoothed() {
    //                 // Update the value of the second section
    //                 text.sections[0].value = format!("{value:.2}");
    //             }
    //         }
    //     }
    // }
}
