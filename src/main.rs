fn main() {
    println!("Hello, world!");
}

fn main() {
  let mut x: i32 = 1;
  x += 2;

  assert_eq!(x, 3);
  println!("Success!");
}

fn main(){
  let x: i32 = 10;
  let y: i32 = 20;

  {
    println!("The value of x is {} and y is {}", x, y);
  }

  println!("The value of x is {} and y is {}", x, y);
}

fn define_x() {
  let x: &str = "Hello";

  println!("{}, world!!", x);
}

fn main() {
  define_x();
}

fn main() {
  let x: i32 = 10;
  {
    let x: i32 = 20;
    assert_eq!(x, 20);
  }
  
  assert_eq!(x, 10);
  let x: i32 = 30;
  println!("{}", x);
}

fn main() {
  let mut _x: i32 = 10;
  _x = 7;
  _x += 5;

  println!("{}", _x);
}

fn main() {
  let ( mut x, y) = (4, 7);
  x+=2;

  assert_eq!(x, 6);
  assert_eq!(y, 7);

  println!("Success!");
}

fn main() {
  let x: &str = "Hello";
  let y: i32 = 90;

  assert_eq!(x, "Hello");
  assert_eq!(y, 90);

  println!("{}, {}", x, y);
}

fn main() {
  let (x, y);
  (x, ..) = (1, 3);
  [.., y] = [4, 5];

  assert_eq!((x, y), (1, 5));
  println!("{}, {}", x, y);
}

fn main() {
  let x = 3;
  let mut y = 9;
  y += x;

  println!("{}, {}", x, y);
}

fn main() {
  let _x : u32 = 38_u8 as u32;
  println!("succes!");
}

fn main() {
  let x: u32 = 9;
  assert_eq!("u32".to_string(), type_of(&x));

  println!("Success!");
}

fn type_of<T>(_: &T) -> String {
  format!("{}", std::any::type_name::<T>())
}

fn main() {
  let x = 215_u16 + 8;
  let y = i16::checked_add(251, 8).unwrap();
  println!("{}, {}", x, y);
}

fn main() {
  assert_eq!(0.1 as f32 + 0.2 as f32, 0.3);
  println!("Success!");
}

fn main() {
  let mut x: i32 = 56;
  x += 1;

  assert_eq!(x, 57);

  println!("{}", x);
}

fn main() {
  let mut sum: i32 = 0;

  for i in -3..2 {
    sum += i;
  }

  assert_eq!(sum, -5);

  for c in 'a'..='z'{
    println!("{}", c as u8);
  }
}

use std::mem::size_of_val;

fn main() {
  let x: char = 'a';	

  assert_eq!(size_of_val(&x), 4);

  println!("{}", size_of_val(&x));
}

fn main() {
  let x: char = 'e';
  print_char(x);
}

fn print_char(x: char) {
  println!("{}", x);
}

fn main() {
  let x: bool = false;
  let y: bool = true && false;

  assert_eq!(x, y);

  println!("Done!");
}

fn main() {
  let x: u32 = 3;

  let y: u32 = {
    let x_squr = x * x;
    let x_cube = x_squr * x;

    x_cube * x_squr + x
  };

  let z: u32 = {
    2 * x
  };

  println!("x is {:?}", x);
  println!("y is {:?}", y);
  println!("z is {:?}", z);
}

fn main() {
  let v = {
    let mut x = 3;
    x+=2;
    x
  };

  assert_eq!(v, 5);

  println!("{}", v);	
}

fn main() {
  let _s = sum(1, 3);

  assert_eq!(_s, ());

  println!("Success!");
}

fn sum (x: i32, y: i32) -> () {
  let _ = x + y;
}

fn main() {
  let (x, y): (char, char) = ('a', 'b');

  let s = sum(x, y);

  assert_eq!(s, ('a', 'b'));
  println!("Success!, {:?}", s);
}

fn sum(x: char, y: char) -> (char, char) {
  (x, y)
}

fn main() {
  let x: bool = true;

  let _y = match x {
    true => 1,
    false => {
      println!("Success!");
      panic!("We get panic!!");
    }
  };

  println!("Fuck!!");
}

fn main() {
  let s1 = String::from("hello");
  let s2 = s1.clone();

  println!("{}, {}", s1, s2);
}

fn main() {
  let _s1 = gives_ownership();

  let s2 = String::from("Hello");
  let _s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
  let some_string = String::from("hello");
  some_string
}

fn takes_and_gives_back(a_string: String) -> String {
  a_string
}

fn main() {
  let s1: String = String::from("Hello and fuck!!");
  let s2: String = takes_owernship(s1);

  println!("{}", s2);
}

fn takes_owernship(s: String) -> String {
  println!("{}", s);
  s
}

fn main() {
  let x: (i32, i32, (), &str) = (1, 2, (), "Hello");
  let y: (i32, i32, (), &str) = (1, 2, (), "Brother");

  println!("{:?}, {:?}", x, y);
}

fn main() {
  let s: String = String::from("Hello, ");
  let mut s1 = s;

  s1.push_str("Brother");
  println!("{:?}", s1);
}

fn main() {
  let x: Box<i32> = Box::new(5);
  let mut y: Box<i32> = Box::new(9);

  *y = 4;

  assert_eq!(*x, 5);
  println!("Succes!!");
}

fn main() {
  let s: (String, String) = (String::from("Hello"), String::from("Brother"));

  // let _y: (String, String) = (s.0, s.1);

  let _y : String = s.0;

  println!("{:?}", s.1);
}

fn main() {
  let x = 5;
  let p: &i32 = &x;

  println!("Memory address of x is {:p}", p)
}

fn main() {
  let x = 5;
  let p: &i32 = &x;

  assert_eq!(5, *p);
  println!("Suceess!!");
}

fn main() {
  let mut s: String = String::from("Hello");

  push_str(&mut s);

  println!("{}", s);
}

fn push_str(s: &mut String) {
  s.push_str(", World");
}

fn main() {
  let mut x: String = String::from("Hello");
  let p: &mut String = &mut x;

  p.push_str(", World");
  println!("{}", x);
}

fn main() {
  let c: char = 'a';
  let p: &char = &c;
  let ref y = c;

  assert_eq!(*p, *y);

  assert_eq!(get_addr(p), get_addr(y));

  print!("{:?}", *p);
}

fn get_addr(p: &char) -> String {
  format!("{:p}", p)
}

fn main() {
  let mut x: String = String::from("");
  x.push_str("Hello");
  x.push('!');

  println!("{}", x);
}

fn main() {
  let mut _x: String = String::from("Hello");

  let s1 = _x.replace("Hello", "Hi");

  assert_eq!(s1, "Hi");

  println!("{}", s1);
}

fn main() {
  let x: String = String::from("Hello");
  let mut y: String = String::from(" ,World");
  y.push_str("!!");

  let s: String = x + y.as_str();
  assert_eq!(s, "Hello ,World!!");

  println!("{}", s);
}

fn main() {
  let x : &str = "Hello";
  greeting(x.to_string());
}

fn greeting(s: String) {
  println!("{}", s);
}

fn main() {
  let x: String = String::from("Hello, World!!");
  let h: &str = &x[0..1];

  assert_eq!(h, "H");

  let h1: &str = &x[3..6];
  assert_eq!(h1, "W");

  let h2: &str = &x[..3];
  assert_eq!(h2, "Hel");

  let h3: &str = &x[..];

  println!("Success!!")
}

fn main() {
  let x: String = String::from("Hello, World!!");	
  let h: &str = 
}

fn main() {
  let arr: [i32; 5] = [1,2,3,4,5];

  assert!(arr.len() == 5);

  print!("This is right");
}

fn main() {
  let _x= [1, 2, 3, 4];
  let y: [_; 4] = ['a', 'b', 'c', 'd'];
  assert!(std::mem::size_of_val(&y) == 16);
  print!("Right");
}

fn main() {
  let list: [i32; 100] = [1; 100];

  assert!(list[0] == 1);
  assert!(list.len() == 100);

  print!("Rigt");
}

fn main(){
  let a: [char; 3] = ['a', 'b', 'c'];

  let b = a[0];

  assert!(b == 'a');	

  print!("Right");
}

fn main() {
  let a : [String; 2] = [String::from("Akash"), "Akash".to_string()];

  let name = a.get(0).unwrap();

  print!("{}", name);
}

fn main() {
  let arr : [char; 3] = ['a', 'b', 'c'];
  let slice: &[char] = &arr[..3];

  assert!(std::mem::size_of_val(&slice) == 16);

  print!("Right");
}

fn main() {
  let a: [i32; 5] = [1, 2, 3, 4, 5];
  let b: &[i32] = &a[1..4];

  assert_eq!(b, &[2, 3, 4]);
  print!("Right");
}

fn main() {
  let a: String = String::from("Akash");
  let b: &str = &a[0..4];
  let c: &str = &a[..4];

  assert_eq!(b, c);

  print!("{}", b);
}

fn main() {
  let s: String = String::from("Akash");
  let slice: &str = &s[0..4];

  assert!(slice == "Akas");
  print!("Right")
}

use std::io;

struct Student {
    name: String,
    age: u8,
    grade: String,
    gpa: f32,
}

fn main() {
    let mut name = String::new();
    let mut age = String::new();
    let mut grade = String::new();
    let mut gpa = String::new();

    println!("Enter student name:");
    io::stdin().read_line(&mut name).expect("Failed to read line");

    println!("Enter student age:");
    io::stdin().read_line(&mut age).expect("Failed to read line");

    println!("Enter student grade:");
    io::stdin().read_line(&mut grade).expect("Failed to read line");

    println!("Enter student GPA:");
    io::stdin().read_line(&mut gpa).expect("Failed to read line");

    let student = Student {
        name: name.trim().to_string(),
        age: age.trim().parse().expect("Please enter a number!"),
        grade: grade.trim().to_string(),
        gpa: gpa.trim().parse().expect("Please enter a number!"),
    };

    println!("Student Name: {}", student.name);
    println!("Student Age: {}", student.age);
    println!("Student Grade: {}", student.grade);
    println!("Student GPA: {}", student.gpa);
}

use std::io;    

struct Employee {
    name : String,
    doj : u16,
    salary : f32,
    experience: u8,
    skill : String
}

fn main() {
    let mut name = String::new();
    let mut doj = String::new();
    let mut salary = String::new();
    let mut experience = String::new();
    let mut skill = String::new();

    println!("Enter employee name:");
    io::stdin().read_line(&mut name).expect("Failed to read line");

    println!("Enter employee date of joining: ");
    io::stdin().read_line(&mut doj).expect("Failed to read line");

    println!("Enter employee salary: ");
    io::stdin().read_line(&mut salary).expect("Failed to read line");

    println!("Enter employee experience: ");
    io::stdin().read_line(&mut experience).expect("Failed to read line");

    println!("Enter employee skill: ");
    io::stdin().read_line(&mut skill).expect("Failed to read line");

    let employee = Employee {
        name: name.trim().to_string(),
        doj: doj.trim().parse().expect("Please enter a number!"),
        salary: salary.trim().parse().expect("Please enter a number!"),
        experience: experience.trim().parse().expect("Please enter a number!"),
        skill: skill.trim().to_string(),
    };

    println!("Employee Name: {}", employee.name);
    println!("Employee Date of Joining: {}", employee.doj);
    println!("Employee Salary: {}", employee.salary);
    println!("Employee Experience: {}", employee.experience);
    println!("Employee Skill: {}", employee.skill);
}

use std::io;
use chrono::NaiveDate;

struct Employee {
    name : String,
    doj : NaiveDate,
    salary : f32,
    experience: u8,
    skill : String
}

fn main() {
    let mut name = String::new();
    let mut doj = String::new();
    let mut salary = String::new();
    let mut experience = String::new();
    let mut skill = String::new();

    println!("Enter employee name:");
    io::stdin().read_line(&mut name).expect("Failed to read line");

    println!("Enter employee date of joining (YYYY-MM-DD): ");
    io::stdin().read_line(&mut doj).expect("Failed to read line");

    let doj = match NaiveDate::parse_from_str(doj.trim(), "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => {
            println!("The date is not in the correct format. Please use YYYY-MM-DD.");
            return;
        },
    };

    println!("Enter employee salary: ");
    io::stdin().read_line(&mut salary).expect("Failed to read line");

    println!("Enter employee experience: ");
    io::stdin().read_line(&mut experience).expect("Failed to read line");

    println!("Enter employee skill: ");
    io::stdin().read_line(&mut skill).expect("Failed to read line");

    let employee = Employee {
        name: name.trim().to_string(),
        doj: doj,
        salary: salary.trim().parse().expect("Please enter a number!"),
        experience: experience.trim().parse().expect("Please enter a number!"),
        skill: skill.trim().to_string(),
    };

    println!("Employee Name: {}", employee.name);
    println!("Employee Date of Joining: {}", employee.doj);
    println!("Employee Salary: {}", employee.salary);
    println!("Employee Experience: {}", employee.experience);
    println!("Employee Skill: {}", employee.skill);
}

use std::io;
use chrono::NaiveDate;

struct Employee {
    name : String,
    doj : NaiveDate,
    salary : f32,
    experience: u8,
    skill : String
}

fn main() {
    let mut name = String::new();
    let mut doj = String::new();
    let mut salary = String::new();
    let mut experience = String::new();
    let mut skill = String::new();

    println!("Enter employee name:");
    io::stdin().read_line(&mut name).expect("Failed to read line");

    let mut doj_parsed = None;
    while doj_parsed.is_none() {
        println!("Enter employee date of joining (YYYY-MM-DD): ");
        io::stdin().read_line(&mut doj).expect("Failed to read line");

        match NaiveDate::parse_from_str(doj.trim(), "%Y-%m-%d") {
            Ok(date) => doj_parsed = Some(date),
            Err(_) => println!("The date is not in the correct format. Please use YYYY-MM-DD."),
        };
        doj.clear();
    }

    println!("Enter employee salary: ");
    io::stdin().read_line(&mut salary).expect("Failed to read line");

    println!("Enter employee experience: ");
    io::stdin().read_line(&mut experience).expect("Failed to read line");

    println!("Enter employee skill: ");
    io::stdin().read_line(&mut skill).expect("Failed to read line");

    let employee = Employee {
        name: name.trim().to_string(),
        doj: doj_parsed.unwrap(),
        salary: salary.trim().parse().expect("Please enter a number!"),
        experience: experience.trim().parse().expect("Please enter a number!"),
        skill: skill.trim().to_string(),
    };

    println!("Employee Name: {}", employee.name);
    println!("Employee Date of Joining: {}", employee.doj);
    println!("Employee Salary: {}", employee.salary);
    println!("Employee Experience: {}", employee.experience);
    println!("Employee Skill: {}", employee.skill);
}
