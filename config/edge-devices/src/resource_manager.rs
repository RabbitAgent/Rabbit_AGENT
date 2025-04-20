use sysinfo::{System, SystemExt};
use tokio::sync::watch;

#[derive(Debug, Clone)]
pub struct DeviceResources {
    pub available_mem_mb: u32,
    pub cpu_usage: f32,
    pub gpu_vram_mb: Option<u32>,
}

pub struct ResourceMonitor {
    sys: System,
    tx: watch::Sender<DeviceResources>,
}

impl ResourceMonitor {
    pub fn new(update_interval: u64) -> (Self, watch::Receiver<DeviceResources>) {
        let (tx, rx) = watch::channel(DeviceResources::default());
        let mut monitor = Self { sys: System::new(), tx };
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(update_interval));
            loop {
                interval.tick().await;
                monitor.refresh();
            }
        });

        (monitor, rx)
    }

    fn refresh(&mut self) {
        self.sys.refresh_memory();
        self.sys.refresh_cpu();
        
        let resources = DeviceResources {
            available_mem_mb: (self.sys.available_memory() / 1024 / 1024) as u32,
            cpu_usage: self.sys.global_cpu_info().cpu_usage(),
            gpu_vram_mb: self.get_gpu_status(),
        };

        let _ = self.tx.send(resources);
    }
}
