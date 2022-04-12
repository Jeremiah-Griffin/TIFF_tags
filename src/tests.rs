#[cfg(test)]

mod tests{
    use crate::get_name;

    #[test]
    fn return_some(){
      assert_eq!( get_name(254 as u16), Some(String::from("NewSubfileType")))
    }

    #[test]
    fn return_none(){
        assert_eq!(get_name(42069 as u16), None)
    }

}