fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[allow(dead_code)]
fn start_application(host: &str, port: u16) {
    if host == "localhost" {
        panic!("You cant use localhost as host");
    } else {
        println!("Starting application on {} {}", host, port);
    }
}

/* #[test]
fn test_simple() {
    println!("Hello Test");
}

#[test]
fn test_add() {
    let result = add(1, 2);
    assert_eq!(result, 3, "Should result 3");   
}

#[test]
#[should_panic]
fn test_start_application() {
    start_application("localhost", 80);
}

#[test]
#[ignore]
fn test_ignore() {
    println!("This test is ignored");
}

#[test]
fn test_add_again() -> Result<(), String> {
    let result = add(1, 2);

    if result == 3 {
        Ok(())
    } else {
        Err("1 + 2 should be 3".to_string())
    }
} */

#[cfg(test)]
mod tests {
    use crate::{add, start_application};
    
    #[test]
    fn test_simple() {
        println!("Hello Test");
    }

    #[test]
    fn test_add() {
        let result = add(1, 2);
        assert_eq!(result, 3, "Should result 3");
    }

    #[test]
    #[should_panic]
    fn test_start_application() {
        start_application("localhost", 80);
    }

    #[test]
    #[ignore]
    fn test_ignore() {
        println!("This test is ignored");
    }

    #[test]
    fn test_add_again() -> Result<(), String> {
        let result = add(1, 2);

        if result == 3 {
            Ok(())
        } else {
            Err("1 + 2 should be 3".to_string())
        }
    }
}