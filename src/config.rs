use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub target: TargetConfig,
    pub offsets: OffsetsConfig,
}

#[derive(Debug, Deserialize)]
pub struct TargetConfig {
    pub process_name: String,
    pub module_name: String,
}

#[derive(Debug, Deserialize)]
pub struct OffsetsConfig {
    pub secret_value: String,
    pub counter: String,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let text = fs::read_to_string(path)
            .with_context(|| format!("无法读取配置文件: {}", path.display()))?;
        let cfg: Config = toml::from_str(&text)
            .with_context(|| format!("配置文件格式错误: {}", path.display()))?;
        Ok(cfg)
    }
}

impl OffsetsConfig {
    pub fn secret_value(&self) -> Result<u64> { parse_hex(&self.secret_value) }
    pub fn counter(&self)      -> Result<u64> { parse_hex(&self.counter) }
}

/// 支持 "0x..." 和纯十六进制两种写法
pub fn parse_hex(s: &str) -> Result<u64> {
    let s = s.trim();
    let s = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")).unwrap_or(s);
    u64::from_str_radix(s, 16)
        .with_context(|| format!("无法解析十六进制: {}", s))
}
