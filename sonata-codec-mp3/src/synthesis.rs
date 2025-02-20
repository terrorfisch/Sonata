// Sonata
// Copyright (c) 2019 The Sonata Project Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The `synthesis` module implements the polyphase synthesis filterbank of the MPEG audio standard.

/// Synthesis window D[i], defined in Table B.3 of ISO/IEC 11172-3.
const SYNTHESIS_D: [f32; 512] = [
     0.000000000, -0.000015259, -0.000015259, -0.000015259,
    -0.000015259, -0.000015259, -0.000015259, -0.000030518,
    -0.000030518, -0.000030518, -0.000030518, -0.000045776,
    -0.000045776, -0.000061035, -0.000061035, -0.000076294,
    -0.000076294, -0.000091553, -0.000106812, -0.000106812,
    -0.000122070, -0.000137329, -0.000152588, -0.000167847,
    -0.000198364, -0.000213623, -0.000244141, -0.000259399,
    -0.000289917, -0.000320435, -0.000366211, -0.000396729,
    -0.000442505, -0.000473022, -0.000534058, -0.000579834,
    -0.000625610, -0.000686646, -0.000747681, -0.000808716,
    -0.000885010, -0.000961304, -0.001037598, -0.001113892,
    -0.001205444, -0.001296997, -0.001388550, -0.001480103,
    -0.001586914, -0.001693726, -0.001785278, -0.001907349,
    -0.002014160, -0.002120972, -0.002243042, -0.002349854,
    -0.002456665, -0.002578735, -0.002685547, -0.002792358,
    -0.002899170, -0.002990723, -0.003082275, -0.003173828,
     0.003250122,  0.003326416,  0.003387451,  0.003433228,
     0.003463745,  0.003479004,  0.003479004,  0.003463745,
     0.003417969,  0.003372192,  0.003280640,  0.003173828,
     0.003051758,  0.002883911,  0.002700806,  0.002487183,
     0.002227783,  0.001937866,  0.001617432,  0.001266479,
     0.000869751,  0.000442505, -0.000030518, -0.000549316,
    -0.001098633, -0.001693726, -0.002334595, -0.003005981,
    -0.003723145, -0.004486084, -0.005294800, -0.006118774,
    -0.007003784, -0.007919312, -0.008865356, -0.009841919,
    -0.010848999, -0.011886597, -0.012939453, -0.014022827,
    -0.015121460, -0.016235352, -0.017349243, -0.018463135,
    -0.019577026, -0.020690918, -0.021789551, -0.022857666,
    -0.023910522, -0.024932861, -0.025909424, -0.026840210,
    -0.027725220, -0.028533936, -0.029281616, -0.029937744,
    -0.030532837, -0.031005859, -0.031387329, -0.031661987,
    -0.031814575, -0.031845093, -0.031738281, -0.031478882,
     0.031082153,  0.030517578,  0.029785156,  0.028884888,
     0.027801514,  0.026535034,  0.025085449,  0.023422241,
     0.021575928,  0.019531250,  0.017257690,  0.014801025,
     0.012115479,  0.009231567,  0.006134033,  0.002822876,
    -0.000686646, -0.004394531, -0.008316040, -0.012420654,
    -0.016708374, -0.021179199, -0.025817871, -0.030609131,
    -0.035552979, -0.040634155, -0.045837402, -0.051132202,
    -0.056533813, -0.061996460, -0.067520142, -0.073059082,
    -0.078628540, -0.084182739, -0.089706421, -0.095169067,
    -0.100540161, -0.105819702, -0.110946655, -0.115921021,
    -0.120697021, -0.125259399, -0.129562378, -0.133590698,
    -0.137298584, -0.140670776, -0.143676758, -0.146255493,
    -0.148422241, -0.150115967, -0.151306152, -0.151962280,
    -0.152069092, -0.151596069, -0.150497437, -0.148773193,
    -0.146362305, -0.143264771, -0.139450073, -0.134887695,
    -0.129577637, -0.123474121, -0.116577148, -0.108856201,
     0.100311279,  0.090927124,  0.080688477,  0.069595337,
     0.057617187,  0.044784546,  0.031082153,  0.016510010,
     0.001068115, -0.015228271, -0.032379150, -0.050354004,
    -0.069168091, -0.088775635, -0.109161377, -0.130310059,
    -0.152206421, -0.174789429, -0.198059082, -0.221984863,
    -0.246505737, -0.271591187, -0.297210693, -0.323318481,
    -0.349868774, -0.376800537, -0.404083252, -0.431655884,
    -0.459472656, -0.487472534, -0.515609741, -0.543823242,
    -0.572036743, -0.600219727, -0.628295898, -0.656219482,
    -0.683914185, -0.711318970, -0.738372803, -0.765029907,
    -0.791213989, -0.816864014, -0.841949463, -0.866363525,
    -0.890090942, -0.913055420, -0.935195923, -0.956481934,
    -0.976852417, -0.996246338, -1.014617920, -1.031936646,
    -1.048156738, -1.063217163, -1.077117920, -1.089782715,
    -1.101211548, -1.111373901, -1.120223999, -1.127746582,
    -1.133926392, -1.138763428, -1.142211914, -1.144287109,
     1.144989014,  1.144287109,  1.142211914,  1.138763428,
     1.133926392,  1.127746582,  1.120223999,  1.111373901,
     1.101211548,  1.089782715,  1.077117920,  1.063217163,
     1.048156738,  1.031936646,  1.014617920,  0.996246338,
     0.976852417,  0.956481934,  0.935195923,  0.913055420,
     0.890090942,  0.866363525,  0.841949463,  0.816864014,
     0.791213989,  0.765029907,  0.738372803,  0.711318970,
     0.683914185,  0.656219482,  0.628295898,  0.600219727,
     0.572036743,  0.543823242,  0.515609741,  0.487472534,
     0.459472656,  0.431655884,  0.404083252,  0.376800537,
     0.349868774,  0.323318481,  0.297210693,  0.271591187,
     0.246505737,  0.221984863,  0.198059082,  0.174789429,
     0.152206421,  0.130310059,  0.109161377,  0.088775635,
     0.069168091,  0.050354004,  0.032379150,  0.015228271,
    -0.001068115, -0.016510010, -0.031082153, -0.044784546,
    -0.057617187, -0.069595337, -0.080688477, -0.090927124,
     0.100311279,  0.108856201,  0.116577148,  0.123474121,
     0.129577637,  0.134887695,  0.139450073,  0.143264771,
     0.146362305,  0.148773193,  0.150497437,  0.151596069,
     0.152069092,  0.151962280,  0.151306152,  0.150115967,
     0.148422241,  0.146255493,  0.143676758,  0.140670776,
     0.137298584,  0.133590698,  0.129562378,  0.125259399,
     0.120697021,  0.115921021,  0.110946655,  0.105819702,
     0.100540161,  0.095169067,  0.089706421,  0.084182739,
     0.078628540,  0.073059082,  0.067520142,  0.061996460,
     0.056533813,  0.051132202,  0.045837402,  0.040634155,
     0.035552979,  0.030609131,  0.025817871,  0.021179199,
     0.016708374,  0.012420654,  0.008316040,  0.004394531,
     0.000686646, -0.002822876, -0.006134033, -0.009231567,
    -0.012115479, -0.014801025, -0.017257690, -0.019531250,
    -0.021575928, -0.023422241, -0.025085449, -0.026535034,
    -0.027801514, -0.028884888, -0.029785156, -0.030517578,
     0.031082153,  0.031478882,  0.031738281,  0.031845093,
     0.031814575,  0.031661987,  0.031387329,  0.031005859,
     0.030532837,  0.029937744,  0.029281616,  0.028533936,
     0.027725220,  0.026840210,  0.025909424,  0.024932861,
     0.023910522,  0.022857666,  0.021789551,  0.020690918,
     0.019577026,  0.018463135,  0.017349243,  0.016235352,
     0.015121460,  0.014022827,  0.012939453,  0.011886597,
     0.010848999,  0.009841919,  0.008865356,  0.007919312,
     0.007003784,  0.006118774,  0.005294800,  0.004486084,
     0.003723145,  0.003005981,  0.002334595,  0.001693726,
     0.001098633,  0.000549316,  0.000030518, -0.000442505,
    -0.000869751, -0.001266479, -0.001617432, -0.001937866,
    -0.002227783, -0.002487183, -0.002700806, -0.002883911,
    -0.003051758, -0.003173828, -0.003280640, -0.003372192,
    -0.003417969, -0.003463745, -0.003479004, -0.003479004,
    -0.003463745, -0.003433228, -0.003387451, -0.003326416,
     0.003250122,  0.003173828,  0.003082275,  0.002990723,
     0.002899170,  0.002792358,  0.002685547,  0.002578735,
     0.002456665,  0.002349854,  0.002243042,  0.002120972,
     0.002014160,  0.001907349,  0.001785278,  0.001693726,
     0.001586914,  0.001480103,  0.001388550,  0.001296997,
     0.001205444,  0.001113892,  0.001037598,  0.000961304,
     0.000885010,  0.000808716,  0.000747681,  0.000686646,
     0.000625610,  0.000579834,  0.000534058,  0.000473022,
     0.000442505,  0.000396729,  0.000366211,  0.000320435,
     0.000289917,  0.000259399,  0.000244141,  0.000213623,
     0.000198364,  0.000167847,  0.000152588,  0.000137329,
     0.000122070,  0.000106812,  0.000106812,  0.000091553,
     0.000076294,  0.000076294,  0.000061035,  0.000061035,
     0.000045776,  0.000045776,  0.000030518,  0.000030518,
     0.000030518,  0.000030518,  0.000015259,  0.000015259,
     0.000015259,  0.000015259,  0.000015259,  0.000015259,
];

/// `SynthesisState` maintains the persistant state of sub-band synthesis.
pub struct SynthesisState {
    v_vec: [[f32; 64]; 16],
    v_front: usize,
}

impl Default for SynthesisState {
    fn default() -> Self {
        SynthesisState {
            v_vec: [[0f32; 64]; 16],
            v_front: 0,
        }
    }
}

/// Sub-band synthesis transforms 32 sub-band blocks containing 18 time-domain samples each into
/// 18 blocks of 32 PCM audio samples.
pub fn synthesis(in_samples: &mut [f32; 576], state: &mut SynthesisState, out: &mut [f32]) {
    let mut u_vec = [0f32; 512];
    let mut s_vec = [0f32; 32];

    // Get a slice to 576 output samples to (hopefully) elide future bounds checking.
    let out_samples = &mut out[0..576];

    // There are 18 synthesized PCM sample blocks.
    for b in 0..18 {
        // First, select the b-th sample from each of the 32 sub-bands, and place them in the s 
        // vector, s_vec..
        for i in (0..32).step_by(4) {
            s_vec[i+0] = in_samples[18*(i+0) + b];
            s_vec[i+1] = in_samples[18*(i+1) + b];
            s_vec[i+2] = in_samples[18*(i+2) + b];
            s_vec[i+3] = in_samples[18*(i+3) + b];
        }

        // Get the front slot of the v_vec FIFO.
        let v_vec = &mut state.v_vec[state.v_front];

        // Matrixing is performed next. As per the standard, matrixing would require 2048 
        // multiplications per sub-band! However, following the method by Konstantinides 
        // published in [1], it is possible to achieve the same result through the use of a 32-point
        // DCT followed by some reconstruction.
        //
        // It should be noted that this is a deceptively simple solution. It is instructive to 
        // derive the algorithm before getting to the implementation to better understand what is 
        // happening, and where the edge-cases are.
        //
        // First, there are a few key observations to this approach:
        //
        //     1) The matrixing operation as per the standard is simply a 32-point MDCT. Remember
        //        that an N-point MDCT produces a 2N-point output.
        //
        //     2) The output of any MDCT contains repeated blocks of values. If the result of a MDCT 
        //        defined as is X[0..64), then: X(16..0] = X(48..32], and X[48..64) = -X[16..32).
        //        
        //        Corollary: Only points [16..48) of the MDCT are actually required!
        //
        //      3) Points [16..48) of the MDCT can be mapped from a 32-point DCT of the input 
        //         vector thus allowing the use of an efficient DCT algorithm.
        //
        // The mappings above can be found graphically by plotting each row of the cosine 
        // coefficient matricies of both the DCT and MDCT side-by side. The mapping becomes readily 
        // apparent, and so too do the exceptions.
        //
        // Using the observations above, if we apply a 32-point DCT transform to the input vector, 
        // s_vec, and place the result of that DCT in v_vec[16..48], we obtain step 1 as shown on 
        // the plot below.
        //
        // Next, map the 32-point DCT to points [16..48] of the 32-point MDCT, as shown in step 2.
        //
        // Finally, map points [16..38] of the MDCT to points [0..16], and [48..64] of the MDCT, as 
        // shown in step 3.
        //
        //              0              16             32             48              64
        // Step 1:      .               .              .              .               .
        //              .               .              .              .               .
        //              .               .     +--------+   +----------+               .
        //              .               +-----+   A    | /     B      |               .
        //              +---------------+--------------+--------------+---------------+
        //              .               .              .              .               .
        // Step 2:      .               .              .              .               .
        //              .               .              .              .               .
        //              +---------------+--------------+--------------+---------------+
        //              .               |     -B     / |   -A   +-----+               .
        //              .               +----------+   +--------+     .               . 
        //              .               .              .              .               .
        // Step 3:      .               .              .              .               .
        //              .               .              .              .               .
        //              .   +-----------+              .              .               .
        //              . /      B      |              .              .               .
        //              +---------------+--------------+--------------+---------------+
        //              .               |     -B     / |   -A   +-----+-----+   -A    |
        //              .               +----------+   +--------+     .     +---------+ 
        //              .               .              .              .               .
        //
        // Note however that the mappings in steps 2 and 3 have exceptions for boundary cases. 
        // These exceptions can be seen when plotting the coefficient matricies as mentioned above,
        // but are listed below for completeness:
        //
        //     1) m[ 0] =  m[32]
        //     2) m[32] = -m[32]
        //     3) m[48] = -m[16]
        //     4) m[16] = 0.0
        //
        // The final algorithm written below combines steps 2 and 3, and excludes the boundaries in
        // the main loop.
        //
        // [1] K. Konstantinides, "Fast subband filtering in MPEG audio coding", Signal Processing 
        // Letters IEEE, vol. 1, no. 2, pp. 26-28, 1994.
        //
        // https://ieeexplore.ieee.org/abstract/document/300309
        dct32(&s_vec, &mut v_vec[16..48]);

        for i in 0..15 {
            let a = -v_vec[16+i+1];
            let b = v_vec[48-i-1];
            v_vec[16-i-1] = b;
            v_vec[16+i+1] = -b;
            v_vec[48-i-1] = a;
            v_vec[48+i+1] = a;
        }

        v_vec[ 0] = v_vec[32];
        v_vec[32] = -v_vec[32];
        v_vec[48] = -v_vec[16];
        v_vec[16] = 0.0;

        // Next, build u_vec by iterating over the 16 slots in v_vec, and copying the first 32 
        // samples of EVEN numbered v_vec slots, and the last 32 samples of ODD numbered v_vec slots 
        // sequentially into u_vec.
        //
        // For example, given:
        //
        //        0   32   64   96  128  160  192  224  256           896  928   960  992 1024
        //        +----+----+----+----+----+----+----+----+ . . . . . . +----+-----+----+----+
        // v_vec  | a  : b  | c  : d  | e  : f  | g  | h  | . . . . . . | w  : x   | y  : z  |
        //        +----+----+----+----+----+----+----+----+ . . . . . . +----+-----+----+----+
        //        [ Slot 0 ][ Slot 1 ][ Slot 2 ][ Slot 3 ]  . . . . . . [ Slot 14 ][ Slot 15 ]
        //
        // Assuming v_front, the front of the FIFO, is slot 0, then u_vec is filled as follows:
        //
        //        0   32   64   96  128       448  480  512
        //        +----+----+----+----+ . . . . +----+----+
        // u_vec  | a  | d  | e  | h  | . . . . | w  | z  | 
        //        +----+----+----+----+ . . . . +----+----+
        //
        for i in 0..8 {
            let v_start = state.v_front + (i << 1);
            let v0 = &state.v_vec[(v_start + 0) & 0xf][ 0..32];
            let v1 = &state.v_vec[(v_start + 1) & 0xf][32..64];

            // Copying of the 32 samples from v_vec to u_vec for even and odd slots are unrolled
            // into a 4x8 operation to improve vectorization.
            for j in (0..32).step_by(4) {
                let k = (i << 6) + j;
                u_vec[k+0 +  0] = v0[j+0];
                u_vec[k+0 + 32] = v1[j+0];

                u_vec[k+1 +  0] = v0[j+1];
                u_vec[k+1 + 32] = v1[j+1];

                u_vec[k+2 +  0] = v0[j+2];
                u_vec[k+2 + 32] = v1[j+2];

                u_vec[k+3 +  0] = v0[j+3];
                u_vec[k+3 + 32] = v1[j+3];
            }
        }

        // Finally, generate the 32 sample PCM blocks. Assuming s[i] is sample i of a PCM sample
        // block, the following equation governs sample generation:
        //
        //         16
        // s[i] = SUM { u_vec[32*j + i] * D[32*j + i] }    for i=0..32
        //        j=0
        // 
        // where:
        //     D[0..512] is the synthesis window provided in table B.3 of ISO/IEC 11172-3.
        //
        // In words, u_vec is logically partitioned into 16 slots of 32 samples each (i.e., 
        // slot 0 spans u_vec[0..32], slot 1 spans u_vec[32..64], and so on). Then, the i-th 
        // sample in the PCM block is the summation of the i-th sample in each of the 16 u_vec 
        // slots after being multiplied by the synthesis window.
        for i in 0..32 {
            let mut sum = 0.0;
            // The 16 iterations of the summation is unrolled into a 4x4 operation to improve
            // vectorization.
            for j in (0..16).step_by(4) {
                let k = (j << 5) + i;
                sum += u_vec[k+ 0] * SYNTHESIS_D[k+ 0];
                sum += u_vec[k+32] * SYNTHESIS_D[k+32];
                sum += u_vec[k+64] * SYNTHESIS_D[k+64];
                sum += u_vec[k+96] * SYNTHESIS_D[k+96];
            }
            out_samples[(b << 5) + i] = sum;
        }

        // Shift the v_vec FIFO. The value v_front is the index of the 64 sample slot in v_vec
        // that will be overwritten next iteration. Conversely, that makes it the front of the 
        // FIFO for the purpose of building u_vec. We would like to overwrite the oldest slot,
        // so we subtract 1 via a wrapping addition to move the front backwards by 1 slot,
        // effectively overwriting the oldest slot with the soon-to-be newest.
        state.v_front = (state.v_front + 15) & 0xf;
    }
}

/// Performs a 32-point Discrete Cosine Transform (DCT) using Byeong Gi Lee's fast algorithm
/// published in article [1] without inverse square-root 2 scaling.
///
/// This is a straight-forward implemention of the recursive algorithm, flattened into a single
/// function body to avoid the overhead of function calls and the stack.
/// 
/// [1] B.G. Lee, "A new algorithm to compute the discrete cosine transform", IEEE Transactions
/// on Acoustics, Speech, and Signal Processing, vol. 32, no. 6, pp. 1243-1245, 1984.
///
/// https://ieeexplore.ieee.org/document/1164443
fn dct32(x: &[f32; 32], y: &mut [f32]) {
    debug_assert!(y.len() == 32);

    // The following tables are pre-computed values of the the following equation:
    //
    // c[i] = 1.0 / [2.0 * cos((PI / N) * (2*i + 1))]    for i = 0..N/2
    //
    // where N = [32, 16, 8, 4, 2], for COS_16, COS8, COS_4, and COS_2, respectively.
    const COS_16: [f32; 16] = [
         0.5006029982351963,  // i= 0
         0.5054709598975436,  // i= 1
         0.5154473099226246,  // i= 2
         0.5310425910897841,  // i= 3
         0.5531038960344445,  // i= 4
         0.5829349682061339,  // i= 5
         0.6225041230356648,  // i= 6
         0.6748083414550057,  // i= 7
         0.7445362710022986,  // i= 8
         0.8393496454155268,  // i= 9
         0.9725682378619608,  // i=10
         1.1694399334328847,  // i=11
         1.4841646163141662,  // i=12
         2.0577810099534108,  // i=13
         3.4076084184687190,  // i=14
        10.1900081235480329,  // i=15
    ];

    const COS_8: [f32; 8] = [
        0.5024192861881557,  // i=0
        0.5224986149396889,  // i=1
        0.5669440348163577,  // i=2
        0.6468217833599901,  // i=3
        0.7881546234512502,  // i=4
        1.0606776859903471,  // i=5
        1.7224470982383342,  // i=6
        5.1011486186891553,  // i=7
    ];

    const COS_4: [f32; 4] = [
        0.5097955791041592,  // i=0
        0.6013448869350453,  // i=1
        0.8999762231364156,  // i=2
        2.5629154477415055,  // i=3
    ];

    const COS_2: [f32; 2] = [
        0.5411961001461970,  // i=0
        1.3065629648763764,  // i=1
    ];

    const COS_1: f32 = 0.7071067811865475;

    // 16-point DCT decomposition
    let mut t0 = [
        (x[ 0] + x[32 -  1]),
        (x[ 1] + x[32 -  2]),
        (x[ 2] + x[32 -  3]),
        (x[ 3] + x[32 -  4]),
        (x[ 4] + x[32 -  5]),
        (x[ 5] + x[32 -  6]),
        (x[ 6] + x[32 -  7]),
        (x[ 7] + x[32 -  8]),
        (x[ 8] + x[32 -  9]),
        (x[ 9] + x[32 - 10]),
        (x[10] + x[32 - 11]),
        (x[11] + x[32 - 12]),
        (x[12] + x[32 - 13]),
        (x[13] + x[32 - 14]),
        (x[14] + x[32 - 15]),
        (x[15] + x[32 - 16]),
        (x[ 0] - x[32 -  1]) * COS_16[0],
        (x[ 1] - x[32 -  2]) * COS_16[1],
        (x[ 2] - x[32 -  3]) * COS_16[2],
        (x[ 3] - x[32 -  4]) * COS_16[3],
        (x[ 4] - x[32 -  5]) * COS_16[4],
        (x[ 5] - x[32 -  6]) * COS_16[5],
        (x[ 6] - x[32 -  7]) * COS_16[6],
        (x[ 7] - x[32 -  8]) * COS_16[7],
        (x[ 8] - x[32 -  9]) * COS_16[8],
        (x[ 9] - x[32 - 10]) * COS_16[9],
        (x[10] - x[32 - 11]) * COS_16[10],
        (x[11] - x[32 - 12]) * COS_16[11],
        (x[12] - x[32 - 13]) * COS_16[12],
        (x[13] - x[32 - 14]) * COS_16[13],
        (x[14] - x[32 - 15]) * COS_16[14],
        (x[15] - x[32 - 16]) * COS_16[15],
    ];

    // 16-point DCT decomposition of t0[0..16]
    {
        let mut t1 = [
            (t0[0] + t0[16 - 1]),
            (t0[1] + t0[16 - 2]),
            (t0[2] + t0[16 - 3]),
            (t0[3] + t0[16 - 4]),
            (t0[4] + t0[16 - 5]),
            (t0[5] + t0[16 - 6]),
            (t0[6] + t0[16 - 7]),
            (t0[7] + t0[16 - 8]),
            (t0[0] - t0[16 - 1]) * COS_8[0],
            (t0[1] - t0[16 - 2]) * COS_8[1],
            (t0[2] - t0[16 - 3]) * COS_8[2],
            (t0[3] - t0[16 - 4]) * COS_8[3],
            (t0[4] - t0[16 - 5]) * COS_8[4],
            (t0[5] - t0[16 - 6]) * COS_8[5],
            (t0[6] - t0[16 - 7]) * COS_8[6],
            (t0[7] - t0[16 - 8]) * COS_8[7],
        ];

        // 8-point DCT decomposition of t1[0..8]
        {
            let mut t2 = [
                (t1[0] + t1[8 - 1]),
                (t1[1] + t1[8 - 2]),
                (t1[2] + t1[8 - 3]),
                (t1[3] + t1[8 - 4]),
                (t1[0] - t1[8 - 1]) * COS_4[0],
                (t1[1] - t1[8 - 2]) * COS_4[1],
                (t1[2] - t1[8 - 3]) * COS_4[2],
                (t1[3] - t1[8 - 4]) * COS_4[3],
            ];

            // 4-point DCT decomposition of t2[0..4]
            {
                let mut t3 = [
                    (t2[0] + t2[4 - 1]),
                    (t2[1] + t2[4 - 2]),
                    (t2[0] - t2[4 - 1]) * COS_2[0],
                    (t2[1] - t2[4 - 2]) * COS_2[1],
                ];

                // 2-point DCT decomposition of t3[0..2]
                {
                    let t4 = [
                        (t3[0] + t3[2-1]),
                        (t3[0] - t3[2-1]) * COS_1,
                    ];

                    t3[0] = t4[0];
                    t3[1] = t4[1];
                }

                // 2-point DCT decomposition of t3[2..4]
                {
                    let t4 = [
                        (t3[2] + t3[4-1]),
                        (t3[2] - t3[4-1]) * COS_1,
                    ];

                    t3[2 + 0] = t4[0];
                    t3[2 + 1] = t4[1];
                }

                t2[0 + 0] = t3[0];
                t2[0 + 1] = t3[2] + t3[3];
                t2[0 + 2] = t3[1];
                t2[0 + 3] = t3[3];
            }

            // 4-point DCT decomposition of t2[4..8]
            {
                let mut t3 = [
                    (t2[4] + t2[8 - 1]),
                    (t2[5] + t2[8 - 2]),
                    (t2[4] - t2[8 - 1]) * COS_2[0],
                    (t2[5] - t2[8 - 2]) * COS_2[1],
                ];

                // 2-point DCT decomposition of t3[0..2]
                {
                    let t4 = [
                        (t3[0] + t3[2-1]),
                        (t3[0] - t3[2-1]) * COS_1,
                    ];

                    t3[0] = t4[0];
                    t3[1] = t4[1];
                }

                // 2-point DCT decomposition of t3[2..4]
                {
                    let t4 = [
                        (t3[2] + t3[4-1]),
                        (t3[2] - t3[4-1]) * COS_1,
                    ];

                    t3[2 + 0] = t4[0];
                    t3[2 + 1] = t4[1];
                }

                t2[4 + 0] = t3[0];
                t2[4 + 1] = t3[2] + t3[3];
                t2[4 + 2] = t3[1];
                t2[4 + 3] = t3[3];
            }

            // Recombine t2[0..4] and t2[4..8], overwriting t1[0..8].
            for i in 0..3 {
                t1[(i<<1) + 0] = t2[i];
                t1[(i<<1) + 1] = t2[4+i] + t2[4+i+1];
            }

            t1[8 - 2] = t2[4 - 1];
            t1[8 - 1] = t2[8 - 1];
        }

        // 8-point DCT decomposition of t1[8..16]
        {
            let mut t2 = [
                (t1[ 8] + t1[16 - 1]),
                (t1[ 9] + t1[16 - 2]),
                (t1[10] + t1[16 - 3]),
                (t1[11] + t1[16 - 4]),
                (t1[ 8] - t1[16 - 1]) * COS_4[0],
                (t1[ 9] - t1[16 - 2]) * COS_4[1],
                (t1[10] - t1[16 - 3]) * COS_4[2],
                (t1[11] - t1[16 - 4]) * COS_4[3],
            ];

            // 4-point DCT decomposition of t2[0..4]
            {
                let mut t3 = [
                    (t2[0] + t2[4 - 1]),
                    (t2[1] + t2[4 - 2]),
                    (t2[0] - t2[4 - 1]) * COS_2[0],
                    (t2[1] - t2[4 - 2]) * COS_2[1],
                ];

                // 2-point DCT decomposition of t3[0..2]
                {
                    let t4 = [
                        (t3[0] + t3[2-1]),
                        (t3[0] - t3[2-1]) * COS_1,
                    ];

                    t3[0] = t4[0];
                    t3[1] = t4[1];
                }

                // 2-point DCT decomposition of t3[2..4]
                {
                    let t4 = [
                        (t3[2] + t3[4-1]),
                        (t3[2] - t3[4-1]) * COS_1,
                    ];

                    t3[2 + 0] = t4[0];
                    t3[2 + 1] = t4[1];
                }

                t2[0 + 0] = t3[0];
                t2[0 + 1] = t3[2] + t3[3];
                t2[0 + 2] = t3[1];
                t2[0 + 3] = t3[3];
            }

            // 4-point DCT decomposition of t2[4..8]
            {
                let mut t3 = [
                    (t2[4] + t2[8 - 1]),
                    (t2[5] + t2[8 - 2]),
                    (t2[4] - t2[8 - 1]) * COS_2[0],
                    (t2[5] - t2[8 - 2]) * COS_2[1],
                ];

                // 2-point DCT decomposition of t3[0..2]
                {
                    let t4 = [
                        (t3[0] + t3[2-1]),
                        (t3[0] - t3[2-1]) * COS_1,
                    ];

                    t3[0] = t4[0];
                    t3[1] = t4[1];
                }

                // 2-point DCT decomposition of t3[2..4]
                {
                    let t4 = [
                        (t3[2] + t3[4-1]),
                        (t3[2] - t3[4-1]) * COS_1,
                    ];

                    t3[2 + 0] = t4[0];
                    t3[2 + 1] = t4[1];
                }

                t2[4 + 0] = t3[0];
                t2[4 + 1] = t3[2] + t3[3];
                t2[4 + 2] = t3[1];
                t2[4 + 3] = t3[3];
            }

            // Recombine t2[0..4] and t2[4..8], overwriting t1[8..16].
            for i in 0..3 {
                t1[8 + (i<<1) + 0] = t2[i];
                t1[8 + (i<<1) + 1] = t2[4+i] + t2[4+i+1];
            }

            t1[16 - 2] = t2[4 - 1];
            t1[16 - 1] = t2[8 - 1];
        }

        // Recombine t1[0..8] and t1[8..16], overwriting t0[0..16].
        for i in 0..7 {
            t0[(i<<1) + 0] = t1[i];
            t0[(i<<1) + 1] = t1[8+i] + t1[8+i+1];
        }

        t0[16 - 2] = t1[8 - 1];
        t0[16 - 1] = t1[16 - 1];
    }

    // 16-point DCT decomposition of t0[16..32]
    {
        let mut t1 = [
            (t0[16] + t0[32 - 1]),
            (t0[17] + t0[32 - 2]),
            (t0[18] + t0[32 - 3]),
            (t0[19] + t0[32 - 4]),
            (t0[20] + t0[32 - 5]),
            (t0[21] + t0[32 - 6]),
            (t0[22] + t0[32 - 7]),
            (t0[23] + t0[32 - 8]),
            (t0[16] - t0[32 - 1]) * COS_8[0],
            (t0[17] - t0[32 - 2]) * COS_8[1],
            (t0[18] - t0[32 - 3]) * COS_8[2],
            (t0[19] - t0[32 - 4]) * COS_8[3],
            (t0[20] - t0[32 - 5]) * COS_8[4],
            (t0[21] - t0[32 - 6]) * COS_8[5],
            (t0[22] - t0[32 - 7]) * COS_8[6],
            (t0[23] - t0[32 - 8]) * COS_8[7],
        ];

        // 8-point DCT decomposition of t1[0..8]
        {
            let mut t2 = [
                (t1[0] + t1[8 - 1]),
                (t1[1] + t1[8 - 2]),
                (t1[2] + t1[8 - 3]),
                (t1[3] + t1[8 - 4]),
                (t1[0] - t1[8 - 1]) * COS_4[0],
                (t1[1] - t1[8 - 2]) * COS_4[1],
                (t1[2] - t1[8 - 3]) * COS_4[2],
                (t1[3] - t1[8 - 4]) * COS_4[3],
            ];

            // 4-point DCT decomposition of t2[0..4]
            {
                let mut t3 = [
                    (t2[0] + t2[4 - 1]),
                    (t2[1] + t2[4 - 2]),
                    (t2[0] - t2[4 - 1]) * COS_2[0],
                    (t2[1] - t2[4 - 2]) * COS_2[1],
                ];

                // 2-point DCT decomposition of t3[0..2]
                {
                    let t4 = [
                        (t3[0] + t3[2-1]),
                        (t3[0] - t3[2-1]) * COS_1,
                    ];

                    t3[0] = t4[0];
                    t3[1] = t4[1];
                }

                // 2-point DCT decomposition of t3[2..4]
                {
                    let t4 = [
                        (t3[2] + t3[4-1]),
                        (t3[2] - t3[4-1]) * COS_1,
                    ];

                    t3[2 + 0] = t4[0];
                    t3[2 + 1] = t4[1];
                }

                t2[0 + 0] = t3[0];
                t2[0 + 1] = t3[2] + t3[3];
                t2[0 + 2] = t3[1];
                t2[0 + 3] = t3[3];
            }

            // 4-point DCT decomposition of t2[4..8]
            {
                let mut t3 = [
                    (t2[4] + t2[8 - 1]),
                    (t2[5] + t2[8 - 2]),
                    (t2[4] - t2[8 - 1]) * COS_2[0],
                    (t2[5] - t2[8 - 2]) * COS_2[1],
                ];

                // 2-point DCT decomposition of t3[0..2]
                {
                    let t4 = [
                        (t3[0] + t3[2-1]),
                        (t3[0] - t3[2-1]) * COS_1,
                    ];

                    t3[0] = t4[0];
                    t3[1] = t4[1];
                }

                // 2-point DCT decomposition of t3[2..4]
                {
                    let t4 = [
                        (t3[2] + t3[4-1]),
                        (t3[2] - t3[4-1]) * COS_1,
                    ];

                    t3[2 + 0] = t4[0];
                    t3[2 + 1] = t4[1];
                }

                t2[4 + 0] = t3[0];
                t2[4 + 1] = t3[2] + t3[3];
                t2[4 + 2] = t3[1];
                t2[4 + 3] = t3[3];
            }

            // Recombine t2[0..4] and t2[4..8], overwriting t1[0..8].
            for i in 0..3 {
                t1[(i<<1) + 0] = t2[i];
                t1[(i<<1) + 1] = t2[4+i] + t2[4+i+1];
            }

            t1[8 - 2] = t2[4 - 1];
            t1[8 - 1] = t2[8 - 1];
        }

        // 8-point DCT decomposition of t1[8..16]
        {
            let mut t2 = [
                (t1[ 8] + t1[16 - 1]),
                (t1[ 9] + t1[16 - 2]),
                (t1[10] + t1[16 - 3]),
                (t1[11] + t1[16 - 4]),
                (t1[ 8] - t1[16 - 1]) * COS_4[0],
                (t1[ 9] - t1[16 - 2]) * COS_4[1],
                (t1[10] - t1[16 - 3]) * COS_4[2],
                (t1[11] - t1[16 - 4]) * COS_4[3],
            ];

            // 4-point DCT decomposition of t2[0..4]
            {
                let mut t3 = [
                    (t2[0] + t2[4 - 1]),
                    (t2[1] + t2[4 - 2]),
                    (t2[0] - t2[4 - 1]) * COS_2[0],
                    (t2[1] - t2[4 - 2]) * COS_2[1],
                ];

                // 2-point DCT decomposition of t3[0..2]
                {
                    let t4 = [
                        (t3[0] + t3[2-1]),
                        (t3[0] - t3[2-1]) * COS_1,
                    ];

                    t3[0] = t4[0];
                    t3[1] = t4[1];
                }

                // 2-point DCT decomposition of t3[2..4]
                {
                    let t4 = [
                        (t3[2] + t3[4-1]),
                        (t3[2] - t3[4-1]) * COS_1,
                    ];

                    t3[2 + 0] = t4[0];
                    t3[2 + 1] = t4[1];
                }

                t2[0 + 0] = t3[0];
                t2[0 + 1] = t3[2] + t3[3];
                t2[0 + 2] = t3[1];
                t2[0 + 3] = t3[3];
            }

            // 4-point DCT decomposition of t2[4..8]
            {
                let mut t3 = [
                    (t2[4] + t2[8 - 1]),
                    (t2[5] + t2[8 - 2]),
                    (t2[4] - t2[8 - 1]) * COS_2[0],
                    (t2[5] - t2[8 - 2]) * COS_2[1],
                ];

                // 2-point DCT decomposition of t3[0..2]
                {
                    let t4 = [
                        (t3[0] + t3[2-1]),
                        (t3[0] - t3[2-1]) * COS_1,
                    ];

                    t3[0] = t4[0];
                    t3[1] = t4[1];
                }

                // 2-point DCT decomposition of t3[2..4]
                {
                    let t4 = [
                        (t3[2] + t3[4-1]),
                        (t3[2] - t3[4-1]) * COS_1,
                    ];

                    t3[2 + 0] = t4[0];
                    t3[2 + 1] = t4[1];
                }

                t2[4 + 0] = t3[0];
                t2[4 + 1] = t3[2] + t3[3];
                t2[4 + 2] = t3[1];
                t2[4 + 3] = t3[3];
            }

            // Recombine t2[0..4] and t2[4..8], overwriting t1[8..16].
            for i in 0..3 {
                t1[8 + (i<<1) + 0] = t2[i];
                t1[8 + (i<<1) + 1] = t2[4+i] + t2[4+i+1];
            }

            t1[16 - 2] = t2[4 - 1];
            t1[16 - 1] = t2[8 - 1];
        }

        // Recombine t1[0..8] and t1[8..16], overwriting t0[0..16].
        for i in 0..7 {
            t0[16 + (i<<1) + 0] = t1[i];
            t0[16 + (i<<1) + 1] = t1[8+i] + t1[8+i+1];
        }

        t0[32 - 2] = t1[8 - 1];
        t0[32 - 1] = t1[16 - 1];
    }

    // Recombine t1[0..16] and t1[16..32] into y.
    for i in 0..15 {
        y[(i<<1) + 0] = t0[i];
        y[(i<<1) + 1] = t0[16 + i] + t0[16 + i + 1];
    }

    y[32 - 2] = t0[16 - 1];
    y[32 - 1] = t0[32 - 1];
}


#[cfg(test)]
mod tests {
    use super::dct32;
    use std::f64;

    fn dct32_analytical(x: &[f32; 32]) -> [f32; 32] {
        const PI_32: f64 = f64::consts::PI / 32.0;

        let mut result = [0f32; 32];
        for i in 0..32 {
            let mut sum = 0.0;
            for j in 0..32 {
                sum += (x[j] as f64)* (PI_32 * (i as f64) * ((j as f64) + 0.5)).cos();
            }
            result[i] = sum as f32;
        }
        result
    }

    #[test]
    fn verify_dct32() {
        const TEST_VECTOR: [f32; 32] = [ 
            0.1710, 0.1705, 0.3476, 0.1866, 0.4784, 0.6525, 0.2690, 0.9996,
            0.1864, 0.7277, 0.1163, 0.6620, 0.0911, 0.3225, 0.1126, 0.5344,
            0.7839, 0.9741, 0.8757, 0.5763, 0.5926, 0.2756, 0.1757, 0.6531,
            0.7101, 0.7376, 0.1924, 0.0351, 0.8044, 0.2409, 0.9347, 0.9417,
        ];

        let mut test_result = [0f32; 32];
        dct32(&TEST_VECTOR, &mut test_result);

        let actual_result = dct32_analytical(&TEST_VECTOR);
        for i in 0..32 {
            assert!((actual_result[i] - test_result[i]).abs() < 0.00001);
        }
    }
}