use crate::App;
use docker_blocker::Rule;
use iptables::IPTables;

pub fn enable(app: &App) {
    let ipt = iptables::new(false).unwrap();

    match app.args.dry_run {
        true => println!("-F DOCKER-USER"),
        false => ipt.flush_chain("filter", "DOCKER-USER").unwrap(),
    };
    for r in &app.config.rules {
        allow_devices(&ipt, r, app.args.dry_run);
    }
    match app.args.dry_run {
        true => println!("-A DOCKER-USER -J DROP"),
        false => ipt.append("filter", "DOCKER-USER", "-j DROP").unwrap(),
    };
}

pub fn disable() {
    let ipt = iptables::new(false).unwrap();

    ipt.flush_chain("filter", "DOCKER-USER").unwrap();
    ipt.append("filter", "DOCKER-USER", "-j RETURN").unwrap();
}

fn allow_devices(ipt: &IPTables, rule: &Rule, dry_run: bool) {
    for device in &rule.allow {
        let src_rule = format!(
            "-s {} -p tcp --dport {} -m comment --comment 'docker-blocker: {}' -j RETURN",
            device, rule.port, rule.service
        );
        let dest_rule = format!(
            "-d {} -p tcp --sport {} -m comment --comment 'docker-blocker: {}' -j RETURN",
            device, rule.port, rule.service
        );
        match dry_run {
            true => {
                println!("-A DOCKER-USER {}", &src_rule);
                println!("-A DOCKER-USER {}", &dest_rule);
            }
            false => {
                ipt.append_unique("filter", "DOCKER-USER", &src_rule)
                    .unwrap();
                ipt.append_unique("filter", "DOCKER-USER", &dest_rule)
                    .unwrap();
            }
        };
    }
}
