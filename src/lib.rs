mod calculator{
    mod basic_fuctions {
        use std::io;
        pub fn add() {
            println!("please enter x = ");
            let mut x = String::new();
            io::stdin().read_line(&mut x);
            let x:i32= x.trim().parse().unwrap();

            println!("please enter y = ");
             let mut y = String::new();
            io::stdin().read_line(&mut y);
            let y:i32= y.trim().parse().unwrap();

            println!("{}",x+y);

        }
        pub fn subtract() {
            println!("please enter x = ");
            let mut x = String::new();
            io::stdin().read_line(&mut x);
            let x:i32= x.trim().parse().unwrap();

            println!("please enter y = ");
             let mut y = String::new();
            io::stdin().read_line(&mut y);
            let y:i32= y.trim().parse().unwrap();

            println!("{}",x-y);
        }
        pub fn multiply() {
            println!("please enter x = ");
            let mut x = String::new();
            io::stdin().read_line(&mut x);
            let x:i32= x.trim().parse().unwrap();

            println!("please enter y = ");
             let mut y = String::new();
            io::stdin().read_line(&mut y);
            let y:i32= y.trim().parse().unwrap();

            println!("{}",x*y);
        }
        pub fn divide() {
            println!("please enter x = ");
            let mut x = String::new();
            io::stdin().read_line(&mut x);
            let x:i32= x.trim().parse().unwrap();

            println!("please enter y = ");
             let mut y = String::new();
            io::stdin().read_line(&mut y);
            let y:i32= y.trim().parse().unwrap();

            println!("{}",x/y);
        }

    }

    mod power_fuctions {
        use std::io;
        pub fn square_function() {
            println!("please enter x = ");
            let mut x = String::new();
            io::stdin().read_line(&mut x);
            let x:i32= x.trim().parse().unwrap();

            println!("{}",x*x);

        }
        pub fn cube_function() {
            println!("please enter x = ");
            let mut x = String::new();
            io::stdin().read_line(&mut x);
            let x:i32= x.trim().parse().unwrap();

            println!("{}",x*x*x);
        }
        pub fn power_fuction() {
            println!("please enter base = ");
            let mut x = String::new();
            io::stdin().read_line(&mut x);
            let x:f32= x.trim().parse().unwrap();

            println!("please enter power = ");
            let mut y = String::new();
            io::stdin().read_line(&mut y);
            let y:f32= y.trim().parse().unwrap();

            println!("{}",x.powf(y));
        }
    }






































}


















