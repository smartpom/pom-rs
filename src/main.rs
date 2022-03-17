#[macro_use] extern crate macro_machine;
declare_machine!(
    PomMachine(idle {counter: 0}) // Name and initial state with initial value
    states[idle, challenger, target, challenge, waitness] // List of states
    commands[Next] // List of commands
    (idle context{counter: i16}: // State node and this state context description with name binding
        >> { // Executed on state idle enter
            println!("Enter idle: {:?}", context);
            context.counter = context.counter + 1;
        }
        << { // Executed on state idle leave
            println!("Leave idle: {:?}", context);
            context.counter = context.counter + 1;
        }
        Next {
            println!("Next in idle: {:?}", context);
            context.counter = context.counter + 1;
        } => challenger {counter: context.counter}; // Command Reaction. Now on command Next we add 1 to our context. Also we change state to challenger and init it with our counter value.
    )
    (challenger context{counter: i16}: // State node and this state context description with name binding
        >> { // Executed on state A enter
            println!("Enter challenger: {:?}", context);
            context.counter = context.counter + 1;
        }
        << { // Executed on state A leave
            println!("Leave challenger: {:?}", context);
            context.counter = context.counter + 1;
        }
        Next {
            println!("Next in challenger: {:?}", context);
            context.counter = context.counter + 1;
        } => target {counter: context.counter}; // Command Reaction. Now on command Next we add 1 to our context. Also we change state to target and init it with our counter value.
    )
    (target context{counter: i16}:
        >> {
            println!("Enter target: {:?}", context);
            context.counter = context.counter + 1;
        }
        << {
            println!("Leave target: {:?}", context);
            context.counter = context.counter + 1;
        }
        Next {
            println!("Next in target: {:?}", context);
            context.counter = context.counter + 1;
        } => challenge {counter: context.counter};
    )
    (challenge context{counter: i16}:
        >> {
            println!("Enter challenge: {:?}", context);
            context.counter = context.counter + 1;
        }
        << {
            println!("Leave challenge: {:?}", context);
            context.counter = context.counter + 1;
        }
        Next {
            println!("Next in challenge: {:?}", context);
            context.counter = context.counter + 1;
        } => waitness {counter: context.counter};
    )
    (waitness context{counter: i16}:
        >> {
            println!("Enter waitness: {:?}", context);
            context.counter = context.counter + 1;
        }
        << {
            println!("Leave waitness: {:?}", context);
            context.counter = context.counter + 1;
        }
        Next {
            println!("Next in waitness: {:?}", context);
            context.counter = context.counter + 1;
        } => idle {counter: context.counter};
    )
);

fn main() {
    use PomMachine::*;
    let mut machine = PomMachine::new();
    machine.execute(&PomMachine::Commands::Next).unwrap();
    machine.execute(&PomMachine::Commands::Next).unwrap();
    machine.execute(&PomMachine::Commands::Next).unwrap();
    machine.execute(&PomMachine::Commands::Next).unwrap();
    machine.execute(&PomMachine::Commands::Next).unwrap();
}
