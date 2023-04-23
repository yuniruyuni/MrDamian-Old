mod client;
use client::Client;
use miette::{IntoDiagnostic, Result, WrapErr};
use std::env;

use std::sync::mpsc::{channel, Receiver, Sender};

use whisper_rs::{FullParams, SamplingStrategy, WhisperContext};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

struct Config {
    bot: String,
    channel: String,
    token: String,
}

impl Config {
    fn load_envs() -> Result<Self> {
        let bot = env::var("TWITCH_BOT_USERNAME")
            .into_diagnostic()
            .wrap_err("TWITCH_BOT_USERNAME must be set.")?;
        let channel = env::var("TWITCH_CHANNEL")
            .into_diagnostic()
            .wrap_err("TWITCH_CHANNEL must be set.")?;
        let token = env::var("TWITCH_OAUTH_TOKEN")
            .into_diagnostic()
            .wrap_err("TWITCH_OAUTH_TOKEN must be set.")?;

        Ok(Self {
            bot,
            channel,
            token,
        })
    }
}

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

    fn process(&mut self, inputs: &[f32]) {
        let data: Vec<f32> = inputs.to_vec();
        self.buffer.extend(data);

        if self.buffer.len() < Self::MIN_CHUNK_SIZE {
            return;
        }
        let (msg, rest) = self.buffer.split_at(Self::MIN_CHUNK_SIZE);
        self.sender.send(msg.to_vec()).unwrap();
        self.buffer = rest.to_vec();
    }
}

async fn audio_translate(receiver: Receiver<Vec<f32>>) -> Result<()> {
    let whisper = WhisperContext::new("./models/ggml-base.bin").into_diagnostic()?;
    whisper.create_key(()).into_diagnostic()?;

    for buf in receiver {
        let mut params = FullParams::new(SamplingStrategy::default());
        params.set_language(Some("ja"));
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_special(false);
        params.set_print_timestamps(false);
        whisper.full(&(), params, &buf[..]).into_diagnostic()?;

        let n = whisper.full_n_segments(&()).into_diagnostic()?;
        for i in 0..n {
            let segment = whisper.full_get_segment_text(&(), i).into_diagnostic()?;
            println!("{}", segment);
        }
    }

    Ok(())
}

async fn run_sound(sender: Sender<Vec<f32>>) -> Result<()> {
    let host = cpal::default_host();
    let device = host.default_input_device().expect("device unknown");
    let device_name = device.name().unwrap();
    println!("device: {}", device_name);

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
                translator.process(&buf[..]);
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

#[tokio::main]
async fn main() -> Result<()> {
    let websocket_thread = tokio::spawn(async move {
        let config = Config::load_envs()?;
        let mut wsclient = Client::new(&config.bot, &config.channel, &config.token).await?;
        wsclient.run().await?;
        Ok(())
    });

    let (sender, receiver) = channel();

    let audio_thread = tokio::spawn(async { run_sound(sender).await });

    let translate_thread = tokio::spawn(async { audio_translate(receiver).await });

    tokio::try_join!(
        flatten(websocket_thread),
        flatten(audio_thread),
        flatten(translate_thread),
    )?;

    Ok(())
}

async fn flatten(h: tokio::task::JoinHandle<Result<()>>) -> Result<()> {
    match h.await {
        Ok(Ok(())) => Ok(()),
        Ok(Err(e)) => Err(e),
        Err(e) => Err(e).into_diagnostic(),
    }
}
