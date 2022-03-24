use docker_blocker::Config;

pub fn reload(config: &Config) {
    for rule in &config.rules {
        println!("{:?}", rule);
    }
}

pub fn disable(config: Config) {}
