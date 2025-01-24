use icicle_babybear::field::ScalarField as Frbb;
use icicle_core::{
    hash::{HashConfig, Hasher},
    poseidon2::Poseidon2,
    traits::FieldImpl,
};
use icicle_runtime::memory::HostSlice;

pub fn hash_test<F: FieldImpl>(test_vec: Vec<F>, config: HashConfig, hash: Hasher) {
    let input_slice = HostSlice::from_slice(&test_vec);
    let out_init: F = F::zero();
    let mut binding = [out_init];
    let out_init_slice = HostSlice::from_mut_slice(&mut binding);
    hash.hash(input_slice, &config, out_init_slice).unwrap();
    println!(
        "computed digest: {:?} ",
        out_init_slice.as_slice().to_vec()[0]
    );
}

fn main() {
    let t_vec = [2, 3, 4, 8, 12, 16, 20, 24];
    let expected_digest_bb: Vec<Frbb> = vec![
        Frbb::from_u32(833751247),
        Frbb::from_u32(1850148672),
        Frbb::from_u32(472702774),
        Frbb::from_u32(1077997898),
        Frbb::from_u32(1605336739),
        Frbb::from_u32(771677727),
        Frbb::from_u32(1224149815),
        Frbb::from_u32(311566256),
    ];
    println!("Baby Bear");
    let config = HashConfig::default();
    for (t, digest) in t_vec.iter().zip(expected_digest_bb.iter()) {
        let instance = Poseidon2::new::<Frbb>(*t, None).unwrap();
        let input_state_bb: Vec<Frbb> = (0..*t).map(Frbb::from_u32).collect();
        println!("test vector {:?}", input_state_bb);
        println!("expected digest {:?}", digest);
        hash_test::<Frbb>(input_state_bb, config.clone(), instance);
        println!(" ");
    }
}
