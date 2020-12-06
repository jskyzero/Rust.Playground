
use config::script::*;

pub fn excute(scripts: & Scripts ) ->Result<(), ()> {
    for script in scripts.scripts.iter() {
        if excute_each(script).is_err() {
            return Err(());
        }
    }
    Ok(())
}

fn excute_each(script: & Script) -> Result<(), ()> {
    match script.script_type {
        ScriptType::LOG => {
            println!("{:?}", script.arguments.to_vec());
        }
        _ => {
            return Err(());
        }
    }
    Ok(())
}