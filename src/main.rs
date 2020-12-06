// use leetcode::part1::part1_0::main336;
use config;
use engine;


fn main() -> Result<(), ()>{
    let scripts = config::default_script();
    engine::excute(&scripts)
}
