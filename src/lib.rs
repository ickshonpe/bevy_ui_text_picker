use bevy::math::Vec4Swizzles;
use bevy::prelude::*;
use bevy::text::DefaultTextPipeline;
use bevy::window::WindowId;

#[derive(Clone, Debug, Default, Deref, DerefMut)]
#[derive(Component)]
pub struct TextRects(pub Vec<(usize, bevy::sprite::Rect)>);

pub fn extract_text_positions(
    texture_atlases: Res<Assets<TextureAtlas>>,
    text_pipeline: Res<DefaultTextPipeline>,
    windows: Res<Windows>,
    mut uinode_query: Query<(
        Entity,
        &Node,
        &GlobalTransform,
        &Text,
        &Visibility,
        &mut TextRects,
    )>,
) {

    let scale_factor = windows.scale_factor(WindowId::primary()) as f32;

    for (entity, uinode, transform, text, visibility, mut text_rects) in uinode_query.iter_mut() {
        text_rects.clear();
        if !visibility.is_visible {
            continue;
        }
        if uinode.size == Vec2::ZERO {
            continue;
        }
        if let Some(text_layout) = text_pipeline.get_glyphs(&entity) {
            let text_glyphs = &text_layout.glyphs;
            let alignment_offset = (uinode.size / -2.0).extend(0.0);

            for text_glyph in text_glyphs {
                let atlas = texture_atlases
                    .get(text_glyph.atlas_info.texture_atlas.clone_weak())
                    .unwrap();
                let index = text_glyph.atlas_info.glyph_index as usize;
                let rect = atlas.textures[index];

                let transform =
                    Mat4::from_rotation_translation(transform.rotation, transform.translation)
                        * Mat4::from_scale(transform.scale / scale_factor)
                        * Mat4::from_translation(
                            alignment_offset * scale_factor + text_glyph.position.extend(0.),
                        );

                text_rects.push((
                    text_glyph.section_index,
                    bevy::sprite::Rect { 
                        min: (transform * (-0.5 * rect.size().extend(0.)).extend(1.)).xy(),
                        max: (transform * (0.5 * rect.size().extend(0.)).extend(1.)).xy(),
                    }
                ));
            }
        }
    }
}

pub struct BevyTextPickerPlugin;

impl Plugin for BevyTextPickerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_to_stage(CoreStage::Last, extract_text_positions);
    }
}