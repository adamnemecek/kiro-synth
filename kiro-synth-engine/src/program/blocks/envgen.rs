use kiro_synth_dsp::envgen::adsr::{EnvGen, Mode};
use kiro_synth_dsp::float::Float;

use crate::program::{Program, SignalRef};
use crate::signal::SignalBus;

#[derive(Debug, Clone)]
pub struct Inputs {
  pub attack: SignalRef,
  pub decay: SignalRef,
  pub sustain: SignalRef,
  pub release: SignalRef,
  pub mode: SignalRef,
  pub legato: SignalRef,
  pub reset_to_zero: SignalRef,
}

#[derive(Debug, Clone)]
pub struct Outputs {
  pub normal: SignalRef,
  pub biased: SignalRef,
  pub voice_off: SignalRef,
}

#[derive(Debug, Clone)]
pub struct Block {
  pub inputs: Inputs,
  pub outputs: Outputs,
}

#[derive(Debug)]
pub(crate) struct Processor<F: Float> {
  envgen: EnvGen<F>,
  block: Block,
}

impl<F: Float> Processor<F> {
  pub fn new(sample_rate: F, block: Block) -> Self {
    Processor {
      envgen: EnvGen::new(sample_rate),
      block,
    }
  }

  pub fn reset(&mut self) {
    self.envgen.reset()
  }

  pub fn process<'a>(&mut self, signals: &mut SignalBus<'a, F>, program: &Program<F>) {
    let Block { inputs, outputs } = self.block.clone();
    let Inputs {
      attack,
      decay,
      sustain,
      release,
      mode,
      legato: _,
      reset_to_zero: _,
    } = inputs;
    let Outputs {
      normal,
      biased,
      voice_off,
    } = outputs;

    let voice = program.voice();

    signals[voice.trigger].if_updated(|value| {
      if value > F::zero() {
        self.envgen.start();
      }
    });

    signals[voice.gate].if_updated(|value| {
      if value == F::zero() {
        self.envgen.note_off();
      }
    });

    signals[attack].if_updated(|value| self.envgen.set_attack_time_sec(value));
    signals[decay].if_updated(|value| self.envgen.set_decay_time_sec(value));
    signals[sustain].if_updated(|value| self.envgen.set_sustain_level(value));
    signals[release].if_updated(|value| self.envgen.set_release_time_sec(value));

    signals[mode].if_updated(|value| match value {
      v if v == F::zero() => self.envgen.set_mode(Mode::Analog),
      v if v == F::one() => self.envgen.set_mode(Mode::Digital),
      _ => {}
    });

    // TODO   signals[legato].if_updated(|value| self.envgen.set_legato(value));
    // TODO   signals[reset_to_zero].if_updated(|value| self.envgen.set_reset_to_zero(value));

    signals[normal].set(self.envgen.generate());
    signals[biased].set(self.envgen.biased_output());

    if self.envgen.is_off() {
      signals[voice_off].set(F::one());
    }
  }
}
