use std::{i32::MAX, cmp::max};

fn main() {
    let input = include_str!("../input copy.txt");
    let mut open_brackets = 0;
    let mut grinch = false;
    let mut grinch_brackets = 0;
    let mut iter = input.chars().peekable();
    let mut depth = 0;
    let mut max_depth = 0;
    while let Some(c) = iter.next() {
        if grinch {
            if c == '(' {
                grinch_brackets += 1;
            } else if c == ')' {
                grinch_brackets -= 1;
            }
            if grinch_brackets == 0 {
                grinch = false;
            }
        }else if c == '(' {
            open_brackets +=1;
            depth+=1;
        } else if c == ')' {
            open_brackets -=1;
            if open_brackets == 0 {
                max_depth = max(depth, max_depth);
                depth = 0;
            }
        }
        else if c == 'G' {
            if iter.peek().unwrap() == &'r' {
                iter.next();
            }
            if iter.peek().unwrap() == &'i' {
                iter.next();
            }
            if iter.peek().unwrap() == &'n' {
                iter.next();
            }
            if iter.peek().unwrap() == &'c' {
                iter.next();
            }
            if iter.peek().unwrap() == &'h' {
                iter.next();
            }
            grinch = true;
        }
    }
    eprintln!("open_brackets = {:?}", open_brackets);
}
