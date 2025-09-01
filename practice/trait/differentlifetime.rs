//The lifetime of return value always has to be tied to one of the parameters.

/* 
The lifetimes of arguments being passed in are called input lifetimes.
The lifetimes of returned values are called output lifetimes.

Lifetime Rules
1. Each parameter that is a reference gets its own lifetime parameter.
2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
3.If there are multiple input lifetime parameters, but one of them is &self or &mut self the lifetime of self is 
assigned to all output lifetime parameters.
*/

// All string literals have a static lifetime.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { //'a is generic lifetime    
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    //different lifetime
    let string1 = String::from("long string");

    {
    let string2 = String::from("short");
    let result = longest(&string1, &string2);
    println!("The longest string is: {}", result);
    }
}
