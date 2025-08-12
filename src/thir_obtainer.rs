use rustc_driver::{Callbacks, Compilation, run_compiler};
use rustc_interface::interface::Compiler;
use rustc_middle::ty::TyCtxt;

use std::env;
use std::process;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Write;

struct MyCallbacks {
    output_file: PathBuf,
    cleared: bool
}

impl Callbacks for MyCallbacks {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &Compiler,
        tcx: TyCtxt<'tcx>
    ) -> Compilation {
        let mut file = match File::options()
            .append(true)
            .create(true)
            .open(&self.output_file)
        {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Er: Faild to open or create output\n{}", e);
                process::exit(1);
            }
        };

        if !self.cleared {
            if let Ok(f) = File::create(&self.output_file) {
                drop(f);
                self.cleared = true;
            }
        }

        for def_id in tcx.mir_keys(()) {
            let (thir, _) = tcx.thir_body(def_id).unwrap();
            let thir = thir.steal();

            if let Err(e) = writeln!(file, "{:#?}", thir){
                eprintln!("Er: Faild to write THIR to output\n{}", e);
                process::exit(1);
            }
        }

        Compilation::Stop
    }
}

fn get_sysroot_path() -> String {
    let sysroot_path = String::from_utf8(
        std::process::Command::new("rustc")
        .arg("--print")
        .arg("sysroot")
        .output()
        .expect("Er: Faild to execute `rustc --print sysroot`").stdout
    )
    .unwrap()
    .trim()
    .to_string();

    return sysroot_path;
}

pub fn obtain_thir() {    
    let cmd_args: Vec<String> = env::args().collect();
    if cmd_args.len() < 3 {
        eprintln!("Er: More args are needed");
        process::exit(1);
    }
    // println!("Args: {:?}", cmd_args);

    let input_file = Path::new(&cmd_args[1]);
    let output_file = PathBuf::from(&cmd_args[2]);
    println!("Input: {}", input_file.display());
    println!("Output: {}", output_file.display());

    let rustc_args = vec![
        "thir_obtainer".to_string(),
        input_file.to_str().unwrap().to_string(),
        "-Zno-steal-thir".to_string(),
        "--sysroot".to_string(),
        get_sysroot_path(),
    ];
    // println!("{:?}", rustc_args);

    let mut mycallbacks = MyCallbacks {
        output_file,
        cleared: false,
    };

    run_compiler(&rustc_args, &mut mycallbacks);
}
