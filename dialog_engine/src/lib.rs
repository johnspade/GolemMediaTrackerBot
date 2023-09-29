pub trait HasDialogMessage {
    fn message(&self) -> Option<String>;
}

pub fn validate_rating(text: &String) -> Result<u32, &str> {
    let rating = text.parse::<u32>();
    match rating {
        Ok(rating) => {
            if rating >= 1 && rating <= 5 {
                Ok(rating)
            } else {
                Err("Rating must be between 1 and 5")
            }
        }
        Err(_) => Err("Rating must be a number")
    }
}
