use std::time::SystemTime;
use std::io::{self, Write};


struct PasswordEntry{
    pass_hash:u64,
    salt:u8
}

impl PasswordEntry{
    fn new(hash:u64, salt:u8) -> Self{
        Self { pass_hash:hash, salt:salt}
    }
}
struct UsrAccount{
    username: String,
    // acc_id: u32,
    // email: String,
    password: PasswordEntry 
}
struct Lcg {
    state: u64,
}

impl Lcg{
    fn new(mut seed: u64) -> Self {
        if seed == 0 {
            seed = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs();
        }
        Lcg { state: seed }
    }
    fn next(&mut self) -> u64{
        self.state = self.state.wrapping_mul(0x41C64E6D).wrapping_add(0x303B);
        self.state
    }
    fn get_state(&self){
        println!("Lcg State: {}",self.state);
    }
    fn init_self(&mut self){
        for _ in 0..3 {
            self.next();
        }
        for _ in 0..(self.next()%30) {
            self.next();
        }
        for _ in 0..(self.next()%37) {
            self.next();
        }
    }
    fn get_range(&mut self, low:u64, high:u64) -> u64{
        (self.next() % (high - low +1))+low
    } 
}


fn mysim_hash(input: &str) -> u64 {
    const FNV_PRIME: u64 = 1099511628211;
    const FNV_OFFSET_BASIS: u64 = 14695981039346656037;

    let mut hash = FNV_OFFSET_BASIS;

    for byte in input.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }

    hash
}

// fn h_lin_cong_rng(seed:u64) -> u64{ 
//     let res = 0x41C64E6D * seed + 0x0000303B;
//     res
// }
// fn lin_cong_rng(mut seed:u64) -> u64{
//     if seed == 0 {
//         match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
//             Ok(n) => seed = n.as_secs(),
//             Err(_) => panic!("SystemTime before UNIX EPOCH!"),
//         }
//     }
//     let res:u64 = 0x41C64E6D * seed + 0x0000303B;
// }

// 1. add user to db.
// 2. attempt login:
//  2a) check  for username in db
//  2b) get UsrAcc elem, get salt
//  3b) get hash, check it to know hash//

fn is_user_in_db(database: &Vec<UsrAccount>, name: &str) -> bool {
    database.iter().any(|entry| entry.username == name)
}

fn login_attempt(username:String, password:String, database:&Vec<UsrAccount>) -> bool{
    //check if user is in db by name

    let db_ent = database.iter().find(|entry| entry.username == username);
    if let Some(user) = db_ent {
    // Found him! 'user' is your &UsrAccount
        println!("[Sever] Found {}", user.username);
        let password_to_hash = format!("{}{}",password,user.password.salt);
        println!("[Server] toHash: {}", password_to_hash);
        let hashed = mysim_hash(&password_to_hash);

        println!("[Server] hashed: {}", hashed);
        if hashed == user.password.pass_hash {
            println!("[Server] correct password!");
            println!("[User] Successfully authenticated!");
            return true
        }else{
            println!("[Server] incorrect hash. sorry");

        }
        

    } else {
        println!("[Server] didnt find username");
    }
    false
}

fn create_user(username:String, password:String, database:&mut Vec<UsrAccount>, rng:&mut Lcg) -> bool{
    let db_ent = database.iter().find(|entry| entry.username == username);
    if let Some(user) = db_ent {
    // Found him! 'user' is your &UsrAccount
        println!("[Sever] Found {}. BAD! already taken", user.username);
        println!("[User] Username already in use. please try again!");
    } else {
        println!("[Server] didnt find username. good");
        

        let new_salt = rng.get_range(1,255) as u8;
        
        let password_to_hash = format!("{}{}",password,new_salt);
        println!("[Server] toHash: {}", password_to_hash);
        let hashed = mysim_hash(&password_to_hash);
        // new_entry.password.pass_hash = hashed;
        let new_entry = UsrAccount{
            username: username, 
            password: PasswordEntry::new(hashed,new_salt)
        };
        database.push(new_entry);
        println!("[Server] added new entry!");
        return true;
    }

    false

}


// fn check_valid_login(database:&Vec<UsrAccount>, usrIndex:u32) -> bool{
//     let mut bool = false;
//     if let Some(userStr) = database.get(usrIndex){
//
//     }
//     else{
//         println!("i fucked up user index checking");
//     }
// }
//

fn main() {
    let mut rng = Lcg::new(0);
    rng.init_self();
    let mut user_db: Vec<UsrAccount> = Vec::new();
    
    let mut input_text = String::new();

    loop    {
    input_text.clear();

    println!("\nChoose: [1] New User  [2] Login  [3] Exit");
    print!("> ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input_text).expect("Failed to read");
    let choice = input_text.trim();

    match choice {
        "1" | "new user" => cu_h(&mut user_db, &mut rng),
        "2" | "login" => li_h(&mut user_db),
        "3" | "exit" => break,
            _ => println!("Invalid option, try again."),
    }
    }

}
fn cu_h(db:&mut Vec<UsrAccount>, rng:&mut Lcg ){
    let (usr,pass) = helper_input(" Create Account ");
    let out = create_user(usr,pass,db, rng);
    if out {
        println!("success!");
    }
    else{
        println!("Failure");
    }
}
fn li_h(db:&mut Vec<UsrAccount>){
    let (usr,pass) = helper_input(" Create Account ");
    let out = login_attempt(usr,pass,db); 
    if out {
        println!("success!");
    }
    else{
        println!("Failure");
    }
}

fn helper_input(label: &str) -> (String,String){

    println!("--- {} ---", label);
    
    let username = prompt("Username: ");
    let password = prompt("Password: ");
    (username, password)
}
fn prompt(msg: &str) -> String {
    let mut buf = String::new();
    print!("{}", msg);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).expect("Read error");
    buf.trim().to_string()
}
