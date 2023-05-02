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
            (2.0, Drones::Simple1),
        ].into(),
        [
            (0.0, Drones::Simple1),
            (4.0, Drones::Simple2),
            (5.0, Drones::Simple2),
        ].into(),
        [
            (0., Drones::Simple2),
            (2., Drones::Simple1),
            (4., Drones::Simple3),
            (6., Drones::Simple1),
        ].into(),
        [
            (0., Drones::Medium1),
        ].into(),
        [
            (0., Drones::Medium1),
            (2., Drones::Simple1),
            (4., Drones::Simple3),
        ].into(),
        [
            (0., Drones::Simple2),
            (2., Drones::Simple1),
            (4., Drones::Simple3),
            (6., Drones::Simple1),
            (10., Drones::Simple2),
            (12., Drones::Simple1),
            (14., Drones::Simple3),
            (16., Drones::Simple1),
        ].into(),
        [
            (0., Drones::Medium2),
            (3., Drones::Medium1),
        ].into(),
        [
            (0., Drones::Medium1),
            (2., Drones::Medium1),
            (4., Drones::Medium2),
            (6., Drones::Medium2),
        ].into(),
        [
            (0., Drones::Medium1),
            (1., Drones::Medium2),
            (2., Drones::Medium3),
            (3., Drones::Medium4),
            (4., Drones::Simple2),
            (5., Drones::Simple1),
            (6., Drones::Simple3),
            (7., Drones::Simple1),
        ].into(),
        [
            (0., Drones::Big1),
            (5., Drones::Big2),
        ].into(),
        [
            (0., Drones::Big1),
            (1., Drones::Medium2),
            (2., Drones::Simple1),
            (3., Drones::Simple3),
            (5., Drones::Big2),
            (6., Drones::Medium3),
            (7., Drones::Medium4),
            (9., Drones::Simple2),
            (10., Drones::Simple1),
        ].into(),
        [
            (0., Drones::Invader),
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
            (5.0, Drones::Medium2),
            (10.0, Drones::Simple1),
        ].into(),
        [
            (0.0, Drones::Medium3),
            (8.0, Drones::Medium2),
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
            (0., Drones::Simple2),
            (2.0, Drones::Simple1),
            (4.0, Drones::Simple3),
            (6.0, Drones::Simple1),
            (8.0, Drones::Simple3),
            (10., Drones::Simple2),
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
            (0., Drones::Medium1),
        ].into(),
        [
            (0., Drones::Medium1),
            (8., Drones::Simple2),
            (10., Drones::Simple2),
        ].into(),
        [
            (0.0, Drones::Medium3),
            (8.0, Drones::Simple2),
            (10., Drones::Simple3),
            (12., Drones::Simple1),
        ].into(),
        [
            (0.0, Drones::Medium1),
            (3.0, Drones::Medium4),
            (8.0, Drones::Simple2),
            (10., Drones::Simple3),
            (12., Drones::Simple1),
        ].into(),
        [
            (0., Drones::Medium1),
            (3., Drones::Medium2),
            (6., Drones::Medium3),
            (9., Drones::Medium4),
        ].into(),
    ];

    static ref WAVES_4: Vec<Wave> = vec![
        [
            (0., Drones::Simple1),
            (2., Drones::Simple2),
        ].into(),
        [
            (0.0, Drones::Simple1),
            (1.5, Drones::Simple3),
            (3.0, Drones::Simple1),
            (4.5, Drones::Simple2),
        ].into(),
        [
            (0.0, Drones::Simple1),
            (1.0, Drones::Simple3),
            (3.0, Drones::Simple1),
            (4.5, Drones::Simple2),
            (6.0, Drones::Simple1),
            (7.5, Drones::Simple3),
            (9.0, Drones::Simple1),
            (10.5, Drones::Simple2),
        ].into(),
        [
            (0.0, Drones::Medium1),
            (1.5, Drones::Medium3),
            (3.0, Drones::Medium4),
            (4.5, Drones::Medium2),
        ].into(),
        [
            (0.0, Drones::Medium1),
            (2.0, Drones::Medium3),
            (4.0, Drones::Medium4),
            (6.0, Drones::Medium2),
            (10.0, Drones::Simple1),
            (11.5, Drones::Simple3),
            (13., Drones::Simple1),
            (14., Drones::Simple2),
            (15., Drones::Simple1),
            (16., Drones::Simple3),
        ].into(),
        [
            (0.0, Drones::Medium1),
            (2.0, Drones::Medium3),
            (4.0, Drones::Medium4),
            (6.0, Drones::Medium2),
            (8.0, Drones::Medium3),
            (10., Drones::Medium1),
            (12., Drones::Medium3),
            (14., Drones::Medium4),
            (16., Drones::Medium2),
            (18., Drones::Medium1),
        ].into(),
        [
            (0., Drones::Big1),
        ].into(),
    ];

    static ref WAVES_5: Vec<Wave> = vec![
          [
            (0., Drones::Simple1),
            (2., Drones::Simple2),
        ].into(),
        [
            (0.0, Drones::Simple1),
            (1.5, Drones::Simple3),
            (3.0, Drones::Simple1),
            (4.5, Drones::Simple2),
        ].into(),
        [
            (0.0, Drones::Medium4),
            (3.0, Drones::Medium3),
            (8.0, Drones::Simple1),
            (9.5, Drones::Simple2),
        ].into(),
        [
            (0.0, Drones::Medium1),
            (3.0, Drones::Medium3),
            (6.0, Drones::Medium2),
            (10., Drones::Simple1),
            (12., Drones::Simple2),
            (14., Drones::Simple3),
            (16., Drones::Simple2),
        ].into(),
        [
            (0.0, Drones::Medium1),
            (2.0, Drones::Medium3),
            (4.0, Drones::Medium2),
            (6.0, Drones::Medium3),
            (10.0, Drones::Simple1),
            (11.5, Drones::Simple2),
            (13.0, Drones::Simple3),
            (14.5, Drones::Simple2),
            (16.0, Drones::Simple1),
            (17.5, Drones::Simple2),
            (19.0, Drones::Simple3),
            (20.5, Drones::Simple2),
        ].into(),
        [
            (0.0, Drones::Big1),
        ].into(),
        [
            (0.0, Drones::Big2),
            (4.0, Drones::Big1),
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
            (10., Drones::Medium1),
            (12., Drones::Medium3),
            (14., Drones::Medium2),
            (16., Drones::Medium3),
            (18., Drones::Medium1),
        ].into(),
        [
            (0.0, Drones::Big2),
            (3.0, Drones::Big1),
            (6.0, Drones::Big2),
            (9.0, Drones::Big2),
        ].into(),
        [
            (0.0, Drones::Invader),
        ].into(),
    ];
}

#[test]
fn ensure_waves_are_sorted() {
    for wave in WAVES.iter() {
        let mut t = 0.;
        for (t1, _) in wave.timed_departures.iter() {
            assert!(*t1 >= t);
            t = *t1;
        }
    }
}

#[test]
fn ensure_iterator_produces_correct_results() {
    for elt in WaveIterator::get_static().upcoming.iter() {
        eprintln!("Got {elt:?}");
        let t = match elt {
            NextDrone(_, t) => *t,
            NextWave(t) => *t,
        };
        assert!(t >= 0.);
    }
}
