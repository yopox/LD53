use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{Animator, Delay, EaseFunction, Tween, TweenCompleted};
use bevy_tweening::lens::TransformPositionLens;

use crate::{GameState, util};
use crate::graphics::{MainBundle, sprite};
use crate::graphics::loading::Textures;
use crate::util::size::{tile_to_f32, WIDTH};

pub struct TransitionPlugin;

impl Plugin for TransitionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems((end_transition, start_transition))
        ;
    }
}

/// Resource to animate the game frame.
/// - Closing transition if [next_state] is Some.
/// - Opening transition else.
#[derive(Resource)]
pub struct Transition {
    in_progress: bool,
    next_state: Option<GameState>,
}

impl Transition {
    pub fn to(state: GameState) -> Self { Self { in_progress: false, next_state: Some(state) } }
    pub fn open() -> Self { Self { in_progress: false, next_state: None } }
    pub fn is_opening(&self) -> bool { self.next_state.is_none() }
}

#[derive(Component)]
pub struct TransitionPane;

pub(in crate::graphics) fn start_transition(
    mut commands: Commands,
    transition: Option<ResMut<Transition>>,
    textures: Option<Res<Textures>>,
    panes: Query<Entity, With<TransitionPane>>,
) {
    let Some(textures) = textures else { return; };
    let Some(mut transition) = transition else { return; };

    if !transition.in_progress {
        transition.in_progress = true;

        // Remove old panes
        panes.iter().for_each(|id| commands.entity(id).despawn_recursive());

        let open = transition.is_opening();
        let tween = |start, end| Tween::new(
            if open { EaseFunction::CubicIn } else { EaseFunction::CubicOut },
            Duration::from_millis(util::transition::SPEED),
            TransformPositionLens {
                start: Vec3::new(0., start, util::z_pos::TRANSITION),
                end: Vec3::new(0., end, util::z_pos::TRANSITION),
            },
        );

        // Spawn tiles
        let mut left_end_y = 0.;
        let mut left_start_y = -tile_to_f32(util::transition::HALF_HEIGHT);
        if open { (left_end_y, left_start_y) = (left_start_y, left_end_y); }
        commands
            .spawn(MainBundle::from_xyz(0., left_start_y, util::z_pos::TRANSITION))
            .insert(TransitionPane)
            .insert(Animator::new(
                tween(left_start_y, left_end_y)
                    .then(Delay::new(Duration::from_millis(util::tweening::DELAY))
                        .with_completed_event(util::tweening::TRANSITION_OVER)
                    )
            ))
            .with_children(|builder| {
                let last_y = util::transition::HALF_HEIGHT;
                for y in 0..last_y {
                    for x in 0..WIDTH {
                        let (index, bg, fg, rotation) = match (x, y) {
                            (_, y) if y == last_y - 1 => (23, 15, 3, 3),
                            (_, y) if y == last_y - 2 => (0, 15, 16, 0),
                            _ => (0, 9, 16, 0),
                        };
                        builder.spawn(sprite(
                            index, x, y, 0.,
                            bg.into(), fg.into(),
                            false, rotation, textures.tileset.clone(),
                        ));
                    }
                }
            });

        let mut right_end_y = tile_to_f32(util::transition::HALF_HEIGHT);
        let mut right_start_y = tile_to_f32(util::transition::HALF_HEIGHT * 2);
        if open { (right_end_y, right_start_y) = (right_start_y, right_end_y); }
        commands
            .spawn(MainBundle::from_xyz(0., right_start_y, util::z_pos::TRANSITION))
            .insert(TransitionPane)
            .insert(Animator::new(tween(right_start_y, right_end_y)))
            .with_children(|builder| {
                for y in 0..util::transition::HALF_HEIGHT {
                    for x in 0..WIDTH {
                        let (index, bg, fg, rotation) = match (x, y) {
                            (_, 0) => (23, 15, 3, 1),
                            (_, 1) => (0, 15, 16, 0),
                            _ => (0, 9, 16, 0),
                        };
                        builder.spawn(sprite(
                            index, x, y, 0.,
                            bg.into(), fg.into(),
                            false, rotation, textures.tileset.clone(),
                        ));
                    }
                }
            });
    }
}

pub(in crate::graphics) fn end_transition(
    mut commands: Commands,
    transition: Option<Res<Transition>>,
    mut tween_completed: EventReader<TweenCompleted>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Some(transition) = transition else { return; };

    for event in tween_completed.iter() {
        if event.user_data == util::tweening::TRANSITION_OVER {
            tween_completed.clear();

            if let Some(state) = transition.next_state {
                commands.insert_resource(Transition::open());
                next_state.set(state);
            } else {
                commands.remove_resource::<Transition>();
            }
            return;
        }
    }
}