use kiro_synth_dsp::float::Float;
use kiro_synth_engine::program::blocks::{dca, envgen, filter, lfo, osc};
use kiro_synth_engine::program::{
  Block, ParamBlock, Program, ProgramBuilder, SignalRef, SourceRef,
};

use crate::synth::program::params::{DcaParams, EnvGenParams, FilterParams, LfoParams, OscParams};
use crate::synth::program::values;

pub struct KiroParams {
  pub pitch_bend: ParamBlock,

  pub lfo1: LfoParams,
  pub lfo2: LfoParams,

  pub eg1: EnvGenParams,

  pub osc1: OscParams,
  pub osc2: OscParams,
  pub osc3: OscParams,
  pub osc4: OscParams,

  pub filter1: FilterParams,

  pub dca: DcaParams,
}

pub struct KiroSignals {
  pub lfo1: SignalRef,
  pub lfo2: SignalRef,
  pub eg1_normal: SignalRef,
  pub eg1_biased: SignalRef,
  pub osc1: SignalRef,
  pub osc2: SignalRef,
  pub osc3: SignalRef,
  pub osc4: SignalRef,
  pub filter1: SignalRef,
  pub dca_left: SignalRef,
  pub dca_right: SignalRef,
}

pub struct KiroSources {
  pub lfo1: SourceRef,
  pub lfo2: SourceRef,
  pub eg1_normal: SourceRef,
  pub eg1_biased: SourceRef,
  pub osc1: SourceRef,
  pub osc2: SourceRef,
  pub osc3: SourceRef,
  pub osc4: SourceRef,
}

pub struct KiroModule {
  pub signals: KiroSignals,
  pub sources: KiroSources,
  pub params: KiroParams,
}

impl KiroModule {
  pub fn new_program<'a, F: Float>(
    num_lfo_shapes: usize,
    num_osc_shapes: usize,
  ) -> (Program<'a, F>, KiroModule) {
    let mut program_builder = ProgramBuilder::new();

    let module = Self::new(&mut program_builder, num_lfo_shapes, num_osc_shapes);

    program_builder.out(module.signals.dca_left, module.signals.dca_right);

    (program_builder.build(), module)
  }

  pub fn new<F: Float>(
    program: &mut ProgramBuilder<F>,
    num_lfo_shapes: usize,
    num_osc_shapes: usize,
  ) -> KiroModule {
    let voice = program.voice().clone();

    let zero = program.const_zero();
    // let one = program.const_one();

    let num_filters = filter::Mode::count();

    let params = KiroParams {
      pitch_bend: program.param("pitch-bend", values::pitch_bend()),

      lfo1: LfoParams {
        shape: program.param("lfo1-shape", values::enumeration(num_lfo_shapes)),
        rate: program.param("lfo1-rate", values::lfo_rate()),
        phase: program.param("lfo1-phase", values::lfo_phase()),
        depth: program.param("lfo1-depth", values::amplitude()),
      },

      lfo2: LfoParams {
        shape: program.param("lfo2-shape", values::enumeration(num_lfo_shapes)),
        rate: program.param("lfo2-rate", values::lfo_rate()),
        phase: program.param("lfo2-phase", values::lfo_phase()),
        depth: program.param("lfo2-depth", values::amplitude()),
      },

      eg1: EnvGenParams {
        attack: program.param("eg1-attack", values::adsr(0.02)),
        decay: program.param("eg1-decay", values::adsr(0.1)),
        sustain: program.param("eg1-sustain", values::adsr(1.0)),
        release: program.param("eg1-release", values::adsr(1.5)),
        mode: program.param("eg1-mode", values::eg_mode()),
        legato: program.param("eg1-legato", values::boolean(false)),
        reset_to_zero: program.param("eg1-reset-to-zero", values::boolean(false)),
        dca_mod: program.param("eg1-dca-mod", values::eg1_dca_amp_mod()),
      },

      osc1: OscParams {
        shape: program.param(
          "osc1-shape",
          values::enumeration(num_osc_shapes).with_initial_value(F::val(2)),
        ),
        amplitude: program.param("osc1-amplitude", values::amplitude()),
        octaves: program.param("osc1-octaves", values::octave()),
        semitones: program.param("osc1-semitones", values::semitones()),
        cents: program.param("osc1-cents", values::cents()),
      },

      osc2: OscParams {
        shape: program.param("osc2-shape", values::enumeration(num_osc_shapes)),
        amplitude: program.param(
          "osc2-amplitude",
          values::amplitude().with_initial_value(F::val(0.25)),
        ),
        octaves: program.param(
          "osc2-octaves",
          values::octave().with_initial_value(F::val(-1)),
        ),
        semitones: program.param("osc2-semitones", values::semitones()),
        cents: program.param("osc2-cents", values::cents()),
      },

      osc3: OscParams {
        shape: program.param("osc3-shape", values::enumeration(num_osc_shapes)),
        amplitude: program.param(
          "osc3-amplitude",
          values::amplitude().with_initial_value(F::zero()),
        ),
        octaves: program.param("osc3-octaves", values::octave()),
        semitones: program.param("osc3-semitones", values::semitones()),
        cents: program.param("osc3-cents", values::cents()),
      },

      osc4: OscParams {
        shape: program.param("osc4-shape", values::enumeration(num_osc_shapes)),
        amplitude: program.param(
          "osc4-amplitude",
          values::amplitude().with_initial_value(F::zero()),
        ),
        octaves: program.param("osc4-octaves", values::octave()),
        semitones: program.param("osc4-semitones", values::semitones()),
        cents: program.param("osc4-cents", values::cents()),
      },

      filter1: FilterParams {
        mode: program.param(
          "filt1-mode",
          values::enumeration(num_filters).with_initial_value(F::val(3)),
        ),
        freq: program.param("filt1-freq", values::filt_freq()),
        q: program.param("filt1-q", values::filt_q()),
      },

      dca: DcaParams {
        amplitude: program.param(
          "dca-amplitude-db",
          values::amplitude_db().with_initial_value(F::val(-3.0)),
        ),
        pan: program.param("dca-pan", values::pan()),
      },
    };

    let signals = KiroSignals {
      lfo1: program.signal(),
      lfo2: program.signal(),
      eg1_normal: program.signal(),
      eg1_biased: program.signal(),
      osc1: program.signal(),
      osc2: program.signal(),
      osc3: program.signal(),
      osc4: program.signal(),
      filter1: program.signal(),
      dca_left: program.signal(),
      dca_right: program.signal(),
    };

    let sources = KiroSources {
      lfo1: program.source("lfo1", signals.lfo1),
      lfo2: program.source("lfo2", signals.lfo2),
      eg1_normal: program.source("eg1", signals.eg1_normal),
      eg1_biased: program.source("eg1-biased", signals.eg1_biased),
      osc1: program.source("osc1", signals.osc1),
      osc2: program.source("osc2", signals.osc2),
      osc3: program.source("osc3", signals.osc3),
      osc4: program.source("osc4", signals.osc4),
    };

    let lfo1 = lfo::Block {
      inputs: lfo::Inputs {
        shape: params.lfo1.shape.out_signal_ref,
        rate: params.lfo1.rate.out_signal_ref,
        phase: params.lfo1.phase.out_signal_ref,
        depth: params.lfo1.depth.out_signal_ref,
      },
      output: signals.lfo1,
    };

    let lfo2 = lfo::Block {
      inputs: lfo::Inputs {
        shape: params.lfo2.shape.out_signal_ref,
        rate: params.lfo2.rate.out_signal_ref,
        phase: params.lfo2.phase.out_signal_ref,
        depth: params.lfo2.depth.out_signal_ref,
      },
      output: signals.lfo2,
    };

    program.modulation(&params.filter1.freq, sources.lfo1, F::val(800));
    program.modulation(&params.filter1.freq, sources.eg1_normal, F::val(700));
    program.modulation(&params.filter1.q, sources.lfo2, F::val(0.09));
    program.modulation(&params.osc1.amplitude, sources.lfo2, F::val(0.1));
    program.modulation(&params.dca.pan, sources.lfo1, F::val(0.1));

    let eg1 = envgen::Block {
      inputs: envgen::Inputs {
        attack: params.eg1.attack.out_signal_ref,
        decay: params.eg1.decay.out_signal_ref,
        sustain: params.eg1.sustain.out_signal_ref,
        release: params.eg1.release.out_signal_ref,
        mode: params.eg1.mode.out_signal_ref,
        legato: params.eg1.legato.out_signal_ref,
        reset_to_zero: params.eg1.reset_to_zero.out_signal_ref,
      },
      outputs: envgen::Outputs {
        normal: signals.eg1_normal,
        biased: signals.eg1_biased,
        voice_off: voice.off,
      },
    };

    let eg1_dca_mod =
      program.expr(|expr| expr.mul_signal_param(eg1.outputs.normal, params.eg1.dca_mod.reference));

    let osc1 = osc::Block {
      inputs: osc::Inputs {
        shape: params.osc1.shape.out_signal_ref,
        amplitude: params.osc1.amplitude.out_signal_ref,
        amp_mod: zero,
        octaves: params.osc1.octaves.out_signal_ref,
        semitones: params.osc1.semitones.out_signal_ref,
        cents: params.osc1.cents.out_signal_ref,
        note_pitch: voice.note_pitch,
        pitch_bend: params.pitch_bend.out_signal_ref,
        freq_mod: zero,
      },
      output: signals.osc1,
    };

    let osc2 = osc::Block {
      inputs: osc::Inputs {
        shape: params.osc2.shape.out_signal_ref,
        amplitude: params.osc2.amplitude.out_signal_ref,
        amp_mod: zero,
        octaves: params.osc2.octaves.out_signal_ref,
        semitones: params.osc2.semitones.out_signal_ref,
        cents: params.osc2.cents.out_signal_ref,
        note_pitch: voice.note_pitch,
        pitch_bend: params.pitch_bend.out_signal_ref,
        freq_mod: zero,
      },
      output: signals.osc2,
    };

    let osc3 = osc::Block {
      inputs: osc::Inputs {
        shape: params.osc3.shape.out_signal_ref,
        amplitude: params.osc3.amplitude.out_signal_ref,
        amp_mod: zero,
        octaves: params.osc3.octaves.out_signal_ref,
        semitones: params.osc3.semitones.out_signal_ref,
        cents: params.osc3.cents.out_signal_ref,
        note_pitch: voice.note_pitch,
        pitch_bend: params.pitch_bend.out_signal_ref,
        freq_mod: zero,
      },
      output: signals.osc3,
    };

    let osc4 = osc::Block {
      inputs: osc::Inputs {
        shape: params.osc4.shape.out_signal_ref,
        amplitude: params.osc4.amplitude.out_signal_ref,
        amp_mod: zero,
        octaves: params.osc4.octaves.out_signal_ref,
        semitones: params.osc4.semitones.out_signal_ref,
        cents: params.osc4.cents.out_signal_ref,
        note_pitch: voice.note_pitch,
        pitch_bend: params.pitch_bend.out_signal_ref,
        freq_mod: zero,
      },
      output: signals.osc4,
    };

    let osc_mix = program.expr(|expr| {
      let sum1 = expr.add_signals(osc1.output, osc2.output);
      let sum2 = expr.add_signals(osc3.output, osc4.output);
      expr.add(sum1, sum2)
    });

    let filter1 = filter::Block {
      input: osc_mix.output,
      params: filter::Params {
        mode: params.filter1.mode.out_signal_ref,
        freq: params.filter1.freq.out_signal_ref,
        freq_mod: zero,
        q: params.filter1.q.out_signal_ref,
      },
      output: signals.filter1,
    };

    let dca = dca::Block {
      inputs: dca::Inputs {
        left: filter1.output,
        right: filter1.output,
        velocity: voice.velocity,
        amplitude: params.dca.amplitude.out_signal_ref,
        amp_mod: zero,
        eg_mod: eg1_dca_mod.output,
        pan: params.dca.pan.out_signal_ref,
        pan_mod: zero,
      },
      outputs: dca::Outputs {
        left: signals.dca_left,
        right: signals.dca_right,
      },
    };

    params.lfo1.add_param_blocks(program);
    program.block(Block::Lfo(lfo1));

    params.lfo2.add_param_blocks(program);
    program.block(Block::Lfo(lfo2));

    params.eg1.add_param_blocks(program);
    program.block(Block::EG(eg1));

    program.block(Block::Expr(eg1_dca_mod));

    params.osc1.add_param_blocks(program);
    program.block(Block::Osc(osc1));

    params.osc2.add_param_blocks(program);
    program.block(Block::Osc(osc2));

    params.osc3.add_param_blocks(program);
    program.block(Block::Osc(osc3));

    params.osc4.add_param_blocks(program);
    program.block(Block::Osc(osc4));

    program.block(Block::Expr(osc_mix));

    params.filter1.add_param_blocks(program);
    program.block(Block::Filter(filter1));

    params.dca.add_param_blocks(program);
    program.block(Block::DCA(dca));

    KiroModule {
      signals,
      sources,
      params,
    }
  }
}
