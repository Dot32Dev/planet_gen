use bevy::ui;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy::render::mesh::{self, PrimitiveTopology, Indices};
use bevy::sprite::Mesh2dHandle;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_prototype_debug_lines::*;

mod generate_vertices;
use generate_vertices::generate_vertices;

#[derive(Default, Resource)]
struct UiState {
    detail_level: u32,
    z_value: f32,
    lerped: bool,
    animate: bool,
    x_y_scale: f32,
    wireframe: bool,
    darkness: f32,
    is_up_to_date: bool,
}

#[derive(Component)]
struct MarchingSquares {
    size: f32,
}

#[derive(Component)]
struct Wireframe {
    lines: Vec<(Vec3, Vec3)>,
}

fn main() {
    App::new()
        // .init_resource::<UiState>()
        .insert_resource(UiState {
            detail_level: 10,
            z_value: 0.0,
            lerped: true,
            animate: false,
            x_y_scale: 0.0225,
            wireframe: false,
            darkness: 0.54,
            is_up_to_date: false,
        })
        .insert_resource(ClearColor(Color::rgb(0.68, 0.97, 0.99)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Marching Squares".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(EguiPlugin)
        .add_plugin(DebugLinesPlugin::default())
        .add_startup_system(setup)
        .add_system(ui_example_system)
        .add_system(marching_squares_system)
        .add_system(wifreframe_update)
        .run();
}

fn setup(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // commands.spawn((
    //     MaterialMesh2dBundle {
    //         transform: Transform::from_xyz(0.0, 0.0, 0.1),
    //         material: materials.add(Color::rgb(108.0/255.0, 177.0/255.0, 5.0/255.0).into()),
    //         ..default()
    //     }, 
    //     MarchingSquares {
    //         size: 300.0,
    //     },
    // ));
    commands.spawn((
        MaterialMesh2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.2),
            material: materials.add(Color::rgb(0.61, 0.86, 0.26).into()),
            ..default()
        }, 
        MarchingSquares {
            size: 300.0,
        },
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.3),
            material: materials.add(Color::rgb(0.53, 0.5, 0.43).into()),
            ..default()
        }, 
        MarchingSquares {
            size: 140.0,
        },
    ));
}

fn ui_example_system(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
    time: Res<Time>,
) {
    egui::Window::new("Settings").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("FPS: {}", (1.0 / Time::delta_seconds(&time)).floor()));
        if ui.add(egui::Slider::new(&mut ui_state.detail_level, 3..=50).text("Grid Size (Lower is better)")).changed() {
            ui_state.is_up_to_date = false;
        }
        // ui.label("Z Value");
        if ui.add(egui::Slider::new(&mut ui_state.z_value, 0.0..=100.0).text("Z Coordinate")).changed() {
            ui_state.is_up_to_date = false;
        }
        // ui.add(egui::Slider::new(&mut ui_state.x_y_scale, 0.001..=0.04).text("X/Y Scale (Zoom)"));
        // ui.add(egui::Slider::new(&mut ui_state.darkness, 0.0..=1.0).text("Darkness"));
        if ui.add(egui::Slider::new(&mut ui_state.x_y_scale, 0.001..=0.04).text("X/Y Scale (Zoom)")).changed() {
            ui_state.is_up_to_date = false;
        }
        if ui.add(egui::Slider::new(&mut ui_state.darkness, 0.0..=1.0).text("Darkness")).changed() {
            ui_state.is_up_to_date = false;
        }
        // ui.label("Lerped or midpoint");
        // ui.checkbox(&mut ui_state.lerped, "Lerped");
        if ui.add(egui::Checkbox::new(&mut ui_state.lerped, "Lerped")).changed() {
            ui_state.is_up_to_date = false;
        }
        ui.checkbox(&mut ui_state.animate, "Animate");
        // ui.checkbox(&mut ui_state.wireframe, "Wireframe");
        if ui.add(egui::Checkbox::new(&mut ui_state.wireframe, "Wireframe")).changed() {
            ui_state.is_up_to_date = false;
        }
    });

    if ui_state.animate {
        ui_state.z_value += 0.01;
        ui_state.is_up_to_date = false;
    }
}

fn marching_squares_system(
    mut ui_state: ResMut<UiState>,
    mut marching_squares_meshes: Query<(&mut Mesh2dHandle, &MarchingSquares, Entity)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    // println!("marching_squares_meshes: {:?}", marching_squares_meshes.iter().len());
    if !ui_state.is_up_to_date {
        for (mut mesh_handle, marching_square, entity) in marching_squares_meshes.iter_mut() {
            // let mut mesh = meshes.get_mut(mesh_handle);
            // if mesh.is_some() {
            //     let (positions, normals, uvs, indices) = generate_tiles(ui_state.detail_level as f32, ui_state.z_value as f32, ui_state.lerped);
                
            //     let mut new_mesh = Mesh::new(PrimitiveTopology::TriangleList);
            //     new_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
            //     new_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
            //     new_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
            //     new_mesh.set_indices(Some(Indices::U32(indices)));

            //     mesh = Some(&mut new_mesh);

            //     println!("Updated mesh")
            // }
            let (mut positions, normals, uvs, mut indices) = generate_vertices(ui_state.detail_level as f32, ui_state.z_value as f32, ui_state.lerped, ui_state.x_y_scale, ui_state.darkness);
                
            // Remove triangles that are outside of the circle
            // let mut to_remove = Vec::new();
            for i in (0..indices.len() / 3).rev() {
                if indices.len() >= i + 3 {
                    let i = i * 3;
                    let a = positions[indices[i] as usize];
                    let b = positions[indices[i + 1] as usize];
                    let c = positions[indices[i + 2] as usize];
                    let dist_a = Vec3::new(a[0], a[1], 0.0).length();
                    let dist_b = Vec3::new(b[0], b[1], 0.0).length();
                    let dist_c = Vec3::new(c[0], c[1], 0.0).length();
                    if dist_a > marching_square.size && dist_b > marching_square.size && dist_c > marching_square.size {
                        // positions.remove(indices[i + 2] as usize);
                        // positions.remove(indices[i + 1] as usize);
                        // positions.remove(indices[i] as usize);
                        indices.remove(i + 2);
                        indices.remove(i + 1);
                        indices.remove(i);
                    }
                }
            }
            // for i in to_remove.iter() {
            //     indices.remove(to_remove[*i] as usize);
            //     positions.remove(to_remove[*i] as usize);
            // }
            
            // Clap vertices to a radius of 300
            for i in 0..positions.len() {
                let mut pos = Vec3::new(positions[i][0], positions[i][1], 0.0);
                let dist = pos.length();
                if dist > marching_square.size {
                    pos = pos.normalize() * marching_square.size;
                }
                positions[i] = pos.into();
            }
            
            if ui_state.wireframe {
                let mut wireframe = Vec::new();
                // Draw debug lines for every triangle (fake wireframe)
                for i in 0..indices.len() / 3 {
                    let i = i * 3;
                    let a = positions[indices[i] as usize];
                    let b = positions[indices[i + 1] as usize];
                    let c = positions[indices[i + 2] as usize];
                    // lines.line_colored(a.extend(0.1), b.extend(0.1), 0.1, Color::RED);
                    // lines.line_colored(b.extend(0.1), c.extend(0.1), 0.1, Color::RED);
                    // lines.line_colored(c.extend(0.1), a.extend(0.1), 0.1, Color::RED);
                    // lines.line_colored(Vec3::new(a[0], a[1], 0.1), Vec3::new(b[0], b[1], 0.1), 0.0, Color::BLACK);
                    // lines.line_colored(Vec3::new(b[0], b[1], 0.1), Vec3::new(c[0], c[1], 0.1), 0.0, Color::BLACK);
                    // lines.line_colored(Vec3::new(c[0], c[1], 0.1), Vec3::new(a[0], a[1], 0.1), 0.0, Color::BLACK);
                    wireframe.push((Vec3::new(a[0], a[1], 0.1), Vec3::new(b[0], b[1], 0.1)));
                    wireframe.push((Vec3::new(b[0], b[1], 0.1), Vec3::new(c[0], c[1], 0.1)));
                    wireframe.push((Vec3::new(c[0], c[1], 0.1), Vec3::new(a[0], a[1], 0.1)));
                }
                // Wireframe {
                //     lines: vec![],
                // },
                commands.entity(entity).insert(Wireframe {
                    lines: wireframe,
                });
                // match wireframe_component {
                //     Some(mut wireframe_comp) => {
                //         wireframe_comp.lines = wireframe;
                //     },
                //     None => ()
                // }
            } else {
                commands.entity(entity).remove::<Wireframe>();
            }

            let mut new_mesh = Mesh::new(PrimitiveTopology::TriangleList);
            new_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
            new_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
            new_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
            new_mesh.set_indices(Some(Indices::U32(indices)));

            *mesh_handle = meshes.add(new_mesh).into();
        }
        ui_state.is_up_to_date = true;
    }
}

fn wifreframe_update(
    query: Query<(&Wireframe)>,
    mut lines: ResMut<DebugLines>,
) {
    for wireframe in query.iter() {
        for line in wireframe.lines.iter() {
            lines.line_colored(line.0, line.1, 0.0, Color::BLACK);
        }
    }
}