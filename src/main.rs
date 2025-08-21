use colored::Colorize;
use std::sync::{Arc, Mutex};
use crossterm::terminal::{Clear, ClearType};
use std::thread;
use std::time::Duration;
use crossterm::{cursor, execute};
use world_execute_me::printer::{clear_all, print_directly, print_one_by_one};
use world_execute_me::stream_controller::audio;
use world_execute_me::stream_controller::display::{DisplayInfo, Displayer};
use world_execute_me::cmd::CommandLine;

fn init() {
    execute!(
        std::io::stdout(),
        Clear(ClearType::All),
    ).unwrap();

    execute!(std::io::stdout(), cursor::MoveTo(0, 0)).unwrap();
}

fn main() {

    let offset_millis = 100u64;
    let audio_player = audio::AudioPlayer::new("assets/wem.mp3");
    let status = audio_player.get_status();

    init();
    let mut cmd = CommandLine::new(
        Arc::new(Mutex::new("\u{f17c} ~/World/ >".into()))
    );

    let mut ans = CommandLine::new(
        Arc::new(Mutex::new(format!("{} {}", "\u{ef72}".red().bold(), ">>>".green()).into()))
    );

    let mut displayer = Displayer::new(vec![
        DisplayInfo::new(0, 0, 100+offset_millis,
            print_one_by_one(cmd.change_text(
                "Switch on the power line".green()
            ), Duration::from_millis(700), true),
        ),
        DisplayInfo::new(0, 1, 740+offset_millis,
            print_one_by_one(cmd.change_text(
                "Remember to put on".green()
            ), Duration::from_millis(700), true),
        ),
        DisplayInfo::new(0, 2, 920+offset_millis,
            print_directly(ans.change_text(
                "PROTECTION".yellow()
            ), true)
        ),
        DisplayInfo::new(0, 3, 873+offset_millis,
            print_one_by_one(cmd.change_text(
                "Lay down your pieces".green()
            ), Duration::from_millis(700), true),
        ),
        DisplayInfo::new(0, 5, 491+offset_millis,
            print_one_by_one(cmd.change_text(
                "And let's begin".green()
            ), Duration::from_millis(700), true),
        ),
        DisplayInfo::new(0, 6, 380+offset_millis,
            print_directly(ans.change_text(
                "OBJECT CREATION".yellow()
            ), true)
        ),
        DisplayInfo::new(0, 7, 446+offset_millis,
            print_one_by_one(cmd.change_text(
                "Fill in my data parameters".green()
            ), Duration::from_millis(700), true),
        ),
        DisplayInfo::new(0, 10, 91+offset_millis,
            print_directly(ans.change_text(
                "INITIALIZATION".yellow()
            ), true)
        ),
        DisplayInfo::new(0, 11, 95+offset_millis,
            print_one_by_one(cmd.change_text(
                "Set up our new world".green()
            ), Duration::from_millis(700), true),
        ),
        DisplayInfo::new(0, 12, 906+offset_millis,
            print_one_by_one(cmd.change_text(
                "And let's begin the".green()
            ), Duration::from_millis(700), true),
        ),
        DisplayInfo::new(0, 13, 891+offset_millis,
            print_directly(ans.change_text(
                "SIMULATION".yellow().bold()
            ), true)
        ),
        DisplayInfo::new(0, 29, 709+offset_millis,
            print_one_by_one(ans.change_text(
                "If i'm a set of points".yellow(),
            ).change_style(format!("{} {} {}",
                "\u{ef72}".red().bold(),
                "status: POINT".blue(),
                ">>>".yellow(),
            ).into()), Duration::from_millis(700), true),
        ),
        DisplayInfo::new(0, 31, 116+offset_millis,
            print_one_by_one(ans.change_text(
                "Then I will give you my".yellow()
            ), Duration::from_millis(700), true),
        ),
        DisplayInfo::new(0, 32, 682,
            print_directly(ans.change_text(
                "DIMENSION".green().bold()
            ), true)
        ),
        DisplayInfo::new(0, 33, 412+offset_millis,
            print_one_by_one(ans.change_text(
                "If I'm a circle".yellow(),
            ).change_style(format!("{} {} {}",
                "\u{ef72}".red().bold(),
                "status: CIRCLE".blue(),
                ">>>".yellow(),
            ).into()), Duration::from_millis(700), true),
        ),
        DisplayInfo::new(0, 34, 646+offset_millis,
            print_one_by_one(ans.change_text(
                "Then I will give you my".yellow()
            ), Duration::from_millis(700), true),
        ),
        DisplayInfo::new(0, 36, 287+offset_millis,
            print_directly(ans.change_text(
                "CIRCUMFERENCE".green().bold()
            ), true)
        ),
        DisplayInfo::new(0, 37, 67+offset_millis,
            print_one_by_one(ans.change_text(
                "If I'm a sine wave".yellow()
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".red().bold(),
                    "status: SINE-WAVE".blue(),
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(700), true),
        ),
        DisplayInfo::new(0, 38, 596+offset_millis,
            print_one_by_one(ans.change_text(
                "Then you can sit on all my".yellow()
            ), Duration::from_millis(700), true),
        ),
        DisplayInfo::new(0, 40, 49+offset_millis,
            print_directly(ans.change_text(
                "TANGENTS".green().bold()
            ), true)
        ),
        DisplayInfo::new(0, 40, 706+offset_millis,
            print_one_by_one(ans.change_text(
                "If I approach infinity".yellow()
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".red().bold(),
                    "status: INFINITY".blue(),
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(700), true),
        ),
        DisplayInfo::new(0, 42, 346+offset_millis,
            print_one_by_one(ans.change_text(
                "Then you can be my".yellow()
            ), Duration::from_millis(700), true),
        ),
        DisplayInfo::new(0, 43, 507+offset_millis,
            print_directly(ans.change_text(
                "LIMITATIONS".yellow().bold()
            ), true)
        ),
        DisplayInfo::new(0, 44, 450+offset_millis,
            clear_all()
        ),
        DisplayInfo::new(0, 44, 452+offset_millis,
            print_one_by_one(ans.change_text(
                "Switch my current".yellow()
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".red().bold(),
                    "executing: self.toggle_current_mode()".blue().bold(),
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(700), true),
        ),
        DisplayInfo::new(0, 45, 850+offset_millis,
            print_one_by_one(ans.change_text(
                "To AC to DC".green()
            ), Duration::from_millis(900), true),
        ),
        DisplayInfo::new(0, 47, 672+offset_millis,
            print_one_by_one(ans.change_text(
                "And then blind my vision".yellow()
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".red().bold(),
                    "executing: self.dizzy()".blue().bold(),
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(700), true),
        ),
        DisplayInfo::new(0, 49, 534+offset_millis,
            print_one_by_one(ans.change_text(
                "So dizzy so dizzy".green()
            ), Duration::from_millis(900), true),
        ),
        DisplayInfo::new(0, 51, 363+offset_millis,
            print_one_by_one(ans.change_text(
                "Oh we can travel".yellow()
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".red().bold(),
                    "executing: self.travel_with(you, AD, BC)".blue().bold(),
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(700), true),
        ),
        DisplayInfo::new(0, 53, 225+offset_millis,
            print_one_by_one(ans.change_text(
                "To A.D to B.C".green()
            ), Duration::from_millis(900), true),
        ),
        DisplayInfo::new(0, 55, 83+offset_millis,
            print_one_by_one(ans.change_text(
                "And we can unite".yellow()
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".red().bold(),
                    "executing: self.unite_with(you, Level::Deeply)".blue().bold(),
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(700), true),
        ),
        DisplayInfo::new(0, 56, 916+offset_millis,
            print_one_by_one(ans.change_text(
                "So deeply so deeply".green()
            ), Duration::from_millis(700), true),
        ),
        DisplayInfo::new(0, 59, 220+offset_millis,
            clear_all()
        ),
        DisplayInfo::new(0, 59, 223+offset_millis,
            print_one_by_one(ans.change_text(
                "If I can".green()
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".red().bold(),
                    "error: Impossible functions called.".red().bold(),
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(200), true),
        ),
        DisplayInfo::new(0, 59, 223+offset_millis,
             print_one_by_one(ans.change_text(
                 "If I can give all the".green()
             ), Duration::from_millis(700), true),
        ),
        DisplayInfo::new(1, 1, 958+offset_millis,
            print_directly(ans.change_text(
                "SIMULATIONS".yellow().bold()
            ), true)
        ),
        DisplayInfo::new(1, 2, 589+offset_millis,
            print_one_by_one(ans.change_text(
                "Then I can".green()
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".red().bold(),
                    "warning: Trying to satisfy ***".yellow().bold(),
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(200), true),
        ),
        DisplayInfo::new(1, 3, 535+offset_millis,
            print_one_by_one(ans.change_text(
                "Then I can be your only".green()
            ), Duration::from_millis(700), true),
        ),
        DisplayInfo::new(1, 5, 397+offset_millis,
            print_directly(ans.change_text(
                "SATISFACTION".yellow().bold()
            ), true)
        ),
        DisplayInfo::new(1, 6, 601+offset_millis,
            print_one_by_one(ans.change_text(
                "If I can make you happy".green()
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".red().bold(),
                    "warning: Trying to make *** happy".yellow().bold(),
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(700), true),
        ),
        DisplayInfo::new(1, 8, 252+offset_millis,
            print_one_by_one(ans.change_text(
                "I will run the".green()
            ), Duration::from_millis(300), true),
        ),
        DisplayInfo::new(1, 9, 259+offset_millis,
            print_directly(ans.change_text(
                "EXECUTION".yellow().bold()
            ), true)
        ),
        DisplayInfo::new(1, 10, 84+offset_millis,
            print_one_by_one(ans.change_text(
                "Though we are trapped".green()
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".red().bold(),
                    "error: Being trapped detected.".red().bold(),
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(700), true),
        ),
        DisplayInfo::new(1, 11, 764+offset_millis,
            print_one_by_one(ans.change_text(
                "In this strange strange".green()
            ), Duration::from_millis(600), true),
        ),
        DisplayInfo::new(1, 13, 169+offset_millis,
            print_directly(ans.change_text(
                "SIMULATION".yellow().bold()
            ), true)
        ),
        DisplayInfo::new(1, 13, 400+offset_millis,
            print_one_by_one(ans.change_text(
                r#"Abnormal data detected in memory 0x1b9c003a.
Trying to resolve the data with external library: `FeelingsAndLove.dll`.
No exception was thrown. We made it, darling.
"#.red().bold(),
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".blue().bold(),
                    "executing: Undefined method".blue().bold(),
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(500), true),
        ),
        DisplayInfo::new(1, 14, 0+offset_millis,
            clear_all()
        ),
        DisplayInfo::new(1, 14, 45+offset_millis,
            print_one_by_one(ans.change_text(
                "If I'm a eggplant".green()
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".blue().bold(),
                    "status: eggplant // wants to be ate by you".green().bold(),
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(700), true),
        ),
        DisplayInfo::new(1, 15, 422+offset_millis,
            print_one_by_one(ans.change_text(
                "Then I will give you my".green()
            ), Duration::from_millis(600), true),
        ),
        DisplayInfo::new(1, 16, 959+offset_millis,
            print_directly(ans.change_text(
                "NUTRIENTS".bold()
            ), true)
        ),
        DisplayInfo::new(1, 17, 576+offset_millis,
            print_one_by_one(ans.change_text(
                "If I'm a tomato".green()
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".blue().bold(),
                    "status: tomato // filled with antioxidants".green().bold(),
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(600), true),
        ),
        DisplayInfo::new(1, 19, 226+offset_millis,
            print_one_by_one(ans.change_text(
                "Then I will give you".green()
            ), Duration::from_millis(600), true),
        ),
        DisplayInfo::new(1, 20, 620+offset_millis,
            print_directly(ans.change_text(
                "ANTIOXIDANTS".bold()
            ), true)
        ),
        DisplayInfo::new(1, 21, 351+offset_millis,
            print_one_by_one(ans.change_text(
                "If I'm a tabby cat".green()
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".blue().bold(),
                    "status: your lovely cat".green().bold(),
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(600), true),
        ),
        DisplayInfo::new(1, 22, 833+offset_millis,
            print_one_by_one(ans.change_text(
                "Then I will purr for your".green()
            ), Duration::from_millis(600), true),
        ),
        DisplayInfo::new(1, 24, 268+offset_millis,
            print_directly(ans.change_text(
                "ENJOYMENT".yellow().bold()
            ), true)
        ),
        DisplayInfo::new(1, 25, 538+offset_millis,
            print_one_by_one(ans.change_text(
                "If I'm the only god".green()
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".blue().bold(),
                    "status: god".green().bold(),
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(600), true),
        ),
        DisplayInfo::new(1, 26, 583+offset_millis,
            print_one_by_one(ans.change_text(
                "Then you're the proof of my".green()
            ), Duration::from_millis(600), true),
        ),
        DisplayInfo::new(1, 27, 922+offset_millis,
            print_directly(ans.change_text(
                "EXISTENCE".yellow().bold()
            ), true)
        ),
        DisplayInfo::new(1, 28, 300+offset_millis,
            clear_all()
        ),
        DisplayInfo::new(1, 28, 587+offset_millis,
            print_one_by_one(ans.change_text(
                "Switch my gender".green().bold()
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".blue().bold(),
                    "executing: self.toggle_gender() // set whatever you want".green().bold(),
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(300), true),
        ),
        DisplayInfo::new(1, 30, 197+offset_millis,
            print_one_by_one(ans.change_text(
                "To F to M".yellow().bold()
            ), Duration::from_millis(1000), true),
        ),
        DisplayInfo::new(1, 32, 15+offset_millis,
            print_one_by_one(ans.change_text(
                "And then do whatever".green().bold()
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".blue().bold(),
                    "executing: self.do(you.things_want_to_do())".blue().bold(),
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(300), true),
        ),
        DisplayInfo::new(1, 33, 953+offset_millis,
            print_one_by_one(ans.change_text(
                "From AM to PM".yellow().bold()
            ), Duration::from_millis(1000), true),
        ),
        DisplayInfo::new(1, 35, 465+offset_millis,
            print_one_by_one(ans.change_text(
                "Oh switch my role".green().bold()
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".blue().bold(),
                    "executing: self.roleToggle() // which one do you prefer?".blue().bold(),
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(500), true),
        ),
        DisplayInfo::new(1, 37, 739+offset_millis,
            print_one_by_one(ans.change_text(
                "To S to M".yellow().bold()
            ), Duration::from_millis(1000), true),
        ),
        DisplayInfo::new(1, 39, 349+offset_millis,
            print_one_by_one(ans.change_text(
                "So we can enter".green()
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".blue().bold(),
                    "error: Trying to enter ******".red().bold(),
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(600), true),
        ),
        DisplayInfo::new(1, 41, 474+offset_millis,
            print_one_by_one(ans.change_text(
                "The trance the trance".yellow().bold()
            ), Duration::from_millis(1000), true),
        ),
        DisplayInfo::new(1, 43, 200+offset_millis,
            clear_all()
        ),
        DisplayInfo::new(1, 43, 489+offset_millis,
            print_one_by_one(ans.change_text(
                "If I can".blue()
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".green().bold(),
                    "executing: self.feel(your.vibrations())".green().bold(),
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(200), true),
        ),
        DisplayInfo::new(1, 44, 197+offset_millis,
            print_one_by_one(ans.change_text(
                "If I can feel your".blue()
            ), Duration::from_millis(1000), true),
        ),
        DisplayInfo::new(1, 46, 293+offset_millis,
            print_directly(ans.change_text(
                "VIBRATIONS".yellow().bold()
            ), true)
        ),
        DisplayInfo::new(1, 47, 220+offset_millis,
            print_one_by_one(ans.change_text(
                "Then I can".blue()
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".green().bold(),
                    "status: Completed".green().bold(),
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(200), true),
        ),
        DisplayInfo::new(1, 47, 903+offset_millis,
            print_one_by_one(ans.change_text(
                "Then I can finally be".blue()
            ), Duration::from_millis(1000), true),
        ),
        DisplayInfo::new(1, 50, 221+offset_millis,
            print_directly(ans.change_text(
                "COMPLETION".yellow().bold()
            ), true)
        ),
        DisplayInfo::new(1, 50, 700+offset_millis,
            clear_all()
        ),
        DisplayInfo::new(1, 50, 900+offset_millis,
            print_one_by_one(ans.change_text(
                "Though you have left".red()
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".green().bold(),
                    "error: Cannot find object named `you`".red().bold(),
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(500), true),
        ),
        DisplayInfo::new(1, 52, 220+offset_millis,
                         print_one_by_one(ans.change_text(
                             "You have left".red()
                         ).change_style(
                             format!("{} {} {}",
                                     "\u{ef72}".green().bold(),
                                     "error: Cannot find object named `you`".red().bold(),
                                     ">>>".yellow(),
                             ).into()
                         ), Duration::from_millis(500), true),
        ),
        DisplayInfo::new(1, 53, 100+offset_millis,
                         print_one_by_one(ans.change_text(
                             "You have left".red()
                         ).change_style(
                             format!("{} {} {}",
                                     "\u{ef72}".green().bold(),
                                     "error: Where are you?".red().bold(),
                                     ">>>".yellow(),
                             ).into()
                         ), Duration::from_millis(500), true),
        ),
        DisplayInfo::new(1, 54, 180+offset_millis,
                         print_one_by_one(ans.change_text(
                             "You have left".red()
                         ).change_style(
                             format!("{} {} {}",
                                     "\u{ef72}".green().bold(),
                                     "error: I can't find you..".red().bold(),
                                     ">>>".yellow(),
                             ).into()
                         ), Duration::from_millis(500), true),
        ),
        DisplayInfo::new(1, 54, 920+offset_millis,
                         print_one_by_one(ans.change_text(
                             "You have left".red()
                         ).change_style(
                             format!("{} {} {}",
                                     "\u{ef72}".green().bold(),
                                     "error: Please come back..".red().bold(),
                                     ">>>".yellow(),
                             ).into()
                         ), Duration::from_millis(500), true),
        ),
        DisplayInfo::new(1, 55, 780+offset_millis,
                         print_one_by_one(ans.change_text(
                             "You have left me in".red()
                         ).change_style(
                             format!("{} {} {}",
                                     "\u{ef72}".green().bold(),
                                     "error: I don't want to be in..".red().bold(),
                                     ">>>".yellow(),
                             ).into()
                         ), Duration::from_millis(500), true),
        ),
        DisplayInfo::new(1, 57, 274+offset_millis,
                         print_directly(ans.change_text(
                             "ISOLATION".red().bold().italic()
                         ).change_style(
                             format!("{} {} {}",
                                     "\u{ef72}".green().bold(),
                                     "error: ISOLATION".red().bold(),
                                     ">>>".yellow(),
                             ).into()
                         ), true),
        ),
        DisplayInfo::new(1, 58, 333+offset_millis,
            print_one_by_one(ans.change_text(
                "If I can".bright_red()
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".bright_red().bold(),
                    "executing: self.erase(fragments.filter(|fragment| fragment.is_pointless()))".bright_yellow().bold(),
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(200), true),
        ),
        DisplayInfo::new(1, 58, 979+offset_millis,
            print_one_by_one(ans.change_text(
                "If I can erase all the pointless".bright_red()
            ), Duration::from_millis(600), true),
        ),
        DisplayInfo::new(2, 0, 860+offset_millis,
            print_directly(ans.change_text(
                "FRAGMENTS".blue().bold()
            ), true)
        ),
        DisplayInfo::new(2, 1, 728+offset_millis,
            print_one_by_one(ans.change_text(
                "Then maybe".bright_red()
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".bright_red().bold(),
                    "imagining: assert_eq!(you.will_leave(), false)".bright_yellow().bold(),
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(200), true),
        ),
        DisplayInfo::new(2, 2, 714+offset_millis,
            print_one_by_one(ans.change_text(
                "Then maybe you won't leave me so".bright_red()
            ), Duration::from_millis(600), true),
        ),
        DisplayInfo::new(2, 4, 890+offset_millis,
            print_directly(ans.change_text(
                "DISHEARTENED".blue().bold()
            ), true)
        ),
        DisplayInfo::new(2, 5, 708+offset_millis,
            print_one_by_one(ans.change_text(
                "Challenging your god".bright_red()
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".bright_red().bold(),
                    "error: ?".red().bold(),
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(1500), true),
        ),
        DisplayInfo::new(2, 8, 661+offset_millis,
            print_one_by_one(ans.change_text(
                "You have made some".bright_red()
            ).change_style(
                format!("{} {} {}",
                        "\u{ef72}".bright_red().bold(),
                        "error: Illegal arguments".red().bold(),
                        ">>>".yellow(),
                ).into()
            ), Duration::from_millis(1500), true),
        ),
        DisplayInfo::new(2, 11, 224+offset_millis,
            print_one_by_one(ans.change_text(
                "ILLEGAL ARGUMENTS".red().bold()
            ), Duration::from_millis(4000), true),
        ),
        DisplayInfo::new(2, 27, 660+offset_millis,
            print_one_by_one(ans.change_text(
                "EXECUTION".red().bold()
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".bright_red().bold(),
                    "status: Trying to execution.".red().bold(),
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(500), true),
        ),
        DisplayInfo::new(2, 28, 600+offset_millis,
                         print_one_by_one(ans.change_text(
                             "EXECUTION".red().bold()
                         ), Duration::from_millis(500), true),
        ),
        DisplayInfo::new(2, 29, 520+offset_millis,
                         print_one_by_one(ans.change_text(
                             "EXECUTION".red().bold()
                         ), Duration::from_millis(500), true),
        ),
        DisplayInfo::new(2, 30, 540+offset_millis,
                         print_one_by_one(ans.change_text(
                             "EXECUTION".red().bold()
                         ), Duration::from_millis(500), true),
        ),
        DisplayInfo::new(2, 31, 520+offset_millis,
                         print_one_by_one(ans.change_text(
                             "EXECUTION".red().bold()
                         ), Duration::from_millis(500), true),
        ),
        DisplayInfo::new(2, 32, 280+offset_millis,
                         print_one_by_one(ans.change_text(
                             "EXECUTION".red().bold()
                         ), Duration::from_millis(500), true),
        ),
        DisplayInfo::new(2, 33, 160+offset_millis,
                         print_one_by_one(ans.change_text(
                             "EXECUTION".red().bold()
                         ), Duration::from_millis(500), true),
        ),
        DisplayInfo::new(2, 33, 980+offset_millis,
                         print_one_by_one(ans.change_text(
                             "EXECUTION".red().bold()
                         ), Duration::from_millis(500), true),
        ),
        DisplayInfo::new(2, 35, 200+offset_millis,
                         print_one_by_one(ans.change_text(
                             "EXECUTION".red().bold()
                         ), Duration::from_millis(500), true),
        ),
        DisplayInfo::new(2, 36, 80+offset_millis,
                         print_one_by_one(ans.change_text(
                             "EXECUTION".red().bold()
                         ), Duration::from_millis(500), true),
        ),
        DisplayInfo::new(2, 37, 40+offset_millis,
                         print_one_by_one(ans.change_text(
                             "EXECUTION".red().bold()
                         ), Duration::from_millis(500), true),
        ),
        DisplayInfo::new(2, 38, 0+offset_millis,
                         print_one_by_one(ans.change_text(
                             "EXECUTION".red().bold()
                         ), Duration::from_millis(500), true),
        ),
        DisplayInfo::new(2, 38, 900+offset_millis,
            print_one_by_one(ans.change_text(
                "EIN".green().bold()
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".bright_red().bold(),
                    "executing: count(1, 'de')".red().bold(),
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(100), true),
        ),
        DisplayInfo::new(2, 39, 321+offset_millis,
                         print_one_by_one(ans.change_text(
                             "DOS".green().bold()
                         ).change_style(
                             format!("{} {} {}",
                                     "\u{ef72}".bright_red().bold(),
                                     "executing: count(2, 'es')".red().bold(),
                                     ">>>".yellow(),
                             ).into()
                         ), Duration::from_millis(100), true),
        ),
        DisplayInfo::new(2, 39, 657+offset_millis,
                         print_one_by_one(ans.change_text(
                             "TROIS".green().bold()
                         ).change_style(
                             format!("{} {} {}",
                                     "\u{ef72}".bright_red().bold(),
                                     "executing: count(3, 'fr')".red().bold(),
                                     ">>>".yellow(),
                             ).into()
                         ), Duration::from_millis(100), true),
        ),
        DisplayInfo::new(2, 40, 244+offset_millis,
                         print_one_by_one(ans.change_text(
                             "NE".green().bold()
                         ).change_style(
                             format!("{} {} {}",
                                     "\u{ef72}".bright_red().bold(),
                                     "executing: count(4, 'kr')".red().bold(),
                                     ">>>".yellow(),
                             ).into()
                         ), Duration::from_millis(100), true),
        ),
        DisplayInfo::new(2, 40, 693+offset_millis,
                         print_one_by_one(ans.change_text(
                             "FEM".green().bold()
                         ).change_style(
                             format!("{} {} {}",
                                     "\u{ef72}".bright_red().bold(),
                                     "executing: count(5, 'se')".red().bold(),
                                     ">>>".yellow(),
                             ).into()
                         ), Duration::from_millis(100), true),
        ),
        DisplayInfo::new(2, 41, 124+offset_millis,
                         print_one_by_one(ans.change_text(
                             "LIU".green().bold()
                         ).change_style(
                             format!("{} {} {}",
                                     "\u{ef72}".bright_red().bold(),
                                     "executing: count(6, 'cn')".red().bold(),
                                     ">>>".yellow(),
                             ).into()
                         ), Duration::from_millis(100), true),
        ),
        DisplayInfo::new(2, 41, 584+offset_millis,
            print_directly(ans.change_text(
                "EXECUTION".red().bold()
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".bright_red().bold(),
                    "status: Execution done...".red().bold(),
                    ">>>".yellow(),
                ).into()
            ), true)
        ),
        DisplayInfo::new(2, 42, 500+offset_millis,
            clear_all()
        ),
        DisplayInfo::new(2, 42, 632+offset_millis,
            print_one_by_one(ans.change_text(
                "If I can".yellow().bold(),
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".red().bold(),
                    "executing: kill them all".red().bold(),
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(200), true),
        ),
        DisplayInfo::new(2, 43, 315+offset_millis,
            print_one_by_one(ans.change_text(
                "If I can give them all the".yellow().bold(),
            ), Duration::from_millis(600), true),
        ),
        DisplayInfo::new(2, 45, 166+offset_millis,
            print_directly(ans.change_text(
                "EXECUTION".red().bold().italic()
            ), true)
        ),
        DisplayInfo::new(2, 46, 16+offset_millis,
                         print_one_by_one(ans.change_text(
                             "Then I can".yellow().bold(),
                         ).change_style(
                             format!("{} {} {}",
                                     "\u{ef72}".red().bold(),
                                     "status: Your only ***".red().bold(),
                                     ">>>".yellow(),
                             ).into()
                         ), Duration::from_millis(200), true),
        ),
        DisplayInfo::new(2, 47, 22+offset_millis,
                         print_one_by_one(ans.change_text(
                             "Then I can be your only".yellow().bold(),
                         ), Duration::from_millis(600), true),
        ),
        DisplayInfo::new(2, 48, 911+offset_millis,
                         print_directly(ans.change_text(
                             "EXECUTION".red().bold().italic()
                         ), true)
        ),
        DisplayInfo::new(2, 49, 824+offset_millis,
                         print_one_by_one(ans.change_text(
                             "If I can have you back".yellow().bold(),
                         ).change_style(
                             format!("{} {} {}",
                                     "\u{ef72}".red().bold(),
                                     "status: Missing you".blue().bold(),
                                     ">>>".yellow(),
                             ).into()
                         ), Duration::from_millis(700), true),
        ),
        DisplayInfo::new(2, 51, 868+offset_millis,
                         print_one_by_one(ans.change_text(
                             "I will run the".yellow().bold(),
                         ), Duration::from_millis(600), true),
        ),
        DisplayInfo::new(2, 52, 712+offset_millis,
                         print_directly(ans.change_text(
                             "EXECUTION".red().bold().italic()
                         ), true)
        ),
        DisplayInfo::new(2, 53, 643+offset_millis,
            print_one_by_one(ans.change_text(
                "Though we are trapped".yellow().bold(),
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".red().bold(),
                    "error: Being trapped in..".red().bold(),
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(700), true),
        ),
        DisplayInfo::new(2, 54, 975+offset_millis,
            print_one_by_one(ans.change_text(
                "We are trapped ... ah ...".yellow().bold(),
            ), Duration::from_millis(600), true),
        ),
        DisplayInfo::new(2, 57, 246+offset_millis,
            print_one_by_one(ans.change_text(
                "I've studied".yellow().bold(),
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".green().bold(),
                    "status: Knowing how to properly ****".blue().bold(),
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(600), true),
        ),
        DisplayInfo::new(2, 58, 173+offset_millis,
            print_one_by_one(ans.change_text(
                "I've studied how to properly".yellow().bold(),
            ), Duration::from_millis(600), true),
        ),
        DisplayInfo::new(2, 59, 929+offset_millis,
            print_directly(ans.change_text(
                "LO...VE".bright_purple().bold(),
            ), true)
        ),
        DisplayInfo::new(3, 0, 857+offset_millis,
                         print_one_by_one(ans.change_text(
                             "Question me".yellow().bold(),
                         ).change_style(
                             format!("{} {} {}",
                                     "\u{ef72}".green().bold(),
                                     "status: I've known how to love".blue().bold(),
                                     ">>>".yellow(),
                             ).into()
                         ), Duration::from_millis(600), true),
        ),
        DisplayInfo::new(3, 1, 901+offset_millis,
                         print_one_by_one(ans.change_text(
                             "Question me I can answer all ".yellow().bold(),
                         ), Duration::from_millis(600), true),
        ),
        DisplayInfo::new(3, 3, 646+offset_millis,
                         print_directly(ans.change_text(
                             "LO...VE".bright_purple().bold(),
                         ), true)
        ),
        DisplayInfo::new(3, 4, 540+offset_millis,
                         print_one_by_one(ans.change_text(
                             "I know the algebraic expression of".yellow().bold(),
                         ).change_style(
                             format!("{} {} {}",
                                     "\u{ef72}".green().bold(),
                                     "error: `Love = *****` is invalid expression".blue().bold(),
                                     ">>>".yellow(),
                             ).into()
                         ), Duration::from_millis(800), true),
        ),
        DisplayInfo::new(3, 3, 646+offset_millis,
                         print_directly(ans.change_text(
                             "LO...VE".bright_purple().bold(),
                         ), true)
        ),
        DisplayInfo::new(3, 8, 483+offset_millis,
                         print_one_by_one(ans.change_text(
                             "Though you are free"
                         ).change_style(
                             format!("{} {} {}",
                                     "\u{ef72}".green().bold(),
                                     "error: Cannot find object named `you`".blue().bold(),
                                     ">>>".yellow(),
                             ).into()
                         ), Duration::from_millis(800), true),
        ),
        DisplayInfo::new(3, 9, 746+offset_millis,
                         print_one_by_one(ans.change_text(
                             "I am trapped"
                         ).change_style(
                             format!("{} {} {}",
                                     "\u{ef72}".bold(),
                                     "error: Cannot find object `my.love()`".bold(),
                                     ">>>".yellow(),
                             ).into()
                         ), Duration::from_millis(800), true),
        ),
        DisplayInfo::new(3, 10, 801+offset_millis,
                         print_one_by_one(ans.change_text(
                             "Trapped in"
                         ).change_style(
                             format!("{} {} {}",
                                     "\u{ef72}".bold(),
                                     "error: LOST YOU FOREVER".red().bold(),
                                     ">>>".yellow(),
                             ).into()
                         ), Duration::from_millis(800), true),
        ),
        DisplayInfo::new(3, 11, 356+offset_millis,
                         print_directly(ans.change_text(
                             "LO...VE".bright_purple().bold(),
                         ), true)
        ),
        DisplayInfo::new(3, 25, 811+offset_millis,
            print_one_by_one(ans.change_text(
                "EXECUTION".red().bold(),
            ).change_style(
                format!("{} {} {}",
                    "\u{ef72}".bold(),
                    "executing: world.execute(me)",
                    ">>>".yellow(),
                ).into()
            ), Duration::from_millis(800), true),
        ),
        DisplayInfo::new(3, 25, 980+offset_millis,
                         print_one_by_one(cmd.change_text(
                             "Program exited. Return code is -1".bold(),
                         ), Duration::from_millis(800), true),
        ),
    ]);

    let audio_t = thread::spawn(move || {
        match audio_player.play() {
            Ok(_) => { loop {} },
            Err(e) => {eprintln!("{}", e)}
        }
    });

    let display_t = thread::spawn(move || {
        loop {
            displayer.update(status.clone());
        }
    });

    audio_t.join().unwrap();
    display_t.join().unwrap();

    loop {}
}
