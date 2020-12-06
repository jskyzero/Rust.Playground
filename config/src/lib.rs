pub mod script;

// use protobuf::Message;
use script::*;

pub fn default_script() -> Scripts{
    let mut script = Script::new();
    let mut scripts = Scripts::new();

    script.script_type = ScriptType::LOG;
    script.arguments.push("Hello World!".to_string());

    scripts.scripts.push(script);

    scripts
}