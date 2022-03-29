use anyhow::{bail, Result};

use crate::App;
use docker_blocker::Rule;
use iptables::IPTables;

pub fn enable(app: &App) {
    let ipt = iptables::new(false).unwrap();

    let preroute_return = "-m addrtype --dst-type LOCAL -m comment --comment 'docker-blocker: preroute_return' -j RETURN";
    let preroute_jump = "-m addrtype --dst-type LOCAL -m comment --comment 'docker-blocker: preroute_jump' -j DOCKER-BLOCKER";

    match app.args.dry_run {
        true => {
            println!("-t nat -N DOCKER-BLOCKER");
            println!("-t nat -I PREROUTING {}", preroute_return);
            println!("-t nat -I PREROUTING {}", preroute_jump);
        }
        false => {
            create_chain(&ipt).unwrap();
            ipt.insert_unique("nat", "DOCKER-BLOCKER", preroute_return, 1)
                .unwrap();
            ipt.insert_unique("nat", "DOCKER-BLOCKER", preroute_jump, 1)
                .unwrap();
        }
    }

    for r in &app.config.rules {
        allow_devices(&ipt, r, app.args.dry_run);
    }
}

pub fn disable() {
    let ipt = iptables::new(false).unwrap();

    // ipt.flush_chain("filter", "DOCKER-USER").unwrap();
    // ipt.append("filter", "DOCKER-USER", "-j RETURN").unwrap();
}

fn create_chain(ipt: &IPTables) -> Result<()> {
    match ipt.new_chain("nat", "DOCKER-BLOCKER") {
        Ok(_) => Ok(()),
        Err(e) => {
            if e.to_string().contains("Chain already exists.") {
                Ok(())
            } else {
                bail!("{}", e)
            }
        }
    }
}

fn allow_devices(ipt: &IPTables, rule: &Rule, dry_run: bool) {
    for device in &rule.allow {
        let rule = format!(
            "-s {} -p tcp -m tcp --dport {} -m state --state NEW,ESTABLISHED -m comment --comment 'docker-blocker: {}' -j DOCKER",
            device, rule.port, rule.service
        );
        match dry_run {
            true => {
                println!("-A DOCKER-BLOCKER {}", &rule);
            }
            false => {
                ipt.append_unique("nat", "DOCKER-BLOCKER", &rule).unwrap();
            }
        };
    }
}
