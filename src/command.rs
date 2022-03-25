use docker_blocker::{Config, Rule};
use iptables::IPTables;

pub fn reload(config: &Config) {
    let ipt = iptables::new(false).unwrap();

    reset_rules(&ipt);
    for r in &config.rules {
        allow_devices(&ipt, r);
        block_port(&ipt, r);
    }
    ipt.append("filter", "DOCKER-USER", "-j RETURN").unwrap();
}

pub fn disable() {
    let ipt = iptables::new(false).unwrap();

    reset_rules(&ipt);
}

// for DEV in $ALLOWED_DEVICES; do
//   iptables -"$METHOD" "$CHAIN" -s "$DEV" -p tcp --dport "$PORT" -j "$TARGET"
// done
// iptables -"$METHOD" "$CHAIN" -p tcp --dport "$PORT" -j REJECT
fn allow_devices(ipt: &IPTables, rule: &Rule) {
    for device in &rule.allow_from {
        let rule = format!(
            "-s {} -p tcp --dport {} -m comment --comment 'docker-blocker: {}' -j RETURN",
            device, rule.port, rule.service
        );
        ipt.append_unique("filter", "DOCKER-USER", &rule).unwrap();
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
