
pub fn parse_input(input: &str) -> Vec<String> {
    let mut args =  Vec::new();
    let mut current = String::new();

    let mut in_single_quote = false;
    let mut in_double_quote = false;

    //we're reading each char through chars()(interation)
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next(){
        match c {
            '\\' if !in_single_quote && !in_double_quote =>{
                if let Some(c) = chars.next(){
                    current.push(c);
                }
            },

            //Backslashes in Double Quotes
            '\\' if in_double_quote =>{
                if let Some(&next) = chars.peek() {
                    match next {
                        '"' | '\\' | '$' | '`' => {
                            chars.next();
                            current.push(next);
                        }
                        _ => current.push('\\'),
                    }
                }
                else{
                    current.push('\\');
                }
            },

            // '\\' if in_double_quote => {
            //     if let Some(next) = chars.next() {
            //         match next {
            //             '"' | '\\' | '$' | '`' => current.push(next),
            //             _ => {
            //                 current.push('\\');
            //                 current.push(next);
            //             }
            //         }
            //     } else {
            //         current.push('\\');
            //     }
            //     //continue;
            // },


            '\'' if !in_double_quote =>{
                in_single_quote = !in_single_quote;
            }, 

            '"' if !in_single_quote => {
                in_double_quote = !in_double_quote;
            }

            ' ' | '\t' if !in_single_quote && !in_double_quote => {
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