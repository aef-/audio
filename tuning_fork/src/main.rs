use std::fs::File;
use std::io::prelude::*;
use plotters::prelude::*;
use std::f64::consts::TAU;
use byteorder::{LittleEndian, WriteBytesExt};
use byteorder::{ByteOrder};

type Seconds = usize;
type SampleRate = usize;
type Frequency = usize;

const FREQUENCY: Frequency = 440;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let duration: Seconds =  20;
    let rate: SampleRate =  44100;
    let amplitude = 1.0_f64;

    let num_of_samples = duration * rate;

    let angle_increase_rate =  TAU * FREQUENCY as f64 / num_of_samples as f64;

    let rate_of_decay = 0.01_f64.powf(1.0/num_of_samples as f64);

    let samples = (0..=num_of_samples)
        .collect::<Vec<usize>>()
        .iter()
        .map(|i| amplitude * (angle_increase_rate * *i as f64).sin() * rate_of_decay.powi(*i as i32))
        .collect::<Vec<f64>>();

    generate_graph(&samples);
    write_to_binary(&samples)
}

/**
 * This generates a raw binary file that can be opened
 * by a program like Audacity.
 * File > Import > Raw Data
 *
 * Select 64-bit Float for encoding
 * Byte Order is Native
 * Channels is Mono
 * Sample rate is 44100
 **/
fn write_to_binary(val: &Vec<f64>) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("tuning_fork.bin")?;
    let mut bytes;
    for v in val {
        bytes = v.to_ne_bytes();
        file.write_all(&bytes)?
    }

    Ok(())
    //println!("{:?}", bytes);
}

fn generate_graph(val: &Vec<f64>) -> Result<(), Box<dyn std::error::Error>> {

    let root = SVGBackend::new("output.svg", (1240, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Tuning Fork", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..val.len(), -1f64..1f64)?;
    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
                val.iter().enumerate().map(|(x, y)| (x, *y)),
                &RED,
        ))?
        .label("sin")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
