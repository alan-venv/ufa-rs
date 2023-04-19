use chrono::{Datelike, Local, Timelike};
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub struct Others {}

#[allow(dead_code)]
impl Others {
    pub fn guid() -> String {
        return Uuid::new_v4().to_string();
    }
}

pub struct Date {}

#[allow(dead_code)]
impl Date {
    pub fn epoch_as_ms() -> String {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        return since_the_epoch.as_millis().to_string();
    }

    pub fn iso() -> String {
        let now = Local::now();
        let year = now.year();
        let month = now.month();
        let day = now.day();
        let hour = now.hour();
        let minute = now.minute();
        let seconds = now.second();

        return format!(
            "{}/{:02}/{:02}T{:02}:{:02}:{:02}.000",
            year, month, day, hour, minute, seconds
        );
    }
}

pub struct Document {}

#[allow(dead_code)]
impl Document {
    pub fn name() -> String {
        let first_name = vec![
            "Alan", "João", "Fabio", "Emanuel", "Luiza", "Gabriele", "Anelise",
        ];
        let last_name = vec![
            "Silva", "Sampaio", "Guedes", "Alvez", "Braga", "Meller", "Oliveira",
        ];

        return format!(
            "{} {}",
            first_name[rand::thread_rng().gen_range(0..last_name.len())],
            last_name[rand::thread_rng().gen_range(0..last_name.len())]
        );
    }

    pub fn cpf() -> String {
        let mut initial: Vec<usize>;
        loop {
            initial = vec![
                rand::thread_rng().gen_range(0..10),
                rand::thread_rng().gen_range(0..10),
                rand::thread_rng().gen_range(0..10),
                rand::thread_rng().gen_range(0..10),
                rand::thread_rng().gen_range(0..10),
                rand::thread_rng().gen_range(0..10),
                rand::thread_rng().gen_range(0..10),
                rand::thread_rng().gen_range(0..10),
                rand::thread_rng().gen_range(0..10),
            ];

            let reversed: Vec<usize> = initial.iter().copied().rev().collect();

            if initial != reversed {
                break;
            }
        }

        for i in 9..11 {
            let mut value: Vec<usize> = vec![];
            for num in 0..i {
                value.append(&mut vec![initial[num] * ((i + 1) - num)]);
            }
            let sum: usize = value.iter().sum();
            let digit: usize = ((sum * 10) % 11) % 10;
            initial.append(&mut vec![digit]);
        }

        return initial.iter().map(ToString::to_string).collect();
    }
}
