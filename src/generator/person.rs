use rand::Rng;

pub fn name() -> String {
    let first_name = vec![
        "Alan", "João", "Fabio", "Emanuel", "Nicolas", "Luiza", "Gabriele",
    ];
    let last_name = vec![
        "Silva", "Sampaio", "Kalinay", "Alvez", "Katte", "Oliveira", "Guedes",
    ];

    return format!(
        "{} {}",
        first_name[rand::rng().random_range(0..last_name.len())],
        last_name[rand::rng().random_range(0..last_name.len())]
    );
}

pub fn cpf() -> String {
    let mut initial: Vec<usize>;
    loop {
        initial = vec![
            rand::rng().random_range(0..10),
            rand::rng().random_range(0..10),
            rand::rng().random_range(0..10),
            rand::rng().random_range(0..10),
            rand::rng().random_range(0..10),
            rand::rng().random_range(0..10),
            rand::rng().random_range(0..10),
            rand::rng().random_range(0..10),
            rand::rng().random_range(0..10),
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
