use threshold_bls::{
    poly::Poly,
    sig::{BlsSignature, BlsVerifier},
};

pub struct DistributedKeyManager {
    polynomial: Poly<BlsVerifier>,
    threshold: usize,
}

impl DistributedKeyManager {
    pub fn setup(threshold: usize, participants: usize) -> Self {
        let poly = Poly::<BlsVerifier>::random(threshold - 1);
        Self {
            polynomial,
            threshold,
        }
    }

    pub fn generate_shares(&self) -> Vec<Share> {
        (1..=self.participants)
            .map(|i| {
                let x = Scalar::from(i as u32);
                Share {
                    index: i,
                    value: self.polynomial.evaluate(&x),
                }
            })
            .collect()
    }

    pub fn combine_signature(shares: &[SignatureShare]) -> Result<BlsSignature> {
        let points: Vec<_> = shares.iter()
            .map(|s| (Scalar::from(s.index as u32), s.signature))
            .collect();
        
        BlsSignature::interpolate(&points)
    }
}

#[derive(Encode, Decode)]
pub struct SignatureShare {
    pub index: u32,
    pub signature: BlsSignature,
}
