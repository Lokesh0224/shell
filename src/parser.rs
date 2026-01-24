pub fn parse_input(input: &str) -> Vec<String> {
    let mut args =  Vec::new();
    let mut current = String::new();
    let mut in_single_quote = false;

    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next(){
        match c {
            '\'' =>{
                in_single_quote = !in_single_quote;
            }

            ' ' | '\t' if !in_single_quote => {
                
            }
        }
    }

    args
}