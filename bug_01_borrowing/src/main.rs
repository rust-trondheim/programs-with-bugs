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

fn print_greeting(attendee: Attendee) {
    println!("Hello {}! Welcome to meetup!", attendee.attendee_id);
}

fn update_num_events(attendee: Attendee) {
    attendee.num_events += 1;
}

// Tenkt å løses vha keywords, ikke refakturering ;)
fn main() {
    let attendee = Attendee {
        attendee_id: 1,
        num_events: 0,
        name: String::from("Anders"),
    };

    print_greeting(attendee);
    update_num_events(attendee);
}
