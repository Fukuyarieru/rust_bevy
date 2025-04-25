use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PeoplePlugin)
        .run();
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hello_world)
            .add_systems(Startup, set_up)
            .add_systems(Update, print_name)
            .add_systems(Update, (print_people_with_jobs, print_people_without_jobs))
            .add_systems(Update, person_does_job);
    }
}

pub fn hello_world() {
    println!("Hello world");
}

pub fn print_name(person_query: Query<&Person>) {
    for person in person_query {
        println!("Name: {}", person.name)
    }
}
pub fn print_people_with_jobs(person_query: Query<&Person, With<Employed>>) {
    for person in person_query {
        println!("{} has a job", person.name);
    }
}
pub fn print_people_without_jobs(person_query: Query<&Person, Without<Employed>>) {
    for person in person_query {
        println!("{} doesnt has a job", person.name)
    }
}

pub fn set_up(mut commands: Commands) {
    commands.spawn((
        (Person {
            name: "Ariel".to_string(),
        }),
        Employed { job: Job::Lawyer },
    ));
    commands.spawn((
        Person {
            name: "PzzaZZa".to_string(),
        },
        // Employed {
        //     job: Job::FireFighter,
        // },
    ));
    commands.spawn((
        Person {
            name: "Fukuya".to_string(),
        },
        Employed { job: Job::Doctor },
    ));
}

pub fn person_does_job(person_query: Query<(&Person, &Employed)>) {
    for (person, employed) in person_query {
        println!(
            "{} is a {}",
            person.name,
            match employed.job {
                Job::Doctor => "Doctor".to_string(),
                Job::FireFighter => "Firefighter".to_string(),
                Job::Lawyer => "Lawyer".to_string(),
            }
        )
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String,
}

#[derive(Component)]
pub struct Employed {
    job: Job,
}

#[derive(Debug)]
pub enum Job {
    Doctor,
    FireFighter,
    Lawyer,
}
