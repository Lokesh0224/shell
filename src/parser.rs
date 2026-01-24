
pub fn parse_input(input: &str) -> Vec<String> {
    let mut args =  Vec::new();
    let mut current = String::new();
    let mut in_single_quote = false;

    //we're reading each char through chars()(interation)
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next(){
        match c {
            '\'' =>{
                in_single_quote = !in_single_quote;
            }

            ' ' | '\t' if !in_single_quote => {
                if !current.is_empty(){
                    args.push(current.clone());
                    current.clear();
                }

                //this checks after a space or tab there is another space or tab through peek(), if it is it passes through next() 
                while let Some(' ' | '\t') = chars.peek() {
                    chars.next();
                }

            }, 

            _ => {
                current.push(c);
            }
        }

    }
    if !current.is_empty() {
        args.push(current);
    }

    args
}