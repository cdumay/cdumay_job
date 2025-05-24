use cdumay_core::Error;
use cdumay_error_standard::Unexpected;
use cdumay_job::{OperationExec, TaskExec, define_operation, define_task};
use rand::Rng;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Context {
    env: String,
}

impl Default for Context {
    fn default() -> Self {
        Context { env: "production".into() }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct DiceRollParam {
    launch_number: u8,
}

define_task! {
    DiceRoll { params: DiceRollParam, metadata: Context }
}

impl TaskExec for DiceRoll {
    fn run(&mut self, mut result: cdumay_job::Result) -> Result<cdumay_job::Result, Error> {
        let mut rng = rand::rng();
        let roll: u8 = rng.random_range(1..=7);
        let score: u16 = match roll {
            2 => 2,
            3 => 3,
            4 => 4,
            5 => 5,
            6 => 60,
            7 => return Err(Unexpected::new().with_message("non-regulatory dice!".into()).into()),
            _ => 100,
        };
        let launch_number = self.params().launch_number;
        Ok({
            result.stdout = Some(format!("Roll {launch_number}: you made a {roll}"));
            result
                .retval
                .insert(format!("LaunchNumber-{launch_number}"), serde_value::Value::U8(roll));
            result.retval.insert(format!("Score-{launch_number}"), serde_value::Value::U16(score));
            result
        })
    }
}

define_task! {
    DisplayScore { metadata: Context }
}

impl TaskExec for DisplayScore {
    fn run(&mut self, mut result: cdumay_job::Result) -> Result<cdumay_job::Result, Error> {
        let score = self
            .result
            .retval
            .iter()
            .filter_map(|(k, v)| {
                if k.starts_with("Score-") {
                    match v {
                        serde_value::Value::U16(data) => Some(data.clone()),
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<u16>>();
        Ok({
            result.stdout = Some(format!("Your score is {}", score.iter().sum::<u16>()));
            result
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct GameSetting {
    nb_launch: u8,
}

impl Default for GameSetting {
    fn default() -> Self {
        GameSetting { nb_launch: 3 }
    }
}

define_operation! {
    Zanzibar { params: GameSetting, metadata: Context }
}

impl OperationExec for Zanzibar {
    fn build_tasks(&self) -> Vec<Box<dyn TaskExec>> {
        let mut tasks: Vec<Box<dyn TaskExec>> = vec![];
        for launch in 1..self.params.nb_launch + 1 {
            tasks.push(Box::new(DiceRoll::new(Some(DiceRollParam { launch_number: launch }), Some(self.metadata.clone()))));
        }
        tasks.push(Box::new(DisplayScore::new(None, Some(self.metadata.clone()))));
        tasks
    }
}

fn play(nb_launch: u8) -> Result<cdumay_job::Result, Error> {
    let mut game = Zanzibar::new(Some(GameSetting { nb_launch }), Some(Context { env: "development".into() }));
    game.build()?;
    Ok(game.execute(None))
}

fn main() {
    env_logger::init();
    match play(5) {
        Ok(result) => println!("{}", serde_json::to_string_pretty(&result).unwrap()),
        Err(e) => println!("{}", serde_json::to_string_pretty(&e).unwrap()),
    };
}
