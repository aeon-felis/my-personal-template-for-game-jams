use bevy::prelude::*;
use bevy_yoleck::prelude::*;
use bevy_yoleck::vpeol_3d::*;

#[derive(Component)]
pub struct AlignedToGrid;

pub struct AlignToGridPlugin;

impl Plugin for AlignToGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_edit_system(align_to_grid_while_editing);
    }
}

fn align_to_grid_while_editing(mut edit: YoleckEdit<&mut Vpeol3dPosition, With<AlignedToGrid>>) {
    for mut vpeol_pos in edit.iter_matching_mut() {
        vpeol_pos.0 = vpeol_pos.0.round();
    }
}
