use std::fmt::Debug;
use std::io::Read;
use std::str::FromStr;

pub fn read_string_from_stdin() -> String {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    buf
}

pub fn read_parsed_stdin<T>() -> Box<[T]>
where
    T: FromStr,
    T::Err: Debug,
{
    read_string_from_stdin()
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<T>())
        .collect::<Result<_, _>>()
        .map(Vec::into_boxed_slice)
        .unwrap()
}
