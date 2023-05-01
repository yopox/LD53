use std::time::Duration;

use bevy::math::{vec3, Vec3Swizzles};
use bevy::prelude::*;
use bevy_tweening::{Animator, Tween};
use bevy_tweening::EaseMethod::Linear;
use bevy_tweening::lens::TransformPositionLens;
use strum_macros::EnumIter;

use crate::battle::{BattleUI, CursorState, Money};
use crate::collision::body_size;
use crate::drones::Enemy;
use crate::graphics::{MainBundle, sprite_from_tile, sprites};
use crate::graphics::grid::{Grid, GridElement};
use crate::graphics::gui::{HoveredPos, HoverPopup};
use crate::graphics::loading::Textures;
use crate::graphics::sprites::TILE;
use crate::logic::tower_stats;
use crate::logic::tower_stats::{MAX_DAMAGE, MAX_RELOAD, MIN_DAMAGE, MIN_RELOAD};
use crate::shot::Shots;
use crate::util;
use crate::util::{with_z, z_pos};
use crate::util::misc::SLOW_DOWN_DELAY;
use crate::util::tweening::SHOT_DESPAWN;

#[derive(Component, Clone)]
pub struct Tower {
    pub model: Towers,
    pub rank: u8,
    x: usize,
    y: usize,
}

impl Tower {
    pub fn reload_delay(&self) -> f32 { tower_stats::reload_delay(self) }
    pub fn range(&self) -> f32 { tower_stats::range(self) }
    pub fn damage(&self) -> f32 { tower_stats::damage(self) }
    pub fn shot_speed(&self) -> f32 { tower_stats::shot_speed(self) }
    pub fn slow_factor(&self) -> f32 { tower_stats::slow_factor(self) }

    pub fn upgrade_cost(&self) -> Option<u16> {
        match self.rank {
            1 => Some(2 * self.model.get_cost()),
            2 => Some(4 * self.model.get_cost()),
            _ => None,
        }
    }

    pub fn sell_price(&self) -> u16 {
        match self.rank {
            1 => self.model.get_cost() / 2,
            2 => self.model.get_cost() * 3 / 2,
            _ => self.model.get_cost() * 7 / 2,
        }
    }

    pub fn get_name(&self) -> &str {
        match self.model {
            Towers::Lightning => "Lightning Tower",
            Towers::PaintBomb => "Paint Bomb",
            Towers::Scrambler => "Scrambler",
        }
    }

    pub fn get_description(&self) -> String {
        match self.upgrade_cost() {
            Some(n) => format!("Rank {} (up: €{})", self.rank, n),
            None => format!("Rank {} (rank max)", self.rank),
        }
    }

    pub fn get_attr1(&self) -> Option<(String, u8)> {
        match self.model {
            Towers::Lightning | Towers::PaintBomb => Some((
                "Damage".to_string(),
                ((self.damage() - MIN_DAMAGE) / (MAX_DAMAGE - MIN_DAMAGE) * 9.0) as u8 + 1,
            )),
            Towers::Scrambler => Some((
                "Slowdown".to_string(),
                self.rank * 3,
            )),
        }
    }

    pub fn get_attr2(&self) -> Option<(String, u8)> {
        Some((
            "Speed".to_string(),
            ((1. / self.reload_delay() - 1. / MAX_RELOAD) / (1. / MIN_RELOAD - 1. / MAX_RELOAD) * 9.0) as u8 + 1,
        ))
    }
}

#[derive(Debug, Copy, Clone, EnumIter, PartialEq)]
pub enum Towers {
    Lightning,
    PaintBomb,
    Scrambler,
}

#[derive(Component)]
pub struct JustFired {
    /// time of firing, as an interval from startup time
    t0: Duration,
    /// Reloading time, as an interval from t0
    delay: Duration,
}

impl JustFired {
    pub fn new(time: &Time, delay: f32) -> Self {
        Self { t0: time.elapsed(), delay: Duration::from_secs_f32(delay) }
    }
}

impl Towers {
    pub const fn instantiate(&self, x: usize, y: usize) -> Tower {
        Tower { model: *self, rank: 1, x, y }
    }

    pub const fn get_shot(&self) -> Option<Shots> {
        match self {
            Towers::Lightning => Some(Shots::Electricity),
            Towers::PaintBomb => Some(Shots::Bomb),
            Towers::Scrambler => None,
        }
    }

    pub const fn get_tiles(&self) -> &[TILE] {
        match &self {
            Towers::Lightning => &sprites::TOWER_1,
            Towers::PaintBomb => &sprites::TOWER_2,
            Towers::Scrambler => &sprites::TOWER_3,
        }
    }

    /// Returns the delay on tower construction
    pub const fn initial_delay(&self) -> f32 {
        2.
    }

    pub const fn get_cost(&self) -> u16 {
        match self {
            Towers::Lightning => 40,
            Towers::PaintBomb => 60,
            Towers::Scrambler => 50,
        }
    }
}

/// Place a tower on (x, y) in grid coordinates.
pub fn place_tower(
    x: usize, y: usize,
    commands: &mut Commands,
    tower: Towers, atlas: &Handle<TextureAtlas>,
    time: &Time,
) {
    let tower = tower.instantiate(x, y);
    let size = body_size(tower.model.get_tiles());
    let tower_pos = util::grid_to_tower_pos(x, y, tower.model);
    commands
        .spawn(tower.clone())
        .insert(
            MainBundle::from_xyz(tower_pos.x, tower_pos.y, z_pos::TOWERS)
        )
        .with_children(|builder|
            sprite_from_tile(builder, tower.model.get_tiles(), atlas, 0.)
        )
        .insert(JustFired::new(time, tower.model.initial_delay()))
        .insert(HoverPopup::new(
            tower.get_name(),
            &tower.get_description(),
            tower.get_attr1(), tower.get_attr2(),
            size.x, size.y,
        ))
        .insert(BattleUI)
        .insert(GridElement)
    ;
}

pub fn sell_tower(
    mut commands: Commands,
    towers: Query<(&Tower, Entity)>,
    mouse: Res<Input<MouseButton>>,
    cursor_state: Option<ResMut<CursorState>>,
    hovered: Option<Res<HoveredPos>>,
    grid: Option<ResMut<Grid>>,
    money: Option<ResMut<Money>>,
) {
    let Some(mut cursor_state) = cursor_state else { return; };

    if !mouse.just_pressed(MouseButton::Left) { return; }
    if cursor_state.ne(&CursorState::Sell) { return; }

    let Some(hovered) = hovered else { return; };
    let Some(mut grid) = grid else { return; };

    let pos = &(hovered.0.0, hovered.0.1);
    if !grid.towers.contains(pos) { return; }

    let Some(mut money) = money else { return; };

    for (t, id) in &towers {
        if t.x == pos.0 && t.y == pos.1 {
            // Actually sell tower
            grid.towers.remove(pos);
            commands.entity(id).despawn_recursive();
            cursor_state.set_if_neq(CursorState::Select);
            money.0 += t.sell_price();
        }
    }
}

pub fn upgrade_tower(
    mut towers: Query<(&mut Tower, &mut HoverPopup)>,
    mouse: Res<Input<MouseButton>>,
    cursor_state: Option<ResMut<CursorState>>,
    hovered: Option<Res<HoveredPos>>,
    money: Option<ResMut<Money>>,
) {
    let Some(mut cursor_state) = cursor_state else { return; };

    if !mouse.just_pressed(MouseButton::Left) { return; }
    if cursor_state.ne(&CursorState::Upgrade) { return; }

    let Some(hovered) = hovered else { return; };
    let Some(mut money) = money else { return; };
    let pos = &(hovered.0.0, hovered.0.1);

    for (mut t, mut hp) in towers.iter_mut() {
        if t.x == pos.0 && t.y == pos.1 {
            match t.upgrade_cost() {
                Some(cost) if cost <= money.0 => {
                    // Actually upgrade tower
                    money.0 -= cost;
                    t.rank += 1;
                    hp.description = t.get_description();
                    hp.attr1 = t.get_attr1();
                    hp.attr2 = t.get_attr2();
                    hp.force_redraw = true;
                    cursor_state.set_if_neq(CursorState::Select);
                }
                Some(_) => {
                    // Not enough money
                }
                None => {
                    // Level max
                }
            }

            return;
        }
    }
}

pub fn tower_fire(
    towers: Query<(Entity, &Transform, &Tower), Without<JustFired>>,
    enemies: Query<(Entity, &Transform, &Enemy)>,
    mut commands: Commands,
    time: Res<Time>,
    textures: Res<Textures>,
) {
    for (e_tower, &t_tower, tower) in towers.iter() {
        match tower.model {
            Towers::Lightning | Towers::PaintBomb => {
                let chosen_enemy = enemies.iter()
                    .filter(|(_, t, _)| t.translation.xy().distance(t_tower.translation.xy()) <= tower.range())
                    .max_by_key(|(_, _, enemy)| (enemy.advance * 4096.) as usize);

                if let Some((_, t_enemy, _)) = chosen_enemy {
                    shoot(&mut commands, &textures, t_tower, tower, t_enemy.translation);
                }
            }
            Towers::Scrambler => {
                enemies.iter()
                    .filter(|(_, t, _)| t.translation.xy().distance(t_tower.translation.xy()) <= tower.range())
                    .for_each(|(e, _, _)| {
                        if let Some(mut entity_commands) = commands.get_entity(e) {
                            entity_commands.insert(Slow {
                                factor: tower.slow_factor(),
                                t_final: time.elapsed() + Duration::from_secs_f32(SLOW_DOWN_DELAY),
                            });
                        }
                    })
                ;
            }
        }

        if let Some(mut entity_commands) = commands.get_entity(e_tower) {
            entity_commands.insert(JustFired::new(&time, tower.reload_delay()));
        }
    }
}

fn shoot(commands: &mut Commands, textures: &Res<Textures>, t_tower: Transform, tower: &Tower, enemy_position: Vec3) {
    let distance = t_tower.translation.distance(enemy_position);
    let shot_translation = vec3(t_tower.translation.x, t_tower.translation.y + 1., z_pos::SHOT);
    let shot_kind = tower.model.get_shot().expect("The tower can't shoot!");
    let shot = shot_kind.instantiate(tower);
    commands
        .spawn(shot.clone())
        .insert(MainBundle::from_translation(shot_translation))
        .insert(Animator::new(Tween::new(
            Linear,
            Duration::from_secs_f32(tower.range() / shot.0.speed),
            TransformPositionLens {
                start: shot_translation,
                end: with_z(
                    shot_translation + (enemy_position - shot_translation) * tower.range() / distance,
                    z_pos::SHOT),
            },
        ).with_completed_event(SHOT_DESPAWN)))
        .with_children(|builder|
            sprite_from_tile(builder, &[shot_kind.get_tile()], &textures.tileset, 0.)
        )
        .insert(BattleUI)
    ;
}

pub fn update_just_fired(
    query: Query<(Entity, &JustFired)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (e, just_fired) in query.iter() {
        if just_fired.t0 + just_fired.delay <= time.elapsed() {
            if let Some(mut entity_commands) = commands.get_entity(e) {
                entity_commands.remove::<JustFired>();
            }
        }
    }
}

#[derive(Component)]
pub struct Slow {
    pub factor: f32,
    t_final: Duration,
}

pub fn remove_slow_down(
    slown_down: Query<(Entity, &Slow)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (e, &Slow { t_final, .. }) in slown_down.iter() {
        if t_final <= time.elapsed() {
            if let Some(mut entity_commands) = commands.get_entity(e) {
                entity_commands.remove::<Slow>();
            }
        }
    }
}