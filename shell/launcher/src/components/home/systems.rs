use bevy::prelude::*;

use crate::components::home::styles::VIEWPORT_SIZE;

#[derive(Component, Debug)]
#[require(ScrollbarDragState)]
pub struct CoreScrollArea;

/// Component used to manage the state of a scrollbar during dragging.
#[derive(Component, Default)]
pub struct ScrollbarDragState {
    /// Whether the scrollbar is currently being dragged.
    dragging: bool,
    /// The value of the scrollbar when dragging started.
    offset: ScrollPosition,
}

pub fn on_drag_start(
    mut trigger: Trigger<Pointer<DragStart>>,
    mut q_scroll_area: Query<(&ScrollPosition, &mut ScrollbarDragState), With<CoreScrollArea>>,
) {
    if let Ok((scroll_pos, mut drag)) = q_scroll_area.get_mut(trigger.target()) {
        trigger.propagate(false);
        drag.dragging = true;
        drag.offset = scroll_pos.clone();
    };
}

pub fn on_drag_end(
    mut trigger: Trigger<Pointer<Drag>>,
    mut q_scroll_area: Query<
        (&mut ScrollPosition, &ComputedNode, &mut ScrollbarDragState),
        With<CoreScrollArea>,
    >,
) {
    //if drag started on scroll content
    if let Ok((mut scroll_pos, scroll_content, drag)) = q_scroll_area.get_mut(trigger.target()) {
        trigger.propagate(false);
        if drag.dragging {
            let distance = trigger.event().distance;
            let visible_size = scroll_content.size() * scroll_content.inverse_scale_factor;
            let content_size = scroll_content.content_size() * scroll_content.inverse_scale_factor;
            let x_range = (content_size.x - visible_size.x).max(0.);

             // --- Snap scrolling to viewport width increments ---
            let mut new_offset_x = if x_range > 0. {
                (drag.offset.offset_x - distance.x).clamp(0., x_range)
            } else {
                0.
            };
            // Snap to nearest viewport width increment
            new_offset_x = (new_offset_x / VIEWPORT_SIZE).round() * VIEWPORT_SIZE;
            // Clamp to valid range
            new_offset_x = new_offset_x.clamp(0., x_range);
            scroll_pos.offset_x = new_offset_x;

            let y_range = (content_size.y - visible_size.y).max(0.);
            scroll_pos.offset_y = if y_range > 0. {
                (drag.offset.offset_y - distance.y).clamp(0., y_range)
            } else {
                0.
            };
        }
    }
}

pub struct CoreScrollbarPlugin;

impl Plugin for CoreScrollbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_drag_start).add_observer(on_drag_end);
    }
}
