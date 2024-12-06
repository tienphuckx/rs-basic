# Variable in Rust

## let, mut
let x = 7; // x can be change
let y = 7; // y can be change

## const
const THREE_HOUR_IN_SECONDS: u32 = 60*60*3;
- When should I use "let" when should I use const ?

## shadow of a variable
let x = 7; // x can be change
let x = x + 1; // redefined the x with new values, this call get shadow of the variable

## Scalar (Nguyen Thuy)

LENGTH         - SIGNED     - UNSIGNED
8 bits         - i8         - u8
16 bits        - i16        - u16
32 bits        - i32        - u32
64 bits        - i64        - u64
128 bits       - i128       - u128

## Tuple
let tup = (500, 7.7, 1);

let (x,y,x) = tup; // now x=500; y=7.7; z=1

let phuc_score: (i32, f64, u8) = (27, 9.9, 1);
phuc_score.0 = 27
phuc_score.1 = 9.9
phuc_score.2 = 1

## function in rust

### function not return
    fn print_message(msg: string){
        ...
    }

### function with params
    fn print_code(code: i32){
        println!("Code is {}", code);
    }
    fn main(){
        print_code(7);
    }
### function with return value
    fn add_two_number(a: i32, b: i32) -> i32 {
        let c = a + b;
        c // not use ";" mean return
    }
