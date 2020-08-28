pub mod animal;

#[cfg(test)]
mod tests {
    use crate::animal::*;

    #[test]
    fn use_cat(){
        assert_eq!(true,Cat::is_cat());
    }

    #[test]
    fn use_dog(){
        assert_eq!(true,Dog::is_dog());
    }
}
