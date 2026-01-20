use std::time::{Duration, Instant};

use bevy::{
    input::{keyboard::KeyboardInput, mouse::MouseButtonInput},
    prelude::*,
    window::WindowFocused,
};

use crate::backend::{CurrentIdleTimeSeconds, LongestIdleTimeSeconds};

pub const TIME_WINDOW: f64 = 1.0;
pub const IDLE_TIME_GROWTH_RATE: f64 = 1.25;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, States)]
pub enum MainGameStates {
    /// start screen rendered in Dioxus, which will communicate with the Bevy Backend telling it to
    /// change states.
    #[default]
    StartScreen,
    InGame,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, States)]
pub enum AutomationStates {
    #[default]
    Manual,
    Automation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Resource)]
pub struct KeyCount(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Component)]
pub struct KeyPress(pub Instant);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Component)]
pub struct LostFocusTimestamp(pub Instant);

/// a function to asertain if teh game should step the loaded automation
pub fn should_automate(idle_time: Res<CurrentIdleTimeSeconds>, key_count: Res<KeyCount>) -> bool {
    **idle_time > 0.0 && key_count.0 == 0
}

fn automation_timer_done(last_lost_focus: Single<Option<&LostFocusTimestamp>>) -> bool {
    last_lost_focus
        .is_some_and(|focus_timer| focus_timer.0.elapsed() > Duration::from_secs_f64(2.0))
}

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MainGameStates>();
        app.init_state::<AutomationStates>();
        app.init_resource::<KeyCount>();
        app.add_systems(
            Update,
            (
                (
                    gather_kbd_input.run_if(on_message::<KeyboardInput>),
                    // gather_mouse_input.run_if(on_message::<MouseButtonInput>),
                    step_inputs,
                    step_idle_time,
                )
                    .chain(),
                step_automating_timer,
                start_automating
                    .run_if(in_state(AutomationStates::Manual))
                    .run_if(automation_timer_done),
            ),
        );
    }
}

fn gather_kbd_input(mut cmds: Commands, mut keyboard_inputs: MessageReader<KeyboardInput>) {
    for _input in keyboard_inputs.read() {
        cmds.spawn(KeyPress(Instant::now()));
    }
}

// fn gather_mouse_input(mut cmds: Commands, mut keyboard_inputs: MessageReader<MouseButtonInput>) {
//     for _input in keyboard_inputs.read() {
//         cmds.spawn(KeyPress(Instant::now()));
//     }
// }

fn step_inputs(
    mut cmds: Commands,
    presses: Query<(Entity, &KeyPress)>,
    mut key_count: ResMut<KeyCount>,
) {
    let mut presses: Vec<(Entity, &KeyPress)> = presses.into_iter().collect();
    presses.retain(|(entity, press)| {
        let is_old = press.0.elapsed() >= Duration::from_secs_f64(TIME_WINDOW);

        if is_old {
            cmds.entity(*entity).despawn();
        }

        !is_old
    });

    key_count.0 = presses.len();
}

fn step_idle_time(
    key_count: Res<KeyCount>,
    mut idle_time: ResMut<CurrentIdleTimeSeconds>,
    mut longest_idle_time: ResMut<LongestIdleTimeSeconds>,
    presses: Query<&KeyPress>,
    time: Res<Time>,
) {
    if key_count.0 > 0 {
        // let input_rate = key_count.0 as f64 * TIME_WINDOW;
        // let increment_amount = input_rate * time.delta_secs_f64();
        let mut presses: Vec<&KeyPress> = presses
            .iter()
            .sort_by::<&KeyPress>(|val1, val2| {
                val1.0
                    .elapsed()
                    .as_secs_f64()
                    .total_cmp(&val2.0.elapsed().as_secs_f64())
            })
            .collect();
        let now = KeyPress(Instant::now());
        presses.push(&now);
        let total_time_delta: Duration = presses
            .windows(2)
            .map(|presses| presses[0].0 - presses[1].0)
            .sum();
        let total_time_delta = total_time_delta.as_secs_f64();
        let avg_press_time_delta = total_time_delta / presses.len() as f64;

        // if avg_press_time_delta > 0.0 {
        let avg_press_time_delta = 1.0 - avg_press_time_delta;
        let input_rate = avg_press_time_delta;
        // let compensater = 10.0;
        let increment_amount = input_rate * IDLE_TIME_GROWTH_RATE * time.delta_secs_f64();

        **idle_time += increment_amount;

        if **idle_time > **longest_idle_time {
            **longest_idle_time = **idle_time;
        }
        // }
    } else {
        let decrement_amount = time.delta_secs_f64();

        **idle_time -= decrement_amount;

        if **idle_time < 0.0 {
            **idle_time = 0.0;
        }
    }
}

fn step_automating_timer(
    mut cmds: Commands,
    mut events: MessageReader<WindowFocused>,
    last_lost_focus: Single<Option<(Entity, &LostFocusTimestamp)>>,
) {
    for event in events.read() {
        if !event.focused && last_lost_focus.is_none() {
            cmds.spawn(LostFocusTimestamp(Instant::now()));
        } else if let Some((entity, _lost_focus)) = *last_lost_focus
            && !event.focused
        {
            cmds.spawn(LostFocusTimestamp(Instant::now()));
            cmds.entity(entity).despawn();
        } else if let Some((entity, _lost_focus)) = *last_lost_focus
            && event.focused
        {
            cmds.entity(entity).despawn();
        }
    }
}

fn start_automating(mut automation_state: ResMut<NextState<AutomationStates>>) {
    automation_state.set(AutomationStates::Automation);
}
