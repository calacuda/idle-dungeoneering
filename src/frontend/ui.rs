use std::ops::Deref;

use crate::{
    backend::{
        idle_time_plugin::{IDLE_SAMPLE_WINDOW, IdleTimeSample},
        *,
    },
    frontend::line::LineChart,
};
use bevy_dioxus_hooks::resource::hook::use_bevy_resource;
use bevy_dioxus_sync::panels::DioxusElementMarker;
use dioxus::prelude::*;

#[derive(Debug)]
pub struct AppUi;

impl DioxusElementMarker for AppUi {
    fn element(&self) -> Element {
        // app_ui()
        game_ui()
    }
}

#[component]
pub fn game_ui() -> Element {
    let idle_time_res = use_bevy_resource::<CurrentIdleTimeSeconds>();
    let best_idle_time_res = use_bevy_resource::<LongestIdleTimeSeconds>();
    // let test_component = use_bevy_query::<(Entity, &TestComponent), ()>();
    let mut max_idle_time: Signal<f32> = use_signal(|| 0.0);
    let mut idle_times: Signal<Vec<_>> = use_signal(|| Vec::new());

    use_effect({
        let best_idle_time_res = best_idle_time_res.clone();

        move || {
            *max_idle_time.write() = best_idle_time_res
                .get()
                .map(|time| time.0 as f32)
                .unwrap_or(0.0);
        }
    });

    use_effect({
        let idle_time_res = idle_time_res.clone();

        move || {
            idle_times.write().push(IdleTimeSample::new(
                idle_time_res.get().map(|time| time.0).unwrap_or(0.0),
            ));
            idle_times
                .write()
                .retain(|time| time.when.elapsed() < IDLE_SAMPLE_WINDOW);
        }
    });

    rsx! {
        document::Stylesheet { href: asset!("src/frontend/ui.css") }

        main {
            // progress_bar { curent_time: idle_time_res.get().map(|time| time.0).unwrap_or(0.0), longest_time: best_idle_time_res.get().map(|time| time.0).unwrap_or(0.0) }
            progress_bar { curent_time: idle_time_res.get().map(|time| time.0 as f32).unwrap_or(0.0), longest_time: max_idle_time }
            idle_time_graph {
                heighest: max_idle_time,
                time_samples: idle_times,
            }
        }
    }
}

#[component]
fn idle_time_graph(
    heighest: ReadSignal<f32>,
    time_samples: ReadSignal<Vec<IdleTimeSample>>,
) -> Element {
    let sort = |samples: Vec<IdleTimeSample>| {
        let mut new_samples = samples.clone();
        new_samples.sort_by(|val1, val2| {
            val2.when
                .elapsed()
                .as_secs_f64()
                .total_cmp(&val1.when.elapsed().as_secs_f64())
        });
        new_samples
    };
    let mut series = use_signal(|| Vec::new());

    use_effect(move || {
        *series.write() = sort(time_samples())
            .iter()
            .map(|time| time.time as f32)
            .collect();
    });

    rsx! {
        div {
            style: "
                width: 25%;
                height: 12.5%;
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
                // TODO: replace with window size measuermenets
                viewbox_width: 1920 / 4,
                viewbox_height: 1080 / 8,
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

        // div {
        //     for message in messages.iter() {
        //         div {
        //             "{message:?}"
        //         }
        //     }
        // }
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
