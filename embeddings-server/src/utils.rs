pub fn device() -> Result<candle_core::Device, candle_core::Error> {
    if candle_core::utils::cuda_is_available() {
        tracing::info!("Running on GPU (cuda)");
        candle_core::Device::new_cuda(0)
    } else if candle_core::utils::metal_is_available() {
        tracing::info!("Running on GPU (metal)");
        candle_core::Device::new_metal(0)
    } else {
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        {
            tracing::warn!(
                "Running on CPU, to run on GPU(metal), build this example with `--features metal`"
            );
        }
        #[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
        {
            tracing::warn!(
                "Running on CPU, to run on GPU, build this example with `--features cuda`"
            );
        }
        Ok(candle_core::Device::Cpu)
    }
}
