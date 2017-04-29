use ::package;
use getopts::Options;
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] [package_dir]\nIf no arguments are given the GUI menu is used instead. (excluding -g)", program);
    print!("{}", opts.usage(&brief));
}

pub fn cli() -> CLIResults {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("l", "list", "List available packages and close");
    opts.optopt("s", "stage",        "Use the stage specified", "NAME");
    opts.optopt("f", "fighters",     "Use the fighters specified", "NAME1,NAME2,NAME3...");
    opts.optopt("p", "players",      "Number of players in the game", "NUMPLAYERS");
    opts.optopt("g", "graphics",     "Graphics backend to use",
        if cfg!(features =  "vulkan") && cfg!(features = "opengl") {
            "[vulkan|opengl|none]"
        } else if cfg!(features =  "vulkan") {
            "[vulkan|none]"
        } else if cfg!(features = "opengl") {
            "[opengl|none]"
        } else {
            "[none]"
        }
    );

    let mut results = CLIResults::new();

    let matches = match opts.parse(&args[1..]) {
        Ok(m)  => { m },
        Err(_) => {
            print_usage(&program, opts);
            results.continue_from = ContinueFrom::Close;
            return results;
        },
    };

    if matches.opt_present("l") {
        package::print_list();
        results.continue_from = ContinueFrom::Close;
        return results;
    }

    if matches.free.len() > 1 {
        print_usage(&program, opts);
        results.continue_from = ContinueFrom::Close;
        return results;
    }
    else if matches.free.len() == 1 {
        results.continue_from = ContinueFrom::Game;
        results.package = Some(matches.free[0].clone());
    }

    if let Some(players) = matches.opt_str("p") {
        if let Ok(players) = players.parse::<usize>() {
            results.continue_from = ContinueFrom::Game;
            results.total_players = Some(players);
        }
        else {
            print_usage(&program, opts);
            results.continue_from = ContinueFrom::Close;
            return results;
        }
    }

    if let Some(fighter_names) = matches.opt_str("f") {
        for fighter_name in fighter_names.split(",") {
            results.continue_from = ContinueFrom::Game;
            results.fighter_names.push(fighter_name.to_string());
        }
    }

    if let Some(stage) = matches.opt_str("s") {
        results.stage_name = Some(stage);
        results.continue_from = ContinueFrom::Game;
    }

    if let Some(backend_string) = matches.opt_str("g") {
        results.graphics_backend = match backend_string.to_lowercase().as_ref() {
            #[cfg(feature = "vulkan")]
            "vulkan" => { GraphicsBackendChoice::Vulkan }
            #[cfg(feature = "opengl")]
            "opengl" => { GraphicsBackendChoice::OpenGL }
            "none"   => { GraphicsBackendChoice::Headless }
            _ => {
                print_usage(&program, opts);
                results.continue_from = ContinueFrom::Close;
                return results;
            }
        };
    }

    results
}

pub struct CLIResults {
    pub graphics_backend: GraphicsBackendChoice,
    pub package:          Option<String>,
    pub total_players:    Option<usize>,
    pub fighter_names:    Vec<String>,
    pub stage_name:       Option<String>,
    pub continue_from:    ContinueFrom,
}

impl CLIResults {
    pub fn new() -> CLIResults {
        CLIResults {
            graphics_backend: GraphicsBackendChoice::Default,
            package:          None,
            total_players:    None,
            fighter_names:    vec!(),
            stage_name:       None,
            continue_from:    ContinueFrom::Menu,
        }
    }
}

pub enum ContinueFrom {
    Menu,
    Game,
    Close
}

pub enum GraphicsBackendChoice {
    #[cfg(feature = "vulkan")]
    Vulkan,
    #[cfg(feature = "opengl")]
    OpenGL,
    Headless,
    Default,
}
