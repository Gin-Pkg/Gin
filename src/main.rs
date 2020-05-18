
use clap::Clap;
use std::fs;
extern crate duct;
use std::env::*;
extern crate dirs;
use std::process::*;
use std::io::prelude::*;
extern crate yaml_rust;
extern crate ansi_term;
use git2::build::{CheckoutBuilder, RepoBuilder};
use git2::{FetchOptions, Progress, RemoteCallbacks};
use indicatif::*;
use std::cell::RefCell;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
struct State {
    progress: Option<Progress<'static>>,
    total: usize,
    current: usize,
    path: Option<PathBuf>,
    newline: bool,
    progressBar: Option<ProgressBar>,
}

fn print(state: &mut State) {
let     stats = state.progress.as_ref().unwrap();
    if (stats.total_objects() == stats.indexed_objects())
        && (stats.total_deltas() == stats.indexed_deltas())
    {
        state.progressBar.as_ref().unwrap().finish_and_clear();
    }
    if let Some(pg) = state.progressBar.as_ref() {
        pg.inc(1);
    } else {
        state.progressBar = Some(ProgressBar::new(
        state.progress.as_ref().unwrap().total_objects() as u64 * 2,
    ));
        let bar = state.progressBar.as_mut();
        bar.unwrap().set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.red/blue} {pos:>7}/{len:7} {msg}"),

    );
    }
}



fn install_repo(url: &str, dir_name: &str) {
    let state = RefCell::new(State {
    progress: None,
    total: 0,
    current: 0,
    path: None,
    newline: false,
    progressBar: None,
    });
    let mut cb = RemoteCallbacks::new();
    cb.transfer_progress(|stats| {
        let mut state = state.borrow_mut();
        state.progress = Some(stats.to_owned());
        print(&mut *state);
        true
    });

    let mut co = CheckoutBuilder::new();
    co.progress(|path, cur, total| {
    let mut state = state.borrow_mut();
    state.path = path.map(|p| p.to_path_buf());
    state.current = cur;
    state.total = total;
    print(&mut *state);
  });
  let mut  path = dirs::home_dir().unwrap();
  path.push("Distillation");
  path.push(dir_name);
    let mut fo = FetchOptions::new();
    fo.remote_callbacks(cb);
      RepoBuilder::new()
    .fetch_options(fo)
    .with_checkout(co)
    .clone(
    url,
        Path::new(&path),
    )
    .unwrap();
    
}
use console::*;
#[derive(Clap)]
#[derive(Debug)]
#[clap(version = "1.0", author = "Jakob Nikolaus Neufeld")]
struct Gin {
    #[clap(subcommand)]
    subcmd: SubCommand,
}
#[derive(Clap)]
#[derive(Debug)]

enum SubCommand {
    #[clap(version = "1.0", author = "Jakob Nikolaus Neufeld")]
    Install(GinInstall)
}
#[derive(Clap)]
#[derive(Debug)]
struct GinInstall {
}

fn install(package: Package) {
    println!("Fetching Package: {}", ansi_term::Colour::Blue.bold().underline().blink().paint(package.name.clone()));
    let mut  path = dirs::home_dir().unwrap();
  path.push("Distillation");
  path.push(package.name.clone());
    if path.exists() {
        std::fs::remove_dir_all(path).unwrap();
    }
    //install_repo(package.source.as_str(), path.as_path().to_str().unwrap());
    install_repo(package.source.as_str(), package.name.as_str().clone());
    for dependencie in package.dependency_files {
       
        let mut pkg_name = dependencie.as_str();
        let last_index = pkg_name.rfind("/").expect("Invalid Git Url");
        let (_, git_location) = pkg_name.split_at(last_index + 1);
        pkg_name = git_location.trim_end_matches(".git");
        let mut  path = dirs::home_dir().unwrap();
        path.push("Distillation");
            path.push(pkg_name);
        if path.exists() {
              std::fs::remove_dir_all(&path).unwrap();
        }
        println!("{}", ansi_term::Color::Yellow.bold().paint(format!("Installing Dependencie {}, Shaking the gin" , pkg_name     )));
        
        install_repo(dependencie.as_str(), pkg_name);
        path.push("Ginfile");
        let file = read_gin_file(&path.to_str().unwrap());
        let dep_pkg = extract_pkg_from_file(file);
        let installation_cmd = dep_pkg.installation_commands.join(" && ");
        println!("Mesages from Package Installer:");
        println!("{:?}", duct::cmd("bash", &["-c", installation_cmd.as_str()]).read().unwrap())
        
    }
    let installation_cmd = package.installation_commands.join(" && ");
       duct::cmd("bash", &["-c", installation_cmd.as_str()]);

}


fn extract_pkg_from_file(yaml_string: String) -> Package {
    println!("Parsing Ginfile");
    let mut package = Package {
        name: String::from(""),
        source: String::from(""),
        dependency_files: vec![],
        test_commands: vec![],
        installation_commands: vec![],
        confilcts: vec![]
    };
    let yaml = yaml_rust::YamlLoader::load_from_str(yaml_string.as_str()).expect("Bad Gin shaker 🥃");
    let gin_spec = &yaml[0];
    package.name = gin_spec["name"].as_str().unwrap().to_string();
    package.source = gin_spec["source"].as_str().unwrap().to_string();
    let mut installation_commands = vec![];
    for command in gin_spec["installationCommands"].as_vec().unwrap() {
        installation_commands.push(command.as_str().unwrap().to_string());
       
    }
 
    let mut test_commands = vec![];
    for command in gin_spec["testCommands"].as_vec().unwrap() {
       
        test_commands.push(command.as_str().unwrap().to_string());
    }

    let mut conflicts = vec![];
    if let Some(_) = gin_spec["conflicts"].as_vec() {
        for command in gin_spec["conflicts"].as_vec().unwrap() {
            conflicts.push(command.as_str().unwrap().to_string());
        
        }
    }
    
    let mut deps = vec![];
    if let Some(_) = gin_spec["dependencyFiles"].as_vec() {
        for command in gin_spec["dependencyFiles"].as_vec().unwrap() {
             deps.push(command.as_str().unwrap().to_string());
   
        }
    }   
    package.installation_commands = installation_commands;
    package.test_commands = test_commands;
    package.confilcts = conflicts;
    package.dependency_files = deps;
    return package;
}
#[derive(Debug)]
struct Package { 
    name: String,
    source: String,
    dependency_files: Vec<String>,
    test_commands: Vec<String>,
    installation_commands: Vec<String>,
    confilcts: Vec<String>



}
#[inline]
fn read_gin_file(path: &str) -> String {
    let mut gin_file = fs::File::open(path).expect("No Ginfile");
    let mut gin_str = String::new();
    gin_file.read_to_string(& mut gin_str).expect("Failed Reading Ginfile");
    gin_str
}
fn cli_install() {
   let gin_str = read_gin_file("Ginfile");
   let pkg = extract_pkg_from_file(gin_str);
   install(pkg);
}
fn main() {
    let gin_cli = Gin::parse();
    match gin_cli.subcmd {
        _install => {cli_install()}
    }
    
}
