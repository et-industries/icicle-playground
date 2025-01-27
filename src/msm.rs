use clap::Parser;
use icicle_bls12_381::curve::{CurveCfg, G1Projective, ScalarCfg};
use icicle_core::{curve::Curve, msm, traits::GenerateRandom};
use icicle_runtime::{
    memory::{DeviceVec, HostSlice},
    stream::IcicleStream,
};

#[derive(Parser, Debug)]
struct Args {
    /// Device type (e.g., "CPU", "CUDA")
    #[arg(short, long, default_value = "CPU")]
    device_type: String,
}

// Load backend and set device
fn try_load_and_set_backend_device() {
    let device = icicle_runtime::Device::new("CPU", 0 /* =device_id*/);
    icicle_runtime::set_device(&device).unwrap();
}

pub fn run() {
    try_load_and_set_backend_device();

    let size = 1 << 10;

    println!("Generating random inputs on host for Bls12-381...");
    let upper_points = CurveCfg::generate_random_affine_points(size);
    let upper_scalars = ScalarCfg::generate_random(size);

    // Setting Bls12-381 points and scalars
    let points = HostSlice::from_slice(&upper_points[..size]);
    let scalars = HostSlice::from_slice(&upper_scalars[..size]);

    println!("Configuring Bls12-381 MSM...");
    let mut msm_results = DeviceVec::<G1Projective>::device_malloc(1).unwrap();
    let mut stream = IcicleStream::create().unwrap();
    let mut cfg = msm::MSMConfig::default();
    cfg.stream_handle = *stream;
    cfg.is_async = true;

    println!("Executing Bls12-381 MSM on device...");
    msm::msm(scalars, points, &cfg, &mut msm_results[..]).unwrap();

    println!("Moving results to host...");
    let mut msm_host_result = vec![G1Projective::zero(); 1];

    stream.synchronize().unwrap();
    msm_results
        .copy_to_host(HostSlice::from_mut_slice(&mut msm_host_result[..]))
        .unwrap();
    println!("Bls12-381 result: {:#?}", msm_host_result);

    println!("Cleaning up Bls12-381...");
    stream.destroy().unwrap();
}
