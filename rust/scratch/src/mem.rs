fn drop<T>(_value: T) {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_drops() {
        let a = String::from("droppable");

        drop(a);

        // println!("{}", a); // Not possible because value is moved inside drop function
    }
}
