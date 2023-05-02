use std::time::Duration;

use bevy::prelude::{Resource, Timer};
use bevy::time::TimerMode;
use lazy_static::lazy_static;

use crate::drones::Drones;
use crate::logic::waves::WaveIteratorElement::{NextDrone, NextWave};

#[derive(Debug, Clone)]
struct Wave {
    /// Spawn time after beginning of wave
    /// first one should be roughly zero
    timed_departures: Vec<(f32, Drones)>,
    /// Delay after last spawn
    end_delay: f32,
}

pub const WAVES_INTERVAL: f32 = 30.;

impl<T> From<T> for Wave where T: Into<Vec<(f32, Drones)>> {
    fn from(timed_departures: T) -> Self {
        Wave {
            end_delay: WAVES_INTERVAL,
            timed_departures: timed_departures.into(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum WaveIteratorElement {
    /// Spawn a drone now, and wait t seconds
    NextDrone(Drones, f32),
    /// Spawn a new wave now, and wait t seconds
    NextWave(f32),
}

#[derive(Resource)]
pub struct WaveIterator {
    pub next: Timer,
    /// Upcoming elements, in reversed order
    /// i.e. pop gives the next element
    pub upcoming: Vec<WaveIteratorElement>,
}

impl WaveIterator {
    fn from_waves(waves: &Vec<Wave>) -> WaveIterator {
        let mut result = Vec::new();
        for wave in waves.iter() {
            let mut prev: Option<(Drones, f32)> = None;

            for (t1, drone) in wave.timed_departures.iter() {
                match prev {
                    Some((drone, t0)) => { result.push(NextDrone(drone, t1 - t0)) }
                    None => { result.push(NextWave(*t1)) }
                }
                prev = Some((*drone, *t1));
            }

            match prev {
                Some((drone, _)) => { result.push(NextDrone(drone, wave.end_delay)) }
                None => { panic!("Waves should not be empty.") }
            }
        }

        result.reverse();

        WaveIterator {
            next: Timer::new(Duration::from_secs_f32(2.), TimerMode::Once),
            upcoming: result,
        }
    }

    pub fn from_level(level: u8) -> WaveIterator {
        WaveIterator::from_waves(
            match level {
                1 => &WAVES_1,
                2 => &WAVES_2,
                3 => &WAVES_3,
                4 => &WAVES_4,
                5 => &WAVES_5,
                _ => &WAVES,
            }
        )
    }
}

lazy_static! {
    static ref WAVES: Vec<Wave> = vec![
        [
            (0.0, Drones::Simple1),
        ].into(),
        [
            (0.0, Drones::Simple2),
        ].into(),
        [
            (0.0, Drones::Simple3),
        ].into(),
        [
            (0.0, Drones::Simple1),
            (5.0, Drones::Simple1),
        ].into(),
        [
            (0.0, Drones::Simple2),
            (5.0, Drones::Simple2),
        ].into(),
        [
            (0.0, Drones::Simple3),
            (5.0, Drones::Simple3),
        ].into(),
        [
            (0.0, Drones::Simple1),
            (5.0, Drones::Simple2),
            (10.0, Drones::Simple2),
        ].into(),
        [
            (0., Drones::Simple2),
            (3., Drones::Simple1),
            (6., Drones::Simple3),
            (9., Drones::Simple1),
            (12., Drones::Simple2),
        ].into(),
        [
            (0., Drones::Medium1),
        ].into(),
        [
            (0., Drones::Medium1),
            (7., Drones::Simple1),
            (10., Drones::Simple3),
        ].into(),
        [
            (0., Drones::Simple2),
            (2.5, Drones::Simple1),
            (5., Drones::Simple3),
            (7.5, Drones::Simple1),
            (12.5, Drones::Simple2),
            (15., Drones::Simple1),
            (17.5, Drones::Simple3),
            (20., Drones::Simple1),
        ].into(),
        [
            (0., Drones::Medium2),
            (4., Drones::Medium1),
        ].into(),
        [
            (0., Drones::Medium1),
            (5., Drones::Medium1),
            (10., Drones::Medium2),
            (15., Drones::Medium2),
        ].into(),
        [
            (0., Drones::Medium1),
            (5., Drones::Medium2),
            (10., Drones::Medium3),
            (15., Drones::Medium4),
            (20., Drones::Simple2),
            (22., Drones::Simple1),
            (24., Drones::Simple3),
            (26., Drones::Simple1),
        ].into(),
        [
            (0., Drones::Big1),
        ].into(),
        [
            (0., Drones::Medium2),
            (5., Drones::Simple1),
            (7., Drones::Simple3),
            (15., Drones::Big2),
            (35., Drones::Medium3),
            (40., Drones::Medium4),
            (55., Drones::Simple2),
            (57., Drones::Simple1),
            (65., Drones::Big1),
        ].into(),
        [
            (0.0, Drones::Big2),
            (10., Drones::Simple1),
            (12., Drones::Simple2),
            (14., Drones::Simple3),
            (16., Drones::Simple2),
            (18., Drones::Simple3),
        ].into(),
        [
            (0.0, Drones::Big1),
            (15., Drones::Medium1),
            (25., Drones::Medium3),
            (35., Drones::Medium2),
            (45., Drones::Medium3),
            (55., Drones::Medium1),
        ].into(),
        [
            (0.0, Drones::Big2),
            (20.0, Drones::Big1),
            (40.0, Drones::Big2),
            (60.0, Drones::Big2),
        ].into(),
        [
            (0.0, Drones::Invader),
            (150., Drones::Medium2),
            (175., Drones::Medium4),
            (300., Drones::Big2),
        ].into(),
        [
            (0.0, Drones::Big2),
            (20.0, Drones::Big2),
            (40.0, Drones::Big2),
            (60.0, Drones::Big2),
            (100.0, Drones::Big1),
            (120.0, Drones::Big1),
            (140.0, Drones::Big1),
            (160.0, Drones::Big1),
            (200.0, Drones::Big2),
            (215.0, Drones::Big1),
            (230.0, Drones::Big2),
            (245.0, Drones::Big1),
            (260.0, Drones::Big2),
        ].into(),
        [
            (0.0, Drones::Invader),
            (250.0, Drones::Invader),
        ].into(),
    ];

    static ref WAVES_1: Vec<Wave> = vec![
        [
            (0.0, Drones::Simple1),
        ].into(),
        [
            (0.0, Drones::Simple2),
            (4.0, Drones::Simple3),
        ].into(),
        [
            (0.0, Drones::Simple1),
            (4.0, Drones::Simple3),
            (8.0, Drones::Simple1),
            (12.0, Drones::Simple2),
        ].into(),
        [
            (0.0, Drones::Medium2),
        ].into(),
        [
            (0.0, Drones::Medium2),
            (6.0, Drones::Medium3),
        ].into(),
    ];

    static ref WAVES_2: Vec<Wave> = vec![
        [
            (0.0, Drones::Simple3),
            (2.0, Drones::Simple3),
        ].into(),
        [
            (0.0, Drones::Simple2),
            (3.0, Drones::Simple1),
            (6.0, Drones::Simple2),
            (9.0, Drones::Simple1),
        ].into(),
        [
            (0.0, Drones::Simple3),
            (3.0, Drones::Simple2),
            (6.0, Drones::Simple3),
            (9.0, Drones::Simple2),
            (12.0, Drones::Simple3),
            (15.0, Drones::Simple2),
        ].into(),
        [
            (0.0, Drones::Medium1),
        ].into(),
        [
            (0.0, Drones::Simple1),
            (4.0, Drones::Simple1),
            (10.0, Drones::Medium1),
        ].into(),
        [
            (0.0, Drones::Medium3),
            (8.0, Drones::Medium2),
        ].into(),
        [
            (0.0, Drones::Medium1),
            (5.0, Drones::Medium2),
            (10.0, Drones::Medium3),
            (15.0, Drones::Medium4),
        ].into(),
        [
            (0.0, Drones::Simple1),
            (3.0, Drones::Simple2),
            (8.0, Drones::Medium1),
            (12.0, Drones::Simple3),
            (15.0, Drones::Simple2),
            (20.0, Drones::Medium4),
        ].into(),
        [
            (0.0, Drones::Simple3),
            (3.0, Drones::Simple3),
            (6.0, Drones::Simple2),
            (9.0, Drones::Simple2),
            (12.0, Drones::Simple1),
            (12.0, Drones::Simple1),
            (20.0, Drones::Big1),
        ].into(),
        [
            (0.0, Drones::Medium2),
            (5.0, Drones::Medium3),
            (12.0, Drones::Big1),
        ].into(),
        [
            (0.0, Drones::Big1),
            (5.0, Drones::Big1),
        ].into(),
        [
            (0.0, Drones::Big1),
            (5.0, Drones::Big1),
            (10.0, Drones::Big1),
        ].into(),
    ];

    static ref WAVES_3: Vec<Wave> = vec![
        [
            (0.0, Drones::Simple1),
            (2.0, Drones::Simple1),
        ].into(),
        [
            (0.0, Drones::Simple1),
            (4.0, Drones::Simple2),
            (6.0, Drones::Simple2),
        ].into(),
        [
            (0., Drones::Simple2),
            (2., Drones::Simple1),
            (4., Drones::Simple3),
            (6., Drones::Simple1),
        ].into(),
        [
            (0., Drones::Medium2),
            (5., Drones::Medium3),
        ].into(),
        [
            (0., Drones::Simple2),
            (2.0, Drones::Simple1),
            (4.0, Drones::Simple3),
            (6.0, Drones::Simple1),
            (8.0, Drones::Simple3),
            (10., Drones::Simple2),
        ].into(),
        [
            (0., Drones::Medium2),
            (5., Drones::Medium3),
            (10., Drones::Medium2),
            (15., Drones::Medium3),
        ].into(),
        [
            (0.0, Drones::Simple1),
            (2.0, Drones::Simple1),
            (4.0, Drones::Simple2),
            (6.0, Drones::Simple2),
            (8.0, Drones::Simple3),
            (10., Drones::Simple3),
            (12., Drones::Simple1),
            (14., Drones::Simple1),
            (16., Drones::Simple2),
            (18., Drones::Simple2),
        ].into(),
        [
            (0., Drones::Simple1),
            (5., Drones::Medium4),
            (10., Drones::Big2),
        ].into(),
        [
            (0., Drones::Medium1),
            (4., Drones::Medium2),
            (8., Drones::Medium3),
            (12., Drones::Medium2),
            (16., Drones::Medium1),
        ].into(),
        [
            (0., Drones::Simple2),
            (4., Drones::Simple2),
            (10., Drones::Medium1),
            (15., Drones::Medium1),
            (20., Drones::Big2),
            (25., Drones::Big2),
        ].into(),
        [
            (0., Drones::Simple1),
            (2.5, Drones::Simple1),
            (5.0, Drones::Simple2),
            (7.5, Drones::Simple2),
            (10., Drones::Simple3),
            (12.5, Drones::Simple3),
            (15., Drones::Simple1),
            (17.5, Drones::Simple1),
            (20., Drones::Simple2),
            (22.5, Drones::Simple2),
            (25., Drones::Simple3),
            (27.5, Drones::Simple3),
        ].into(),
        [
            (0., Drones::Big1),
            (8., Drones::Big2),
            (16., Drones::Big1),
            (24., Drones::Big2),
        ].into(),
    ];

    static ref WAVES_4: Vec<Wave> = vec![
        [
            (0., Drones::Simple1),
            (2., Drones::Simple2),
        ].into(),
        [
            (0.0, Drones::Simple1),
            (2.0, Drones::Simple3),
            (4.0, Drones::Simple1),
            (6.0, Drones::Simple2),
        ].into(),
        [
            (0.0, Drones::Simple1),
            (2.0, Drones::Simple3),
            (4.0, Drones::Simple1),
            (6.0, Drones::Simple2),
            (8.0, Drones::Simple1),
            (10.0, Drones::Simple3),
            (12.0, Drones::Simple1),
            (14.0, Drones::Simple2),
        ].into(),
        [
            (0.0, Drones::Medium1),
            (8.0, Drones::Medium4),
        ].into(),
        [
            (0.0, Drones::Simple3),
            (2.0, Drones::Simple3),
            (12.0, Drones::Medium4),
            (16.0, Drones::Medium4),
        ].into(),
        [
            (0.0, Drones::Medium1),
            (4.0, Drones::Medium3),
            (8.0, Drones::Medium4),
            (12.0, Drones::Medium2),
        ].into(),
        [
            (0., Drones::Simple1),
            (2., Drones::Simple2),
            (4., Drones::Simple3),
            (6., Drones::Simple2),
            (8., Drones::Simple1),
            (20., Drones::Big1),
        ].into(),
        [
            (0., Drones::Medium4),
            (8., Drones::Medium3),
            (20., Drones::Big1),
        ].into(),
        [
            (0., Drones::Big2),
            (10., Drones::Big1),
        ].into(),
    ];

    static ref WAVES_5: Vec<Wave> = vec![
          [
            (0., Drones::Simple1),
            (2., Drones::Simple2),
        ].into(),
        [
            (0.0, Drones::Simple1),
            (2.0, Drones::Simple3),
            (4.0, Drones::Simple1),
            (6.0, Drones::Simple2),
        ].into(),
        [
            (0.0, Drones::Simple1),
            (2.0, Drones::Simple1),
            (4.0, Drones::Simple1),
            (12.0, Drones::Medium4),
        ].into(),
        [
            (0.0, Drones::Medium1),
            (5.0, Drones::Medium3),
            (15., Drones::Simple1),
            (17., Drones::Simple2),
            (19., Drones::Simple3),
            (21., Drones::Simple2),
        ].into(),
        [
            (0.0, Drones::Simple2),
            (2.5, Drones::Simple1),
            (5.0, Drones::Simple2),
            (15.0, Drones::Medium1),
            (20.0, Drones::Medium3),
            (25.0, Drones::Medium3),
            (35.0, Drones::Simple1),
            (37.5, Drones::Simple2),
            (40.0, Drones::Simple3),
        ].into(),
        [
            (0.0, Drones::Simple2),
            (2.5, Drones::Simple1),
            (5.0, Drones::Simple2),
            (7.5, Drones::Simple1),
            (10.0, Drones::Simple2),
            (12.5, Drones::Simple1),
            (15.0, Drones::Simple2),
            (17.5, Drones::Simple1),
            (20.0, Drones::Simple2),
            (22.5, Drones::Simple1),
        ].into(),
        [
            (0.0, Drones::Medium1),
            (5.0, Drones::Medium2),
            (10.0, Drones::Medium3),
            (15.0, Drones::Medium4),
            (20.0, Drones::Medium1),
            (25.0, Drones::Medium2),
            (30.0, Drones::Medium3),
            (35.0, Drones::Medium4),
        ].into(),
        [
            (0.0, Drones::Big1),
        ].into(),
        [
            (0.0, Drones::Medium2),
            (10.0, Drones::Big1),
        ].into(),
        [
            (0.0, Drones::Big2),
            (10., Drones::Simple1),
            (12., Drones::Simple2),
            (14., Drones::Simple3),
            (16., Drones::Simple2),
            (18., Drones::Simple3),
        ].into(),
        [
            (0.0, Drones::Big1),
            (15., Drones::Medium1),
            (25., Drones::Medium3),
            (35., Drones::Medium2),
            (45., Drones::Medium3),
            (55., Drones::Medium1),
        ].into(),
        [
            (0.0, Drones::Big2),
            (20.0, Drones::Big1),
            (40.0, Drones::Big2),
            (60.0, Drones::Big2),
        ].into(),
        [
            (0.0, Drones::Invader),
        ].into(),
    ];
}

#[test]
fn ensure_waves_are_sorted() {
    let check_wave = |w: &Wave| {
        let mut t = 0.;
        for (t1, _) in w.timed_departures.iter() {
            assert!(*t1 >= t);
            t = *t1;
        }
    };
    for wave in WAVES.iter() { check_wave(wave); }
    for wave in WAVES_1.iter() { check_wave(wave); }
    for wave in WAVES_2.iter() { check_wave(wave); }
    for wave in WAVES_3.iter() { check_wave(wave); }
    for wave in WAVES_4.iter() { check_wave(wave); }
    for wave in WAVES_5.iter() { check_wave(wave); }
}
