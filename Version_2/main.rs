use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;

struct Pet {
    pet_name: String,
    intense: i32,
    heat: i32,
    noise: i32,
    sweet: i32,
    pretty: i32,
    damp: i32,
    colour: i32,
}

struct Health {
    growth: i32,
    growth_speed: i32,
    pet_health: i32,
    health_i: i32,
    health_h: i32,
    health_n: i32,
    health_s: i32,
    health_p: i32,
    health_d: i32,
    health_c: i32,
    fave_i: bool,
    fave_h: bool,
    fave_n: bool,
    fave_s: bool,
    fave_p: bool,
    fave_d: bool,
    fave_c: bool,
}

struct Event {
    speak: String,
    attchange1: u8,
    attchange2: u8,
    attchange3: u8,
}

fn main() {
    //--------Egg hatching phase-----------------v
    let mut eggvar: i32 = 0;
    print!("\x1B[2J");
    println!("\nI found this egg, would you help me look after it?\n");
    println!("   __\n  /  \\\n |    |\n  \\__/\n");
    thread::sleep(Duration::from_millis(1500));
    println!("Type '1' Sing to egg, Type '2' Give the egg peace and quiet\n");
    thread::sleep(Duration::from_millis(1500));
    egg_checker(&mut eggvar, 1);
    thread::sleep(Duration::from_millis(1500));
    println!("'1' Wrap the egg up nice and warm, '2' Leave it undisturbed\n");
    thread::sleep(Duration::from_millis(1500));
    egg_checker(&mut eggvar, 10);
    thread::sleep(Duration::from_millis(1500));
    println!("'1' Mist the egg with water, '2' Wipe the egg dry\n");
    thread::sleep(Duration::from_millis(1500));
    egg_checker(&mut eggvar, 100);
    thread::sleep(Duration::from_millis(1500));
    print!("\x1B[2J");
    println!("\nIt hatched!");
    thread::sleep(Duration::from_millis(1500));
    print!("\x1B[2J");
    //----------Egg hatched, defining pet phase--------v

    let mut health_data = Health {
        growth: 0,
        growth_speed: 0,
        pet_health: 0,
        health_i: 0,
        health_h: 0,
        health_n: 0,
        health_s: 0,
        health_p: 0,
        health_d: 0,
        health_c: 0,
        fave_i: false,
        fave_h: false,
        fave_n: false,
        fave_s: false,
        fave_p: false,
        fave_d: false,
        fave_c: false,
    };
    let mut hatch_pet = match eggvar {
        0 => Pet {
            pet_name: "Loork".to_string(),
            intense: 170,
            heat: 190,
            noise: 190,
            sweet: 170,
            pretty: 170,
            damp: 170,
            colour: 210,
        },

        1 => Pet {
            pet_name: "Kring".to_string(),
            intense: 110,
            heat: 130,
            noise: 90,
            sweet: 130,
            pretty: 130,
            damp: 190,
            colour: 170,
        },

        10 => Pet {
            pet_name: "Spreek".to_string(),
            intense: 170,
            heat: 130,
            noise: 170,
            sweet: 110,
            pretty: 90,
            damp: 170,
            colour: 110,
        },

        11 => Pet {
            pet_name: "Snuggler".to_string(),
            intense: 190,
            heat: 110,
            noise: 130,
            sweet: 90,
            pretty: 130,
            damp: 170,
            colour: 130,
        },

        100 => Pet {
            pet_name: "Golbert".to_string(),
            intense: 110,
            heat: 170,
            noise: 190,
            sweet: 170,
            pretty: 210,
            damp: 130,
            colour: 170,
        },

        101 => Pet {
            pet_name: "Frimpi".to_string(),
            intense: 130,
            heat: 190,
            noise: 130,
            sweet: 130,
            pretty: 130,
            damp: 110,
            colour: 90,
        },

        110 => Pet {
            pet_name: "Breel".to_string(),
            intense: 130,
            heat: 130,
            noise: 170,
            sweet: 190,
            pretty: 170,
            damp: 90,
            colour: 190,
        },

        111 => Pet {
            pet_name: "Dragoo".to_string(),
            intense: 90,
            heat: 110,
            noise: 110,
            sweet: 170,
            pretty: 170,
            damp: 130,
            colour: 130,
        },

        _ => Pet {
            pet_name: "No pet!".to_string(),
            intense: 0,
            heat: 0,
            noise: 0,
            sweet: 0,
            pretty: 0,
            damp: 0,
            colour: 0,
        },
    };

    health_data.fave_i = match eggvar {
        0 => false,

        1 => true,

        10 => false,

        11 => true,

        100 => true,

        101 => false,

        110 => false,

        111 => true,

        _ => false,
    };

    health_data.fave_h = match eggvar {
        0 => true,

        1 => false,

        10 => false,

        11 => true,

        100 => false,

        101 => true,

        110 => false,

        111 => true,

        _ => false,
    };

    health_data.fave_n = match eggvar {
        0 => true,

        1 => true,

        10 => false,

        11 => false,

        100 => true,

        101 => false,

        110 => false,

        111 => true,

        _ => false,
    };

    health_data.fave_s = match eggvar {
        0 => false,

        1 => false,

        10 => true,

        11 => true,

        100 => false,

        101 => false,

        110 => true,

        111 => false,

        _ => false,
    };

    health_data.fave_p = match eggvar {
        0 => false,

        1 => false,

        10 => true,

        11 => false,

        100 => true,

        101 => false,

        110 => false,

        111 => false,

        _ => false,
    };

    health_data.fave_d = match eggvar {
        0 => false,

        1 => true,

        10 => false,

        11 => false,

        100 => false,

        101 => true,

        110 => true,

        111 => false,

        _ => false,
    };

    health_data.fave_c = match eggvar {
        0 => true,

        1 => false,

        10 => true,

        11 => false,

        100 => false,

        101 => true,

        110 => true,

        111 => false,

        _ => false,
    };

    println!("\n  >o o<\n    ^ \n    v\n");
    thread::sleep(Duration::from_millis(1500));
    println!("\nYou got {}!\n", hatch_pet.pet_name);
    thread::sleep(Duration::from_millis(1500));
    println!(
        "What a wonderful pet, let's look after it.\nWe should try and understand its needs.\n"
    );
    thread::sleep(Duration::from_millis(3500));
    print!("\x1B[2J");
    //---------Main Phase, looking after pet---------v

    //Beginning of Food Phase

    let mut combo = Event {
        speak: "".to_string(),
        attchange1: 1,
        attchange2: 1,
        attchange3: 1,
    };
    loop {
        println!("\n~~~It's time for some Food!~~~");
    thread::sleep(Duration::from_millis(2500));
        println!("\nHere's {}'s state right now", hatch_pet.pet_name);
    thread::sleep(Duration::from_millis(1500));
        happiness_h(&hatch_pet);
    thread::sleep(Duration::from_millis(500));
        happiness_s(&hatch_pet);
    thread::sleep(Duration::from_millis(500));
        happiness_d(&hatch_pet);
    thread::sleep(Duration::from_millis(1500));

        //Add random number generator to assign value to variable to determin combos
        let rngcombo = rand::thread_rng().gen_range(1..=8);
        //Heat +(3),-(4), Sweet +(7), -(8), Damp +(11), -(12)
        let combo1 = match rngcombo {
            1 => Event {
                speak: "Sweeties".to_string(),
                attchange1: 4,
                attchange2: 7,
                attchange3: 12,
            },
            2 => Event {
                speak: "Baked Potato".to_string(),
                attchange1: 3,
                attchange2: 8,
                attchange3: 12,
            },
            3 => Event {
                speak: "Water".to_string(),
                attchange1: 4,
                attchange2: 8,
                attchange3: 11,
            },
            4 => Event {
                speak: "Bread".to_string(),
                attchange1: 4,
                attchange2: 8,
                attchange3: 12,
            },
            5 => Event {
                speak: "Soup".to_string(),
                attchange1: 3,
                attchange2: 8,
                attchange3: 11,
            },
            6 => Event {
                speak: "Fruit Juice".to_string(),
                attchange1: 4,
                attchange2: 7,
                attchange3: 11,
            },
            7 => Event {
                speak: "Hot Cross Bun".to_string(),
                attchange1: 3,
                attchange2: 7,
                attchange3: 12,
            },
            8 => Event {
                speak: "Hot Chocolate".to_string(),
                attchange1: 3,
                attchange2: 7,
                attchange3: 11,
            },

            _ => Event {
                speak: "Error".to_string(),
                attchange1: 1,
                attchange2: 1,
                attchange3: 1,
            },
        };

        let combo2 = match rngcombo {
            1 => Event {
                speak: "Soup".to_string(),
                attchange1: 3,
                attchange2: 8,
                attchange3: 11,
            },
            2 => Event {
                speak: "Fruit Juice".to_string(),
                attchange1: 4,
                attchange2: 7,
                attchange3: 11,
            },
            3 => Event {
                speak: "Hot Cross Bun".to_string(),
                attchange1: 3,
                attchange2: 7,
                attchange3: 12,
            },
            4 => Event {
                speak: "Hot Chocolate".to_string(),
                attchange1: 3,
                attchange2: 7,
                attchange3: 11,
            },
            5 => Event {
                speak: "Sweeties".to_string(),
                attchange1: 4,
                attchange2: 7,
                attchange3: 12,
            },
            6 => Event {
                speak: "Baked Potato".to_string(),
                attchange1: 3,
                attchange2: 8,
                attchange3: 12,
            },
            7 => Event {
                speak: "Water".to_string(),
                attchange1: 4,
                attchange2: 8,
                attchange3: 11,
            },
            8 => Event {
                speak: "Bread".to_string(),
                attchange1: 4,
                attchange2: 8,
                attchange3: 12,
            },

            _ => Event {
                speak: "Error".to_string(),
                attchange1: 1,
                attchange2: 1,
                attchange3: 1,
            },
        };

        let combo3 = match rngcombo {
            1 => Event {
                speak: "Water".to_string(),
                attchange1: 4,
                attchange2: 8,
                attchange3: 11,
            },
            2 => Event {
                speak: "Bread".to_string(),
                attchange1: 4,
                attchange2: 8,
                attchange3: 12,
            },
            3 => Event {
                speak: "Soup".to_string(),
                attchange1: 3,
                attchange2: 8,
                attchange3: 11,
            },
            4 => Event {
                speak: "Fruit Juice".to_string(),
                attchange1: 4,
                attchange2: 7,
                attchange3: 11,
            },
            5 => Event {
                speak: "Hot Cross Bun".to_string(),
                attchange1: 3,
                attchange2: 7,
                attchange3: 12,
            },
            6 => Event {
                speak: "Hot Chocolate".to_string(),
                attchange1: 3,
                attchange2: 7,
                attchange3: 11,
            },
            7 => Event {
                speak: "Sweeties".to_string(),
                attchange1: 4,
                attchange2: 7,
                attchange3: 12,
            },
            8 => Event {
                speak: "Baked Potato".to_string(),
                attchange1: 3,
                attchange2: 8,
                attchange3: 12,
            },

            _ => Event {
                speak: "Error".to_string(),
                attchange1: 1,
                attchange2: 1,
                attchange3: 1,
            },
        };

    thread::sleep(Duration::from_millis(500));
        println!(
            "\nWhat do you want to feed {}? \nType 1 for {}, 2 for {}, 3 for {}\n",
            hatch_pet.pet_name, combo1.speak, combo2.speak, combo3.speak
        );

        combo_changer(&mut combo, combo1, combo2, combo3);

        attribute_changer(&mut hatch_pet, &combo);
    thread::sleep(Duration::from_millis(1500));
        println!("{} gobbles up the {}\n", hatch_pet.pet_name, combo.speak);
    thread::sleep(Duration::from_millis(500));
        println!(
            "This is how {} is doing after their meal",
            hatch_pet.pet_name
        );
    thread::sleep(Duration::from_millis(1500));
        happiness_h(&hatch_pet);
    thread::sleep(Duration::from_millis(500));
        happiness_s(&hatch_pet);
    thread::sleep(Duration::from_millis(500));
        happiness_d(&hatch_pet);
    thread::sleep(Duration::from_millis(3500));
        match hatch_pet.heat {
            1..=50 => {
                println!(
                    "Oh no, poor {}, the stress of it has turned them back into an egg.",
                    hatch_pet.pet_name
                );
                break;
            }
            251..=300 => {
                println!(
                    "Oh no, poor {}, the stress of it has turned them back into an egg.",
                    hatch_pet.pet_name
                );
                break;
            }
            _ => print!(""),
        };
        match hatch_pet.sweet {
            1..=50 => {
                println!(
                    "Oh no, poor {}, the stress of it has turned them back into an egg.",
                    hatch_pet.pet_name
                );
                break;
            }
            251..=300 => {
                println!(
                    "Oh no, poor {}, the stress of it has turned them back into an egg.",
                    hatch_pet.pet_name
                );
                break;
            }
            _ => print!(""),
        };
        match hatch_pet.damp {
            1..=50 => {
                println!(
                    "Oh no, poor {}, the stress of it has turned them back into an egg.",
                    hatch_pet.pet_name
                );
                break;
            }
            251..=300 => {
                println!(
                    "Oh no, poor {}, the stress of it has turned them back into an egg.",
                    hatch_pet.pet_name
                );
                break;
            }
            _ => print!(""),
        };

        //End of food phase

        //Start of Exercise Phase

        let mut combo = Event {
            speak: "".to_string(),
            attchange1: 1,
            attchange2: 1,
            attchange3: 1,
        };

    print!("\x1B[2J");
        println!("\n~~~It's time to do some Exercise!~~~");
    thread::sleep(Duration::from_millis(2500));
        println!("\nHere's {}'s state right now", hatch_pet.pet_name);
    thread::sleep(Duration::from_millis(1500));
        happiness_i(&hatch_pet);
    thread::sleep(Duration::from_millis(500));
        happiness_p(&hatch_pet);
    thread::sleep(Duration::from_millis(500));
        happiness_d(&hatch_pet);
    thread::sleep(Duration::from_millis(500));

        let rngcombo = rand::thread_rng().gen_range(1..=8);
        //Intense +(1),-(2), Pretty +(9), -(10), Damp +(11), -(12)
        let combo1 = match rngcombo {
            1 => Event {
                speak: "Lifting weights in the Gym".to_string(),
                attchange1: 1,
                attchange2: 10,
                attchange3: 12,
            },
            2 => Event {
                speak: "Walking in Nature".to_string(),
                attchange1: 2,
                attchange2: 9,
                attchange3: 12,
            },
            3 => Event {
                speak: "Gentle Swimming Pool laps".to_string(),
                attchange1: 2,
                attchange2: 10,
                attchange3: 11,
            },
            4 => Event {
                speak: "Gentle Stretching".to_string(),
                attchange1: 2,
                attchange2: 10,
                attchange3: 12,
            },
            5 => Event {
                speak: "Gentle Swimming in a Lake".to_string(),
                attchange1: 2,
                attchange2: 9,
                attchange3: 11,
            },
            6 => Event {
                speak: "Fast Swimming Pool laps".to_string(),
                attchange1: 1,
                attchange2: 10,
                attchange3: 11,
            },
            7 => Event {
                speak: "Sprinting in Nature".to_string(),
                attchange1: 1,
                attchange2: 9,
                attchange3: 12,
            },
            8 => Event {
                speak: "Swimming against the current of the River".to_string(),
                attchange1: 1,
                attchange2: 9,
                attchange3: 11,
            },

            _ => Event {
                speak: "Error".to_string(),
                attchange1: 1,
                attchange2: 1,
                attchange3: 1,
            },
        };

        let combo2 = match rngcombo {
            1 => Event {
                speak: "Gentle Swimming in a Lake".to_string(),
                attchange1: 2,
                attchange2: 9,
                attchange3: 11,
            },
            2 => Event {
                speak: "Fast Swimming Pool laps".to_string(),
                attchange1: 1,
                attchange2: 10,
                attchange3: 11,
            },
            3 => Event {
                speak: "Sprinting in Nature".to_string(),
                attchange1: 1,
                attchange2: 9,
                attchange3: 12,
            },
            4 => Event {
                speak: "Swimming against the current of the River".to_string(),
                attchange1: 1,
                attchange2: 9,
                attchange3: 11,
            },
            5 => Event {
                speak: "Lifting weights in the Gym".to_string(),
                attchange1: 1,
                attchange2: 10,
                attchange3: 12,
            },
            6 => Event {
                speak: "Walking in Nature".to_string(),
                attchange1: 2,
                attchange2: 9,
                attchange3: 12,
            },
            7 => Event {
                speak: "Gentle Swimming Pool laps".to_string(),
                attchange1: 2,
                attchange2: 10,
                attchange3: 11,
            },
            8 => Event {
                speak: "Gentle Stretching".to_string(),
                attchange1: 2,
                attchange2: 10,
                attchange3: 12,
            },

            _ => Event {
                speak: "Error".to_string(),
                attchange1: 1,
                attchange2: 1,
                attchange3: 1,
            },
        };

        let combo3 = match rngcombo {
            1 => Event {
                speak: "Sprinting in Nature".to_string(),
                attchange1: 1,
                attchange2: 9,
                attchange3: 12,
            },
            2 => Event {
                speak: "Swimming against the current of the River".to_string(),
                attchange1: 1,
                attchange2: 9,
                attchange3: 11,
            },
            3 => Event {
                speak: "Lifting weights in the Gym".to_string(),
                attchange1: 1,
                attchange2: 10,
                attchange3: 12,
            },
            4 => Event {
                speak: "Walking in Nature".to_string(),
                attchange1: 2,
                attchange2: 9,
                attchange3: 12,
            },
            5 => Event {
                speak: "Gentle Swimming Pool laps".to_string(),
                attchange1: 2,
                attchange2: 10,
                attchange3: 11,
            },
            6 => Event {
                speak: "Gentle Stretching".to_string(),
                attchange1: 2,
                attchange2: 10,
                attchange3: 12,
            },
            7 => Event {
                speak: "Gentle Swimming in a Lake".to_string(),
                attchange1: 2,
                attchange2: 9,
                attchange3: 11,
            },
            8 => Event {
                speak: "Fast Swimming Pool laps".to_string(),
                attchange1: 1,
                attchange2: 10,
                attchange3: 11,
            },

            _ => Event {
                speak: "Error".to_string(),
                attchange1: 1,
                attchange2: 1,
                attchange3: 1,
            },
        };

        println!(
            "\nWhat exercise should {} do? \nType 1 for {}, 2 for {}, 3 for {}\n",
            hatch_pet.pet_name, combo1.speak, combo2.speak, combo3.speak
        );

        combo_changer(&mut combo, combo1, combo2, combo3);

        attribute_changer(&mut hatch_pet, &combo);
    thread::sleep(Duration::from_millis(1500));
        println!(
            "{} gets some exercise doing {}\n",
            hatch_pet.pet_name, combo.speak
        );
    thread::sleep(Duration::from_millis(1500));
        println!(
            "This is how {} is doing after their exercise",
            hatch_pet.pet_name
        );
    thread::sleep(Duration::from_millis(1500));
        happiness_i(&hatch_pet);
    thread::sleep(Duration::from_millis(500));
        happiness_p(&hatch_pet);
    thread::sleep(Duration::from_millis(500));
        happiness_d(&hatch_pet);
    thread::sleep(Duration::from_millis(3500));
        match hatch_pet.intense {
            1..=50 => {
                println!(
                    "Oh no, poor {}, the stress of it has turned them back into an egg.",
                    hatch_pet.pet_name
                );
                break;
            }
            251..=300 => {
                println!(
                    "Oh no, poor {}, the stress of it has turned them back into an egg.",
                    hatch_pet.pet_name
                );
                break;
            }
            _ => print!(""),
        };
        match hatch_pet.pretty {
            1..=50 => {
                println!(
                    "Oh no, poor {}, the stress of it has turned them back into an egg.",
                    hatch_pet.pet_name
                );
                break;
            }
            251..=300 => {
                println!(
                    "Oh no, poor {}, the stress of it has turned them back into an egg.",
                    hatch_pet.pet_name
                );
                break;
            }
            _ => print!(""),
        };
        match hatch_pet.damp {
            1..=50 => {
                println!(
                    "Oh no, poor {}, the stress of it has turned them back into an egg.",
                    hatch_pet.pet_name
                );
                break;
            }
            251..=300 => {
                println!(
                    "Oh no, poor {}, the stress of it has turned them back into an egg.",
                    hatch_pet.pet_name
                );
                break;
            }
            _ => print!(""),
        };

        //End of Exercise Phase

        //Start of Gardening Phase

        let mut combo = Event {
            speak: "".to_string(),
            attchange1: 1,
            attchange2: 1,
            attchange3: 1,
        };

    print!("\x1B[2J");
        println!("\n~~~It's time to do some Gardening!~~~");
    thread::sleep(Duration::from_millis(2500));
        println!("\nHere's {}'s state right now", hatch_pet.pet_name);
    thread::sleep(Duration::from_millis(1500));
        happiness_s(&hatch_pet);
    thread::sleep(Duration::from_millis(500));
        happiness_p(&hatch_pet);
    thread::sleep(Duration::from_millis(500));
        happiness_c(&hatch_pet);
    thread::sleep(Duration::from_millis(1500));

        let rngcombo = rand::thread_rng().gen_range(1..=8);
        //Sweet +(7),-(8), Pretty +(9), -(10), Colour +(13), -(14)
        let combo1 = match rngcombo {
            1 => Event {
                speak: "Pick some Blackberries".to_string(),
                attchange1: 7,
                attchange2: 10,
                attchange3: 14,
            },
            2 => Event {
                speak: "Plant some White Lillies".to_string(),
                attchange1: 8,
                attchange2: 9,
                attchange3: 14,
            },
            3 => Event {
                speak: "Place a colourful Gnome in the Garden".to_string(),
                attchange1: 8,
                attchange2: 10,
                attchange3: 13,
            },
            4 => Event {
                speak: "Plant a row of Cabbages".to_string(),
                attchange1: 8,
                attchange2: 10,
                attchange3: 14,
            },
            5 => Event {
                speak: "Plant some Tulips".to_string(),
                attchange1: 8,
                attchange2: 9,
                attchange3: 13,
            },
            6 => Event {
                speak: "Pick some Red Apples".to_string(),
                attchange1: 7,
                attchange2: 10,
                attchange3: 13,
            },
            7 => Event {
                speak: "Place a beautiful Beehive in the Garden".to_string(),
                attchange1: 7,
                attchange2: 9,
                attchange3: 14,
            },
            8 => Event {
                speak: "Pick from the beautiful Peach Tree".to_string(),
                attchange1: 7,
                attchange2: 9,
                attchange3: 13,
            },

            _ => Event {
                speak: "Error".to_string(),
                attchange1: 1,
                attchange2: 1,
                attchange3: 1,
            },
        };

        let combo2 = match rngcombo {
            1 => Event {
                speak: "Plant some Tulips".to_string(),
                attchange1: 8,
                attchange2: 9,
                attchange3: 13,
            },
            2 => Event {
                speak: "Pick some Red Apples".to_string(),
                attchange1: 7,
                attchange2: 10,
                attchange3: 13,
            },
            3 => Event {
                speak: "Place a beautiful Beehive in the Garden".to_string(),
                attchange1: 7,
                attchange2: 9,
                attchange3: 14,
            },
            4 => Event {
                speak: "Pick from the beautiful Peach Tree".to_string(),
                attchange1: 7,
                attchange2: 9,
                attchange3: 13,
            },
            5 => Event {
                speak: "Pick some Blackberries".to_string(),
                attchange1: 7,
                attchange2: 10,
                attchange3: 14,
            },
            6 => Event {
                speak: "Plant some White Lillies".to_string(),
                attchange1: 8,
                attchange2: 9,
                attchange3: 14,
            },
            7 => Event {
                speak: "Place a colourful Gnome in the Garden".to_string(),
                attchange1: 8,
                attchange2: 10,
                attchange3: 13,
            },
            8 => Event {
                speak: "Plant a row of Cabbages".to_string(),
                attchange1: 8,
                attchange2: 10,
                attchange3: 14,
            },

            _ => Event {
                speak: "Error".to_string(),
                attchange1: 1,
                attchange2: 1,
                attchange3: 1,
            },
        };

        let combo3 = match rngcombo {
            1 => Event {
                speak: "Place a beautiful Beehive".to_string(),
                attchange1: 7,
                attchange2: 9,
                attchange3: 14,
            },
            2 => Event {
                speak: "Pick from the beautiful Peach Tree".to_string(),
                attchange1: 7,
                attchange2: 9,
                attchange3: 13,
            },
            3 => Event {
                speak: "Pick some Blackberries".to_string(),
                attchange1: 7,
                attchange2: 10,
                attchange3: 14,
            },
            4 => Event {
                speak: "Plant some White Lillies".to_string(),
                attchange1: 8,
                attchange2: 9,
                attchange3: 14,
            },
            5 => Event {
                speak: "Place a colourful Gnome".to_string(),
                attchange1: 8,
                attchange2: 10,
                attchange3: 13,
            },
            6 => Event {
                speak: "Plant a row of Cabbages".to_string(),
                attchange1: 8,
                attchange2: 10,
                attchange3: 14,
            },
            7 => Event {
                speak: "Plant some Tulips".to_string(),
                attchange1: 8,
                attchange2: 9,
                attchange3: 13,
            },
            8 => Event {
                speak: "Pick some Red Apples".to_string(),
                attchange1: 7,
                attchange2: 10,
                attchange3: 13,
            },

            _ => Event {
                speak: "Error".to_string(),
                attchange1: 1,
                attchange2: 1,
                attchange3: 1,
            },
        };

        println!(
            "\nWhat should {} do in the Garden? \nType 1 for {}, 2 for {}, 3 for {}\n",
            hatch_pet.pet_name, combo1.speak, combo2.speak, combo3.speak
        );

        combo_changer(&mut combo, combo1, combo2, combo3);

        attribute_changer(&mut hatch_pet, &combo);
    thread::sleep(Duration::from_millis(1500));
        println!(
            "{} did this {} in the Garden\n",
            hatch_pet.pet_name, combo.speak
        );
    thread::sleep(Duration::from_millis(1500));
        println!(
            "This is how {} is doing after their Gardening",
            hatch_pet.pet_name
        );
    thread::sleep(Duration::from_millis(1500));
        happiness_s(&hatch_pet);
    thread::sleep(Duration::from_millis(500));
        happiness_p(&hatch_pet);
    thread::sleep(Duration::from_millis(500));
        happiness_c(&hatch_pet);
    thread::sleep(Duration::from_millis(3500));
        match hatch_pet.sweet {
            1..=50 => {
                println!(
                    "Oh no, poor {}, the stress of it has turned them back into an egg.",
                    hatch_pet.pet_name
                );
                break;
            }
            251..=300 => {
                println!(
                    "Oh no, poor {}, the stress of it has turned them back into an egg.",
                    hatch_pet.pet_name
                );
                break;
            }
            _ => print!(""),
        };
        match hatch_pet.pretty {
            1..=50 => {
                println!(
                    "Oh no, poor {}, the stress of it has turned them back into an egg.",
                    hatch_pet.pet_name
                );
                break;
            }
            251..=300 => {
                println!(
                    "Oh no, poor {}, the stress of it has turned them back into an egg.",
                    hatch_pet.pet_name
                );
                break;
            }
            _ => print!(""),
        };
        match hatch_pet.colour {
            1..=50 => {
                println!(
                    "Oh no, poor {}, the stress of it has turned them back into an egg.",
                    hatch_pet.pet_name
                );
                break;
            }
            251..=300 => {
                println!(
                    "Oh no, poor {}, the stress of it has turned them back into an egg.",
                    hatch_pet.pet_name
                );
                break;
            }
            _ => print!(""),
        };

        //End of Gardening Phase

        //Start of Activity Phase

        let mut combo = Event {
            speak: "".to_string(),
            attchange1: 1,
            attchange2: 1,
            attchange3: 1,
        };

    print!("\x1B[2J");
        println!("\n~~~It's time to do an Activity!~~~");
    thread::sleep(Duration::from_millis(2500));
        println!("\nHere's {}'s state right now", hatch_pet.pet_name);
    thread::sleep(Duration::from_millis(1500));
        happiness_i(&hatch_pet);
    thread::sleep(Duration::from_millis(500));
        happiness_n(&hatch_pet);
    thread::sleep(Duration::from_millis(500));
        happiness_c(&hatch_pet);
    thread::sleep(Duration::from_millis(1500));

        let rngcombo = rand::thread_rng().gen_range(1..=8);
        //Intense +(1),-(2), Noise +(5), -(6), Colour +(13), -(14)
        let combo1 = match rngcombo {
            1 => Event {
                speak: "Martial Arts".to_string(),
                attchange1: 1,
                attchange2: 6,
                attchange3: 14,
            },
            2 => Event {
                speak: "Singing".to_string(),
                attchange1: 2,
                attchange2: 5,
                attchange3: 14,
            },
            3 => Event {
                speak: "Painting".to_string(),
                attchange1: 2,
                attchange2: 6,
                attchange3: 13,
            },
            4 => Event {
                speak: "Meditation".to_string(),
                attchange1: 2,
                attchange2: 6,
                attchange3: 14,
            },
            5 => Event {
                speak: "Painting to Music".to_string(),
                attchange1: 2,
                attchange2: 5,
                attchange3: 13,
            },
            6 => Event {
                speak: "Paint Ball".to_string(),
                attchange1: 1,
                attchange2: 6,
                attchange3: 13,
            },
            7 => Event {
                speak: "Build something with Power Tools".to_string(),
                attchange1: 1,
                attchange2: 5,
                attchange3: 14,
            },
            8 => Event {
                speak: "Go to a Disco".to_string(),
                attchange1: 1,
                attchange2: 5,
                attchange3: 13,
            },

            _ => Event {
                speak: "Error".to_string(),
                attchange1: 1,
                attchange2: 1,
                attchange3: 1,
            },
        };

        let combo2 = match rngcombo {
            1 => Event {
                speak: "Painting to Music".to_string(),
                attchange1: 2,
                attchange2: 5,
                attchange3: 13,
            },
            2 => Event {
                speak: "Paint Ball".to_string(),
                attchange1: 1,
                attchange2: 6,
                attchange3: 13,
            },
            3 => Event {
                speak: "Build something with Power Tools".to_string(),
                attchange1: 1,
                attchange2: 5,
                attchange3: 14,
            },
            4 => Event {
                speak: "Go to a Disco".to_string(),
                attchange1: 1,
                attchange2: 5,
                attchange3: 13,
            },
            5 => Event {
                speak: "Martial Arts".to_string(),
                attchange1: 1,
                attchange2: 6,
                attchange3: 14,
            },
            6 => Event {
                speak: "Singing".to_string(),
                attchange1: 2,
                attchange2: 5,
                attchange3: 14,
            },
            7 => Event {
                speak: "Painting".to_string(),
                attchange1: 2,
                attchange2: 6,
                attchange3: 13,
            },
            8 => Event {
                speak: "Meditation".to_string(),
                attchange1: 2,
                attchange2: 6,
                attchange3: 14,
            },

            _ => Event {
                speak: "Error".to_string(),
                attchange1: 1,
                attchange2: 1,
                attchange3: 1,
            },
        };

        let combo3 = match rngcombo {
            1 => Event {
                speak: "Build something with Power Tools".to_string(),
                attchange1: 1,
                attchange2: 5,
                attchange3: 14,
            },
            2 => Event {
                speak: "Go to a Disco".to_string(),
                attchange1: 1,
                attchange2: 5,
                attchange3: 13,
            },
            3 => Event {
                speak: "Martial Arts".to_string(),
                attchange1: 1,
                attchange2: 6,
                attchange3: 14,
            },
            4 => Event {
                speak: "Singing".to_string(),
                attchange1: 2,
                attchange2: 5,
                attchange3: 14,
            },
            5 => Event {
                speak: "Painting".to_string(),
                attchange1: 2,
                attchange2: 6,
                attchange3: 13,
            },
            6 => Event {
                speak: "Meditation".to_string(),
                attchange1: 2,
                attchange2: 6,
                attchange3: 14,
            },
            7 => Event {
                speak: "Painting to Music".to_string(),
                attchange1: 2,
                attchange2: 5,
                attchange3: 13,
            },
            8 => Event {
                speak: "Paint Ball".to_string(),
                attchange1: 1,
                attchange2: 6,
                attchange3: 13,
            },

            _ => Event {
                speak: "Error".to_string(),
                attchange1: 1,
                attchange2: 1,
                attchange3: 1,
            },
        };

        println!(
            "\nWhat activity should {} do? \nType 1 for {}, 2 for {}, 3 for {}\n",
            hatch_pet.pet_name, combo1.speak, combo2.speak, combo3.speak
        );

        combo_changer(&mut combo, combo1, combo2, combo3);

        attribute_changer(&mut hatch_pet, &combo);
        println!("{} finished their {}\n", hatch_pet.pet_name, combo.speak);
    thread::sleep(Duration::from_millis(1500));
        println!(
            "This is how {} is doing after their activity",
            hatch_pet.pet_name
        );
    thread::sleep(Duration::from_millis(1500));
        happiness_i(&hatch_pet);
    thread::sleep(Duration::from_millis(500));
        happiness_n(&hatch_pet);
    thread::sleep(Duration::from_millis(500));
        happiness_c(&hatch_pet);
    thread::sleep(Duration::from_millis(3500));
        match hatch_pet.intense {
            1..=50 => {
                println!(
                    "Oh no, poor {}, the stress of it has turned them back into an egg.",
                    hatch_pet.pet_name
                );
                break;
            }
            251..=300 => {
                println!(
                    "Oh no, poor {}, the stress of it has turned them back into an egg.",
                    hatch_pet.pet_name
                );
                break;
            }
            _ => print!(""),
        };
        match hatch_pet.noise {
            1..=50 => {
                println!(
                    "Oh no, poor {}, the stress of it has turned them back into an egg.",
                    hatch_pet.pet_name
                );
                break;
            }
            251..=300 => {
                println!(
                    "Oh no, poor {}, the stress of it has turned them back into an egg.",
                    hatch_pet.pet_name
                );
                break;
            }
            _ => print!(""),
        };
        match hatch_pet.colour {
            1..=50 => {
                println!(
                    "Oh no, poor {}, the stress of it has turned them back into an egg.",
                    hatch_pet.pet_name
                );
                break;
            }
            251..=300 => {
                println!(
                    "Oh no, poor {}, the stress of it has turned them back into an egg.",
                    hatch_pet.pet_name
                );
                break;
            }
            _ => print!(""),
        };

        //End of Activity Phase

        //Start of Sleep Phase

        let mut combo = Event {
            speak: "".to_string(),
            attchange1: 1,
            attchange2: 1,
            attchange3: 1,
        };

    print!("\x1B[2J");
        println!("\n~~~It's time to go to Sleep now~~~");
    thread::sleep(Duration::from_millis(2500));
        println!("\nHere's {}'s state right now", hatch_pet.pet_name);
    thread::sleep(Duration::from_millis(1500));
        happiness_n(&hatch_pet);
    thread::sleep(Duration::from_millis(500));
        happiness_h(&hatch_pet);
    thread::sleep(Duration::from_millis(1500));

        let rngcombo = rand::thread_rng().gen_range(1..=4);
        //Heat +(3), -(4), Noise +(5), -(6)
        let combo1 = match rngcombo {
            1 => Event {
                speak: "Sing to sleep with Window Open".to_string(),
                attchange1: 4,
                attchange2: 5,
                attchange3: 15,
            },
            2 => Event {
                speak: "Peace and Quiet with Hot-water Bottle".to_string(),
                attchange1: 3,
                attchange2: 6,
                attchange3: 15,
            },
            3 => Event {
                speak: "Sing to sleep with Hot-water Bottle".to_string(),
                attchange1: 3,
                attchange2: 5,
                attchange3: 15,
            },
            4 => Event {
                speak: "Peace and Quiet with Window Open".to_string(),
                attchange1: 4,
                attchange2: 6,
                attchange3: 15,
            },

            _ => Event {
                speak: "Error".to_string(),
                attchange1: 1,
                attchange2: 1,
                attchange3: 1,
            },
        };

        let combo2 = match rngcombo {
            1 => Event {
                speak: "Sing to sleep with Hot-water Bottle".to_string(),
                attchange1: 3,
                attchange2: 5,
                attchange3: 15,
            },
            2 => Event {
                speak: "Peace and Quiet with Window Open".to_string(),
                attchange1: 4,
                attchange2: 6,
                attchange3: 15,
            },
            3 => Event {
                speak: "Sing to sleep with Window Open".to_string(),
                attchange1: 4,
                attchange2: 5,
                attchange3: 15,
            },
            4 => Event {
                speak: "Peace and Quiet with Hot-water Bottle".to_string(),
                attchange1: 3,
                attchange2: 6,
                attchange3: 15,
            },

            _ => Event {
                speak: "Error".to_string(),
                attchange1: 1,
                attchange2: 1,
                attchange3: 1,
            },
        };

        let combo3 = match rngcombo {
            1 => Event {
                speak: "Peace and Quiet with Window Open".to_string(),
                attchange1: 4,
                attchange2: 6,
                attchange3: 15,
            },
            2 => Event {
                speak: "Sing to sleep with Window Open".to_string(),
                attchange1: 4,
                attchange2: 5,
                attchange3: 15,
            },
            3 => Event {
                speak: "Peace and Quiet with Hot-water Bottle".to_string(),
                attchange1: 3,
                attchange2: 6,
                attchange3: 15,
            },
            4 => Event {
                speak: "Sing to sleep with Hot-water Bottle".to_string(),
                attchange1: 3,
                attchange2: 5,
                attchange3: 15,
            },

            _ => Event {
                speak: "Error".to_string(),
                attchange1: 1,
                attchange2: 1,
                attchange3: 1,
            },
        };

        println!("\nWhat do you think {} needs to have a good nights sleep? \nType 1 for {}, 2 for {}, 3 for {}\n",hatch_pet.pet_name, combo1.speak, combo2.speak, combo3.speak);

        combo_changer(&mut combo, combo1, combo2, combo3);

        attribute_changer(&mut hatch_pet, &combo);
        println!("{} settles down to {}\n", hatch_pet.pet_name, combo.speak);
    thread::sleep(Duration::from_millis(1500));
        println!(
            "This is how {} is doing after their Sleep",
            hatch_pet.pet_name
        );
    thread::sleep(Duration::from_millis(1500));
        happiness_h(&hatch_pet);
    thread::sleep(Duration::from_millis(500));
        happiness_n(&hatch_pet);
    thread::sleep(Duration::from_millis(3500));
        match hatch_pet.heat {
            1..=50 => {
                println!(
                    "Oh no, poor {}, the stress of it has turned them back into an egg.",
                    hatch_pet.pet_name
                );
                break;
            }
            251..=300 => {
                println!(
                    "Oh no, poor {}, the stress of it has turned them back into an egg.",
                    hatch_pet.pet_name
                );
                break;
            }
            _ => print!(""),
        };
        match hatch_pet.noise {
            1..=50 => {
                println!(
                    "Oh no, poor {}, the stress of it has turned them back into an egg.",
                    hatch_pet.pet_name
                );
                break;
            }
            251..=300 => {
                println!(
                    "Oh no, poor {}, the stress of it has turned them back into an egg.",
                    hatch_pet.pet_name
                );
                break;
            }
            _ => print!(""),
        };
        //End of Sleep Phase

        health_data.health_i = match hatch_pet.intense {
            1..=50 => 0,
            51..=75 => 1,
            76..=100 => 2,
            101..=124 => 5,
            125..=139 => 7,
            140..=160 => 10,
            161..=175 => 7,
            176..=200 => 5,
            201..=225 => 2,
            226..=250 => 1,
            251..=300 => 0,
            _ => 0,
        };

        health_data.health_h = match hatch_pet.heat {
            1..=50 => 0,
            51..=75 => 1,
            76..=100 => 2,
            101..=124 => 5,
            125..=139 => 7,
            140..=160 => 10,
            161..=175 => 7,
            176..=200 => 5,
            201..=225 => 2,
            226..=250 => 1,
            251..=300 => 0,
            _ => 0,
        };

        health_data.health_n = match hatch_pet.noise {
            1..=50 => 0,
            51..=75 => 1,
            76..=100 => 2,
            101..=124 => 5,
            125..=139 => 7,
            140..=160 => 10,
            161..=175 => 7,
            176..=200 => 5,
            201..=225 => 2,
            226..=250 => 1,
            251..=300 => 0,
            _ => 0,
        };

        health_data.health_s = match hatch_pet.sweet {
            1..=50 => 0,
            51..=75 => 1,
            76..=100 => 2,
            101..=124 => 5,
            125..=139 => 7,
            140..=160 => 10,
            161..=175 => 7,
            176..=200 => 5,
            201..=225 => 2,
            226..=250 => 1,
            251..=300 => 0,
            _ => 0,
        };

        health_data.health_p = match hatch_pet.pretty {
            1..=50 => 0,
            51..=75 => 1,
            76..=100 => 2,
            101..=124 => 5,
            125..=139 => 7,
            140..=160 => 10,
            161..=175 => 7,
            176..=200 => 5,
            201..=225 => 2,
            226..=250 => 1,
            251..=300 => 0,
            _ => 0,
        };

        health_data.health_d = match hatch_pet.damp {
            1..=50 => 0,
            51..=75 => 1,
            76..=100 => 2,
            101..=124 => 5,
            125..=139 => 7,
            140..=160 => 10,
            161..=175 => 7,
            176..=200 => 5,
            201..=225 => 2,
            226..=250 => 1,
            251..=300 => 0,
            _ => 0,
        };

        health_data.health_c = match hatch_pet.colour {
            1..=50 => 0,
            51..=75 => 1,
            76..=100 => 2,
            101..=124 => 5,
            125..=139 => 7,
            140..=160 => 10,
            161..=175 => 7,
            176..=200 => 5,
            201..=225 => 2,
            226..=250 => 1,
            251..=300 => 0,
            _ => 0,
        };
        //health checker
        health_data.pet_health = health_data.health_i
            + health_data.health_h
            + health_data.health_n
            + health_data.health_s
            + health_data.health_p
            + health_data.health_d
            + health_data.health_c;
        if health_data.fave_i == true {
            health_data.pet_health = health_data.pet_health + health_data.health_i
        };
        if health_data.fave_h == true {
            health_data.pet_health = health_data.pet_health + health_data.health_h
        };
        if health_data.fave_n == true {
            health_data.pet_health = health_data.pet_health + health_data.health_n
        };
        if health_data.fave_s == true {
            health_data.pet_health = health_data.pet_health + health_data.health_s
        };
        if health_data.fave_p == true {
            health_data.pet_health = health_data.pet_health + health_data.health_p
        };
        if health_data.fave_d == true {
            health_data.pet_health = health_data.pet_health + health_data.health_d
        };
        if health_data.fave_c == true {
            health_data.pet_health = health_data.pet_health + health_data.health_c
        };

        health_data.growth_speed = match health_data.pet_health {
            1..=20 => 0,
            21..=40 => 7,
            41..=60 => 15,
            61..=80 => 26,
            81..=100 => 35,
            _ => 0,
        };

        health_data.growth = health_data.growth + health_data.growth_speed;
        // Create match function based upon health of pet and size of pet.
        //println!("Growth is {}",health_data.growth);
        //Pet is X health, pet is X size
    print!("\x1B[2J");
        println!("\n~~~After a long day of looking after {} it's time to check their overall health and size~~~",hatch_pet.pet_name);
    thread::sleep(Duration::from_millis(2500));
        match health_data.pet_health {
            0..=15 => println!("{} is incredibly ill", hatch_pet.pet_name),
            16..=25 => println!("{} is seriously unhealthy", hatch_pet.pet_name),
            26..=40 => println!("{} is unhealthy", hatch_pet.pet_name),
            41..=55 => println!("{} is doing okay, health wise", hatch_pet.pet_name),
            56..=60 => println!("{} is in reasonable health", hatch_pet.pet_name),
            61..=75 => println!("{} is healthy!", hatch_pet.pet_name),
            76..=90 => println!("{} is really healthy!", hatch_pet.pet_name),
            91..=150 => println!("{} is exceptionally healthy!", hatch_pet.pet_name),
            _ => println!("Error"),
        };

    thread::sleep(Duration::from_millis(1500));
        match health_data.growth {
            0..=9 => println!("{} is teeny tiny!", hatch_pet.pet_name),
            10..=19 => println!("{} is tiny!", hatch_pet.pet_name),
            20..=29 => println!("{} is very small!", hatch_pet.pet_name),
            30..=39 => println!("{} is small!", hatch_pet.pet_name),
            40..=49 => println!("{} is getting bigger!", hatch_pet.pet_name),
            50..=59 => println!("{} is quite big now!", hatch_pet.pet_name),
            60..=69 => println!("{} is big!", hatch_pet.pet_name),
            70..=79 => println!("{} is very big!", hatch_pet.pet_name),
            80..=89 => println!("{} is huge!", hatch_pet.pet_name),
            90..=99 => println!("{} is absolutely massive!", hatch_pet.pet_name),
            100..=150 => {
                println!("{} is at its maximum size!", hatch_pet.pet_name);
                println!(
                    "~~~Congratulations! You helped {} grow from an egg to a healthy adult~~~",
                    hatch_pet.pet_name
                );
                break;
            }

            _ => println!("Error"),
        };
    thread::sleep(Duration::from_millis(2500));
    print!("\x1B[2J");
    } //End of phases loop
} //<----------Main function end scope

fn combo_changer(combo: &mut Event, combo1: Event, combo2: Event, combo3: Event) {
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess {
            1 => {
                *combo = combo1;
                println!("{}", combo.speak);
                break;
            }
            2 => {
                *combo = combo2;
                println!("{}", combo.speak);
                break;
            }
            3 => {
                *combo = combo3;
                println!("{}", combo.speak);
                break;
            }
            _ => {
                *combo = combo1;
                println!("{}", combo.speak);
                break;
            }
        };

        //Next: create IO that loops and calls attribute_changer
    }
}

//-----Function checking Egg input-----v
fn egg_checker(eggvar: &mut i32, egg_grow: i32) {
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess {
            1 => {
                *eggvar = *eggvar + egg_grow;
                println!("The egg is responding!");
                break;
            }
            2 => {
                println!("The egg is responding!");
                break;
            }
            _ => println!("Welp"),
        };
    }
} //<---------End scope of egg_checker

fn attribute_changer(hatch_pet: &mut Pet, combo: &Event) {
    match combo.attchange1 {
        1 => hatch_pet.intense = hatch_pet.intense + 14,
        2 => hatch_pet.intense = hatch_pet.intense - 14,
        3 => hatch_pet.heat = hatch_pet.heat + 14,
        4 => hatch_pet.heat = hatch_pet.heat - 14,
        5 => hatch_pet.noise = hatch_pet.noise + 14,
        6 => hatch_pet.noise = hatch_pet.noise - 14,
        7 => hatch_pet.sweet = hatch_pet.sweet + 14,
        8 => hatch_pet.sweet = hatch_pet.sweet - 14,
        9 => hatch_pet.pretty = hatch_pet.pretty + 14,
        10 => hatch_pet.pretty = hatch_pet.pretty - 14,
        11 => hatch_pet.damp = hatch_pet.damp + 14,
        12 => hatch_pet.damp = hatch_pet.damp - 14,
        13 => hatch_pet.colour = hatch_pet.colour + 14,
        14 => hatch_pet.colour = hatch_pet.colour - 14,
        _ => println!("Error"),
    };
    match combo.attchange2 {
        1 => hatch_pet.intense = hatch_pet.intense + 14,
        2 => hatch_pet.intense = hatch_pet.intense - 14,
        3 => hatch_pet.heat = hatch_pet.heat + 14,
        4 => hatch_pet.heat = hatch_pet.heat - 14,
        5 => hatch_pet.noise = hatch_pet.noise + 14,
        6 => hatch_pet.noise = hatch_pet.noise - 14,
        7 => hatch_pet.sweet = hatch_pet.sweet + 14,
        8 => hatch_pet.sweet = hatch_pet.sweet - 14,
        9 => hatch_pet.pretty = hatch_pet.pretty + 14,
        10 => hatch_pet.pretty = hatch_pet.pretty - 14,
        11 => hatch_pet.damp = hatch_pet.damp + 14,
        12 => hatch_pet.damp = hatch_pet.damp - 14,
        13 => hatch_pet.colour = hatch_pet.colour + 14,
        14 => hatch_pet.colour = hatch_pet.colour - 14,
        _ => println!("Error"),
    };
    match combo.attchange3 {
        1 => hatch_pet.intense = hatch_pet.intense + 14,
        2 => hatch_pet.intense = hatch_pet.intense - 14,
        3 => hatch_pet.heat = hatch_pet.heat + 14,
        4 => hatch_pet.heat = hatch_pet.heat - 14,
        5 => hatch_pet.noise = hatch_pet.noise + 14,
        6 => hatch_pet.noise = hatch_pet.noise - 14,
        7 => hatch_pet.sweet = hatch_pet.sweet + 14,
        8 => hatch_pet.sweet = hatch_pet.sweet - 14,
        9 => hatch_pet.pretty = hatch_pet.pretty + 14,
        10 => hatch_pet.pretty = hatch_pet.pretty - 14,
        11 => hatch_pet.damp = hatch_pet.damp + 14,
        12 => hatch_pet.damp = hatch_pet.damp - 14,
        13 => hatch_pet.colour = hatch_pet.colour + 14,
        14 => hatch_pet.colour = hatch_pet.colour - 14,
        _ => hatch_pet.sweet = hatch_pet.sweet,
    };
}

//---------Happiness checkers----------v (Telling user about state of happiness)
fn happiness_i(hatch_pet: &Pet) {
    match hatch_pet.intense {
        1..=50 => println!("{} reverted to an egg from boredom", hatch_pet.pet_name), //die
        51..=75 => println!("{} is going insane from boredom", hatch_pet.pet_name),   //emergency
        76..=100 => println!("{} is feeling really bored", hatch_pet.pet_name),       //warning
        101..=124 => println!("{} could do with a bit more excitement", hatch_pet.pet_name), //norm
        125..=139 => println!("{} is at ease", hatch_pet.pet_name),                   //good
        140..=160 => println!("{} is perfectly calm and engaged", hatch_pet.pet_name), //optimum
        161..=175 => println!("{} is excited", hatch_pet.pet_name),                   //good
        176..=200 => println!("{} could do with a bit more calm", hatch_pet.pet_name), //norm
        201..=225 => println!("{} is stressed", hatch_pet.pet_name),                  //warning
        226..=250 => println!("{} is extremely overwhelmed", hatch_pet.pet_name),     //emergency
        251..=300 => println!("{} reverted to an egg from panic", hatch_pet.pet_name), //die
        _ => println!("{} is doing okay", hatch_pet.pet_name),                        // okay
    };
}

fn happiness_h(hatch_pet: &Pet) {
    match hatch_pet.heat {
        1..=50 => println!(
            "{} reverted to an egg from the freezing cold",
            hatch_pet.pet_name
        ), //die
        51..=75 => println!("{} is freezing cold", hatch_pet.pet_name), //emergency
        76..=100 => println!("{} is really cold", hatch_pet.pet_name),  //warning
        101..=124 => println!("{} could do with being a bit warmer", hatch_pet.pet_name), //norm
        125..=139 => println!("{} is nice and cool", hatch_pet.pet_name), //good
        140..=160 => println!("{} is at the perfect temperature", hatch_pet.pet_name), //optimum
        161..=175 => println!("{} is nice and warm", hatch_pet.pet_name), //good
        176..=200 => println!("{} could do with being a little cooler", hatch_pet.pet_name), //norm
        201..=225 => println!("{} is really hot", hatch_pet.pet_name),  //warning
        226..=250 => println!("{} is boiling hot", hatch_pet.pet_name), //emergency
        251..=300 => println!(
            "{} reverted to an egg from the boiling heat",
            hatch_pet.pet_name
        ), //die
        _ => println!("{} is doing okay", hatch_pet.pet_name),          // okay
    };
}

fn happiness_n(hatch_pet: &Pet) {
    match hatch_pet.noise {
        1..=50 => println!(
            "{} reverted to an egg from the deafening silence",
            hatch_pet.pet_name
        ), //die
        51..=75 => println!("{} cannot bare the silence", hatch_pet.pet_name), //emergency
        76..=100 => println!("{} is not enjoying the silence", hatch_pet.pet_name), //warning
        101..=124 => println!("{} would like something to listen to", hatch_pet.pet_name), //norm
        125..=139 => println!("{} enjoys the quiet", hatch_pet.pet_name),      //good
        140..=160 => println!("{} loves the ambiance", hatch_pet.pet_name),    //optimum
        161..=175 => println!("{} enjoys the sounds", hatch_pet.pet_name),     //good
        176..=200 => println!("{} would prefer it a bit quieter", hatch_pet.pet_name), //norm
        201..=225 => println!("It is too loud for {}", hatch_pet.pet_name),    //warning
        226..=250 => println!("{} is panicking with the noise", hatch_pet.pet_name), //emergency
        251..=300 => println!(
            "{} reverted to an egg from extreme noise",
            hatch_pet.pet_name
        ), //die
        _ => println!("{} is doing okay", hatch_pet.pet_name),                 // okay
    };
}

fn happiness_s(hatch_pet: &Pet) {
    match hatch_pet.sweet {
        1..=50 => println!(
            "{} reverted to an egg from sugar cravings",
            hatch_pet.pet_name
        ), //die
        51..=75 => println!("{} is going wild with sugar cravings", hatch_pet.pet_name), //emergency
        76..=100 => println!("{} really wants something sweet", hatch_pet.pet_name),     //warning
        101..=124 => println!("{} would like a little something sweet", hatch_pet.pet_name), //norm
        125..=139 => println!("{} is enjoying the savoury flavours", hatch_pet.pet_name), //good
        140..=160 => println!(
            "{} is loving the perfect balance of sweet and savoury",
            hatch_pet.pet_name
        ), //optimum
        161..=175 => println!("{} is enjoying the sweet flavours", hatch_pet.pet_name),  //good
        176..=200 => println!("{} could do with a bit of savoury food", hatch_pet.pet_name), //norm
        201..=225 => println!(
            "{} is feeling rather queezy from the sugar",
            hatch_pet.pet_name
        ), //warning
        226..=250 => println!(
            "{} feels nauseous from the incredible amount of sugar",
            hatch_pet.pet_name
        ), //emergency
        251..=300 => println!(
            "{} reverted to an egg because of a sugar overload",
            hatch_pet.pet_name
        ), //die
        _ => println!("{} is doing okay", hatch_pet.pet_name),                           // okay
    };
}

fn happiness_p(hatch_pet: &Pet) {
    match hatch_pet.pretty {
        1..=50 => println!(
            "The environment is so drab {} reverted to an egg",
            hatch_pet.pet_name
        ), //die
        51..=75 => println!(
            "{} is incredibly uninspired by this drab environment",
            hatch_pet.pet_name
        ), //emergency
        76..=100 => println!(
            "{} needs something pretty to inspire them",
            hatch_pet.pet_name
        ), //warning
        101..=124 => println!("{} could do with a touch of beauty", hatch_pet.pet_name), //norm
        125..=139 => println!(
            "{} is enjoying the practicality of this environment",
            hatch_pet.pet_name
        ), //good
        140..=160 => println!(
            "{} loves how this environment seems beautiful yet practical",
            hatch_pet.pet_name
        ), //optimum
        161..=175 => println!("{} likes the beauty here", hatch_pet.pet_name),           //good
        176..=200 => println!(
            "{} could do with a more practical environment",
            hatch_pet.pet_name
        ), //norm
        201..=225 => println!(
            "{} is frustrated by the impracticality of this environment",
            hatch_pet.pet_name
        ), //warning
        226..=250 => println!(
            "{} is furious with how impractical this environment is",
            hatch_pet.pet_name
        ), //emergency
        251..=300 => println!(
            "The environment is so impractical {} reverted to an egg",
            hatch_pet.pet_name
        ), //die
        _ => println!("{} is doing okay", hatch_pet.pet_name),                           // okay
    };
}

fn happiness_d(hatch_pet: &Pet) {
    match hatch_pet.damp {
        1..=50 => println!(
            "{} reverted to an egg from extreme dryness",
            hatch_pet.pet_name
        ), //die
        51..=75 => println!("{} is suffering from extreme dryness", hatch_pet.pet_name), //emergency
        76..=100 => println!("{} is really in need of some moisture", hatch_pet.pet_name), //warning
        101..=124 => println!("{} could do with a bit more moisture", hatch_pet.pet_name), //norm
        125..=139 => println!("{} is enjoying the dryness", hatch_pet.pet_name),         //good
        140..=160 => println!("{} loves the balance of dry and humid", hatch_pet.pet_name), //optimum
        161..=175 => println!("{} is enjoying the humidity", hatch_pet.pet_name),           //good
        176..=200 => println!("{} would prefer it were dryer", hatch_pet.pet_name),         //norm
        201..=225 => println!("{} is struggling with the damp", hatch_pet.pet_name), //warning
        226..=250 => println!("{} is absolutely drenched", hatch_pet.pet_name),      //emergency
        251..=300 => println!(
            "{} is so drenched it reverted to an egg",
            hatch_pet.pet_name
        ), //die
        _ => println!("{} is doing okay", hatch_pet.pet_name),                       // okay
    };
}
//colour - unf
fn happiness_c(hatch_pet: &Pet) {
    match hatch_pet.colour {
        1..=50 => println!(
            "{} reverted to an egg from the darkness",
            hatch_pet.pet_name
        ), //die
        51..=75 => println!("{} really hates how dark everything is", hatch_pet.pet_name), //emergency
        76..=100 => println!("{} wants some more colour", hatch_pet.pet_name),             //warning
        101..=124 => println!("{} could do with a touch of colour", hatch_pet.pet_name),   //norm
        125..=139 => println!("{} likes the muted colours", hatch_pet.pet_name),           //good
        140..=160 => println!(
            "{} loves the balance of bright and muted colours",
            hatch_pet.pet_name
        ), //optimum
        161..=175 => println!("{} likes the bright colours", hatch_pet.pet_name),          //good
        176..=200 => println!(
            "{} could do with the colours toned down a little",
            hatch_pet.pet_name
        ), //norm
        201..=225 => println!(
            "{} is overwhelmed with the brightness of all this colour",
            hatch_pet.pet_name
        ), //warning
        226..=250 => println!(
            "{} cannot bare the obnoxious bright colour everywhere",
            hatch_pet.pet_name
        ), //emergency
        251..=300 => println!(
            "{} reverted to an egg from colour overload",
            hatch_pet.pet_name
        ), //die
        _ => println!("{} is doing okay", hatch_pet.pet_name),                             // okay
    };
}
//-------End of Happiness checkers----------^ (Functions checking pet happiness)


/*
To do

Next step
-Introduce time variable
-At end of life give an estimation upon how well you looked after pet
+Check point: At this state evaluate the project

Future: Maybe, in gardening phase, garden has some variables to cover what is in it, just for flavour.


    # indicated pet's favourite atribute. ^ is preffered
Loork: Likes: Relax-I, ^cold-H, ^quiet-N, savoury-S, practical-P, dry-D, #dark-C
Kring: Likes: ^Intense+I, cold-H, #loud+N, sweet+S, pretty+P, ^dry-D, dark-C
Spreek: Likes: Relax-I, hot+H, quiet-N, ^sweet+S, #pretty+P, dry-D, ^colourful+C
Snuggler Likes: ^Relax-I, ^warm+H, loud+N, #sweet+S, pretty+P, dry-D, colourful+C
Golbert Likes: ^Intense+I, cold-H, ^quiet-N, savoury-S, #pract-P, damp+D, dark-C
Frimpi Likes: Intense+I, ^cold-H, noise+N, sweet+S, pretty+P, ^damp+D, #colourful+C
Breel Likes: Relax-I, warm+H, quiet-N,, ^savoury-S, pract-P, #damp+D, ^dark-C
Dragoo Likes: #Intense+I, ^hot+H, ^loud+N, savoury-S, pract-P, damp+D, colour+C
*/
