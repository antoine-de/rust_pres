fn small_computation(val: usize) -> Result<usize, String> {
    if val < 42 {
        println!("everything's fine for {}", val);
        Ok(val + 10)
    } else {
        Err(format!("I don't like {}", val))
    }
}

fn big_computation() -> Result<usize, String> {
    let res = small_computation(16);
    // use pattern matching to handle error
    let my_val = match res {
        Ok(val) => val,
        Err(e) => return Err(e)
    };
    assert_eq!(my_val, 26);
    
    let my_res = small_computation(my_val);
    // chain function call on the Result
    let my_res = my_res.map(|v| v + 7);
    assert_eq!(my_res, Ok(43));
    
    let another_value = small_computation(my_res?);
    assert_eq!(another_value, Err("I don't like 43".to_string()));
    another_value
}

fn main() -> Result<(), String> {
    // the `?` make an early return
    let _final_value = big_computation()?;
    println!("we'll never reach this line!");
    Ok(())
}

