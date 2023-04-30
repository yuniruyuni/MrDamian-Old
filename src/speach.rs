use std::sync::mpsc::{Receiver, Sender};
use miette::{IntoDiagnostic, Result};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

struct AudioTranslator {
    buffer: Vec<f32>,
    sender: Sender<Vec<f32>>,
}

impl AudioTranslator {
    const SAMPLE_RATE: usize = 16000;
    const INTERVAL: usize = 3;
    const MIN_CHUNK_SIZE: usize = Self::SAMPLE_RATE * Self::INTERVAL;

    fn new(sender: Sender<Vec<f32>>) -> Result<Self> {
        Ok(Self {
            buffer: vec![],
            sender,
        })
    }

    fn process(&mut self, inputs: &[f32]) -> Result<()> {
        let data: Vec<f32> = inputs.to_vec();
        self.buffer.extend(data);

        if self.buffer.len() < Self::MIN_CHUNK_SIZE {
            return Ok(());
        }
        let (msg, rest) = self.buffer.split_at(Self::MIN_CHUNK_SIZE);
        self.sender.send(msg.to_vec()).into_diagnostic()?;
        self.buffer = rest.to_vec();

        Ok(())
    }
}

pub async fn audio_translate(path: &str, receiver: Receiver<Vec<f32>>) -> Result<()> {
    let whisper = WhisperContext::new(path).into_diagnostic()?;
    let mut state = whisper.create_state().into_diagnostic()?;

    for buf in receiver {
        let mut params = FullParams::new(SamplingStrategy::default());
        params.set_language(Some("ja"));
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_special(false);
        params.set_print_timestamps(false);

        state.full(params, &buf[..]).into_diagnostic()?;

        let n = state.full_n_segments().into_diagnostic()?;
        for i in 0..n {
            let segment = state.full_get_segment_text(i).into_diagnostic()?;
            println!("{}", segment);
        }
    }

    Ok(())
}

pub async fn run(sender: Sender<Vec<f32>>) -> Result<()> {
    let host = cpal::default_host();
    let device = host.default_input_device().expect("device unknown");
    // let device_name = device.name().unwrap();

    let config = device
        .supported_input_configs()
        .into_diagnostic()?
        .find(|conf| conf.channels() == 2)
        .map(|conf| conf.with_sample_rate(cpal::SampleRate(48000)))
        .unwrap();

    let mut translator = AudioTranslator::new(sender)?;

    let stream = device
        .build_input_stream(
            &config.into(),
            move |data, _: &_| {
                let mut buf: Vec<f32> = vec![];
                for (i, v) in data.iter().enumerate() {
                    // It needs sampling rate 16000 mono but my device have just 48000 stereo.
                    // So i'm changing sampling rate and cut stereo voice from bufer by skipping buffer.
                    if i % 6 == 0 {
                        buf.push(*v);
                    }
                }
                translator.process(&buf[..]).unwrap();
            },
            |_| (),
            None,
        )
        .into_diagnostic()?;

    stream.play().into_diagnostic()?;

    loop {
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
