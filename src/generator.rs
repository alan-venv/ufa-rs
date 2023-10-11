use chrono::{Datelike, Local, Timelike};
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub struct Others {}

impl Others {
    pub fn guid() -> String {
        return Uuid::new_v4().to_string();
    }

    pub fn number(min: i64, max: i64) -> String {
        let mut rng = rand::thread_rng();
        let number = rng.gen_range(min..max + 1);
        return format!("{}", number);
    }

    pub fn digits(size: i128) -> String {
        let mut rng = rand::thread_rng();
        let mut min: i128 = 1;
        let mut max: i128 = 10;
        let mut i = 1;
        while i < size && i < 38 {
            min = min * 10;
            max = max * 10;
            i = i + 1;
        }
        let number = rng.gen_range(min..max);
        return format!("{}", number);
    }
}

pub struct Date {}

impl Date {
    pub fn epoch_as_ms() -> String {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        return since_the_epoch.as_millis().to_string();
    }

    pub fn epoch_as_secs() -> String {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        return since_the_epoch.as_secs().to_string();
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
            "{}-{:02}-{:02}T{:02}:{:02}:{:02}.000",
            year, month, day, hour, minute, seconds
        );
    }
}

pub struct Person {}

impl Person {
    pub fn name() -> String {
        let first_name = vec![
            "Alan", "João", "Fabio", "Emanuel", "Nicolas", "Luiza", "Gabriele",
        ];
        let last_name = vec![
            "Silva", "Sampaio", "Kalinay", "Alvez", "Katte", "Oliveira", "Peixoto",
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
