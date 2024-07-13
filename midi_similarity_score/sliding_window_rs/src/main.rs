use anyhow::Context;
use midi_score::PianoRoll;
use midly::Smf;
use std::{
    fs::{self},
    iter,
    path::PathBuf,
};

#[derive(Debug, bpaf::Bpaf)]
#[bpaf(options, version)]
struct Options {
    #[bpaf(positional("TARGET"))]
    target: PathBuf,
    #[bpaf(positional("PREDICTION"))]
    prediction: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let opt = options().fallback_to_usage().run();
    tracing_subscriber::fmt::init();

    let bytes =
        fs::read(&opt.prediction).with_context(|| format!("read '{}'", opt.target.display()))?;
    let smf_pred =
        Smf::parse(&bytes).with_context(|| format!("parse '{}'", opt.target.display()))?;
    let piano_roll_pred = PianoRoll::from_midi(smf_pred)?;

    let bytes =
        fs::read(&opt.target).with_context(|| format!("read '{}'", opt.target.display()))?;
    let smf_target =
        Smf::parse(&bytes).with_context(|| format!("parse '{}'", opt.target.display()))?;
    let piano_roll_target = PianoRoll::from_midi(smf_target)?;

    let pos_score = iter::zip(&piano_roll_target.buckets, &piano_roll_pred.buckets)
        .map(|(target, pred)| iter::zip(target, pred).filter(|(x, y)| x == y).count())
        .sum::<usize>();
    let neg_score = iter::zip(piano_roll_target.buckets, piano_roll_pred.buckets)
        .map(|(target, pred)| iter::zip(target, pred).filter(|(x, y)| x != y).count())
        .sum::<usize>();
    println!("positive score: {pos_score}");
    println!("negative score: {neg_score}");
    println!(
        "{}% correct",
        pos_score as f32 / ((pos_score + neg_score) as f32) * 100.
    );

    Ok(())
}
