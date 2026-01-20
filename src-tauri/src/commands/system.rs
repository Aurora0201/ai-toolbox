use crate::error::{AppResult, AppError};
use log::debug;
use tauri::Manager;

/// Command to set the global log level at runtime.
#[tauri::command]
pub async fn set_log_level(level: String) -> AppResult<()> {
    let level_filter = match level.to_lowercase().as_str() {
        "off" => log::LevelFilter::Off,
        "error" => log::LevelFilter::Error,
        "warn" => log::LevelFilter::Warn,
        "info" => log::LevelFilter::Info,
        "debug" => log::LevelFilter::Debug,
        "trace" => log::LevelFilter::Trace,
        _ => log::LevelFilter::Info,
    };
    
    debug!("Setting log level to: {:?}", level_filter);
    log::set_max_level(level_filter);
    
    Ok(())
}

/// Command to open the application log directory in the file explorer.
#[tauri::command]
pub async fn open_log_dir(app: tauri::AppHandle) -> AppResult<()> {
    let log_path = app.path().app_log_dir()
        .map_err(|e| AppError::Io(format!("Failed to get log dir: {}", e)))?;

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(log_path)
            .spawn()
            .map_err(|e| AppError::Io(format!("Failed to open log dir: {}", e)))?;
    }
    
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(log_path)
            .spawn()
            .map_err(|e| AppError::Io(format!("Failed to open log dir: {}", e)))?;
    }
    
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(log_path)
            .spawn()
            .map_err(|e| AppError::Io(format!("Failed to open log dir: {}", e)))?;
    }

    Ok(())
}

#[derive(serde::Serialize, Clone, Default)]
pub struct GpuInfo {
    pub name: String,
    pub total_mb: u64,
    pub used_mb: u64,
}

/// Command to get GPU info (Name, Total VRAM, Used VRAM).
/// Uses PowerShell on Windows for broad support (AMD/Intel/NVIDIA).
#[tauri::command]
pub async fn get_gpu_info() -> AppResult<GpuInfo> {
    #[allow(unused_mut)]
    let mut info = GpuInfo::default();

    #[cfg(target_os = "windows")]
    {
        let ps_cmd = r#"
            $total = Get-ItemProperty -Path 'HKLM:\SYSTEM\ControlSet001\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}\*' -ErrorAction SilentlyContinue | Where-Object { $_.'HardwareInformation.QwMemorySize' -gt 0 } | Sort-Object 'HardwareInformation.QwMemorySize' -Descending | Select-Object -First 1 DriverDesc, 'HardwareInformation.QwMemorySize'
            $used = Get-Counter '\GPU Adapter Memory(*)\Dedicated Usage' -ErrorAction SilentlyContinue | Select-Object -ExpandProperty CounterSamples | Sort-Object CookedValue -Descending | Select-Object -First 1 CookedValue
            
            if ($total) {
                @{
                    Name = $total.DriverDesc
                    Total = $total.'HardwareInformation.QwMemorySize'
                    Used = if ($used) { $used.CookedValue } else { 0 }
                } | ConvertTo-Json
            }
        "#;
        
        let output = tokio::process::Command::new("powershell")
            .args(["-NoProfile", "-Command", ps_cmd])
            .output()
            .await
            .map_err(|e| AppError::Io(format!("Failed to execute PowerShell: {}", e)))?;

        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout);
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&s) {
                if let Some(name) = v["Name"].as_str() {
                    info.name = name.to_string();
                }
                if let Some(total) = v["Total"].as_u64() {
                    info.total_mb = total / 1024 / 1024;
                }
                if let Some(used) = v["Used"].as_f64() {
                    info.used_mb = (used as u64) / 1024 / 1024;
                } else if let Some(used) = v["Used"].as_u64() {
                    info.used_mb = used / 1024 / 1024;
                }
            }
        }
    }

    Ok(info)
}
