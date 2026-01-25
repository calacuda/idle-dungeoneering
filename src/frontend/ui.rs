use std::{any::TypeId, ops::Deref, time::Duration};

use crate::{
    backend::{
        idle_time_plugin::{
            AutomationSpeedSample, IDLE_SAMPLE_WINDOW, IdleTimeSample, WResolution,
        },
        *,
    },
    frontend::line::LineChart,
};
use bevy_dioxus_hooks::{BevyValue, resource::hook::use_bevy_resource};
use bevy_dioxus_sync::panels::DioxusElementMarker;
use crossbeam::channel::Receiver;
use dioxus::prelude::*;

#[derive(Debug)]
pub struct AppUi {
    pub idle_time: Receiver<IdleTimeSample>,
    pub automation_speed: Receiver<AutomationSpeedSample>,
}

impl DioxusElementMarker for AppUi {
    fn element(&self) -> Element {
        // app_ui()
        game_ui(self.idle_time.clone(), self.automation_speed.clone())
    }
}

// #[component]
pub fn game_ui(
    idle_time: Receiver<IdleTimeSample>,
    speed_samples: Receiver<AutomationSpeedSample>,
) -> Element {
    let idle_time_res = use_bevy_resource::<CurrentIdleTimeSeconds>();
    let best_idle_time_res = use_bevy_resource::<LongestIdleTimeSeconds>();
    // let automation_speed_res = use_bevy_resource::<AutomationSpeed>();
    let window_size = use_bevy_resource::<WResolution>();
    // let test_component = use_bevy_query::<(Entity, &TestComponent), ()>();
    let mut max_idle_time: Signal<f32> = use_signal(|| 0.0);
    let mut idle_times: Signal<Vec<_>> =
        use_signal(|| vec![IdleTimeSample::new(0.0), IdleTimeSample::new(0.0)]);
    let mut automation_speed_samples: Signal<Vec<_>> = use_signal(|| {
        vec![
            AutomationSpeedSample::new(0.0),
            AutomationSpeedSample::new(0.0),
        ]
    });
    let mut max_automation_speed: Signal<f32> = use_signal(|| 0.0);

    let _idle_time_th = spawn(async move {
        loop {
            while let Ok(sample) = idle_time.try_recv() {
                idle_times.write().push(sample);
                idle_times
                    .write()
                    .retain(|time| time.when.elapsed() < IDLE_SAMPLE_WINDOW);

                *max_idle_time.write() = idle_times
                    .read()
                    .iter()
                    .max_by(|a, b| a.time.total_cmp(&b.time).then(std::cmp::Ordering::Less))
                    .map(|time| time.time)
                    .unwrap_or(1.0) as f32;
            }

            portable_async_sleep::async_sleep(Duration::from_secs_f32(0.25)).await;
        }
    });

    let _speed_th = spawn(async move {
        loop {
            while let Ok(sample) = speed_samples.try_recv() {
                automation_speed_samples.write().push(sample);
                automation_speed_samples
                    .write()
                    .retain(|time| time.when.elapsed() < IDLE_SAMPLE_WINDOW);

                *max_automation_speed.write() = idle_times
                    .read()
                    .iter()
                    .max_by(|a, b| a.time.total_cmp(&b.time).then(std::cmp::Ordering::Less))
                    .map(|time| time.time)
                    .unwrap_or(1.0) as f32;
            }

            portable_async_sleep::async_sleep(Duration::from_secs_f32(0.25)).await;
        }
    });

    // use_effect({
    //     // let idle_time_res = idle_time_res.clone();
    //
    //     move || {
    //         idle_times.write().push(IdleTimeSample::new(
    //             idle_time_res
    //                 // .get()
    //                 .read()
    //                 .deref()
    //                 .read_value()
    //                 .map(|time| time.0)
    //                 .unwrap_or(0.0),
    //         ));
    //         idle_times
    //             .write()
    //             .retain(|time| time.when.elapsed() < IDLE_SAMPLE_WINDOW);
    //     }
    // });

    // use_effect(move || {
    //     let new_value = automation_speed_res
    //         // .get()
    //         .read()
    //         .deref()
    //         .read_value()
    //         .map(|speed| speed.0 as f32)
    //         .unwrap_or(0.0);
    //
    //     automation_speed_samples.write().push(new_value);
    //
    //     // get just the end
    //     let n_samples = 60 * 2;
    //
    //     if automation_speed_samples.read().len() > n_samples {
    //         let samples = automation_speed_samples.read().clone();
    //         *automation_speed_samples.write() = samples[{ samples.len() - n_samples }..].to_vec();
    //     }
    //
    //     *heighest_automation_speed.write() = *automation_speed_samples
    //         .read()
    //         .iter()
    //         .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Less))
    //         .unwrap_or(&0.0);
    // });

    rsx! {
        document::Stylesheet { href: asset!("src/frontend/ui.css") }

        main {
            // progress_bar { curent_time: idle_time_res.get().map(|time| time.0).unwrap_or(0.0), longest_time: best_idle_time_res.get().map(|time| time.0).unwrap_or(0.0) }
            // progress_bar { curent_time: idle_time_res.get().map(|time| time.0 as f32).unwrap_or(0.0), longest_time: max_idle_time }
            progress_bar {
                curent_time: idle_time_res
                    .read()
                    .deref()
                    .read_value()
                    .map(|time| time.0 as f32)
                    .unwrap_or(0.0),
                longest_time: best_idle_time_res
                    .read()
                    .deref()
                    .read_value()
                    .map(|time| time.0 as f32)
                    .unwrap_or(0.0)
            }

            div {
                style: "
                width: 100%;
                height: 12.5%;
                display: flex;
                flex-direction: row;
                ",

                // skills seciton (skill order & known skills)
                div {
                    style: "
                    width: 50%;
                    height: 100%;
                    "

                    // TODO: battle skill order.
                    // TODO: known battle skills.
                }


                // TODO: speed graph.
                automation_speed_graph { speed_samples: automation_speed_samples, window_size: window_size }

                // idle speed graph
                idle_time_graph {
                    heighest: max_idle_time,
                    // heighest: best_idle_time_res
                    //     .read()
                    //     .deref()
                    //     .read_value()
                    //     .map(|time| time.0 as f32)
                    //     .unwrap_or(0.0),
                    time_samples: idle_times,
                    window_size: window_size,
                }
            }
        }
    }
}

// TODO: make timeing more consistent by:
// mk a bevy system that every quarter/half second send measeaurments over a crossbeam channel.
// this system reads them & displays them. this should increase consistency in the timings of the
// measearments.
#[component]
fn automation_speed_graph(
    // heighest: ReadSignal<f32>,
    speed_samples: ReadSignal<Vec<AutomationSpeedSample>>,
    window_size: Signal<BevyValue<WResolution, TypeId, ()>, SyncStorage>,
) -> Element {
    let mut series = use_signal(|| Vec::new());

    use_effect(move || {
        *series.write() = speed_samples()
            .iter()
            .map(|time| time.time as f32)
            .collect();
    });

    rsx! {
        div {
            style: "
                width: 25%;
                height: 100%;
                border: 5px solid black;
                margin: 5;
                border-radius: calc(1.5rem / 2);
                margin-radius: calc(1.5rem / 2);
                color: #11111bff;
            ",

            LineChart {
                width: "100%",
                height: "100%",
                max_ticks: 8,
                viewbox_width: {
                    window_size.read().read_value().map(|size| size.w as i32).unwrap_or(1920) / 4
                },
                // viewbox_width: (window_size.w as i32) / 4,
                // viewbox_height: 1080 / 8,
                viewbox_height: {
                    window_size.read().read_value().map(|size| size.h as i32).unwrap_or(1080) / 8
                },
                padding_top: 0,
                padding_left: 0,
                padding_right: 0,
                padding_bottom: 0,
                show_grid_ticks: false,
                show_dotted_grid: true,
                show_labels: true,
                show_line_labels: false,
                show_dots: false,
                show_lines: true,
                lowest: Some(0.0),
                // highest: Some(heighest.read().deref().to_owned()),
                highest: series().iter().max_by(|a, b| a.total_cmp(&b).then(std::cmp::Ordering::Less)).map(|f| *f),
                label_interpolation: (|_v| "".into()) as fn(f32) -> String,
                series: vec![
                    series(),
                    // speed_samples(),
                ],
                labels: Some((0..speed_samples.len()).map(|_i| "".into()).collect()),
            }
        }
    }
}

#[component]
fn idle_time_graph(
    heighest: ReadSignal<f32>,
    time_samples: ReadSignal<Vec<IdleTimeSample>>,
    window_size: Signal<BevyValue<WResolution, TypeId, ()>, SyncStorage>,
) -> Element {
    let mut series = use_signal(|| Vec::new());

    use_effect(move || {
        *series.write() = time_samples().iter().map(|time| time.time as f32).collect();
    });

    rsx! {
        div {
            style: "
                width: 25%;
                height: 100%;
                border: 5px solid black;
                margin: 5;
                border-radius: calc(1.5rem / 2);
                margin-radius: calc(1.5rem / 2);
                color: #11111bff;
            ",

            LineChart {
                width: "100%",
                height: "100%",
                max_ticks: 8,
                viewbox_width: {
                    window_size.read().read_value().map(|size| size.w as i32).unwrap_or(1920) / 4
                },
                // viewbox_height: 1080 / 8,
                viewbox_height: {
                    window_size.read().read_value().map(|size| size.h as i32).unwrap_or(1080) / 8
                },
                padding_top: 0,
                padding_left: 0,
                padding_right: 0,
                padding_bottom: 0,
                show_grid_ticks: false,
                show_dotted_grid: true,
                show_labels: true,
                show_line_labels: false,
                show_dots: false,
                show_lines: true,
                lowest: Some(0.0),
                highest: Some(heighest.read().deref().to_owned()),
                // highest: Some(25.0),
                label_interpolation: (|_v| "".into()) as fn(f32) -> String,
                series: vec![
                    series(),
                ],
                labels: Some((0..series.len()).map(|_i| "".into()).collect()),
            }
        }
    }
}

#[component]
fn bevy_fps() -> Element {
    let fps = use_bevy_resource::<FPS>();

    rsx! {
        div {
            p { "Bevy framerate: {fps}" }
        }
    }
}

#[component]
// fn progress_bar(curent_time: f64, longest_time: f64) -> Element {
fn progress_bar(curent_time: f32, longest_time: ReadSignal<f32>) -> Element {
    rsx! {
        div {
            style: "
                width: 100%;
                height: 1.5rem;
                background-color: #585b70ff;
                border-radius: calc(1.5rem / 2);
                overflow: hidden;
                color: #11111bff;
                text-justify: center;
                align-content: center;
                align-items: center;
                display: flex;
                flex-direction: row;
                position: relative;
            ",

            div {
                style: "
                    height: 100%;
                    width: 100%;
                    position: absolute;
                    text-align: left;
                ",

                div {
                    style: format!("
                        height: 100%;
                        background-color: #a6e3a1ff;
                        width: {}%;
                        z-index: 1;
                    ",
                        if curent_time > 0.0 && longest_time() > 0.0 {
                            curent_time / longest_time() * 100.
                        } else {
                            0.0
                        }

                    ),
                }

                div {
                    style: "
                        height: 100%;
                        background-color: #00000000;
                        color: #11111bff;
                        width: auto;
                        z-index: 2;
                    ",

                    "{curent_time:.2} sec / {longest_time:.1} sec"
                }
            }
        }
    }
}
