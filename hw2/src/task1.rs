/*
----> ЗАДАНИЕ 1 "Поиск слова в строке"

Вывести номер строки в котором встречается нужное слово и саму строку в формате:
2: строка...

 */

const SEARCH_TERM: &str = "picture";
const QUOTE: &str = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";
 
 
fn main() {
    find_term(SEARCH_TERM, QUOTE);
}
 
fn find_term(search_term: &str, quote: &str) -> String {
    let mut i = 1;
    let mut result_line = "";
    for line in quote.split('\n'){
        if line.contains(search_term){
            result_line=line;
            break;
        }
        i += 1;
    }
    String::from(format!("{}: {}", i, result_line))
}
 
// ----> TESTS
#[cfg(test)]
mod tests {
    use crate::find_term;
    use crate::{SEARCH_TERM, QUOTE};

    #[test]
    fn correct_line() {
        let answer = find_term(SEARCH_TERM, QUOTE);

        assert_eq!("2: dark square is a picture feverishly turned--in search of what?", answer)
    }
}