use bevy::prelude::*;
use bevy_mod_picking::*;

use crate::pieces::*;

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        println!("Board build creation called");
        app.init_resource::<SelectedSquare>()
            .init_resource::<SquareMaterials>()
            .init_resource::<SelectedPiece>()
            .add_startup_system(create_board)
            .add_system(color_squares.system())
            .add_system(select_square.system());
    }
}
#[derive(Component)]
struct Square {
    pub x: u8,
    pub y: u8,
}

impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }
}

#[derive(Default)]
struct SelectedSquare {
    entity: Option<Entity>,
}

#[derive(Default)]
struct SelectedPiece {
    entity: Option<Entity>,
}

struct SquareMaterials {
    highlight_color: Handle<StandardMaterial>,
    selected_color: Handle<StandardMaterial>,
    white_color: Handle<StandardMaterial>,
    black_color: Handle<StandardMaterial>,
}

impl FromWorld for SquareMaterials {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        SquareMaterials {
            highlight_color: materials.add(Color::rgb(0.8, 0.3, 0.3).into()),
            selected_color: materials.add(Color::rgb(0.9, 0.1, 0.1).into()),
            white_color: materials.add(Color::rgb(1.0, 0.9, 0.9).into()),
            black_color: materials.add(Color::rgb(0.0, 0.1, 0.1).into()),
        }
    }
}

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<SquareMaterials>,
) {
    // Add meshes and materials
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1.0 }));

    // Spawn 64 squares
    for i in 0..8 {
        for j in 0..8 {
            commands
                .spawn_bundle(PbrBundle {
                    mesh: mesh.clone(),
                    //Change material according to position to get alternating pattern
                    material: if (i + j + 1) % 2 == 0 {
                        materials.white_color.clone()
                    } else {
                        materials.black_color.clone()
                    },
                    transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                    ..Default::default()
                })
                .insert_bundle(PickableBundle::default())
                .insert(Square { x: i, y: j });
            // println!("{:?}", mesh);
        }
    }
}

fn color_squares(
    selected_square: Res<SelectedSquare>,
    materials: Res<SquareMaterials>,
    mut query: Query<(Entity, &Square, &mut Handle<StandardMaterial>)>,
    picking_camera_query: Query<&PickingCamera>,
) {
    //Get entity under cursor if avaialable
    let top_entity = match picking_camera_query.iter().last() {
        Some(picking_camera) => match picking_camera.intersect_top() {
            Some((entity, _intersection)) => Some(entity),
            None => None,
        },
        None => None,
    };

    for (entity, square, mut material) in query.iter_mut() {
        //Get material
        *material = if Some(entity) == top_entity {
            materials.highlight_color.clone()
        } else if Some(entity) == selected_square.entity {
            materials.selected_color.clone()
        } else if square.is_white() {
            materials.white_color.clone()
        } else {
            materials.black_color.clone()
        };
    }
}

fn select_square(
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    squares_query: Query<&Square>,
    mut pieces_query: Query<(Entity, &mut Piece)>,
    picking_camera_query: Query<&PickingCamera>,
) {
    //Only left button is selection
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    //If we have something already selcted
    if let Some(picking_camera) = picking_camera_query.iter().last() {
        if let Some((square_entity, _intersection)) = picking_camera.intersect_top() {
            if let Ok(square) = squares_query.get(square_entity) {
                //Mark as selected
                selected_square.entity = Some(square_entity);

                if let Some(selected_piece_entity) = selected_piece.entity {
                    //Move the piece
                    if let Ok((_piece_entity, mut piece)) =
                        pieces_query.get_mut(selected_piece_entity)
                    {
                        piece.x = square.x;
                        piece.y = square.y;
                    }
                    selected_square.entity = None;
                    selected_piece.entity = None;
                } else {
                    for (piece_entity, piece) in pieces_query.iter_mut() {
                        if piece.x == square.x && piece.y == square.y {
                            selected_piece.entity = Some(piece_entity);
                            break;
                        }
                    }
                }
            }
        }
    } else {
        //Player clicked outside the board
        selected_square.entity = None;
        selected_piece.entity = None;
    }
}
