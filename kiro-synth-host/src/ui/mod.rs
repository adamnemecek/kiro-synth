pub mod knob;
mod model;
mod widget;

use std::sync::{Arc, Mutex};

use druid::{WindowDesc, LocalizedString, AppLauncher};

use kiro_synth_core::float::Float;

use crate::synth::SynthClient;

pub use model::SynthData;


pub fn start<F: Float + 'static>(synth_data: SynthData,
                                 synth_client: Arc<Mutex<SynthClient<F>>>) {

  let data = synth_data.clone();

  let window = WindowDesc::new(move || widget::build(&synth_data, synth_client.clone()))
      .title(
        LocalizedString::new("custom-widget-demo-window-title")
            .with_placeholder("Kiro Synth")
      )
      .window_size((480.0, 440.0));

  AppLauncher::with_window(window)
      .configure_env(|env, _data| knob::theme::init(env))
      .use_simple_logger()
      .launch(data)
      .expect("UI launch failed");
}