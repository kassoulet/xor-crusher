use lv2::prelude::*;
use log::info;

// The input and output ports.
#[derive(PortCollection)]
struct XorCrusherPorts {
    audio_in_1: InputPort<Audio>,
    audio_in_2: InputPort<Audio>,
    audio_out: OutputPort<Audio>,
    bit_depth: InputPort<Control>,
    dry_wet: InputPort<Control>,
}

#[derive(FeatureCollection)]
struct Features {
}

#[uri("http://example.com/xor_crusher")]
struct XorCrusher {
}

impl Plugin for XorCrusher {
    type Ports = XorCrusherPorts;
    type InitFeatures = Features;
    type AudioFeatures = ();

    fn new(_plugin_info: &PluginInfo, _features: &mut Features) -> Option<Self> {
        info!("XOR Crusher Plugin Instantiated.");
        Some(Self {})
    }

    fn run(&mut self, ports: &mut XorCrusherPorts, _: &mut (), _: u32) {
        // Get the number of bits from the control port.
        // Clamp it to a safe range [1, 32].
        let bits = ports.bit_depth.clamp(1.0, 32.0).round() as u32;

        // Get the dry/wet mix from the control port.
        // Clamp it to a safe range [0.0, 1.0].
        let dry_wet = ports.dry_wet.clamp(0.0, 1.0);

        // Determine the bit shift amount.
        // A shift of 32 or more would zero out the number, so handle that.
        let shift = if bits > 32 { 0 } else { 32 - bits };

        // Process each sample in the block.
        for (i, (in1_sample, in2_sample)) in ports
            .audio_in_1
            .iter()
            .zip(ports.audio_in_2.iter())
            .enumerate()
        {
            // 1. Convert float samples [-1.0, 1.0] to 32-bit signed integers.
            let int_sample1 = (*in1_sample * i32::MAX as f32) as i32;
            let int_sample2 = (*in2_sample * i32::MAX as f32) as i32;

            // 2. Quantize by shifting right to discard lower bits, then shifting left.
            // This effectively reduces the bit depth.
            let quantized1 = (int_sample1 >> shift) << shift;
            let quantized2 = (int_sample2 >> shift) << shift;

            // 3. Perform the bitwise XOR operation.
            let xor_result = quantized1 ^ quantized2;

            // 4. Convert the result back to a float for the wet signal.
            let wet_signal = xor_result as f32 / i32::MAX as f32;

            // 5. Calculate the dry signal (mono mix of inputs).
            let dry_signal = (*in1_sample + *in2_sample) / 2.0;

            // 6. Mix dry and wet signals.
            ports.audio_out[i] = (dry_signal * (1.0 - dry_wet)) + (wet_signal * dry_wet);
        }
    }
}

lv2_descriptors!(XorCrusher);
