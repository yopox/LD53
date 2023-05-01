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
            next: Timer::new(Duration::ZERO, TimerMode::Once),
            upcoming: result,
        }
    }

    pub fn get_static() -> WaveIterator {
        WaveIterator::from_waves(&WAVES)
    }

    pub fn from_level(level: u8) -> WaveIterator {
        WaveIterator::from_waves(
            match level {
                0 => &WAVES_0,
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

static ref WAVES_0: Vec<Wave> = vec![
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
            (3.0, Drones::Medium2),
            (8.0, Drones::Simple2),
            (10., Drones::Simple3),
            (12., Drones::Simple1),
        ].into(),
        [
            (0., Drones::Medium1),
            (3., Drones::Medium2),
            (6., Drones::Medium3),
            (9., Drones::Medium3),
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
