/**
 * Hodge podge of functions for messing around with audio DSP.
 **/

const TONE_RATIO: f64 = 1.0594630943592953; //2_f64.powf(1.0/12.0)  Equal Temperament Semitone Ratio
const NOTE_C5: f64 = 261.6255653005987; // 220.0 * TONE_RATIO.powf(3.0);
const NOTE_C0: f64 = 8.175798915643709; // NOTE_C5 * 0.5_f64.powf(5.0);
type Midi = u32;
type Note = f64;


fn midi_to_freq(midi_note: Midi) -> Note {
    NOTE_C0 * TONE_RATIO.powf(midi_note.into())
}

fn freq_to_midi(freq: Note) -> Midi {
    ((freq / NOTE_C0).ln() / TONE_RATIO.ln()).round() as Midi
}

fn freq_deviation(freq: Note, midi_note: Midi) -> Note {
    let actual_freq = midi_to_freq(midi_note);
    10.0 / (freq - actual_freq)
}

fn scale(midi_note: Midi, n: u16) -> Vec<f64> {
    let freq = midi_to_freq(midi_note);
    let ratio = 2.0_f64.powf(1.0/n as f64);
    (1..=n)
        .collect::<Vec<u16>>()
        .iter()
        .map(|i| freq * ratio.powf(*i as f64))
        .collect::<Vec<f64>>()
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_converts_midi_to_freq() {
        assert_eq!(midi_to_freq(60), 261.62556530059936);
    }

    #[test]
    fn it_converts_freq_to_midi() {
        assert_eq!(freq_to_midi(261.62556530059936), 60);
    }

    #[test]
    fn it_returns_freq_deviation_converting_to_midi() {
        assert_eq!(freq_deviation(265.0, 60), 0.4);
    }

    #[test]
    fn it_returns_a_scale() {
        assert_eq!(scale(60, 3), vec![1.0,2.0]);
    }
}
