use rand::Rng;

fn main() {
    // World settings
    let mut world = Vec::new();
    let height = 50;
    let width = 100;

    let mut choice = 0;

    //⬜
    //⬛

    let mut count = 0;
    loop
    {
        if count >= height {break;}

        // Generates random particles

        choice = rand::thread_rng().gen_range(0..3);
        if choice == 0 
        {
            world.push(vec!['⬜'; width]);

            let mut _w = 0;
            loop
            {
                if _w >= width {break;}

                if rand::thread_rng().gen_range(0..2) == 1
                {
                    world[count][_w] = '⬛';
                }

                _w += 1;
            }
        }
        else
        {
            world.push(vec!['⬛'; width]);
        }

        count += 1;
    }
    
    // Update settings
    let update_rate = 20000000;
    count = 0;

    // Will store the final string
    let mut final_string = String::new();    
    loop
    {
        
        if count == update_rate
        {
            count = 0;
            
            
            // Analyzes the world and makes the particles to react
            for y in 0..world.len()
            {
                // Y row
                for x in 0..world[y].len()
                {

                    // Updates the particles from bottom to top
                    let mut y_final_value = height - 1 - y;
                    // X row
                    if world[y_final_value][x] == '⬜' && y_final_value < world.len() - 1
                    {
                        // Makes it fall
                        world[y_final_value][x] = '⬛';

                        if world[y_final_value + 1][x] == '⬜'
                        {
                            // If there's already a particle under this one, move to the laterals
                            if x != 0 && world[y_final_value + 1][x - 1] != '⬜' && y_final_value < world.len() - 1
                            {
                                world[y_final_value + 1][x - 1] = '⬜';
                            }
                            else if x != width - 1 && world[y_final_value + 1][x + 1] != '⬜' && y_final_value < world.len() - 1
                            {
                                world[y_final_value + 1][x + 1] = '⬜';
                            }
                            else 
                            {
                                // If there's no way to move downwards, only moves sideways if possible
                                choice = rand::thread_rng().gen_range(0..2);

                                if x > 0 && world[y_final_value][x - 1] != '⬜' && choice == 0
                                {
                                    world[y_final_value][x - 1] = '⬜';
                                }
                                else
                                {
                                    if x < width - 1 && world[y_final_value][x + 1] != '⬜' && choice == 1
                                    {
                                        world[y_final_value][x + 1] = '⬜';
                                    }
                                    else 
                                    {
                                        world[y_final_value][x] = '⬜';
                                    }
                                }                                
                            }
                        }
                        else 
                        {
                            world[y_final_value + 1][x] = '⬜';
                        }
                    }
                }
                final_string.push('\n');
            }

            // Gets information about the world to display it
            final_string = String::from("");

            for y in 0..world.len()
            {
                for x in 0..world[y].len()
                {
                    final_string.push(world[y][x])
                }
                final_string.push('\n');
            }

            print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clears console and prints information on top of it
            println!("{}", final_string);
        }

        // Adds to update variable
        count += 1;
    }
}

