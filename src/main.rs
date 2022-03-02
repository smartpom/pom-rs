#[macro_use] extern crate macro_machine;
declare_machine!(
    MyMachine(A {counter: 0}) // Name and initial state with initial value
    states[A,B] // List of states
    commands[Next] // List of commands
    (A context{counter: i16}: // State node and this state context description with name binding
        >> { // Executed on state A enter
            println!("Enter A: {:?}", context);
            context.counter = context.counter + 1;
        }
        << { // Executed on state A leave
            println!("Leave A: {:?}", context);
            context.counter = context.counter + 1;
        }
        Next {
            println!("Next in A: {:?}", context);
            context.counter = context.counter + 1;
        } => B {counter: context.counter}; // Command Reaction. Now on command Next we add 1 to our context. Also we change state to B and init it with our counter value.
    )
    (B context{counter: i16}:
        >> {
            println!("Enter B: {:?}", context);
            context.counter = context.counter + 1;
        }
        << {
            println!("Leave B: {:?}", context);
            context.counter = context.counter + 1;
        }
        Next {
            println!("Next in B: {:?}", context);
            context.counter = context.counter + 1;
        } => A {counter: context.counter};
    )
);

fn main() {
    use MyMachine::*;
    let mut machine = MyMachine::new();
    machine.execute(&MyMachine::Commands::Next).unwrap();
    machine.execute(&MyMachine::Commands::Next).unwrap();
}
