/// this file has engine functionalities
/// plugs with:
///  json.rs
///  kahn.rs
///  logs.rs
///  process.rs
///  task_manager.rs
///

use super::json::EngineTemplate;

pub type LogingError = fn (msg: String);
pub type LogingWarning = fn (msg: String);
pub type LogingNote = fn (msg: String);
pub type Quit = fn ();

pub trait Spec {
    fn init(template: &str) -> Self;
    fn run(self, error: LogingError,
        warning: LogingWarning,
        note: LogingNote,
        quit: Quit
        );
}

pub struct Engine {
    pub template: EngineTemplate,
}

impl Engine {
    pub fn new(template: &str) -> Self {
        Self { 
            template : EngineTemplate::new(template),
        }
    }
}

impl Spec for Engine {
    fn init(template: &str) -> Self {
        Engine::new(template)
    }

    fn run(self, error: LogingError,
        _warning: LogingWarning,
        _note: LogingNote,
        quit: Quit
    ) {
        match self.template.get_error() {
            Some(err) => {
                error(err);
                quit()
            }
            _ => (),
        }

        let _model = self.template.parse(); //  this is the module that will be used in
                                                   //  kahn's algorithm and start launching processes
    }

}
