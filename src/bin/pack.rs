use std::fmt::Debug;

use latte::client::connect;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "latte-build")]
struct Cli {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    New,
    Build,
}

impl Cli {}

#[tokio::main(flavor = "current_thread")]
async fn main() -> latte::Result<()> {
    let cli = Cli::from_args();

    match cli.command {
        Command::New => println!("not implemented"),
        Command::Build => {
            let tmp_pkg_name = "tmp.pkg.tar";
            let latte_server = "127.0.0.1:6379";
            let pack_cmd = std::process::Command::new("sh")
                .arg("-c")
                .arg("wasm-pack")
                .arg("build")
                .output()
                .expect("Latte: failed to pack the project");
            let tar_cmd = std::process::Command::new("sh")
                .arg("-c")
                .args(["tar", "-cf", tmp_pkg_name, "pkg"])
                .output()
                .expect("Latte: failed to tar the project");
            let tar_file = tokio::fs::read(tmp_pkg_name).await?;
            let latte_client = connect(latte_server).await?;
            
        }
    };
    Ok(())
}
