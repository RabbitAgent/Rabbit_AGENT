impl InferenceEngine {
    pub async fn run_with_fallback(&self, input: Tensor) -> Result<Tensor> {
        let mut attempt = 0;
        loop {
            match self.execute_model(input.clone()).await {
                Ok(result) => return Ok(result),
                Err(e) if attempt < MAX_RETRIES => {
                    self.reload_model().await?;
                    attempt += 1;
                }
                Err(e) => return Err(e),
            }
        }
    }
}
