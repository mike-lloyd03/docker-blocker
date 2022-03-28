use docker_blocker::{Config, Rule};
use iptables::IPTables;

pub fn enable(config: &Config) {
    let ipt = iptables::new(false).unwrap();

    ipt.flush_chain("filter", "DOCKER-USER").unwrap();
    for r in &config.rules {
        allow_devices(&ipt, r);
    }
    ipt.append("filter", "DOCKER-USER", "-j DROP").unwrap();
}

pub fn disable() {
    let ipt = iptables::new(false).unwrap();

    ipt.flush_chain("filter", "DOCKER-USER").unwrap();
    ipt.append("filter", "DOCKER-USER", "-j RETURN").unwrap();
}

fn allow_devices(ipt: &IPTables, rule: &Rule) {
    for device in &rule.allow {
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
