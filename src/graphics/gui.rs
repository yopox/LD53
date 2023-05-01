use bevy::app::App;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_text_mode::TextModeTextureAtlasSprite;
use strum::IntoEnumIterator;

use crate::{GameState, tower, util};
use crate::battle::{BattleUI, CursorState, Money};
use crate::collision::body_size;
use crate::graphics::{circle, MainBundle, sprite, sprite_f32, sprite_from_tile_with_alpha, sprite_from_tile_with_alpha_and_x_offset, text};
use crate::graphics::circle::Circles;
use crate::graphics::grid::{Grid, RoadElement};
use crate::graphics::loading::{Fonts, Textures};
use crate::graphics::palette::Palette;
use crate::graphics::text::TextStyles;
use crate::music::{PlaySfxEvent, SFX};
use crate::tower::{Tower, Towers};
use crate::util::{is_in, z_pos};
use crate::util::size::{f32_tile_to_f32, is_oob, tile_to_f32};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(setup.in_schedule(OnEnter(GameState::Battle)))
            .add_systems(
                (update_money, update_cursor, update_popup, update_tower_button,
                 update_text_button, place_tower, show_radius)
                    .in_set(OnUpdate(GameState::Battle)))
        ;
    }
}

#[derive(Component)]
struct Cursor;

#[derive(Resource)]
pub struct HoveredPos(pub (usize, usize));

#[derive(Component)]
struct TowerButton(Towers);

#[derive(Component, Clone, PartialEq)]
enum TextButton {
    Upgrade,
    Sell,
    X2,
    Pause,
}

impl TextButton {
    fn get_text(&self) -> &str {
        // Change get_size after changing strings as width is hardcoded :(
        match self {
            TextButton::Upgrade => "Upgrade",
            TextButton::Sell => "Sell",
            TextButton::X2 => "Turbo",
            TextButton::Pause => "Pause",
        }
    }

    fn get_size(&self) -> Vec2 {
        match self {
            TextButton::Upgrade => Vec2::new(f32_tile_to_f32(5.75), f32_tile_to_f32(1.25)),
            TextButton::Sell => Vec2::new(f32_tile_to_f32(2.5), f32_tile_to_f32(1.25)),
            TextButton::X2 => Vec2::new(f32_tile_to_f32(3.3), f32_tile_to_f32(1.25)),
            TextButton::Pause => Vec2::new(f32_tile_to_f32(3.3), f32_tile_to_f32(1.25)),
        }
    }
}

#[derive(Component)]
struct MoneyText;

fn setup(
    mut commands: Commands,
    fonts: Res<Fonts>,
    textures: Res<Textures>,
) {
    // Text
    let left_margin = f32_tile_to_f32(2.);
    for (x, y, text, style) in [
        (left_margin, f32_tile_to_f32(3.6), "Level 1", TextStyles::Heading),
        (left_margin, f32_tile_to_f32(2.1), "Haunted streets", TextStyles::Body),
    ] {
        commands
            .spawn(text::ttf(
                x, y, z_pos::GUI_FG,
                text, style, &fonts, Palette::D,
            ))
            .insert(BattleUI)
        ;
    }

    commands
        .spawn(text::ttf(
            left_margin, f32_tile_to_f32(0.5), z_pos::GUI_FG,
            "€0", TextStyles::Heading, &fonts, Palette::D,
        ))
        .insert(MoneyText)
        .insert(BattleUI)
    ;

    // Background
    for x in 0..util::size::WIDTH {
        for y in 0..util::size::GUI_HEIGHT {
            let (i, bg, fg) = match y {
                5 => (418, Palette::E, Palette::D),
                _ => (416, Palette::E, Palette::Transparent),
            };
            commands.spawn(sprite(
                i, x, y, z_pos::GUI_BG,
                bg, fg, false, 0,
                textures.tileset.clone(),
            )).insert(BattleUI);
        }
    }

    // Cursor
    commands
        .spawn(MainBundle::from_xyz(0., 0., z_pos::CURSOR))
        .with_children(|builder| {
            for (x, y, r) in [(0, 1, 0), (1, 1, 1), (1, 0, 2), (0, 0, 3)] {
                builder.spawn(sprite(
                    417, x, y, 0.,
                    Palette::Transparent, Palette::B,
                    false, r, textures.tileset.clone(),
                ));
            }
        })
        .insert(Cursor)
        .insert(BattleUI)
    ;

    // Tower buttons
    for (i, tower) in Towers::iter().enumerate() {
        let width = body_size(tower.get_tiles()).x;
        commands
            .spawn(TowerButton(tower))
            .insert(MainBundle::from_xyz(tile_to_f32(14 + 4 * i), f32_tile_to_f32(2.), z_pos::GUI_FG))
            .with_children(|builder| {
                sprite_from_tile_with_alpha_and_x_offset(builder, tower.get_tiles(), &textures.tileset, 0., ButtonState::CanBuild.get_alpha(), (tile_to_f32(2) - width) / 2.);
                builder.spawn(text::ttf_anchor(
                    f32_tile_to_f32(1.0), f32_tile_to_f32(0.3), z_pos::GUI_FG,
                    &format!("€{}", tower.get_cost()),
                    TextStyles::Heading, &fonts, Palette::D,
                    Anchor::TopCenter,
                ));
            })
            .insert(BattleUI);
    }

    // Text buttons
    for (x, y, b) in [
        (util::size::WIDTH as f32 - 2., 4.75, TextButton::Upgrade),
        (util::size::WIDTH as f32 - 2., 3.0, TextButton::Sell),
        (util::size::WIDTH as f32 - 6., 1.25, TextButton::Pause),
        (util::size::WIDTH as f32 - 2., 1.25, TextButton::X2),
    ] {
        commands
            .spawn(text::ttf_anchor(
                f32_tile_to_f32(x), f32_tile_to_f32(y), z_pos::GUI_FG,
                b.get_text(), TextStyles::Heading, &fonts, Palette::D,
                Anchor::CenterRight,
            ))
            .insert(b.clone())
            .insert(BattleUI)
        ;
    }
}

fn update_money(
    money: Res<Money>,
    mut text: Query<&mut Text, With<MoneyText>>,
) {
    if money.is_changed() || money.is_added() {
        let mut text = text.single_mut();
        text.sections[0].value = format!("€{}", money.0);
    }
}

fn update_cursor(
    mut commands: Commands,
    grid: Option<Res<Grid>>,
    windows: Query<&Window>,
    mut cursor: Query<(&mut Transform, &mut Visibility, Entity), With<Cursor>>,
    children: Query<&Children>,
    mut tile: Query<&mut TextModeTextureAtlasSprite>,
    hovered: Option<ResMut<HoveredPos>>,
    cursor_state: Option<Res<CursorState>>,
) {
    let mut clean = || commands.remove_resource::<HoveredPos>();
    let Ok((mut pos, mut vis, id)) = cursor.get_single_mut() else {
        clean();
        return;
    };
    vis.set_if_neq(Visibility::Hidden);

    let Some(grid) = grid else {
        clean();
        return;
    };
    let Some(cursor_pos) = util::cursor_pos(windows) else {
        clean();
        return;
    };

    // Update cursor color
    if let Some(cursor_state) = cursor_state {
        if cursor_state.is_changed() {
            for child in children.iter_descendants(id) {
                let Ok(mut sprite) = tile.get_mut(child) else { continue; };
                sprite.fg = cursor_state.get_color().into();
            }
        }
    }

    // Get hovered tile
    let tile_size = tile_to_f32(1);
    let (x, y) = (cursor_pos.x / tile_size, cursor_pos.y / tile_size);
    let (x, y) = (x as isize, y as isize - util::size::GUI_HEIGHT as isize);
    let (x, y) = (x / 2, y / 2);

    // Set visibility and position on [grid::RoadElement::Rock] hover.
    if is_oob(x, y) { return; }

    if grid.elements[y as usize][x as usize] == RoadElement::Rock {
        vis.set_if_neq(Visibility::Inherited);
        let new_hovered = (x as usize, y as usize);
        match hovered {
            Some(mut res) if res.0.0 != new_hovered.0 || res.0.1 != new_hovered.1 => res.0 = new_hovered,
            Some(_) => {},
            None => commands.insert_resource(HoveredPos(new_hovered)),
        }
        pos.translation.x = tile_to_f32(x as usize * 2);
        pos.translation.y = tile_to_f32(y as usize * 2 + util::size::GUI_HEIGHT);
    } else {
        clean();
    }
}

/// Add this component to entities that should display a popup on hover.
#[derive(Component)]
pub struct HoverPopup {
    name: String,
    pub description: String,
    pub attr1: Option<(String, u8)>,
    pub attr2: Option<(String, u8)>,
    width: f32,
    height: f32,
    pub force_redraw: bool,
}

impl HoverPopup {
    pub fn new(name: &str, description: &str, attr1: Option<(String, u8)>, attr2: Option<(String, u8)>, width: f32, height: f32) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            attr1,
            attr2,
            width,
            height,
            force_redraw: false,
        }
    }
}

/// A 12*6 information popup showed on [Information] hover
#[derive(Component)]
struct Popup(Entity);

fn update_popup(
    mut commands: Commands,
    mut hover_popup: Query<(&Transform, &mut HoverPopup, Entity), Without<Popup>>,
    popup: Query<(&Popup, Entity), Without<HoverPopup>>,
    windows: Query<&Window>,
    textures: Res<Textures>,
    fonts: Res<Fonts>,
) {
    let Some(cursor_pos) = util::cursor_pos(windows) else { return; };

    for (pos, mut info, id) in hover_popup.iter_mut() {
        if is_in(cursor_pos, pos.translation.xy(), Vec2::new(info.width, info.height)) {
            let mut recreate_popup = info.force_redraw;
            info.force_redraw = false;

            match popup.get_single() {
                Ok((p, popup_id)) if p.0 == id => {
                    // Popup exists -> correct entity -> do nothing
                    if recreate_popup { commands.entity(popup_id).despawn_recursive(); }
                }
                Ok((_, popup_id)) => {
                    // Popup exists -> wrong entity -> respawn popup
                    commands.entity(popup_id).despawn_recursive();
                    recreate_popup = true;
                }
                _ => {
                    // Popup doesn't exist -> spawn a popup
                    recreate_popup = true;
                }
            }

            if recreate_popup { spawn_popup(&mut commands, pos, id, &info, &textures, &fonts); }
            return;
        }
    }

    if let Ok((_, id)) = popup.get_single() { commands.entity(id).despawn_recursive(); }
}

fn spawn_popup(
    commands: &mut Commands,
    owner_pos: &Transform,
    owner_id: Entity,
    info: &HoverPopup,
    textures: &Res<Textures>,
    fonts: &Res<Fonts>,
) {
    commands
        .spawn(MainBundle::from_xyz(
            owner_pos.translation.x + info.width + f32_tile_to_f32(0.5),
            owner_pos.translation.y + info.height - tile_to_f32(3),
            z_pos::POPUP_BG,
        ))
        .insert(Popup(owner_id))
        .insert(BattleUI)
        .with_children(|builder| {
            for y in 0..6 {
                for x in 0..12 {
                    let (i, r) = match (x, y) {
                        (0, 0) => (420, 3),
                        (11, 0) => (420, 2),
                        (11, 5) => (420, 1),
                        (0, 5) => (420, 0),
                        _ => (421, 0)
                    };
                    let mut bundle = sprite(
                        i, x, y, 0.,
                        Palette::Transparent, Palette::F,
                        false, r,
                        textures.tileset.clone(),
                    );
                    bundle.sprite.alpha = 0.75;
                    builder.spawn(bundle);
                }
            }

            let fg_z = z_pos::POPUP_FG - z_pos::POPUP_BG;

            for (text, style, y) in [(&info.name, TextStyles::Heading, 3.9), (&info.description, TextStyles::Body, 2.6)] {
                builder.spawn(text::ttf(
                    f32_tile_to_f32(1.), f32_tile_to_f32(y), fg_z,
                    text, style,
                    &fonts, Palette::B,
                ));
            }

            for (attr, y) in [(&info.attr1, 1.3), (&info.attr2, 0.3)] {
                if let Some((t, i)) = attr {
                    builder.spawn(text::ttf(
                        f32_tile_to_f32(1.), f32_tile_to_f32(y), fg_z,
                        t, TextStyles::Body,
                        &fonts, Palette::B,
                    ));

                    for x in 0..10 {
                        builder.spawn(sprite_f32(
                            419, f32_tile_to_f32(x as f32 / 2. + 6.), f32_tile_to_f32(y + 7. / 16.), fg_z,
                            Palette::Transparent, if x < *i { Palette::K } else { Palette::P },
                            false, 0, textures.tileset.clone(),
                        ));
                    }
                }
            }
        });
}

#[derive(Component)]
struct RadiusInfo(usize, usize);

fn show_radius(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    hovered_pos: Option<Res<HoveredPos>>,
    radius: Query<(&RadiusInfo, Entity)>,
    towers: Query<&Tower>,
    circles: Res<Circles>,
) {
    if let Ok((info, id)) = radius.get_single() {
        // Radius is already displayed
        if let Some(hovered_pos) = hovered_pos {
            let (x, y) = (hovered_pos.0.0, hovered_pos.0.1);
            if info.0 == x && info.1 == y {
                // Hover on the same tile: OK
            } else {
                // Hover on a different tile
                // Delete existing radius
                commands.entity(id).despawn_recursive();

                // Spawn new radius if needed
                for tower in &towers {
                    if tower.x == x && tower.y == y {
                        // Show this tower radius
                        spawn_radius(&mut commands, &mut materials, &circles, x, y, tower);
                        return;
                    }
                }
            }
        } else {
            // Delete existing radius
            commands.entity(id).despawn_recursive();
        }
    } else {
        // Radius isn't displayed
        let Some(hovered_pos) = hovered_pos else { return; };
        let (x, y) = (hovered_pos.0.0, hovered_pos.0.1);
        for tower in &towers {
            if tower.x == x && tower.y == y {
                // Show this tower radius
                spawn_radius(&mut commands, &mut materials, &circles, x, y, tower);
                return;
            }
        }
    }
}

fn spawn_radius(commands: &mut Commands, materials: &mut ResMut<Assets<ColorMaterial>>, circles: &Res<Circles>, x: usize, y: usize, tower: &Tower) {
    let radius_f32 = tower.range();
    let handle = materials.add(Palette::B.transparent(0.1).into());
    let tower_center = util::tower_center(x, y);
    commands
        .spawn(circle::mesh(
            &circles, &handle,
            radius_f32,
            tower_center.x, tower_center.y, z_pos::TOWER_RADIUS,
        ))
        .insert(RadiusInfo(x, y))
    ;
}

enum ButtonState {
    CantBuild,
    CanBuild,
    Selected,
}

impl ButtonState {
    fn get_alpha(&self) -> f32 {
        match self {
            ButtonState::CantBuild => 0.1,
            ButtonState::CanBuild => 0.5,
            ButtonState::Selected => 1.,
        }
    }
}

fn update_tower_button(
    cursor_state: Option<ResMut<CursorState>>,
    buttons: Query<(&TowerButton, &Transform, Entity)>,
    children: Query<&Children>,
    mut sprites: Query<&mut TextModeTextureAtlasSprite>,
    windows: Query<&Window>,
    mouse: Res<Input<MouseButton>>,
    money: Res<Money>,
) {
    let Some(mut cursor_state) = cursor_state else { return; };
    let Some(cursor_pos) = util::cursor_pos(windows) else { return; };
    let clicked = mouse.just_pressed(MouseButton::Left);

    for (button, pos, id) in &buttons {
        let button_state: ButtonState;
        if money.0 < button.0.get_cost() {
            button_state = ButtonState::CantBuild;
        } else if is_in(cursor_pos, pos.translation.xy(), Vec2::new(tile_to_f32(2), tile_to_f32(3))) {
            button_state = ButtonState::Selected;
            if clicked {
                cursor_state.set_if_neq(CursorState::Build(button.0))
            }
        } else if let CursorState::Build(t) = cursor_state.as_ref() {
            button_state = if *t == button.0 { ButtonState::Selected } else { ButtonState::CanBuild };
        } else {
            // Check button hover
            button_state = ButtonState::CanBuild;
        }

        for id in children.iter_descendants(id) {
            let Ok(mut sprite) = sprites.get_mut(id) else { continue };
            sprite.alpha = button_state.get_alpha();
        }
    }
}

#[derive(Component)]
struct TransparentTower;

fn place_tower(
    mut commands: Commands,
    mut sfx: EventWriter<PlaySfxEvent>,
    state: Option<ResMut<CursorState>>,
    cursor: Option<Res<HoveredPos>>,
    mut transparent_tower: Query<(&mut Transform, Entity), With<TransparentTower>>,
    textures: Res<Textures>,
    mouse: Res<Input<MouseButton>>,
    time: Res<Time>,
    grid: Option<ResMut<Grid>>,
    mut money: ResMut<Money>,
) {
    let Some(mut state) = state else { return; };
    let Some(mut grid) = grid else { return; };
    let cursor_changed = match cursor {
        Some(ref res) => res.is_changed(),
        _ => false,
    };
    let cursor: Option<(usize, usize)> = match cursor {
        Some(_) => Some(cursor.unwrap().0),
        _ => None,
    };

    if let Ok((mut pos, id)) = transparent_tower.get_single_mut() {
        // The transparent tower exists
        match (state.as_ref().clone(), cursor) {
            (CursorState::Build(t), Some((x, y))) => {
                if cursor_changed {
                    // Update its position
                    let tower_pos = util::grid_to_tower_pos(x, y, *t);
                    pos.translation.x = tower_pos.x;
                    pos.translation.y = tower_pos.y;
                }

                if mouse.just_pressed(MouseButton::Left) && !grid.towers.contains(&(x, y)) {
                    // Build the tower
                    sfx.send(PlaySfxEvent(SFX::PlaceTower));
                    if money.0 >= t.get_cost() {
                        money.0 -= t.get_cost();
                        grid.towers.insert((x, y));
                        tower::place_tower(x, y, &mut commands, *t, &textures.tileset, &time);
                    }
                    state.set_if_neq(CursorState::Select);
                    return;
                }
            }
            _ => {
                // Delete the tower :(
                commands.entity(id).despawn_recursive();
            }
        }
    } else {
        // There is no transparent tower
        if let CursorState::Build(t) = state.as_ref() {
            let Some((x, y)) = cursor else { return; };
            let tower_pos = util::grid_to_tower_pos(x, y, *t);
            commands
                .spawn(TransparentTower)
                .insert(MainBundle::from_xyz(tower_pos.x, tower_pos.y, z_pos::TRANSPARENT_TOWER))
                .with_children(|builder| {
                    sprite_from_tile_with_alpha(builder, t.get_tiles(), &textures.tileset, 0., 0.85);
                });
        }
    }
}

fn update_text_button(
    cursor_state: Option<ResMut<CursorState>>,
    mut buttons: Query<(&TextButton, &Transform, &mut Text)>,
    windows: Query<&Window>,
    mouse: Res<Input<MouseButton>>,
) {
    let Some(mut cursor_state) = cursor_state else { return; };
    let Some(cursor_pos) = util::cursor_pos(windows) else { return; };
    let clicked = mouse.just_pressed(MouseButton::Left);

    for (button, pos, mut text) in buttons.iter_mut() {
        let size = button.get_size();
        // TextButton-s have Anchor::CenterRight
        let bottom_left = Vec2::new(pos.translation.x - size.x, pos.translation.y - size.y / 2. - f32_tile_to_f32(0.25));
        let hovered = is_in(cursor_pos, bottom_left, size);
        let mut highlight = hovered;

        if *button == TextButton::Sell && cursor_state.eq(&CursorState::Sell) { highlight = true; } else if *button == TextButton::Upgrade && cursor_state.eq(&CursorState::Upgrade) { highlight = true; }

        if clicked && hovered {
            match button {
                TextButton::Upgrade => { cursor_state.set_if_neq(CursorState::Upgrade); }
                TextButton::Sell => { cursor_state.set_if_neq(CursorState::Sell); }
                TextButton::X2 => {}
                TextButton::Pause => {}
            }
        }

        text.sections[0].style.color = if highlight { Palette::B.into() } else { Palette::D.into() };
    }
}