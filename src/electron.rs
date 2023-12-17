use std::path::{PathBuf, Path};
use std::{fs, env};
use std::process::{Child, Command, ExitCode};
use tera::Tera;
use dotenv::dotenv;

fn init_core(events: Vec<&str>, handlers: Vec<&str>, invokes: Vec<&str>) -> Result<(), ExitCode>{
    let tera = Tera::new("templates/**/*.txt");

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

fn get_path() {
    let path = Path::new("./myapp/myapp-win_x64.exe");
    println!("{:?}", path.canonicalize());
}

pub async fn init_electron(events: Vec<&str>, handlers: Vec<&str>, invokes: Vec<&str>) {
    dotenv().ok();
   
    let build_env = std::env::var("BUILD_ENV").unwrap_or_else(|_| "dev".to_string());
    if build_env == "dev" {
        init_core(events, handlers, invokes).expect("test");
        println!("spawing electron");
        
        Command::new("neu.cmd")
        .args(["run"])
        .current_dir("./myapp")
        .spawn()
        .expect("test");
    } else {
        println!("{:?}", env::current_dir());
        let path = Path::new("./myapp/myapp-win_x64.exe");
        println!("{:?}", path.canonicalize());

        Command::new(path.canonicalize().unwrap().to_str().unwrap())
        .spawn()
        .expect("fe");
    }

   

    println!("spawned");
}