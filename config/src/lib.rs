pub mod script;

// use protobuf::Message;
use protobuf::RepeatedField; 
use script::*;

pub fn default_script() -> Scripts {
    let script_vec = vec![
        make_script(ScriptType::LOG, vec![
            "Hello World!".to_string(),
            "From Playground".to_string(),
        ]),
        make_script(ScriptType::TEST_UNDER_CONSTRUCTION, vec![]),
    ];

    let mut scripts = Scripts::new();
    scripts.scripts = RepeatedField::from_vec(script_vec);

    scripts
}


pub fn make_script(script_type: ScriptType, arguments: Vec<String>) -> Script{
    let mut script = Script::new();
    script.script_type = script_type;
    script.arguments = RepeatedField::from_vec(arguments);
    return script;
}