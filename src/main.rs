use std::fmt::{Display, Formatter};

use async_trait::async_trait;
use cqrs_es::{DomainEvent, Aggregate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub enum Command {
    CommandA { value: String }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MyEvent {
    CommandAFiredEvent {
        value: String
    }
}

impl DomainEvent for MyEvent {
    fn event_type(&self) -> String {
        let event_type: &str = match self {
            MyEvent::CommandAFiredEvent { .. } => "CommandAFiredEvent",
        };
        event_type.to_string()
    }
    
    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

#[derive(Debug)]
pub struct MyError(String);

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0)
    }
}

impl std::error::Error for MyError {}

impl From<&str> for MyError {
    fn from(message: &str) -> Self {
        MyError(message.to_string())
    }
}

pub struct MyServices;

#[derive(Serialize, Default, Deserialize)]
pub struct MyAggregate {
    pub fieldAggregate: u16
}

#[async_trait]
impl Aggregate for MyAggregate {
    type Command = crate::Command;
    type Event = MyEvent;
    type Error = MyError;
    type Services = MyServices;

    fn aggregate_type() -> String {
        "MyAggregate".to_string()
    }

    async fn handle(
        &self,
        command: Self::Command,
        services: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            Command::CommandA { value } => {
                Ok(vec![Self::Event::CommandAFiredEvent {
                    value
                }])
            }
            _ => Ok(vec![])
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            MyEvent::CommandAFiredEvent { mut value} => {
                let x = value;
                value = String::from("x");
                self.fieldAggregate = 0; 
            }
        }
    }
}

#[cfg(test)]
mod aggregate_tests {
    use super::*;
    use cqrs_es::test::TestFramework;

    type MyTestFramework = TestFramework<MyAggregate>;

    #[test]
    fn test_my_aggregate() {
        let expected = MyEvent::CommandAFiredEvent { value: (String::from("x")) };

        MyTestFramework::with(MyServices)
        .given_no_previous_events()
        .when(Command::CommandA { value: (String::from("x")) })
        .then_expect_events(vec![expected]);
    }
}

fn main() {}