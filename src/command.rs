use anyhow::{bail, Result};
use docker_blocker::{Config, Rule};
use iptables::IPTables;
use std::net::Ipv4Addr;

pub fn reload(config: &Config) {
    let ipt = iptables::new(false).unwrap();

    ipt.flush_chain("filter", "DOCKER-USER").unwrap();
    for r in &config.rules {
        allow_devices(&ipt, r);
    }
    ipt.append("filter", "DOCKER-USER", "-j DROP").unwrap();
}

pub fn disable() {
    let ipt = iptables::new(false).unwrap();

    reset_rules(&ipt);
}

fn allow_devices(ipt: &IPTables, rule: &Rule) {
    for device in &rule.allow_from {
        let src_rule = format!(
            "-s {} -p tcp --dport {} -m comment --comment 'docker-blocker: {}' -j RETURN",
            device, rule.port, rule.service
        );
        let dest_rule = format!(
            "-d {} -p tcp --sport {} -m comment --comment 'docker-blocker: {}' -j RETURN",
            device, rule.port, rule.service
        );
        ipt.append_unique("filter", "DOCKER-USER", &src_rule)
            .unwrap();
        ipt.append_unique("filter", "DOCKER-USER", &dest_rule)
            .unwrap();
    }
}

fn block_port(ipt: &IPTables, rule: &Rule) {
    let rule = format!(
        "-p tcp --dport {} -m comment --comment 'docker-blocker: {}' -j REJECT",
        rule.port, rule.service
    );
    ipt.append_unique("filter", "DOCKER-USER", &rule).unwrap()
}

fn reset_rules(ipt: &IPTables) {
    let commands = ipt
        .list("filter", "DOCKER-USER")
        .expect("Failed to list table 'DOCKER-USER'.");
    for c in commands {
        if !c.contains("docker-blocker") {
            continue;
        }
        let rule: Vec<&str> = c.splitn(3, ' ').collect();
        ipt.delete("filter", "DOCKER-USER", rule[2]).unwrap();
    }
}

fn parse_device(config: &Config, device: String) -> Result<String> {
    if device.parse::<Ipv4Addr>().is_ok() {
        return Ok(device);
    };
    match &config.known_devices {
        Some(d) => return Ok(d.get(&device).unwrap().to_string()),
        None => (),
    }
    bail!("Nope")
}
