use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Resource)]
pub struct Player {
    pub known_skills: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Component)]
pub struct PlayerMarker;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Component)]
pub struct AllyMarker;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Component)]
pub struct EnemyMarker;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Component)]
pub struct NPCMarker;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharacterStat<T> {
    pub current: T,
    pub max: T,
    pub min: Option<T>,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Component)]
pub struct CharacterStats {
    /// HP
    pub health: CharacterStat<usize>,
    /// used for casting spells and skills OUTSIDE of battle
    pub mana: CharacterStat<usize>,
    /// numbers between 0 and 10 represnets a multiplier to physical damage
    pub attack: CharacterStat<f64>,
    /// numbers between zero and one representing the percentage of total physical damage that gets negated
    pub defence: CharacterStat<f64>,
    /// numbers between 0 and 10 represnets a multiplier to magical damage
    pub mag_atk: CharacterStat<f64>,
    /// numbers between zero and one representing the percentage of total magical damage that gets negated
    pub mag_def: CharacterStat<f64>,
}

impl Default for CharacterStats {
    fn default() -> Self {
        Self {
            health: CharacterStat {
                current: 10,
                max: 10,
                min: Some(0),
            },
            mana: CharacterStat {
                current: 5,
                max: 5,
                min: Some(0),
            },
            attack: CharacterStat {
                current: 0.25,
                max: 0.25,
                min: Some(0.125),
            },
            defence: CharacterStat {
                current: 0.125,
                max: 0.125,
                min: Some(0.0625),
            },
            mag_atk: CharacterStat {
                current: 0.25,
                max: 0.25,
                min: Some(0.125),
            },
            mag_def: CharacterStat {
                current: 0.125,
                max: 0.125,
                min: Some(0.0625),
            },
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Player>();
        app.add_systems(Startup, spawn_player_stats);
    }
}

fn spawn_player_stats(mut commands: Commands) {
    commands.spawn((CharacterStats::default(), PlayerMarker));
}
