struct Attendee {
    attendee_id: u32,
    num_events: u32,
    name: String,
}

impl Drop for Attendee {
    fn drop(&mut self) {
        println!("{} DROPPED!!", self.name)
    }
}

fn print_greeting(attendee: &Attendee) {
    println!("Hello {}! Welcome to meetup!", attendee.name);
}

fn update_num_events(attendee: &mut Attendee) {
    attendee.num_events += 1;
}

fn return_best_attendee(attendee_one: &Attendee, attendee_two: &Attendee) -> &Attendee {
    if attendee_one.num_events > attendee_two.num_events {
        return attendee_one;
    }

    return attendee_two;
}

//  Solve by specifying lifetimes in the correct places :)
fn main() {
    let mut attendee_anders = Attendee {
        attendee_id: 1,
        num_events: 2,
        name: String::from("Anders"),
    };

    let mut attendee_christian = Attendee {
        attendee_id: 2,
        num_events: 3,
        name: String::from("Christian"),
    };

    print_greeting(&attendee_anders);
    update_num_events(&mut attendee_anders);

    print_greeting(&attendee_christian);
    update_num_events(&mut attendee_christian);

    let best_attendee = return_best_attendee(&attendee_anders, &attendee_christian);
    println!("{} is best!", best_attendee.name);
}
