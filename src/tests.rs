#[cfg(test)]

mod tests{
    use crate::get_name;
    use crate::get_id;


    #[test]
    fn return_some_name(){
      assert_eq!( get_name(254 as u16), Some(String::from("NewSubfileType")))
    }

    #[test]
    fn return_none_name(){
        assert_eq!(get_name(42069 as u16), None)
    }

    #[test]
    fn return_some_id(){
        assert_eq!(get_id("NewSubfileType"), Some(254))

    }

    #[test]
    fn return_none_id(){
        assert_eq!(get_id("It Won't Be Long by Winston K. is a great song!"), None)
    }
}