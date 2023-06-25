use serde::Deserialize;
use std::fs::File;

use crate::errors::ConfigError;

#[derive(Default, Deserialize, Debug, Clone)]
struct SecretsFromReader {
    secrets: Option<Secrets>,
}

#[derive(Default, Deserialize, Debug, Clone)]
struct Secrets {
    redis_addr: Option<String>,
    redis_password: Option<String>,
    proxy_addr: Option<String>,
    metrics_addr: Option<String>,
}

#[derive(Default, Deserialize, Debug, Clone)]
struct ProxySettingsFromReader {
    proxy_settings: Option<ProxySettings>,
}

#[derive(Default, Deserialize, Debug, Clone)]
struct ProxySettings {
    service_ports: Option<Vec<u32>>,
    team_ips: Option<Vec<String>>,
    targets: Option<Vec<TargetFromReader>>,
}

#[derive(Default, Deserialize, Debug, Clone)]
struct TargetFromReader {
    port: Option<u32>,
    team_ip: Option<String>,
}

/// Config for env variables. It is used to initialize.
#[derive(Default, Debug, Clone)]
pub struct SecretsConfig {
    pub redis_addr: String,
    pub redis_password: String,
    pub proxy_addr: String,
    pub metrics_addr: String,
}

/// Config for proxy settings.
#[derive(Default, Debug, Clone)]
pub struct ProxySettingsConfig {
    pub service_ports: Vec<u32>,
    pub team_ips: Vec<String>,
    pub targets: Vec<Target>,
}

#[derive(Default, Debug, Clone)]
pub struct Target {
    pub port: u32,
    pub team_ip: String,
}

fn open_file(path: &str) -> Result<File, ConfigError> {
    match std::fs::File::open(path) {
        Ok(res) => return Ok(res),
        Err(e) => {
            return Err(ConfigError::Etc {
                description: format!("couldn't open file `{}`", path),
                error: e.into(),
            })
        }
    }
}

pub fn build_secrets_config() -> Result<SecretsConfig, ConfigError> {
    let config_file = open_file("config.yaml")?;
    let config_from_reader: SecretsFromReader = match serde_yaml::from_reader(config_file) {
        Ok(res) => res,
        Err(e) => {
            return Err(ConfigError::Etc {
                description: "couldn't read config values".to_string(),
                error: e.into(),
            })
        }
    };

    let secrets = match config_from_reader.secrets {
        Some(res) => res,
        None => {
            return Err(ConfigError::NoKey {
                key: "secrets".to_string(),
            });
        }
    };

    Ok(SecretsConfig {
        redis_addr: match secrets.redis_addr {
            Some(res) => res,
            None => {
                return Err(ConfigError::NoGroupKey {
                    group: "secrets".to_string(),
                    key: "redis_addr".to_string(),
                    value_example: "127.0.0.1:2137".to_string(),
                });
            }
        },
        redis_password: match secrets.redis_password {
            Some(res) => res,
            None => {
                return Err(ConfigError::NoGroupKey {
                    group: "secrets".to_string(),
                    key: "redis_password".to_string(),
                    value_example: "password".to_string(),
                });
            }
        },
        proxy_addr: match secrets.proxy_addr {
            Some(res) => res,
            None => {
                return Err(ConfigError::NoGroupKey {
                    group: "secrets".to_string(),
                    key: "proxy_addr".to_string(),
                    value_example: "0.0.0.0:1337".to_string(),
                });
            }
        },
        metrics_addr: match secrets.metrics_addr {
            Some(res) => res,
            None => {
                return Err(ConfigError::NoGroupKey {
                    group: "secrets".to_string(),
                    key: "metrics_addr".to_string(),
                    value_example: "0.0.0.0:8989".to_string(),
                });
            }
        },
    })
}

pub fn build_proxy_settings_config() -> Result<ProxySettingsConfig, ConfigError> {
    let config_file = open_file("config.yaml")?;
    let config_from_reader: ProxySettingsFromReader = match serde_yaml::from_reader(config_file) {
        Ok(res) => res,
        Err(e) => {
            return Err(ConfigError::Etc {
                description: "couldn't read config values".to_string(),
                error: e.into(),
            })
        }
    };

    let proxy_settings = match config_from_reader.proxy_settings {
        Some(res) => res,
        None => {
            return Err(ConfigError::NoKey {
                key: "proxy_settings".to_string(),
            })
        }
    };

    Ok(ProxySettingsConfig {
        service_ports: match proxy_settings.service_ports {
            Some(res) => res,
            None => {
                return Err(ConfigError::NoGroupKey {
                    group: "proxy_settings".to_string(),
                    key: "service_ports".to_string(),
                    value_example: "[ 3444, 3445, 3446 ]".to_string(),
                })
            }
        },
        team_ips: match proxy_settings.team_ips {
            Some(res) => res,
            None => {
                return Err(ConfigError::NoGroupKey {
                    group: "proxy_settings".to_string(),
                    key: "team_ips".to_string(),
                    value_example: "[ 10.0.12.23, 10.0.12.24, 10.0.12.25 ]".to_string(),
                })
            }
        },
        targets: {
            let targets = match proxy_settings.targets {
                Some(res) => res,
                None => {
                    return Err(ConfigError::NoKey {
                        key: "targets".to_string(),
                    })
                }
            };

            let mut result: Vec<Target> = Vec::new();

            for target in targets.iter() {
                result.push(Target {
                    port: match target.clone().port {
                        Some(res) => res,
                        None => {
                            return Err(ConfigError::NoListElement {
                                list_name: "targets".to_string(),
                                element_example: "{ team_ip: 127.0.0.1, port: 4554 }".to_string(),
                            })
                        }
                    },
                    team_ip: match target.clone().team_ip {
                        Some(res) => res,
                        None => {
                            return Err(ConfigError::NoListElement {
                                list_name: "targets".to_string(),
                                element_example: "{ team_ip: 127.0.0.1, port: 4554 }".to_string(),
                            })
                        }
                    },
                })
            }

            result
        },
    })
}
