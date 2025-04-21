impl SecretSharer {
    pub fn split_secret(&self, secret: &[u8], n: u8, k: u8) -> Vec<Share> {
        let polynomial = self.create_polynomial(secret, k-1);
        
        (1..=n)
            .map(|x| {
                let x_scalar = Scalar::from(x);
                let y = polynomial.evaluate(&x_scalar);
                Share { x, y }
            })
            .collect()
    }
    
    pub fn recover_secret(shares: &[Share]) -> Result<Vec<u8>> {
        let points: Vec<(Scalar, Scalar)> = shares.iter()
            .map(|s| (Scalar::from(s.x), s.y))
            .collect();
            
        Polynomial::interpolate(&points)
            .and_then(|poly| poly.constant_term())
    }
}
