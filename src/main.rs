use hound;

fn main() {
    // Synthesis parameters
    let sample_rate = 44100;
    let duration_secs = 5;
    let amplitude = 0.5;
    let carrier_freq = 440.0;      // Frequency of the carrier signal (Hz)
    let modulator_freq = 220.0;    // Frequency of the modulator signal (Hz)
    let modulation_index = 2.0;    // Modulation index (depth of modulation)

    // Create a WAV file writer
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("fm_synth.wav", spec).unwrap();

    // Total number of samples
    let total_samples = duration_secs * sample_rate as usize;

    // Generate samples
    for n in 0..total_samples {
        let t = n as f32 / sample_rate as f32;

        // FM synthesis formula
        let sample = amplitude
            * (2.0 * std::f32::consts::PI * carrier_freq * t
                + modulation_index
                    * (2.0 * std::f32::consts::PI * modulator_freq * t).sin())
                .sin();

        // Convert the sample to a 16-bit integer
        let sample_i16 = (sample * i16::MAX as f32) as i16;
        writer.write_sample(sample_i16).unwrap();
    }

    // Finalize the WAV file
    writer.finalize().unwrap();
}

