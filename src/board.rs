use bevy::prelude::*;
use bevy_mod_picking::*;

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SelectedSquare>()
            .init_resource::<SquareMaterials>()
            .add_startup_system(create_board.system())
            .add_system(color_squares.system());
    }
}

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

struct SquareMaterials {
    highlight_color: Handle<StandardMaterial>,
    selected_color: Handle<StandardMaterial>,
    white_color: Handle<StandardMaterial>,
    black_color: Handle<StandardMaterial>,
}

impl FromResources for SquareMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<StandardMaterial>>().unwrap();
        SquareMaterials {
            highlight_color: materials.add(Color::rgb(0.8, 0.3, 0.3).into()),
            selected_color: materials.add(Color::rgb(0.9, 0.1, 0.1).into()),
            white_color: materials.add(Color::rgb(1.0, 0.9, 0.9).into()),
            black_color: materials.add(Color::rgb(0.0, 0.1, 0.1).into()),
        }
    }
}

fn create_board(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<SquareMaterials>,
) {
    // Add meshes and materials
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1.0 }));

    // Spawn 64 squares
    for i in 0..8 {
        for j in 0..8 {
            commands
                .spawn(PbrBundle {
                    mesh: mesh.clone(),
                    // Change material according to position to get alternating pattern
                    material: if (i + j + 1) % 2 == 0 {
                        materials.white_color.clone()
                    } else {
                        materials.black_color.clone()
                    },
                    transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                    ..Default::default()
                })
                .with(PickableMesh::default())
                .with(Square { x: i, y: j });
        }
    }
}

fn color_squares(
    pick_state: Res<PickState>,
    selected_square: Res<SelectedSquare>,
    materials: Res<SquareMaterials>,
    mut query: Query<(Entity, &Square, &mut Handle<StandardMaterial>)>,
) {
    //Get entity under cursor if avaialable
    let top_entity = if let Some((entity, _intersection)) = pick_state.top(Group::default()) {
        Some(*entity)
    } else {
        None
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
