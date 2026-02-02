use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy_yoleck::prelude::*;
use bevy_yoleck::vpeol::prelude::*;

use crate::utils::CachedPbrMaker;

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        // Only one of these two `YoleckEntityType`s is actually needed - pick the one you want and
        // delete the irrelevant functions/types from this file.
        app.add_yoleck_entity_type({
            // This platform can freely be placed, resized, and rotated.
            YoleckEntityType::new("BoxPlatform")
                .with::<Vpeol3dPosition>()
                .with::<Vpeol3dScale>()
                .with::<Vpeol3dRotation>()
                .insert_on_init(|| IsPlatform)
        });
        app.add_yoleck_entity_type({
            // This platform cannot be rotated, and its position and size are aligned to a grid.
            YoleckEntityType::new("BlocksPlatform")
                .with::<Vpeol3dPosition>()
                .with::<Vpeol3dScale>()
                .insert_on_init(|| IsPlatform)
                .insert_on_init(|| AlignedToGrid)
        });

        app.add_systems(
            YoleckSchedule::Populate,
            (
                // Here, too, remove the one you don't use.
                populate_box_platform,
                // populate_blocks_platform,
            ),
        );
    }
}

#[derive(Component)]
pub struct IsPlatform;

#[derive(Component)]
struct AlignedToGrid;

fn populate_box_platform(
    mut populate: YoleckPopulate<(), (With<IsPlatform>, Without<AlignedToGrid>)>,
    mut pbr: CachedPbrMaker,
) {
    populate.populate(|ctx, mut cmd, _| {
        if ctx.is_first_time() {
            cmd.insert(pbr.make_pbr_with(
                || Mesh::from(Cuboid::new(1.0, 1.0, 1.0)),
                || StandardMaterial::from_color(css::GRAY),
            ));
        }
    });
}
