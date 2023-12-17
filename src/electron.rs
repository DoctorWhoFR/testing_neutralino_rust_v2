use std::fs;
use std::process::{Child, Command, exit, ExitCode};
use tera::Tera;

fn init_core(events: Vec<&str>, handlers: Vec<&str>, invokes: Vec<&str>) -> Result<(), ExitCode>{
    let mut tera = Tera::new("templates/**/*.txt");

    let tera = match tera {
        Ok(tera) => tera,
        Err(err) => {
            println!("no templates file find {:?}", err);

            return Err(ExitCode::FAILURE);
        }
    };

    let mut context = tera::Context::new();
    context.insert("events", &events);
    context.insert("handlers", &handlers);
    context.insert("invokes", &invokes);

    let rendered = tera.render("test.txt", &context);

    let rendered = match rendered {
        Ok(s) => s,
        Err(err) => {
            println!("error in rendering {:?}", err);

            return Err(ExitCode::FAILURE);
        }
    };

    println!("{}", rendered);

    let write_res = fs::write("./myapp/resources/core.js", rendered);

    match write_res {
        Ok(_) => {}
        Err(err) => {
            println!("error in writing {:?}", err);

            return Err(ExitCode::FAILURE);
        }
    }

    Ok(())
}

pub fn init_electron(events: Vec<&str>, handlers: Vec<&str>, invokes: Vec<&str>) -> std::io::Result<Child> {
    init_core(events, handlers, invokes).expect("test");
    println!("spawned");

    Command::new("neu.cmd")
        .args(["run"])
        .current_dir("./myapp")
        .spawn()
}